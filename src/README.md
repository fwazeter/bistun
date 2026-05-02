# MODULE-README: Public SDK & Library Entry

![Blueprint](https://img.shields.io/badge/Blueprint-001--LMS--CORE-blue)
![Domain](https://img.shields.io/badge/Domain-Orchestration-green)
![Location](https://img.shields.io/badge/Location-src%2F-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This directory serves as the primary library entry point and SDK orchestrator for the Bistun Linguistic Metadata Service. It defines the public API contract through `lib.rs` and implements the `LinguisticManager`, which coordinates the 5-phase pipeline and manages the thread-safe memory pool.

### 2. System Impact
As the "Front Door" of the service, failure here renders the entire system inaccessible to consuming applications. If the `LinguisticManager` fails to initialize or transition its state correctly, the system's **Circuit Breaker** must activate to provide safe defaults and prevent application-level crashes.
x
### 3. Design Patterns
* **Facade Pattern**: The `LinguisticManager` provides a simplified, high-level interface to the complex internal resolution, aggregation, and validation domains.
* **SDK State Machine**: Manages the operational health of the service through explicit states (`Bootstrapping`, `Ready`, `Degraded`) to ensure thread-safe operation during hot-swaps.
* **Circuit Breaker**: A safety mechanism that intercepts requests during `Degraded` states to return hardcoded system defaults (e.g., `en-US`), ensuring zero-downtime availability.

### 4. Local Glossary
* **LMS Library**: The compiled Rust crate providing the authoritative mechanisms to resolve linguistic DNA.
* **SDK Orchestrator**: The logic in `manager.rs` responsible for the lifecycle of linguistic data and orchestrating the resolution pipeline.
* **Public Contract**: The set of re-exported types (`CapabilityManifest`, `LmsError`, `LinguisticManager`) that define the stable API for external consumers.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method | Input | Output | Purpose |
| :--- | :--- | :--- | :--- |
| `LinguisticManager::new()` | None | `Self` | Initializes the orchestrator and hydrates the initial state. |
| `resolve_capabilities()` | `&str` (Tag) | `Result<CapabilityManifest, LmsError>` | The main entry point for transforming BCP 47 tags into functional traits. |

### 2. Side Effects & SLIs
* **Telemetry**: Initiates the root diagnostic span for every resolution request to track the total pipeline latency.
* **Performance**: Target latency: **< 1ms** p99. Complexity is dominated by the underlying 5-Phase pipeline execution.
* **Dependencies**: Flattened re-exports rely on `src/models` and `src/core/resolver` to maintain a predictable API.

### 3. Quirks & Invariants
* **Flattened API**: `lib.rs` flattens the internal module hierarchy; consumers should import directly from the crate root.
* **Invariant**: The `LinguisticManager` must always return a valid `CapabilityManifest` (even if it is a fallback default) unless a critical `SecurityFault` is detected.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::manager::{LinguisticManager, SdkState};

fn main() {
    // Instantiate the manager to boot the capability engine
    let manager = LinguisticManager::new();
    
    // Explicit use to satisfy the compiler and verify readiness
    assert_eq!(manager.status(), SdkState::Ready);
}
```

### 2. The "Golden Path" Example
```rust
use crate::LinguisticManager;
use crate::models::manifest::CapabilityManifest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = LinguisticManager::new();
    
    // Primary SDK call to resolve a locale
    let manifest: CapabilityManifest = manager.resolve_capabilities("ar-EG")?;
    
    println!("Resolved {} with direction: {:?}", 
        manifest.resolved_locale, 
        manifest.traits.get(&crate::models::traits::TraitKey::PrimaryDirection)
    );
    
    Ok(())
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the root orchestrator (e.g., adding a new `initialize_with_config` method):
1.  **Red Phase**: Add a failing test case in `manager.rs` verifying that the manager correctly ingests custom configuration.
2.  **Implementation**: Update the `LmsManager` struct and mirror the `# Logic Trace` with `// [STEP X]` comments.
3.  **Flat Export**: Ensure any new public types are re-exported in `lib.rs` to maintain API ergonomics.
4.  **Verification**: Run `just verify-all` to confirm that the system boot time and resolution overhead remain within budget.

### 2. Troubleshooting & Common Failures
* **System Failure (Degraded)**: If the manager remains in a `Degraded` state, check the telemetry logs for Phase 0 (Hydration) errors or signature verification failures.
* **SDK Delegation Failure**: Occurs if `lib.rs` re-exports are out of sync with internal module changes. Ensure `just verify-docs` passes to catch broken intra-doc links.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-04-29
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When modifying `lib.rs` or `manager.rs`, maintain the **Ergonomic SDK Boundary**. Do not expose internal submodules (like `core` or `data`) directly unless required for specialized administrative tools. All public-facing logic must be "Novice-Friendly" and explicitly satisfy the **LmsError** propagation standard.
