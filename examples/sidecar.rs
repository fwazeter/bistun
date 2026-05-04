// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # The Sidecar Microservice (API Layer)
//! Ref: [001-LMS-CORE]
//! Location: `examples/sidecar.rs`
//!
//! **Why**: This binary serves as the HTTP/gRPC entry point for the Bistun capability engine.
//! **Impact**: This is the literal network interface. If it fails, downstream NLP and rendering services cannot fetch linguistic capabilities.
//!
//! ### Glossary
//! * **Axum**: A highly ergonomic and extremely fast web framework built on Tokio.
//! * **State Extraction**: Axum's mechanism for safely sharing the LinguisticManager across concurrent request threads.

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use bistun::manager::LinguisticManager;
use tracing::{error, info, instrument};

/// The primary HTTP handler for resolving a BCP 47 tag.
///
/// Time: O(1) mostly | Space: O(1) map serialization
///
/// # Logic Trace (Internal)
/// 1. Extract the `locale` from the URI path and the `manager` from shared app state.
/// 2. Delegate the resolution to the 5-Phase pipeline via `manager.resolve_capabilities()`.
/// 3. If successful, serialize the manifest to JSON and return HTTP 200 OK.
/// 4. If failed, map the domain error to an HTTP 400 status code.
///
/// # Arguments
/// * `State(manager)`: The shared, thread-safe LinguisticManager.
/// * `Path(locale)`: The target BCP 47 tag (e.g., "ar-EG").
#[instrument(skip(manager), fields(tag = %locale))]
async fn resolve_handler(
    State(manager): State<LinguisticManager>,
    Path(locale): Path<String>,
) -> impl IntoResponse {
    // [STEP 1 & 2]: Extract and Delegate
    match manager.resolve_capabilities(&locale) {
        Ok(manifest) => {
            // [STEP 3]: Success - Serialize to JSON safely bypassing macro issues
            let status = StatusCode::from_u16(200).unwrap();
            (status, Json(manifest)).into_response()
        }
        Err(e) => {
            // [STEP 4]: Failure - Map domain error to HTTP error
            error!("Resolution failed for {}: {:?}", locale, e);

            let status = StatusCode::from_u16(400).unwrap();
            (status, e.to_string()).into_response()
        }
    }
}

/// Bootstraps the Tokio runtime and starts the Axum web server.
#[tokio::main]
async fn main() {
    // Initialize standard tracing subscriber for stdout observability
    tracing_subscriber::fmt::init();
    info!("🚀 Bistun LMS Sidecar Bootstrapping...");

    // [STEP 1]: Initialize the Core Engine
    let manager = LinguisticManager::new();

    // [STEP 2]: Set up the Router
    let app =
        Router::new().route("/v1/manifest/{locale}", get(resolve_handler)).with_state(manager);

    // [STEP 3]: Bind and Serve
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("✅ Sidecar listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
