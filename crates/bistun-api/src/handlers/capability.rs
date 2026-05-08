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
    http::StatusCode,
    response::IntoResponse,
};
use bistun_lms::LinguisticManager;
use tracing::{debug, instrument};

/// Routes a BCP 47 request into the capability engine and yields the manifest.
///
/// Time: O(N) where N is tag length | Space: O(1) beyond JSON serialization
///
/// # Logic Trace (Internal)
/// 1. Extract the `locale` string from the URL path via the Axum extractor.
/// 2. Delegate the tag to the [`LinguisticManager`] for the 5-Phase resolution pipeline.
/// 3. If successful, wrap the immutable [`CapabilityManifest`] in a `200 OK` JSON response.
/// 4. If an error occurs, the `?` operator triggers an automatic conversion to [`AppError`] for HTTP mapping.
///
/// # Examples
/// ```text
/// // Typically invoked via: GET /v1/manifest/th-TH
/// ```
///
/// # Arguments
/// * `manager` ([`State<LinguisticManager>`]): The thread-safe capability engine injected via Axum application state.
/// * `locale` ([`Path<String>`]): The raw BCP 47 language tag requested by the client, extracted from the URL.
///
/// # Returns
/// * `Result<impl IntoResponse, AppError>`: Yields a serialized JSON manifest on success or a mapped HTTP error code on failure.
///
/// # Golden I/O
/// * **Input**: `GET /v1/manifest/ar-EG`
/// * **Output**: `200 OK | {"resolved_locale": "ar-EG", "traits": {...}}`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns `BAD_REQUEST` for invalid tags or `NOT_FOUND` if resolution exhausts the fallback chain.
/// * **Panics**: None.
/// * **Safety**: Safe asynchronous execution; the manager uses wait-free reads.
///
/// # Side Effects
/// * Records a tracing span to the system observability sink per **007-LMS-OPS**.
#[instrument(level = "info", skip(manager), fields(tag = %locale))]
pub async fn resolve_handler(
    State(manager): State<LinguisticManager>,
    Path(locale): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    debug!("Ingressing resolution request for tag: {}", locale);

    // [STEP 1 & 2]: Request resolution from the engine orchestrator
    let manifest = manager.resolve_capabilities(&locale)?;

    // [STEP 3]: Yield success response
    Ok((StatusCode::OK, Json(manifest)))
}
