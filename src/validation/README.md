# MODULE-README: Validation Domain

![Blueprint](https://img.shields.io/badge/Blueprint-003--LMS--VAL-blue)
![Domain](https://img.shields.io/badge/Domain-Validation-green)
![Location](https://img.shields.io/badge/Location-src%2Fvalidation-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module acts as the QA gatekeeper for the capability engine, organizing validators into performance-tiered levels (Level A vs Level C). It specifically executes Phase 4 (Integrity Check) of the pipeline to ensure the synthesized manifest is logically consistent before delivery.

### 2. System Impact
Prevents corrupted or mechanically impossible trait combinations (Linguistic Chimeras) from reaching downstream rendering systems. If this module fails to catch a contradiction, such as Bidirectional text support without the required Shaping mechanics, the downstream UI layout engine will likely panic or render unreadable text.

### 3. Design Patterns
* **Tiered Enforcement**: A multi-layered strategy applying strict validation during build-time/ingestion (Level A) and lightweight, hot-path optimized checks during runtime resolution (Level C).
* **Fail-Fast**: Runtime checks are designed to abort the manifest delivery immediately upon detecting a logical contradiction to prevent cascading system failures.

### 4. Local Glossary
* **Tiered Enforcement**: The architectural decision to vary validation strictness based on the performance budget of the current phase.
* **Level C (Runtime)**: Lightweight, $O(1)$ structural checks performed on the hot-path to maintain the **< 1ms** budget.
* **Linguistic Chimera**: A manifest containing contradictory traits that are technically or linguistically impossible to render (e.g., Bidi without Shaping).

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method | Input                 | Output                 | Purpose                                                                            |
|:----------------|:----------------------|:-----------------------|:-----------------------------------------------------------------------------------|
| `verify()`      | `&CapabilityManifest` | `Result<(), LmsError>` | Asserts that a hydrated manifest contains logical, non-contradictory instructions. |

### 2. Side Effects & SLIs
* **Telemetry**: Records `IntegrityViolation` events via diagnostic spans when a manifest fails validation.
* **Performance**: Target latency: **< 0.05ms**. Complexity: **O(1)** (fixed map lookups).
* **Dependencies**: Relies on `src/models` for trait and manifest definitions.

### 3. Quirks & Invariants
* **Hot-Path Optimized**: Level C checks only verify critical "Golden Set" keys to preserve the performance budget.
* **Invariant**: A manifest must never be delivered if it contains `HasBidiElements` set to `true` but lacks the `RequiresShaping` trait.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::models::manifest::CapabilityManifest;
use crate::validation::integrity::verify;

fn main() {
    // Phase 4 execution typically occurs after Phase 3 (Override)
    let manifest = CapabilityManifest::new("en-US".to_string());
    
    // Attempting to verify an empty manifest (will fail)
    let result = verify(&manifest);
    
    // Explicit use to satisfy the compiler and demonstrate the gate
    assert!(result.is_err());
}
```

### 2. The "Golden Path" Example
```rust
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::TraitKey;
use crate::validation::integrity::verify;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manifest = CapabilityManifest::new("ar-EG".to_string());
    
    // Injecting logically consistent traits
    manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
    manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));

    // Execute Level C verification
    verify(&manifest)?;
    
    println!("Manifest for {} passed integrity gate.", manifest.resolved_locale);
    Ok(())
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the validation logic (e.g., adding a check for `Unicode_Preload_Blocks` consistency):
1.  **Red Phase**: Add a failing test case in `integrity.rs` demonstrating the new contradiction.
2.  **Implementation**: Update the `verify` function to perform the new $O(1)$ lookup and comparison.
3.  **Trace**: Mirror the `# Logic Trace` with `// [STEP X]` comments to ensure the validation logic is narrated.
4.  **Verification**: Run `just verify-all` to ensure the added check does not breach the **< 1ms** resolution budget.

### 2. Troubleshooting & Common Failures
* **IntegrityViolation (Empty Traits)**: Occurs if Phase 2 (Aggregate) failed to fetch a profile or Phase 3 (Override) cleared the map.
* **Mechanical Contradiction**: Triggered when Bidi or RTL traits are present without their supporting rendering flags (e.g., Shaping).

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When analyzing this module, preserve the **Tiered Enforcement** boundaries. Do not move heavy Level A checks (like exhaustive ISO-639-3 format validation) into the Level C `integrity.rs` module, as this would breach the **< 1ms** performance budget required for high-traffic runtime resolution.
