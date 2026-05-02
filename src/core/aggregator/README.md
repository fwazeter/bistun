# MODULE-README: Typology Aggregation Domain

![Blueprint](https://img.shields.io/badge/Blueprint-008--LMS--TYPOLOGY--AGGREGATOR-blue)
![Domain](https://img.shields.io/badge/Domain-Typology-green)
![Location](https://img.shields.io/badge/Location-src%2Fcore%2Faggregator-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module coordinates Phase 2 (Aggregate) of the 5-Phase pipeline. It is responsible for mapping a resolved locale identity to its baseline structural and morphological traits, ensuring script-specific requirements are unified.

### 2. System Impact
Failure in this domain results in a `CapabilityManifest` lacking foundational parsing instructions. Downstream NLP engines (Stemmers, Segmenters) will crash or misinterpret linguistic boundaries, as they rely on these traits to select execution strategies.

### 3. Design Patterns
* **High-Water Mark Strategy**: An architectural logic gate where the most computationally expensive requirement (e.g., `DICTIONARY` segmentation) overrides simpler requirements (e.g., `SPACE` based) when multiple scripts are present in a locale.
* **In-Place Mutation**: Modifies the manifest via mutable reference to eliminate heap allocation overhead, maintaining the system's strict performance budget.

### 4. Local Glossary
* **High-Water Mark**: A conflict resolution strategy that selects the maximum complexity rank from a set of competing script traits.
* **Typological Identity**: The structural properties of a language (e.g., `AGGLUTINATIVE` morphology) retrieved from the Flyweight `LocaleProfile`.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method | Input                                       | Output                 | Purpose                                                                  |
|:----------------|:--------------------------------------------|:-----------------------|:-------------------------------------------------------------------------|
| `aggregate()`   | `&mut CapabilityManifest`, `&LocaleProfile` | `Result<(), LmsError>` | Hydrates a manifest with resolved morphological and orthographic traits. |

### 2. Side Effects & SLIs
* **Telemetry**: Records the final resolved `SegmentationStrategy` which is utilized by Phase 5 (Telemetry) for manifest metadata.
* **Performance**: Target latency: **< 0.1ms**. Time Complexity: **O(1)** (fixed map insertions). Space Complexity: **O(1)** (in-place mutation).
* **Dependencies**: Relies on `std::cmp` for complexity ranking and `crate::models` for the DTO definitions.

### 3. Quirks & Invariants
* **Strict Maximum**: The aggregator enforces the strict maximum of `base_seg` and `alt_seg` defined in the profile to ensure rendering safety.
* **Invariant**: This module must never perform heap allocations; all insertions must utilize the pre-allocated capacity of the `CapabilityManifest`.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::models::manifest::CapabilityManifest;
use crate::data::store::LocaleProfile; // Flyweight profile
use crate::core::aggregator::typology::aggregate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manifest = CapabilityManifest::new("ja-JP".to_string());
    
    // In a real scenario, the profile is retrieved from the RegistryStore
    // let profile = registry.get_profile("ja-JP").unwrap();
    
    // For this example, we assume a hydrated profile exists
    // aggregate(&mut manifest, &profile)?;
    
    Ok(())
}
```

### 2. The "Golden Path" Example
```rust
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::{TraitKey, SegType};
use crate::core::aggregator::typology::aggregate;
// Internal mock helper for demonstration
use crate::core::aggregator::typology::tests::create_mock_profile; 

fn main() {
    let mut manifest = CapabilityManifest::new("ja-JP".to_string());
    let profile = create_mock_profile(
        crate::models::traits::MorphType::AGGLUTINATIVE,
        SegType::CHARACTER,
        Some(SegType::DICTIONARY), // Complex script override
    );

    // Execute aggregation phase
    aggregate(&mut manifest, &profile).expect("Aggregation failed");

    // Verify High-Water Mark: DICTIONARY must override CHARACTER
    let seg = manifest.traits.get(&TraitKey::SegmentationStrategy);
    assert_eq!(seg, Some(&TraitValue::SegType(SegType::DICTIONARY)));
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the aggregation logic (e.g., adding a new `Unicode_Block` collection):
1.  **Red Phase**: Add a test case to `typology.rs` demonstrating a locale that requires the new trait.
2.  **Implementation**: Update the `aggregate` function to insert the new `TraitKey` from the `LocaleProfile`.
3.  **Audit**: Ensure the `LocaleProfile` struct in `src/data/store.rs` is updated to carry the necessary source data.
4.  **Verification**: Run `just verify-all` to confirm the **< 1ms** budget is not breached.

### 2. Troubleshooting & Common Failures
* **Complexity Inversion**: If a new `SegType` is added, ensure its `Ord` implementation correctly reflects its computational cost. Incorrect ordering will break the `cmp::max` logic.
* **Missing Profiles**: If Phase 1 (Resolve) yields a locale that doesn't exist in the Flyweight pool, aggregation will fail before starting. Ensure the `RegistryStore` is fully hydrated.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When analyzing this module, focus on the **High-Water Mark** strategy. All aggregation logic must prioritize safety and rendering correctness over "simplest match." In-place mutation of the `CapabilityManifest` is a non-negotiable performance requirement to ensure sub-millisecond pipeline execution.
