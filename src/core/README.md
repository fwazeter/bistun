# MODULE-README: The Core Engine Domain

![Blueprint](https://img.shields.io/badge/Blueprint-001--LMS--CORE-blue)
![Domain](https://img.shields.io/badge/Domain-Orchestration-green)
![Location](https://img.shields.io/badge/Location-src%2Fcore-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module acts as the primary orchestrator for the **5-Phase Capability Pipeline**. It routes incoming requests through the Taxonomic Resolver, Typological Aggregator, and Orthographic Extension mapper to synthesize a finalized, immutable `CapabilityManifest`.

### 2. System Impact
If this module or its internal routing is misconfigured, the service boundaries will fail to communicate. This prevents the transformation of raw BCP 47 tags into actionable rendering and processing instructions, effectively disconnecting the entire capability engine from the system.

### 3. Design Patterns
* **Orchestrator Pattern**: The `pipeline.rs` coordinator manages sub-engine execution without embedding specific business logic in the dispatcher.
* **Sequential Pipeline**: Implements a strict, linear flow (Resolve → Aggregate → Override → Integrity → Telemetry) to ensure deterministic hydration of linguistic traits.

### 4. Local Glossary
* **Capability Pipeline**: The standard execution sequence used to hydrate a manifest with linguistic DNA.
* **Orchestration**: The process of coordinating complex sub-module interactions (Resolver, Aggregator, Extension) into a unified result.
* **Coordinator**: The central functional unit in `pipeline.rs` that manages the data flow between phases.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method       | Input                               | Output                                 | Purpose                                                 |
|:----------------------|:------------------------------------|:---------------------------------------|:--------------------------------------------------------|
| `generate_manifest()` | `&str` (Tag), `&dyn IRegistryState` | `Result<CapabilityManifest, LmsError>` | Executes the full 5-Phase pipeline to yield a manifest. |

### 2. Side Effects & SLIs
* **Telemetry**: Every pipeline execution is wrapped in a diagnostic span, recording the `resolution_path` and `resolution_time_ms` into the manifest metadata.
* **Performance**: Target latency: **< 1.0ms** p99 for cached registry hits.
* **Dependencies**: Coordinates between `resolver`, `aggregator`, and `extension` submodules.

### 3. Quirks & Invariants
* **Stateless Execution**: The coordinator itself maintains no state; it relies entirely on the thread-safe `IRegistryState` provided at runtime.
* **Invariant**: The pipeline must always execute Phase 1 (Resolve) successfully before proceeding to subsequent trait-hydration phases.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::core::pipeline::generate_manifest;
use crate::data::swap::IRegistryState;

// Standard execution within a service handler
async fn handle_request(tag: &str, state: &dyn IRegistryState) {
    // The pipeline coordinates all sub-modules (resolver, aggregator, etc.)
    let result = generate_manifest(tag, state);
    
    if let Ok(manifest) = result {
        println!("Manifest generated for locale: {}", manifest.resolved_locale);
    }
}
```

### 2. The "Golden Path" Example
```rust
use crate::core::pipeline::generate_manifest;
use crate::data::swap::MockRegistryState; // Assuming mock from LMS-TEST
use crate::models::manifest::CapabilityManifest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mock_state = MockRegistryState::new();
    // Setup mock expectations for resolution and profile retrieval...

    // Full 5-Phase execution
    let manifest = generate_manifest("ar-EG-u-ca-islamic", &mock_state)?;
    
    // Verify terminal pipeline state
    assert_eq!(manifest.resolved_locale, "ar-EG");
    assert!(manifest.metadata.contains_key("resolution_time_ms"));
    
    Ok(())
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the core pipeline (e.g., adding a new Phase 4.5 Security Check):
1.  **Red Phase**: Add a failing integration test in `tests/` that requires the new phase's output.
2.  **Implementation**: Add the new module call to the sequential block in `pipeline.rs`.
3.  **Trace**: Update the `# Logic Trace` in `pipeline.rs` to reflect the new step.
4.  **Verification**: Run `just verify-all` to ensure the **< 1ms** performance budget is not breached by the added overhead.

### 2. Troubleshooting & Common Failures
* **Phase Disconnection**: If `generate_manifest` returns a manifest without traits, verify that the `aggregator` and `extension` modules are correctly receiving the `LocaleProfile` from the `resolver`.
* **Orchestration Panic**: Usually occurs if the provided `IRegistryState` lock is poisoned during a concurrent hot-swap.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When modifying the core engine, you must preserve the **Phase-Linked** structure of `pipeline.rs`. Do not introduce branching logic that allows trait aggregation to occur before taxonomic resolution is finalized. The integrity of the **System of Record** depends on this strict sequential ordering.
