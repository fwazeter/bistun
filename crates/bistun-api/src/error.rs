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

//! # API Error Mapping
//! Ref: [LMS-PROCESS-ERROR]
//! Location: `crates/bistun-api/src/error.rs`
//!
//! **Why**: This module translates internal capability engine failures ([`LmsError`]) into standardized, client-friendly HTTP responses.
//! **Impact**: If this mapping fails or is bypassed, API consumers will receive opaque 500 Internal Server Errors instead of actionable 4xx diagnostics, breaking client-side error handling and SLI monitoring.
//!
//! ### Glossary
//! * **AppError**: A NewType wrapper allowing the implementation of Axum's [`IntoResponse`] trait on external error types.
//! * **NewType Pattern**: A Rust design pattern used to bypass the orphan rule by wrapping an external type in a local tuple struct.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bistun_core::LmsError;
use serde_json::json;

/// A wrapper around internal errors to provide standard HTTP responses.
///
/// Time: O(1) | Space: O(1)
pub struct AppError(pub LmsError);

impl From<LmsError> for AppError {
    /// Converts a [`LmsError`] into an [`AppError`] for API response handling.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Ingest the internal [`LmsError`] instance.
    /// 2. Wrap the instance in the [`AppError`] NewType container to enable trait implementation.
    /// 3. Return the wrapped instance for use in route handlers.
    ///
    /// # Examples
    /// ```rust
    /// # use bistun_core::LmsError;
    /// # use bistun_api::error::AppError;
    /// let lms_err = LmsError::InvalidTag {
    ///     tag: "invalid".to_string(),
    ///     pipeline_step: "Resolve".to_string(),
    ///     reason: "Bad subtag".to_string()
    /// };
    /// let app_err = AppError::from(lms_err);
    /// ```
    ///
    /// # Arguments
    /// * `inner` ([`LmsError`]): The authoritative engine error variant to be mapped.
    ///
    /// # Returns
    /// * `Self`: An [`AppError`] instance capable of generating an HTTP response.
    ///
    /// # Golden I/O
    /// * **Input**: `LmsError::InvalidTag`
    /// * **Output**: `AppError(LmsError::InvalidTag)`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None; this conversion is infallible.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous execution.
    fn from(inner: LmsError) -> Self {
        // [STEP 1 & 2]: Wrap the internal error
        Self(inner)
    }
}

impl IntoResponse for AppError {
    /// Translates the internal [`LmsError`] into a standardized JSON HTTP Response.
    ///
    /// Time: O(1) | Space: O(1) beyond JSON serialization allocations.
    ///
    /// # Logic Trace (Internal)
    /// 1. Match the internal [`LmsError`] variant to determine the semantic HTTP [`StatusCode`].
    /// 2. Assign a machine-readable error category string (e.g., "INVALID_TAG").
    /// 3. Construct a standardized JSON payload containing the error category and the engine's failure narrative.
    /// 4. Yield the finalized tuple of `(StatusCode, Json)` to the Axum runtime for transport.
    ///
    /// # Examples
    /// ```text
    /// // Used internally by Axum via the '?' operator in handlers:
    /// let manifest = manager.resolve_capabilities(tag).map_err(AppError::from)?;
    /// ```
    ///
    /// # Arguments
    /// * `self` (receiver): The wrapped [`LmsError`] to be transformed.
    ///
    /// # Returns
    /// * [`Response`]: An Axum-compatible response containing the status code and error DTO.
    ///
    /// # Golden I/O
    /// * **Input**: `LmsError::InvalidTag { ... }`
    /// * **Output**: `400 Bad Request | {"error": "INVALID_TAG", "message": "..."}`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None; the function provides a 500 fallback for unclassified variants.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous execution.
    fn into_response(self) -> Response {
        // [STEP 1 & 2]: Map internal variants to HTTP semantics
        let (status, error_type) = match &self.0 {
            LmsError::InvalidTag { .. } => (StatusCode::BAD_REQUEST, "INVALID_TAG"),
            LmsError::ResolutionFailed { .. } => (StatusCode::NOT_FOUND, "LOCALE_NOT_FOUND"),
            LmsError::SecurityFault { .. } => (StatusCode::FORBIDDEN, "SECURITY_FAULT"),
            LmsError::IntegrityViolation { .. } => (StatusCode::CONFLICT, "INTEGRITY_VIOLATION"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        // [STEP 3]: Construct the JSON DTO
        let body = Json(json!({
            "error": error_type,
            "message": self.0.to_string(),
        }));

        // [STEP 4]: Finalize response
        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use bistun_core::LmsError;

    #[test]
    fn test_maps_invalid_tag_to_400_bad_request() {
        // [1] Set up Mock Error: Exactly matching bistun-core definition
        let internal_err = LmsError::InvalidTag {
            tag: "ar@EG".to_string(),
            pipeline_step: "Phase 1: Resolve".to_string(),
            reason: "Illegal character '@' in subtag".to_string(),
        };
        let app_err = AppError::from(internal_err);

        // [2] Execute
        let response = app_err.into_response();

        // [3] Assert Equation
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_maps_resolution_failure_to_404_not_found() {
        // [1] Set up Mock Error using a Golden Input
        let internal_err = LmsError::ResolutionFailed {
            tag: "th-TH".to_string(),
            pipeline_step: "Phase 1: Resolve".to_string(),
            reason: "Tag not found in WORM registry".to_string(),
        };
        let app_err = AppError::from(internal_err);

        // [2] Execute
        let response = app_err.into_response();

        // [3] Assert Equation
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_maps_security_fault_to_403_forbidden() {
        // [1] Set up Mock Error with required 'reason' field
        let internal_err = LmsError::SecurityFault {
            pipeline_step: "Hydration".to_string(),
            context: "Registry Signature Verification".to_string(),
            reason: "Ed25519 signature mismatch".to_string(),
        };
        let app_err = AppError::from(internal_err);

        // [2] Execute
        let response = app_err.into_response();

        // [3] Assert Equation
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_maps_integrity_violation_to_409_conflict() {
        // [1] Set up Mock Error: removed 'entity' field per core definition
        let internal_err = LmsError::IntegrityViolation {
            pipeline_step: "Phase 4: Integrity".to_string(),
            context: "Cross-trait Validation".to_string(),
            reason: "Conflicting orthography rules detected".to_string(),
        };
        let app_err = AppError::from(internal_err);

        // [2] Execute
        let response = app_err.into_response();

        // [3] Assert Equation
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_maps_unknown_errors_to_500_internal_error() {
        // [1] Set up Mock Error using valid 'PersistenceFault' to test fallback arm
        let internal_err = LmsError::PersistenceFault {
            pipeline_step: "Phase 0: WORM Hydration".to_string(),
            context: "data/snapshot.json".to_string(),
            reason: "Simulated hardware timeout".to_string(),
        };
        let app_err = AppError::from(internal_err);

        // [2] Execute
        let response = app_err.into_response();

        // [3] Assert Equation
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
