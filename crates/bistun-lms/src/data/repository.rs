// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! # WORM Repository Hydration
//! Crate: `bistun-lms`
//! Ref: [002-LMS-DATA]
//! Location: `crates/bistun-lms/src/data/repository.rs`
//!
//! **Why**: This module compiles raw snapshot data into a highly optimized `RegistryStore` memory pool in the background, isolating heavy `I/O` from the critical execution path.
//! **Impact**: If this module fails, the service boots with an empty database or cannot process updates, rendering the capability engine inert.
//!
//! ### Architectural Note: Asynchronous Traits and Thread Safety
//! We explicitly avoid using the standard `async fn` syntax inside the `ISnapshotProvider` trait.
//! While Rust supports Async Functions in Traits (`AFIT`), the compiler cannot natively guarantee
//! that the returned `Future` is thread-safe (`Send`).
//!
//! Because the `LinguisticManager` passes these providers into detached `tokio::spawn` background
//! workers, the Tokio runtime strictly demands `Send` futures so tasks can move between `CPU` cores.
//! By returning a `Pin<Box<dyn Future + Send>>`, we manually enforce this thread-safety contract
//! without needing to pull in heavy third-party macros like the `async-trait` crate.
//!
//! ### Glossary
//! * **WORM**: Write-Once, Read-Many. The paradigm where registry snapshots are immutable once compiled.
//! * **Hydration**: The process of reading static data and inflating it into operational memory structures.

use crate::security::verifier;
use bistun_core::{LmsError, RegistryStore, WormPayload};
use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Type alias for the complex pinned future returned by providers to satisfy Clippy constraints.
///
/// Time: `O(1)` | Space: `O(1)`
pub type PayloadFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(String, String), LmsError>> + Send + 'a>>;

/// A lightweight envelope used exclusively for Pre-Crypto Header Validation.
/// By ignoring the massive data arrays of the full `WormPayload`, this allows
/// us to inspect the headers in sub-millisecond time.
#[derive(Debug, Deserialize)]
struct WormEnvelope {
    metadata: HashMap<String, String>,
}

/// Interface for retrieving the `WORM` payload, enabling Dependency Inversion.
pub trait ISnapshotProvider: Send + Sync {
    /// Fetches the raw `JSON` payload and its cryptographic signature.
    ///
    /// # Returns
    /// * `Result<(String, String), LmsError>`: A tuple containing `(JSON_Payload, Signature)`.
    fn fetch_payload(&self) -> PayloadFuture<'_>;
}

// ---------------------------------------------------------
// SIMULATION GATED BLOCK
// ---------------------------------------------------------
#[cfg(feature = "simulation")]
/// A concrete provider utilizing embedded seed data and a dynamically generated `Ed25519` signature.
pub struct SimulatedSnapshotProvider {
    /// The raw `JSON` payload representing the registry state.
    pub payload: String,
    /// The `Base64` encoded `Ed25519` signature of the payload.
    pub signature: String,
    /// The `Base64` encoded public key used for verification.
    pub public_key: String,
}

#[cfg(feature = "simulation")]
impl Default for SimulatedSnapshotProvider {
    /// Initialized with default simulated credentials.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "simulation")]
impl SimulatedSnapshotProvider {
    /// Generates a valid `Ed25519` cryptographic keypair and signs the simulated payload.
    ///
    /// Time: `O(1)` | Space: `O(M)` where M is payload size.
    #[must_use]
    pub fn new() -> Self {
        use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
        use bistun_core::SIMULATED_WORM_JSON;
        use ed25519_dalek::{Signer, SigningKey};
        use rand::rngs::OsRng;

        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        let public_key = BASE64.encode(signing_key.verifying_key().as_bytes());
        let signature = BASE64.encode(signing_key.sign(SIMULATED_WORM_JSON.as_bytes()).to_bytes());

        Self { payload: SIMULATED_WORM_JSON.to_string(), signature, public_key }
    }
}

#[cfg(feature = "simulation")]
impl ISnapshotProvider for SimulatedSnapshotProvider {
    /// Fetches the static simulated payload and signature.
    fn fetch_payload(&self) -> PayloadFuture<'_> {
        let p = self.payload.clone();
        let s = self.signature.clone();
        Box::pin(async move { Ok((p, s)) })
    }
}

/// Validates the mandatory metadata headers before executing heavy cryptography.
///
/// Time: O(1) relative to payload depth | Space: O(1) for header map
///
/// # Logic Trace (Internal)
/// 1. [Lightweight Parse]: Extract only the metadata map from the raw JSON string.
/// 2. [Schema Check]: Enforce the presence and validity of the `schema_version`.
/// 3. [Temporal Check]: Enforce the presence of the `build_date`.
/// 4. [Return]: Bubble up `SecurityFault` if the envelope is malformed or invalid.
fn validate_pre_crypto_headers(json_payload: &str) -> Result<(), LmsError> {
    let envelope: WormEnvelope =
        serde_json::from_str(json_payload).map_err(|e| LmsError::SecurityFault {
            pipeline_step: "Phase 0: WORM Hydration".to_string(),
            context: "Header Validation".to_string(),
            reason: format!("Payload lacks a valid metadata envelope: {e}"),
        })?;

    // Validate Schema Version
    let schema = envelope.metadata.get("version").ok_or_else(|| LmsError::SecurityFault {
        pipeline_step: "Phase 0: WORM Hydration".to_string(),
        context: "Header Validation".to_string(),
        reason: "Missing mandatory 'version' header".to_string(),
    })?;

    // Allow both v1 and v2 standard/simulated versions
    if !schema.starts_with("v1.")
        && !schema.starts_with("v2.")
        && schema != "1.0.0"
        && schema != "2.0.0"
    {
        return Err(LmsError::SecurityFault {
            pipeline_step: "Phase 0: WORM Hydration".to_string(),
            context: "Header Validation".to_string(),
            reason: format!("Unsupported registry version: {schema}"),
        });
    }

    // Validate Build Date
    if !envelope.metadata.contains_key("build_date") {
        return Err(LmsError::SecurityFault {
            pipeline_step: "Phase 0: WORM Hydration".to_string(),
            context: "Header Validation".to_string(),
            reason: "Missing mandatory 'build_date' header. Cannot prevent replay attacks."
                .to_string(),
        });
    }

    Ok(())
}

