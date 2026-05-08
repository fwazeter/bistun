# MODULE-README: API Routes Orchestrator

---

## I. Strategic Overview

### 1. The "Why"

This module centralizes the assembly of versioned sub-routers and global diagnostic probes into a unified application tree. It defines the authoritative endpoint map for the capability engine's public interface, ensuring requests are correctly routed to versioned handlers.

### 2. System Impact

Acts as the system's "Traffic Controller"; if this module is compromised, external consumers cannot reach resolution handlers, resulting in a 404 status for all system requests, including critical health checks. This breaks the entire capability delivery pipeline for downstream consumers.

### 3. Design Patterns

* **Route Nesting**: Utilizes Axum's nesting capabilities to isolate versioned routes (e.g., `/v1`) from global operational endpoints.
* **State Injection**: Implements dependency injection by sharing the `LinguisticManager` across the routing tree, enabling wait-free access for all associated handlers.

### 4. Local Glossary

* **Route**: A URL pattern mapped to a specific executable handler.
* **Traffic Controller**: The role of the master router in orchestrating global and versioned ingress paths.

---

## II. Technical Interface

### 1. Primary Capability

| Function/Method | Input | Output | Purpose |
| --- | --- | --- | --- |
| `app_router()` | `LinguisticManager` | `Router` | Assembles the master application router for the sidecar. |
| `v1::router()` | `None` | `Router<LinguisticManager>` | Constructs the scoped v1 capability router. |

### 2. Side Effects & SLIs

* **Telemetry**: Routes are mapped to handlers that record diagnostic spans via `tracing` per **007-LMS-OPS**.
* **Performance**: Target latency: **< 1ms**. Complexity: **O(1)** for route registration.
* **Dependencies**: Relies on `axum` for routing primitives and `bistun_lms::LinguisticManager` for state management.

### 3. Quirks & Invariants

* **Version Isolation**: v1 routes are nested under the `/v1` prefix to ensure forward compatibility with future API iterations.
* **Infallible Registration**: Route registration is a synchronous, static process that must never fail during the service bootstrapping phase.

---

## III. Usage & Implementation

### 1. Basic Instantiation

```rust
use bistun_lms::LinguisticManager;
use bistun_api::routes::app_router;

fn main() {
    // Instantiate the capability engine
    let manager = LinguisticManager::new();

    // Assemble the master router with injected state
    let app = app_router(manager);
    
    // The router is now ready to be served via Axum
}

```

### 2. The "Golden Path" Example

```rust
// Internal logic for assembling versioned routes
pub fn app_router(manager: LinguisticManager) -> Router {
    Router::new()
        // Global operational probe
        .route("/health", get(health_handler))
        // Versioned capability ingress
        .nest("/v1", v1::router())
        .with_state(manager)
}

```

---

## IV. Development & Extension Guide

### 1. How to Build it Up

To add a new API version (e.g., `v2`):

1. **Red Phase**: Create a failing integration test in `tests/api_integration.rs` targeting a `/v2` endpoint.
2. **Implementation**: Create a new `v2.rs` module and mirror the `# Logic Trace` defined in the existing `v1.rs`.
3. **Registration**: Nest the new router under `/v2` in the master `app_router` within `mod.rs`.

### 2. Troubleshooting & Common Failures

* **404 Not Found (Global)**: Indicates the root `/health` route is not properly bound in the master orchestrator.
* **404 Not Found (v1)**: Suggests a mismatch in the `/v1` nesting prefix or a failure in the scoped router registration.

---

## V. Metadata

* **Author**: Francis Xavier Wazeter IV
* **Version**: 1.0.0
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-08
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents

When extending this module, ensure that any new routes correctly implement the state extractor for the `LinguisticManager`. Do not introduce middleware that performs blocking I/O, as this would breach the system's p99 resolution latency requirements defined in **007-LMS-OPS**.