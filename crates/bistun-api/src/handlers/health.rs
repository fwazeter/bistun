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
//! Crate: `bistun-api`
//! Ref: [007-LMS-OPS]
//! Location: `crates/bistun-api/src/handlers/health.rs`
//!
//! **Why**: Exposes the operational visibility of the [`LinguisticManager`] to external load balancers and orchestrators.
//! **Impact**: If this handler fails or returns false positives, Kubernetes may route traffic to an engine that hasn't finished hydrating its WORM payload, resulting in dropped requests.
//!
//! ### Glossary
//! * **Probe**: A diagnostic request used by orchestrators to determine if a container is alive and ready to serve traffic.

use axum::{
    Json,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
};
use bistun_core::ops::{SdkState, SyncMetrics};
use bistun_lms::LinguisticManager;
use serde::Serialize;

/// The structured payload returned by the health probe.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct HealthResponse {
    /// The current operational health state (e.g., Ready, Degraded).
    pub status: SdkState,
    /// Statistics regarding background synchronization and hot-swaps.
    pub metrics: SyncMetrics,
}

/// Evaluates the operational readiness of the capability engine.
///
/// Time: $O(1)$ | Space: $O(1)$
///
/// # Logic Trace (Internal)
/// 1. \[Ingestion\]: Request the current [`SdkState`] and [`SyncMetrics`] from the injected manager instance.
/// 2. \[Evaluation\]: Evaluate the status: if [`SdkState::Ready`], assign `200 OK`.
/// 3. \[Bypass\]: If the status is `Bootstrapping` or `Degraded`, assign `503 Service Unavailable` to signal unreadability.
/// 4. \[Return\]: Serialize the telemetry data into a JSON response body and return.
///
/// # Examples
/// ```rust
/// # use axum::extract::State;
/// # use bistun_lms::LinguisticManager;
/// # use bistun_api::handlers::health::health_handler;
/// # async fn run_probe() {
/// # let manager = LinguisticManager::new();
/// let response = health_handler(State(manager)).await;
/// # }
/// ```
///
/// # Arguments
/// * `manager` (State<[`LinguisticManager`]>): The active capability engine injected via Axum state.
///
/// # Returns
/// * `impl IntoResponse`: An HTTP response containing the status code and serialized telemetry JSON.
///
/// # Golden I/O
/// * **Input**: `GET /health`
/// * **Output**: `200 OK | {"status": "Ready", "metrics": { ... }}`
///
/// # Errors
/// * None. This endpoint returns health status dynamically even in degraded modes.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Fully safe synchronous execution using non-blocking [`std::sync::RwLock`] reads.
///
/// # Side Effects
/// * Performs memory lock-reads on global operational state fields.
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

/// Exports operational telemetry in Prometheus plain-text format.
///
/// Time: $O(1)$ | Space: $O(1)$
///
/// # Logic Trace (Internal)
/// 1. \[Ingestion\]: Request synchronization and resolution metrics from the manager.
/// 2. \[Formatting\]: Format the atomic counters into official Prometheus `# HELP` and `# TYPE` text structures.
/// 3. \[Return\]: Return the mapped string with the mandatory `text/plain; version=0.0.4` header.
///
/// # Arguments
/// * `manager` ([`State<LinguisticManager>`]): The active capability engine.
///
/// # Returns
/// * `impl IntoResponse`: A plain-text Prometheus payload.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous read of atomic metrics.
///
/// # Side Effects
/// * Exposes internal processing volumes to network scrapers.
pub async fn metrics_handler(State(manager): State<LinguisticManager>) -> impl IntoResponse {
    let sync = manager.metrics();
    let res = manager.resolution_metrics();

    let metrics_text = format!(
        "# HELP lms_sync_errors_total Total WORM background hydration errors.\n\
         # TYPE lms_sync_errors_total counter\n\
         lms_sync_errors_total {sync_err}\n\
         \n\
         # HELP lms_last_successful_sync_timestamp_seconds Unix timestamp of the last successful memory hot-swap.\n\
         # TYPE lms_last_successful_sync_timestamp_seconds gauge\n\
         lms_last_successful_sync_timestamp_seconds {sync_success}\n\
         \n\
         # HELP lms_manifests_resolved_total Total BCP 47 capability resolutions processed.\n\
         # TYPE lms_manifests_resolved_total counter\n\
         lms_manifests_resolved_total {res_total}\n",
        sync_err = sync.sync_error_count,
        sync_success = sync.last_successful_sync,
        res_total = res.total_manifests_resolved,
    );

    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], metrics_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bistun_lms::data::repository::SimulatedSnapshotProvider;

    #[tokio::test]
    async fn test_health_handler_returns_503_during_bootstrapping() {
        // [STEP 1]: Setup - Initialize a clean manager instance.
        let manager = LinguisticManager::new();

        // [STEP 2]: Execute - Invoke the handler while it's bootstrapping.
        let response = health_handler(State(manager)).await.into_response();

        // [STEP 3]: Assert - Ensure response indicates Service Unavailable.
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_health_handler_returns_200_when_ready() {
        // [STEP 1]: Setup - Initialize and fully hydrate the manager with simulated golden records.
        let manager = LinguisticManager::new();
        let provider = SimulatedSnapshotProvider::new();
        manager.initialize(&provider, &provider.public_key).await;

        // [STEP 2]: Execute - Invoke the handler.
        let response = health_handler(State(manager)).await.into_response();

        // [STEP 3]: Assert - Ensure response returns 200 OK.
        assert_eq!(response.status(), StatusCode::OK);
    }
}
