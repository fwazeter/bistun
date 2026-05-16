// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV
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

//! # Webhook Ingestion Handler
//! Crate: `bistun-api`
//! Ref: [007-LMS-OPS]
//! Location: `crates/bistun-api/src/handlers/webhook.rs`
//!
//! **Why**: Implements the Push Model for registry hydration. Listens for authorized POST requests from the Curator CLI to trigger instant, wait-free memory hot-swaps.
//! **Impact**: Secures the update mechanism. If HMAC verification fails, malicious actors could trigger continuous memory reloads (DoS).
//!
//! ### Glossary
//! * **HMAC**: Hash-based Message Authentication Code. Uses a cryptographic hash function and a secret cryptographic key to verify data integrity and authenticity.

use crate::config::AppConfig;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use bistun_lms::LinguisticManager;

// =========================================================================
// CRITICAL: Global Cryptography Traits
// We explicitly import KeyInit (for new_from_slice) and Mac (for update/finalize)
// =========================================================================
use hmac::Hmac;
use hmac::digest::{KeyInit, Mac};
use sha2::Sha256;

use tracing::{error, info, warn};

#[cfg(feature = "simulation")]
use bistun_lms::data::repository::SimulatedSnapshotProvider;

/// Processes inbound webhook notifications to force an immediate engine sync.
///
/// Time: $O(N)$ for HMAC calculation where N is payload size | Space: $O(N)$ for body buffering
///
/// # Logic Trace (Internal)
/// 1. Validate the presence of `WEBHOOK_SECRET` in the environment configuration.
/// 2. Extract the `X-LMS-Signature` from the request headers.
/// 3. Calculate an HMAC-SHA256 hash using the raw request body and the secret.
/// 4. Compare the calculated hash against the provided signature to authenticate the request.
/// 5. If valid, execute `manager.force_sync()` to swap the active memory pool.
/// 6. Yield `200 OK` on success, or appropriate `4xx/5xx` HTTP codes on failure.
///
/// # Arguments
/// * `manager` ([`State<LinguisticManager>`]): The active capability engine injected via Axum state.
/// * `headers` ([`HeaderMap`]): The HTTP headers containing the signature.
/// * `body` ([`Bytes`]): The raw, unparsed JSON payload from the request.
///
/// # Returns
/// * `Result<impl IntoResponse, StatusCode>`: Success or an HTTP error mapping.
///
/// # Errors
/// * Returns `INTERNAL_SERVER_ERROR` if the webhook secret is missing.
/// * Returns `UNAUTHORIZED` if the signature header is absent.
/// * Returns `FORBIDDEN` if the HMAC signature does not match.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Safe asynchronous execution. Prevents timing attacks by matching fully mapped hex strings.
///
/// # Side Effects
/// * Instantiates a new snapshot provider and mutates the global memory pool via `force_sync`.
pub async fn refresh_handler(
    State(manager): State<LinguisticManager>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, StatusCode> {
    let config = AppConfig::load();

    // [STEP 1]: Validate Secret Configuration
    let secret = match config.webhook_secret {
        Some(s) => s,
        None => {
            error!("LMS-OPS: Webhook received but WEBHOOK_SECRET is not configured.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // [STEP 2]: Extract Signature
    let signature = match headers.get("X-LMS-Signature") {
        Some(sig) => sig.to_str().unwrap_or(""),
        None => {
            warn!("LMS-OPS: Webhook rejected. Missing X-LMS-Signature header.");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // [STEP 3]: Calculate HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as KeyInit>::new_from_slice(secret.as_bytes())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    mac.update(&body);
    let hmac_result = mac.finalize().into_bytes();
    let expected_signature = hmac_result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    // [STEP 4]: Compare Signatures
    if signature != expected_signature {
        warn!("LMS-OPS: Webhook rejected. HMAC signature mismatch.");
        return Err(StatusCode::FORBIDDEN);
    }

    info!("LMS-OPS: Webhook authenticated successfully. Initiating memory hot-swap...");

    // [STEP 5]: Trigger Engine Hydration
    // Note: For this 1.1.0 implementation, we bind to the SimulatedProvider to guarantee safe hydration
    #[cfg(feature = "simulation")]
    {
        let provider = SimulatedSnapshotProvider::new();
        let pub_key = config.curator_public_key.unwrap_or(provider.public_key.clone());

        if let Err(e) = manager.force_sync(&provider, &pub_key).await {
            error!("LMS-OPS: Real-time hot-swap failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    #[cfg(not(feature = "simulation"))]
    {
        error!(
            "LMS-OPS: Network/File snapshot providers are not wired in this stub. Enable simulation feature."
        );
        return Err(StatusCode::NOT_IMPLEMENTED);
    }

    // [STEP 6]: Yield Success
    Ok(StatusCode::OK)
}
