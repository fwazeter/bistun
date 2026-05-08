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

//! # The Sidecar Microservice Bootstrapper
//! Ref: [001-LMS-CORE], [007-LMS-OPS], [010-LMS-MEM]
//! Location: `crates/bistun-api/src/main.rs`
//!
//! **Why**: This module serves as the authoritative entry point for the Bistun API sidecar, orchestrating environment ingestion, engine hydration, and server execution. It transforms raw configuration into a production-ready Atomic Capability Provider.
//! **Impact**: If this bootstrapper fails or is misconfigured, the system cannot resolve linguistic traits, resulting in catastrophic rendering and NLP failures for downstream consumers.
//!
//! ### Glossary
//! * **Bootstrapping**: The sequential process of loading configurations and initializing dependencies before a service accepts traffic.
//! * **Hydration**: The process of inflating the in-memory registry Flyweight pool from a static WORM snapshot.
//! * **Sidecar**: A deployment pattern where an auxiliary service provides specialized capabilities to a primary application.

use bistun_api::config::AppConfig;
use bistun_api::routes;
use bistun_core::ops::SdkState;
use bistun_lms::LinguisticManager;
use tracing::{error, info};

/// Initializes the runtime environment and starts the asynchronous API server.
///
/// Time: O(M) where M is registry size | Space: O(M) for the in-memory Flyweight pool.
///
/// # Logic Trace (Internal)
/// 1. **Initialize Observability**: Spin up the `tracing-subscriber` to enable structured log events per 007-LMS-OPS.
/// 2. **Ingest Environment**: Load `AppConfig` from system variables and `.env` files, ensuring the Curator Public Key is present.
/// 3. **Instantiate Orchestrator**: Create a new `LinguisticManager` instance to manage the thread-safe registry state.
/// 4. **Dynamic Hydration Routing**: Evaluate the `lms_registry_url` to select between `HttpSnapshotProvider` or `FileSnapshotProvider`.
/// 5. **Security Gate**: Execute the initial registry hydration, mathematically verifying the JWS signature using Ed25519.
/// 6. **Assemble Router**: Construct the Axum application tree and inject the hydrated manager as shared state.
/// 7. **Commence Serving**: Bind to the network interface and enter the asynchronous polling loop to handle BCP 47 resolution requests.
///
/// # Examples
/// ```text
/// // Execution via CLI
/// $ cargo run --bin bistun-api
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `()`: This function runs indefinitely until a process signal (SIGTERM/SIGINT) is received.
///
/// # Golden I/O
/// * **Input**: Valid `.env` with `CURATOR_PUBLIC_KEY`.
/// * **Output**: Process listening on `0.0.0.0:8080`.
///
/// # Errors, Panics, & Safety
/// * **Panics**:
///     * Panics if `CURATOR_PUBLIC_KEY` is missing from the environment.
///     * Panics if a registry URL is provided but the `network` feature is disabled.
///     * Panics if the TCP port `8080` is already in use.
/// * **Safety**: Safe asynchronous entry point using the Tokio multithreaded runtime.
#[tokio::main]
async fn main() {
    // [STEP 1]: Initialize Observability
    tracing_subscriber::fmt::init();
    info!("🚀 Bistun LMS Sidecar Bootstrapping...");

    // [STEP 2]: Ingest Environment
    let config = AppConfig::load();

    // [STEP 3]: Instantiate Orchestrator
    let manager = LinguisticManager::new();

    // [STEP 4 & 5]: Dynamic Hydration Routing & Security Gate
    if let Some(url) = &config.lms_registry_url {
        #[cfg(feature = "network")]
        {
            info!("📡 Network Hydration selected. Fetching from: {}", url);
            let provider = bistun_lms::data::providers::HttpSnapshotProvider::new(url.clone());
            manager.initialize(&provider, &config.curator_public_key).await;
        }
        #[cfg(not(feature = "network"))]
        panic!(
            "CRITICAL: LMS_REGISTRY_URL ({}) is set, but the 'network' feature is disabled!",
            url
        );
    } else {
        #[cfg(feature = "fs")]
        {
            info!("💾 Local Disk Hydration selected. Fetching from data/ directory...");
            let provider = bistun_lms::data::providers::FileSnapshotProvider::new(
                "data/snapshot.json".to_string(),
                "data/snapshot.sig".to_string(),
            );
            manager.initialize(&provider, &config.curator_public_key).await;
        }
        #[cfg(not(feature = "fs"))]
        panic!("CRITICAL: No LMS_REGISTRY_URL provided, and the 'fs' feature is disabled!");
    }

    // Validation check for registry state
    if manager.status() == SdkState::Ready {
        info!("✅ Registry Hydrated & Cryptographically Verified. Engine is Ready.");
    } else {
        error!("⚠️ Registry Hydration Failed. Check snapshot/URL and CURATOR_PUBLIC_KEY!");
    }

    // [STEP 6]: Assemble Router
    let app = routes::app_router(manager);

    // [STEP 7]: Commence Serving
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("🎧 Service listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
