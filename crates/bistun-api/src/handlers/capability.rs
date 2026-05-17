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

//! # Capability Resolution Handler
//! Crate: `bistun-api`
//! Ref: [001-LMS-CORE], [012-LMS-ENG]
//! Location: `crates/bistun-api/src/handlers/capability.rs`
//!
//! **Why**: This is the primary ingress point for the microservice. It maps raw HTTP requests to the capability engine's sub-microsecond resolution pipeline.
//! **Impact**: This is the hot-path. If this handler introduces heavy allocations or blocking I/O, the entire system breaches its `< 1ms` latency budget.
//!
//! ### Glossary
//! * **Hot-Path**: The execution sequence that processes the highest volume of critical user requests.

use crate::error::AppError;
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use bistun_lms::LinguisticManager;
use tracing::{Instrument, debug};

/// Routes a BCP 47 request into the capability engine and yields the manifest.
///
/// Time: $O(N)$ where N is tag length | Space: $O(1)$ beyond JSON serialization
///
/// # Logic Trace (Internal)
/// 1. Extract the `locale` string from the URL path via the Axum extractor.
/// 2. Delegate the tag to the [`LinguisticManager`] for the 5-Phase resolution pipeline.
/// 3. If successful, wrap the immutable [`bistun_core::CapabilityManifest`] in a `200 OK` JSON response.
/// 4. If an error occurs, the `?` operator triggers an automatic conversion to [`AppError`] for HTTP mapping.
///
/// # Examples
/// ```text
/// // Typically invoked via: GET /v1/manifest/th-TH
/// ```
///
/// # Arguments
/// * `manager` ([`State<LinguisticManager>`]): The thread-safe capability engine injected via Axum application state.
/// * `headers` ([`HeaderMap`]): The inbound HTTP header map containing distributed tracking metadata.
/// * `locale` ([`Path<String>`]): The raw BCP 47 language tag requested by the client, extracted from the URL.
///
/// # Returns
/// * `Result<impl IntoResponse, AppError>`: Yields a serialized JSON manifest on success or a mapped HTTP error code on failure.
///
/// # Golden I/O
/// * **Input**: `GET /v1/manifest/ar-EG`
/// * **Output**: `200 OK | {"resolved_locale": "ar-EG", "traits": {...}}`
///
/// # Errors
/// * Returns [`AppError`] containing `BAD_REQUEST` for malformed language tags or `NOT_FOUND` if the resolution chain is fully exhausted.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Fully safe asynchronous execution; the core manager leverages lock-free memory pools.
///
/// # Side Effects
/// * Records an asynchronous tracing span to the structural telemetry observability sink per **007-LMS-OPS**.
#[tracing::instrument(level = "info", skip(manager, headers), fields(tag = %locale))]
pub async fn resolve_handler(
    State(manager): State<LinguisticManager>,
    headers: HeaderMap,
    Path(locale): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Extract distributed trace tokens if injected by parent API Gateways
    let trace_parent =
        headers.get("traceparent").and_then(|v| v.to_str().ok()).unwrap_or("00-detached-span-00");

    // Enforce span context binding across microservice boundaries using fully qualified macros
    let span = tracing::info_span!("5_phase_resolution_pipeline", edge_trace = %trace_parent);

    async move {
        debug!("Processing resolution request for tag: {}", locale);

        // [STEP 1 & 2]: Request resolution from the engine orchestrator
        let manifest = manager.resolve_capabilities(&locale)?;

        // [STEP 3]: Yield success response
        Ok((StatusCode::OK, Json(manifest)))
    }
    .instrument(span)
    .await
}
