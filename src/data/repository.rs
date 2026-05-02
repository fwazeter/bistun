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
//! Ref: [002-LMS-DATA]
//! Location: `src/data/repository.rs`
//!
//! **Why**: This module compiles raw snapshot data into a highly optimized `RegistryStore` memory pool in the background, isolating heavy I/O from the critical execution path.
//! **Impact**: If this module fails, the service boots with an empty database or cannot process updates, rendering the capability engine inert.
//!
//! ### Glossary
//! * **WORM**: Write-Once, Read-Many. The paradigm where registry snapshots are immutable once compiled.
//! * **Hydration**: The process of reading static data and inflating it into operational memory structures.

use crate::core::resolver::bcp47::LmsError;
use crate::data::store::{LocaleProfile, RegistryStore};
use crate::security::verifier;

/// The hardcoded WORM snapshot fallback (used if snapshot.json is missing)
/// Expanded to include necessary test locales for hermetic verification.
/// Ref: [012-LMS-ENG], [003-LMS-VAL]
const SIMULATED_WORM_JSON: &str = r#"
[
  {
    "id": "en-US",
    "morph": "FUSIONAL",
    "base_seg": "SPACE",
    "alt_seg": null,
    "direction": "LTR",
    "has_bidi": false,
    "requires_shaping": false,
    "plurals": ["one", "other"]
  },
  {
    "id": "ar-EG",
    "morph": "TEMPLATIC",
    "base_seg": "SPACE",
    "alt_seg": null,
    "direction": "RTL",
    "has_bidi": true,
    "requires_shaping": true,
    "plurals": ["zero", "one", "two", "few", "many", "other"]
  },
  {
    "id": "he",
    "morph": "TEMPLATIC",
    "base_seg": "SPACE",
    "alt_seg": null,
    "direction": "RTL",
    "has_bidi": true,
    "requires_shaping": false,
    "plurals": ["one", "two", "many", "other"]
  },
  {
    "id": "id",
    "morph": "FUSIONAL",
    "base_seg": "SPACE",
    "alt_seg": null,
    "direction": "LTR",
    "has_bidi": false,
    "requires_shaping": false,
    "plurals": ["other"]
  },
  {
    "id": "nb",
    "morph": "FUSIONAL",
    "base_seg": "SPACE",
    "alt_seg": null,
    "direction": "LTR",
    "has_bidi": false,
    "requires_shaping": false,
    "plurals": ["one", "other"]
  },
  {
    "id": "zh-Hant",
    "morph": "ISOLATING",
    "base_seg": "CHARACTER",
    "alt_seg": null,
    "direction": "TTB",
    "has_bidi": false,
    "requires_shaping": false,
    "plurals": ["other"]
  },
  {
    "id": "zh-Hans",
    "morph": "ISOLATING",
    "base_seg": "CHARACTER",
    "alt_seg": null,
    "direction": "LTR",
    "has_bidi": false,
    "requires_shaping": false,
    "plurals": ["other"]
  },
  {
    "id": "th-TH",
    "morph": "ISOLATING",
    "base_seg": "DICTIONARY",
    "alt_seg": null,
    "direction": "LTR",
    "has_bidi": false,
    "requires_shaping": true,
    "plurals": ["other"]
  }
]
"#;
/// Interface for retrieving the WORM payload, enabling Dependency Inversion.
pub trait ISnapshotProvider: Send + Sync {
    /// Fetches the raw JSON payload and its cryptographic signature.
    ///
    /// # Returns
    /// * `Result<(String, String), LmsError>`: A tuple containing `(JSON_Payload, Signature)`.
    fn fetch_payload(&self) -> Result<(String, String), LmsError>;
}

/// A concrete implementation of the snapshot provider utilizing embedded seed data.
#[derive(Default)]
pub struct SimulatedSnapshotProvider;

impl SimulatedSnapshotProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ISnapshotProvider for SimulatedSnapshotProvider {
    fn fetch_payload(&self) -> Result<(String, String), LmsError> {
        Ok((SIMULATED_WORM_JSON.to_string(), "valid-lms-signature".to_string()))
    }
}

/// Hydrates a fresh `RegistryStore` from a dynamically injected provider.
///
/// Time: O(M) where M is the number of locales | Space: O(M) for the new map allocation
///
/// # Logic Trace (Internal)
/// 1. Fetch the raw payload and signature from the injected `ISnapshotProvider`.
/// 2. Verify the payload's cryptographic signature via the security module.
/// 3. Parse the JSON WORM payload into a vector of `LocaleProfile` objects.
/// 4. Create an isolated, fresh `RegistryStore` and populate it.
/// 5. Yield the hydrated store to be hot-swapped into the active state.
///
/// # Examples
/// ```rust
///   use bistun::data::repository::{hydrate_snapshot, SimulatedSnapshotProvider};
///   let provider = SimulatedSnapshotProvider::new();
///   let fresh_store = hydrate_snapshot(&provider).unwrap();
///   assert!(fresh_store.get_profile("th-TH").is_some());
/// ```
///
/// # Arguments
/// * `provider` (&impl ISnapshotProvider): The injected provider responsible for supplying the raw WORM payload and signature.
///
/// # Returns
/// * `Result<RegistryStore, LmsError>`: A fully hydrated memory pool ready for the hot-swap.
///
/// # Golden I/O
/// * **Input**: `&SimulatedSnapshotProvider`
/// * **Output**: `RegistryStore { pools: { "en-US": Arc<LocaleProfile>, ... } }`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns `LmsError::SecurityFault` if the signature is invalid or JSON parsing fails.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous background execution.
pub fn hydrate_snapshot(provider: &impl ISnapshotProvider) -> Result<RegistryStore, LmsError> {
    // [STEP 1]: Fetch Payload
    let (json_payload, signature) = provider.fetch_payload()?;

    // [STEP 2]: Security Gate [Ref: 006-LMS-SEC]
    verifier::verify_snapshot(&json_payload, &signature)?;

    // [STEP 3]: Deserialization
    let profiles: Vec<LocaleProfile> = serde_json::from_str(&json_payload)
        .map_err(|e| LmsError::SecurityFault(format!("Failed to parse WORM JSON: {}", e)))?;

    // [STEP 4]: Instantiation
    let mut store = RegistryStore::new();
    for profile in profiles {
        store.insert_stub(profile);
    }

    // [STEP 5]: Return
    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    // LMS-TEST: Hermetic Mocking of SnapshotProvider behavior
    mock! {
        pub SnapshotProvider {}
        impl ISnapshotProvider for SnapshotProvider {
            fn fetch_payload(&self) -> Result<(String, String), LmsError>;
        }
    }

    #[test]
    fn test_hydrate_snapshot_succeeds() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create a mock provider returning the golden JSON.
        let mut mock_provider = MockSnapshotProvider::new();
        mock_provider
            .expect_fetch_payload()
            .returning(|| Ok((SIMULATED_WORM_JSON.to_string(), "valid-lms-signature".to_string())));

        // [STEP 2]: Execute: Call the hydrator with our hermetic mock.
        let store = hydrate_snapshot(&mock_provider).expect("Hydration failed");

        // [STEP 3]: Assert: Verify the returned store is populated with expected golden stubs.
        assert!(store.get_profile("en-US").is_some(), "System Default must exist");
        assert!(store.get_profile("ar-EG").is_some());
        assert!(store.get_profile("th-TH").is_some());
        assert!(store.get_profile("zh-Hant").is_some());
        assert!(store.get_profile("invalid-locale").is_none());
    }
}
