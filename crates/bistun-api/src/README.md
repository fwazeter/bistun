# MODULE-README: `bistun-api/src` Root

---

## I. Strategic Overview

### 1. The "Why"

This directory serves as the architectural root of the API sidecar, orchestrating environment ingestion, error mapping, and the assembly of versioned HTTP routes. It acts as the bridge between the standalone `bistun-lms` engine and external network consumers, facilitating the final phase of the 5-phase resolution pipeline (Telemetry).

### 2. System Impact

As the primary integration boundary, any compromise or structural failure within this directory prevents the sidecar from exposing its public interface. Failure in the bootstrapper (`main.rs`) specifically results in a catastrophic service outage, preventing downstream consumers from resolving linguistic DNA.

### 3. Design Patterns

* **Sidecar Pattern**: Decouples the capability resolution engine into a discrete service that runs alongside a primary application.
* **Newtype Pattern**: Implemented in `error.rs` to wrap external `LmsError` types, allowing the implementation of Axum's `IntoResponse` trait for standardized HTTP failure narratives.

### 4. Local Glossary

* **Bootstrapper**: The `main.rs` logic responsible for initializing the async runtime, loading configuration, and starting the server listener.
* **Hydration**: The process of inflating the in-memory registry Flyweight pool from a cryptographically signed WORM snapshot during startup.

---

## II. Technical Interface

### 1. Primary Capability

| Function/Method | Input | Output | Purpose |
| --- | --- | --- | --- |
| `AppConfig::load()` | None | `AppConfig` | Ingests environment variables and .env keys to drive service configuration. |
| `main()` | None | `()` | The authoritative entry point that boots the sidecar service. |
| `AppError::from()` | `LmsError` | `AppError` | Maps engine-level failures to HTTP-compatible error objects. |

### 2. Side Effects & SLIs

* **Telemetry**: Records structured log events and tracing spans for resolution requests per `007-LMS-OPS`.
* **Performance**: Target resolution latency: **< 1ms**. Complexity is dominated by engine hydration: **O(M)** where M is registry size.
* **Dependencies**: Tight coupling with `bistun-core` for models and `bistun-lms` for the resolution orchestrator.

### 3. Quirks & Invariants

* **Fail-Fast Security**: The service will panic and abort during boot if the mandatory `CURATOR_PUBLIC_KEY` is missing from the environment.
* **Hermetic Integrity**: Route handlers are isolated from the bootstrapper logic to ensure they can be tested in a vacuum.

---

## III. Usage & Implementation

### 1. Basic Instantiation

```rust
// Typical boot sequence orchestrated by the root module
use bistun_api::config::AppConfig;
use bistun_lms::LinguisticManager;

#[tokio::main]
async fn main() {
    // 1. Ingest runtime configuration
    let config = AppConfig::load(); 

    // 2. Initialize the resolution engine
    let manager = LinguisticManager::new(); 

    // 3. Status check to ensure 'Ready' state before serving
    assert_eq!(manager.status(), bistun_core::ops::SdkState::Bootstrapping);
}

```

### 2. The "Golden Path" Example

```rust
// Demonstrating the error mapping flow used in handlers
use bistun_core::LmsError;
use bistun_api::error::AppError;
use axum::response::IntoResponse;

fn handle_failure() -> impl IntoResponse {
    let err = LmsError::InvalidTag { 
        tag: "??".to_string(), 
        pipeline_step: "Resolve".to_string(), 
        reason: "Invalid UTF-8".to_string() 
    };
    
    // Convert to AppError to yield an Axum Response
    AppError::from(err).into_response()
}

```

---

## IV. Development & Extension Guide

### 1. How to Build it Up

To extend the sidecar's root logic (e.g., adding a new global configuration parameter):

1. **Update Model**: Add the field to the `AppConfig` struct in `config.rs`.
2. **Implementation**: Update `AppConfig::load()` with the corresponding logic trace step and environment variable ingestion.
3. **Audit**: Ensure the new parameter is correctly utilized in `main.rs` for engine or router initialization.

### 2. Troubleshooting & Common Failures

* **Panic: CURATOR_PUBLIC_KEY missing**: Verify the `.env` file exists or environment variables are correctly exported to the process.
* **Degraded Engine Status**: Indicates a failure during the hydration phase in `main.rs`, likely due to an invalid registry signature or missing snapshot file.

---

## V. Metadata

* **Author**: Francis Xavier Wazeter IV
* **Version**: 1.0.0
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-08
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents

When analyzing the `src` root, treat `lib.rs` as the authoritative map of the sidecar's modular structure. Do not suggest adding business logic directly to `main.rs`; all resolution logic must be delegated to the `handlers` module to maintain the system's wait-free performance characteristics defined in **010-LMS-MEM**.