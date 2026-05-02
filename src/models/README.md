# MODULE-README: Shared Data Models (DTOs)

![Blueprint](https://img.shields.io/badge/Blueprint-011--LMS--DTO-blue)
![Domain](https://img.shields.io/badge/Domain-Typology%20|%20Orthography-green)
![Location](https://img.shields.io/badge/Location-src%2Fmodels-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module serves as the central hub for the system's Data Transfer Objects (DTOs) and shared vocabulary. It flattens the internal module hierarchy to provide a predictable, decoupled API for consuming sidecars and SDKs during the 5-phase resolution pipeline.

### 2. System Impact
As the authoritative "Contract Layer" of the service, any compromise to these models will cause widespread UI rendering failures and algorithmic errors in downstream components like Search and Font Automation.

### 3. Design Patterns
* **DTO (Data Transfer Object)**: The `CapabilityManifest` encapsulates resolved linguistic instructions into an immutable package for transport.
* **High-Water Mark Helper**: The `SegType` enum utilizes Rust's `Ord` trait to rank segmentation complexity, allowing the `TraitAggregator` to resolve multi-script conflicts automatically.
* **Untagged Serialization**: `TraitValue` uses Serde's `untagged` pattern to output clean JSON primitives, ensuring cross-language compatibility without variant-name overhead.

### 4. Local Glossary
* **Linguistic DNA**: The unique set of Typological and Orthographic traits that define a locale's functional capabilities.
* **High-Water Mark**: A conflict resolution strategy that selects the most complex requirement (e.g., `DICTIONARY` segmentation) when merging multi-script traits.
* **Untagged Serialization**: A configuration where enums are serialized as their underlying value rather than being wrapped in variant names.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method             | Input                 | Output        | Purpose                                                         |
|:----------------------------|:----------------------|:--------------|:----------------------------------------------------------------|
| `CapabilityManifest::new()` | `String` (Locale Tag) | `Self`        | Constructs a new, empty instruction container.                  |
| `serde::Serialize`          | `&Self`               | `JSON String` | Converts the manifest into a standard-aligned transport format. |

### 2. Side Effects & SLIs
* **Telemetry**: Metadata fields are reserved for recording `resolution_time_ms` to verify SLI compliance against the **< 1ms** target.
* **Performance**: Instantiation is **O(1)**. Lookups in the traits map are **O(1)** via `hashbrown`.
* **Dependencies**: Relies on `serde` for serialization, `serde_json` for DTO validation, and `hashbrown` for high-performance map operations.

### 3. Quirks & Invariants
* **Screaming Snake Case**: All `TraitKey` variants serialize to `SCREAMING_SNAKE_CASE` to match the DTO standard.
* **Heap Pre-allocation**: `CapabilityManifest::new()` initializes empty maps to prevent initial reallocations during the Aggregation phase.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::models::manifest::CapabilityManifest;

fn main() {
    // Initialize a manifest container for an Arabic locale
    let manifest = CapabilityManifest::new("ar-EG".to_string());
    
    // Explicit use of the variable to satisfy compiler and prove readiness
    assert_eq!(manifest.resolved_locale, "ar-EG");
}
```

### 2. The "Golden Path" Example
```rust
use crate::models::{TraitKey, TraitValue, Direction, CapabilityManifest};

fn main() {
    let mut manifest = CapabilityManifest::new("ar-EG".to_string());

    // Injecting Orthographic DNA into the manifest
    manifest.traits.insert(
        TraitKey::PrimaryDirection, 
        TraitValue::Direction(Direction::RTL)
    );

    // Verify successful trait mapping
    let direction = manifest.traits.get(&TraitKey::PrimaryDirection);
    assert!(direction.is_some());
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To add a new capability (e.g., a new `NumberingSystem` or `SynthesisDegree`):
1.  **Define**: Add the new variant to `TraitKey` in `traits.rs`.
2.  **Map**: Update the `TraitValue` enum in `manifest.rs` if a new return type is required.
3.  **Validate**: Add a test case in `manifest.rs` verifying that the new trait serializes correctly to JSON.
4.  **Audit**: Run `just verify-all` to ensure the registry versioning logic remains intact.

### 2. Troubleshooting & Common Failures
* **Serialization Mismatch**: Ensure new enum variants match the `SCREAMING_SNAKE_CASE` requirement; failure to do so will break client-side SDK parsing.
* **High-Water Mark Regressions**: If adding a new `SegType`, verify its position in the enum matches its complexity rank, as `Ord` is derived from variant order.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.1
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When analyzing this module via RAG, prioritize the **Linguistic DNA** consistency matrix and the **High-Water Mark** conflict strategy. All code examples provided in response must be "Self-Contained" (including visible `use` statements and `main` wrappers) and must explicitly use all declared variables to pass the project's strict IDE quality gates.
