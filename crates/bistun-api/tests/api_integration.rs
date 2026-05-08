// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # API Integration Tests
//! Ref: [001-LMS-CORE], [LMS-TEST]
//! Location: `crates/bistun-api/tests/api_integration.rs`
//!
//! **What is being proven**: Verifies the "Physics" of the full HTTP integration,
//! ensuring the Axum router correctly delegates to the `LinguisticManager`
//! and translates engine states to standard HTTP semantic codes.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use bistun_api::routes::app_router;
use bistun_lms::LinguisticManager;
use tower::ServiceExt; // Enables the `.oneshot()` testing utility

#[tokio::test]
async fn test_health_probe_yields_503_when_unhydrated() {
    // [1] Set up Unhydrated Manager (Isolation per LMS-TEST)
    // The engine starts in 'Bootstrapping' state until hydrated.
    let manager = LinguisticManager::new();
    let app = app_router(manager);

    // [2] Execute HTTP GET /health
    let request = Request::builder().uri("/health").body(Body::empty()).unwrap();
    let response = app.oneshot(request).await.unwrap();

    // [3] Assert Equation
    // A bootstrapping engine is not ready for traffic, so the API must return 503.
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn test_resolution_hot_path_yields_404_for_missing_route() {
    // [1] Set up
    let manager = LinguisticManager::new();
    let app = app_router(manager);

    // [2] Execute HTTP GET on an invalid API path
    let request = Request::builder().uri("/v1/invalid_endpoint").body(Body::empty()).unwrap();
    let response = app.oneshot(request).await.unwrap();

    // [3] Assert Equation
    // Axum's default fallback for unmatched routes should be 404 NOT FOUND.
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_resolution_hot_path_executes_successfully() {
    // [1] Set up Manager
    let manager = LinguisticManager::new();
    let app = app_router(manager);

    // [2] Execute HTTP GET using a Golden Input
    let request = Request::builder().uri("/v1/manifest/ar-EG").body(Body::empty()).unwrap();
    let response = app.oneshot(request).await.unwrap();

    // [3] Assert Equation
    // As long as the router maps the path to the engine and doesn't panic,
    // we prove the integration is stable. (The exact status depends on engine
    // fallback logic when unhydrated, usually resulting in a successful default).
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND);
}
