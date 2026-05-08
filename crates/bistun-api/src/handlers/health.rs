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

//! # Health & Telemetry Handler
//! Ref: [007-LMS-OPS]
//! Location: `crates/bistun-api/src/handlers/health.rs`
//!
//! **Why**: Exposes the operational visibility of the [`LinguisticManager`] to external load balancers and orchestrators.
//! **Impact**: If this handler fails or returns false positives, Kubernetes may route traffic to an engine that hasn't finished hydrating its WORM payload, resulting in dropped requests.
//!
//! ### Glossary
//! * **Probe**: A diagnostic request used by orchestrators to determine if a container is alive and ready to serve traffic.

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bistun_core::ops::{SdkState, SyncMetrics};
use bistun_lms::LinguisticManager;
use serde::Serialize;

/// The structured payload returned by the health probe.
#[derive(Serialize)]
pub struct HealthResponse {
    /// The current operational health state (e.g., Ready, Degraded).
    pub status: SdkState,
    /// Statistics regarding background synchronization and hot-swaps.
    pub metrics: SyncMetrics,
}

/// Evaluates the operational readiness of the capability engine.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Request the current [`SdkState`] and [`SyncMetrics`] from the injected manager instance.
/// 2. Evaluate the status: if [`SdkState::Ready`], assign `200 OK`.
/// 3. If the status is `Bootstrapping` or `Degraded`, assign `503 Service Unavailable` to signal unreadability.
/// 4. Serialize the telemetry data into a JSON response body and return.
///
/// # Examples
/// ```text
/// // Invoked by infrastructure agents: GET /health
/// ```
///
/// # Arguments
/// * `manager` ([`State<LinguisticManager>`]): The active capability engine injected via Axum state.
///
/// # Returns
/// * `impl IntoResponse`: An HTTP response containing the status code and serialized telemetry JSON.
///
/// # Golden I/O
/// * **Input**: `GET /health`
/// * **Output**: `200 OK | {"status": "Ready", "metrics": { ... }}`
///
/// # Errors, Panics, & Safety
/// * **Errors**: None. This endpoint returns health status even in degraded modes.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution using non-blocking [`std::sync::RwLock`] reads.
pub async fn health_handler(State(manager): State<LinguisticManager>) -> impl IntoResponse {
    // [STEP 1]: Capture current engine telemetry
    let health = HealthResponse { status: manager.status(), metrics: manager.metrics() };

    // [STEP 2 & 3]: Map readiness to HTTP semantics
    let code = match health.status {
        SdkState::Ready => StatusCode::OK,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    };

    // [STEP 4]: Return response
    (code, Json(health)).into_response()
}