// -------------------------------------------------------------------

/// Hydrates a fresh `RegistryStore` from a dynamically injected provider.
///
/// Time: `O(M)` where M is the number of locales | Space: `O(M)` for map allocations
///
/// # Logic Trace (Internal)
/// 1. Fetch the raw payload and signature from the injected [`ISnapshotProvider`].
/// 2. Verify the payload's cryptographic signature via the security module.
/// 3. Parse the `JSON` `WORM` payload into the internal [`WormPayload`] `DTO`.
/// 4. Create an isolated, fresh [`RegistryStore`].
/// 5. Inject the parsed authoritative identity ([`bistun_core::RegistryMetadata`]) into the store.
/// 6. Inject the arrays of Flyweights and dynamic aliases into the store's maps.
/// 7. Yield the hydrated store to be hot-swapped into the active state.
///
/// # Examples
/// ```text
/// // See internal `tests` module for hermetic execution.
/// ```
///
/// # Arguments
/// * `provider` (&impl [`ISnapshotProvider`]): The provider responsible for supplying the raw `WORM` payload.
/// * `public_key_b64` (&str): The `Base64` encoded `Ed25519` public key of the Curator compiler.
///
/// # Returns
/// * `Result<RegistryStore, LmsError>`: A fully hydrated memory pool ready for the atomic hot-swap.
///
/// # Golden I/O
/// * **Input**: `&SimulatedSnapshotProvider`, `"Base64_Public_Key"`
/// * **Output**: `Ok(RegistryStore { ... })`
///
/// # Errors
/// * Returns [`LmsError::SecurityFault`] if the cryptographic signature is invalid or `JSON` parsing fails.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Safe asynchronous background execution.
pub async fn hydrate_snapshot(
    provider: &impl ISnapshotProvider,
    public_key_b64: &str, // Security Pin injection
) -> Result<RegistryStore, LmsError> {
    // [STEP 1]: Fetch Payload
    let (json_payload, signature) = provider.fetch_payload().await?;

    // =========================================================================
    // NEW HARDENING: PHASE 1.1 (PRE-CRYPTO HEADER VALIDATION)
    // =========================================================================
    // [STEP 1.5]: Reject malformed payloads before burning CPU on Ed25519 math.
    validate_pre_crypto_headers(&json_payload)?;
    // =========================================================================

    // [STEP 2]: Security Gate [Ref: 006-LMS-SEC]
    verifier::verify_snapshot(&json_payload, &signature, public_key_b64)?;

    // [STEP 3]: Deserialization via structured DTO
    let payload: WormPayload =
        serde_json::from_str(&json_payload).map_err(|e| LmsError::SecurityFault {
            pipeline_step: "Phase 0: WORM Hydration".to_string(),
            context: "Deserialization".to_string(),
            reason: format!("Failed to parse WORM JSON: {e}"),
        })?;

    // [STEP 4]: Instantiation
    let mut store = RegistryStore::new();

    // [STEP 5]: Inject Identity
    store.set_metadata(payload.metadata);

    // [STEP 6]: Inject Flyweights and Aliases
    for profile in payload.profiles {
        store.insert_stub(profile);
    }
    for (alias, canonical) in payload.aliases {
        store.insert_alias(alias, canonical);
    }

    // [STEP 7]: Return
    Ok(store)
}

#[cfg(all(test, feature = "simulation"))]
mod tests {
    use super::*;
    use mockall::mock;

    // LMS-TEST: Hermetic Mocking of SnapshotProvider behavior
    mock! {
        pub SnapshotProvider {}
        impl ISnapshotProvider for SnapshotProvider {
           fn fetch_payload<'a>(&'a self) -> PayloadFuture<'_>;
        }
    }

    #[tokio::test]
    async fn test_hydrate_snapshot_succeeds() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create a dynamic simulated provider.
        let real_provider = SimulatedSnapshotProvider::new();
        let p_clone = real_provider.payload.clone();
        let s_clone = real_provider.signature.clone();

        let mut mock_provider = MockSnapshotProvider::new();
        mock_provider.expect_fetch_payload().returning(move || {
            let p = p_clone.clone();
            let s = s_clone.clone();
            Box::pin(async move { Ok((p, s)) })
        });

        // [STEP 2]: Execute: Call the hydrator with our hermetic mock.
        let store = hydrate_snapshot(&mock_provider, &real_provider.public_key)
            .await
            .expect("LMS-TEST: Hydration failed");

        // [STEP 3]: Assert: Verify the returned store is populated.
        assert!(store.get_profile("en-US").is_some(), "System Default must exist");
        assert!(store.get_profile("ar-EG").is_some());
        assert!(store.get_profile("th-TH").is_some());
        assert!(store.get_profile("zh-Hant").is_some());

        // Assert Aliases parsed successfully
        assert_eq!(store.resolve_alias("in"), Some("id".to_string()));
        assert_eq!(store.resolve_alias("zh-TW"), Some("zh-Hant".to_string()));
    }
}
