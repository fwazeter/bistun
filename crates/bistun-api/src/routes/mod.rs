// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV
// ... (Standard GPL Header) ...

//! # Master Router Orchestrator
//! Ref: [001-LMS-CORE]
//! Location: `crates/bistun-api/src/routes/mod.rs`
//!
//! **Why**: Centralizes the assembly of versioned sub-routers and global diagnostic probes into a unified application tree.
//! **Impact**: Acts as the "Traffic Controller." Failure here results in a 404 for all system requests, including health checks.

pub mod v1;

use crate::handlers::health::{health_handler, metrics_handler};
use axum::{Router, routing::get};
use bistun_lms::LinguisticManager;

/// Assembles the master application router for the sidecar microservice.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Define the root `Router`.
/// 2. Bind the `/health` liveness probe and `/metrics` prometheus exporter to the global namespace.
/// 3. Nest the `v1` sub-router under the `/v1` prefix to ensure future version compatibility.
/// 4. Inject the `LinguisticManager` into the router's state, enabling wait-free access for handlers.
/// 5. Return the finalized application router.
///
/// # Examples
/// ```rust
/// # use bistun_lms::LinguisticManager;
/// # use bistun_api::routes::app_router;
/// let manager = LinguisticManager::new();
/// let app = app_router(manager);
/// ```
///
/// # Arguments
/// * `manager` ([`LinguisticManager`]): The thread-safe capability engine instance to be shared across all routes.
///
/// # Returns
/// * `Router`: A fully assembled Axum `Router` ready to be served via a TCP listener.
///
/// # Golden I/O
/// * **Input**: `LinguisticManager` instance
/// * **Output**: `Router` (Master tree with /health and /v1)
///
/// # Errors, Panics, & Safety
/// * **Errors**: Infallible assembly.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn app_router(manager: LinguisticManager) -> Router {
    // [STEP 1, 2 & 3]: Global Assembly and Nesting
    Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .nest("/v1", v1::router())
        // [STEP 4]: State Injection
        .with_state(manager)
}
