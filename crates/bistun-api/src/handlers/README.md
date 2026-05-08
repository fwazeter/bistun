# MODULE-README: API Handlers

---

## I. Strategic Overview

### 1. The "Why"

This module encapsulates the isolated HTTP handlers for the sidecar API, decoupling Axum routing definitions from the actual business logic of executing engine capabilities. It serves as the primary ingress point for the microservice, mapping raw HTTP requests to the capability engine's sub-microsecond resolution pipeline.

### 2. System Impact

If this module is compromised, the system loses its ability to resolve linguistic traits or report operational health to orchestrators. Furthermore, any inefficiency introduced here directly threatens the system's strict `< 1ms` latency budget.

### 3. Design Patterns

* **Handler Pattern**: Decouples endpoint definitions from request processing logic to allow for clean API versioning.
* **State Injection**: Utilizes Axum's `State` extractor to provide handlers with thread-safe, wait-free access to the `LinguisticManager`.

### 4. Local Glossary

* **Handler**: An asynchronous function responsible for processing an incoming HTTP request and returning a response.
* **Hot-Path**: The execution sequence that processes the highest volume of critical user requests.
* **Probe**: A diagnostic request used by orchestrators to determine if a container is alive and ready to serve traffic.

---

## II. Technical Interface

### 1. Primary Capability

| Function/Method | Input | Output | Purpose |
| --- | --- | --- | --- |
| `resolve_handler` | `State<LinguisticManager>`, `Path<String>` | `Result<impl IntoResponse, AppError>` | Routes BCP 47 requests into the 5-phase pipeline. |
| `health_handler` | `State<LinguisticManager>` | `impl IntoResponse` | Exposes engine readiness and sync metrics to orchestrators. |

### 2. Side Effects & SLIs

* **Telemetry**: Records tracing spans for every request to the system observability sink per `007-LMS-OPS`.
* **Performance**: Target latency: **< 1ms**. Complexity: **O(N)** for resolution (tag length), **O(1)** for health.
* **Dependencies**: Relies on `bistun_lms::LinguisticManager` for core logic and `bistun_core::ops` for state/metrics definitions.

### 3. Quirks & Invariants

* **Wait-Free Reads**: Handlers must only use the manager's wait-free read methods to ensure the hot-path is never blocked by background hydration.
* **HTTP Mapping**: All internal `LmsError` variants must be explicitly mapped to standard HTTP status codes via `AppError` to maintain API contract integrity.

---

## III. Usage & Implementation

### 1. Basic Instantiation

```rust
use ax_core::routing::get;
use axum::Router;
use bistun_api::handlers::{capability::resolve_handler, health::health_handler};
use bistun_lms::LinguisticManager;

fn main() {
    let manager = LinguisticManager::new();
    
    // Register handlers within the Axum Router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/v1/manifest/:locale", get(resolve_handler))
        .with_state(manager);
}

```

### 2. The "Golden Path" Example

```rust
// Typical HTTP request handled by this module:
// GET /v1/manifest/ar-EG

// The resolve_handler executes the following trace:
// 1. Extract "ar-EG" from the Path.
// 2. Delegate to manager.resolve_capabilities("ar-EG").
// 3. Serialize the resulting CapabilityManifest to JSON.
// 4. Return 200 OK with the payload.

```

---

## IV. Development & Extension Guide

### 1. How to Build it Up

To add a new handler (e.g., for a batch resolution or custom metrics):

1. **Red Phase**: Add a failing integration test in `crates/bistun-api/tests/` verifying the new endpoint response.
2. **Implementation**: Mirror the `# Logic Trace` defined in the new handler's documentation with `// [STEP X]` comments in the code.
3. **Audit**: Ensure the new handler does not introduce blocking I/O or heavy allocations that breach the 1ms budget.

### 2. Troubleshooting & Common Failures

* **503 Service Unavailable**: Likely indicates the engine is still `Bootstrapping` or in a `Degraded` state.
* **404 Not Found**: Occurs when a BCP 47 tag cannot be resolved even after the fallback chain is exhausted.

---

## V. Metadata

* **Author**: Francis Xavier Wazeter IV
* **Version**: 1.0.0
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-08
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents

When extending these handlers, prioritize the **Hot-Path** latency constraints. Do not introduce any logic that requires a write lock on the `LinguisticManager` state within a handler, as this will violate the high-throughput p99 SLAs defined in **007-LMS-OPS**.