# MODULE-README: Orthographic Extension Domain

![Blueprint](https://img.shields.io/badge/Blueprint-004--LMS--EXT-blue)
![Domain](https://img.shields.io/badge/Domain-Orthography-green)
![Location](https://img.shields.io/badge/Location-src%2Fcore%2Fextension-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module coordinates Phase 3 (Override/Extension) of the 5-Phase pipeline. It extracts requested Unicode BCP 47 extension subtags (e.g., `-u-nu-latn`) and merges them with the baseline typological profile to create a customized behavioral manifest.

### 2. System Impact
If this domain is bypassed or fails, specific client-side rendering preferences—such as numbering systems or calendars—will be ignored. Catastrophic unreadability occurs if script-direction overrides are not applied, causing text in RTL (Right-To-Left) or TTB (Top-To-Bottom) languages to render in the wrong direction.

### 3. Design Patterns
* **Override Pattern**: The logic allows specific user-provided extensions to take precedence over the system's "Mechanical Defaults".
* **Atomic Strategy**: Overrides are applied as an atomic transformation of the `CapabilityManifest`, ensuring the manifest remains in a consistent state throughout the pipeline.

### 4. Local Glossary
* **Extension Parsing**: The process of extracting specific Unicode BCP 47 extension subtags to modify the behavioral manifest.
* **Mechanical Defaults**: The baseline rendering traits (Direction, Bidi) defined by the script before any specific overrides are applied.
* **Bidi (Bidirectional)**: Text that contains both LTR and RTL scripts natively, requiring complex shaping and layout algorithms.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method            | Input                                                     | Output                 | Purpose                                                               |
|:---------------------------|:----------------------------------------------------------|:-----------------------|:----------------------------------------------------------------------|
| `apply_rendering_traits()` | `&mut CapabilityManifest`, `&LocaleProfile`, `&str` (Tag) | `Result<(), LmsError>` | Maps mechanical rendering requirements and applies Unicode overrides. |

### 2. Side Effects & SLIs
* **Telemetry**: Records diagnostic spans for extension extraction to measure the overhead of complex tag parsing.
* **Performance**: Target latency: **< 0.1ms**. Time Complexity: **O(N)** relative to the number of extension subtags.
* **Dependencies**: Relies on `src/models` for trait definitions and `src/data/store` for the `LocaleProfile`.

### 3. Quirks & Invariants
* **Directional Priority**: Script direction overrides retrieved from the profile always serve as the starting point before BCP 47 `-u-` extensions are evaluated.
* **Invariant**: This module must ensure that the `PRIMARY_DIRECTION` trait is never null in the final manifest for supported locales.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::models::manifest::CapabilityManifest;
use crate::core::extension::orthography::apply_rendering_traits;
use crate::data::store::LocaleProfile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Phase 1 & 2 must be complete before this module is called
    let mut manifest = CapabilityManifest::new("ar-EG".to_string());
    
    // In a production pipeline, the profile is fetched from the RegistryStore
    // let profile = registry.get_profile("ar-EG").unwrap();
    
    Ok(())
}
```

### 2. The "Golden Path" Example
```rust
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::{TraitKey, Direction};
use crate::core::extension::orthography::apply_rendering_traits;

// Using a mock profile for demonstration per LMS-TEST standards
fn main() {
    let mut manifest = CapabilityManifest::new("ar-EG".to_string());
    
    // Logic trace: Applying RTL traits to an Arabic locale
    // In a real implementation, 'profile' comes from src/data/store
    // apply_rendering_traits(&mut manifest, &profile, "ar-EG-u-nu-latn").unwrap();
    
    // Demonstration of expected result
    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));
    
    let direction = manifest.traits.get(&TraitKey::PrimaryDirection);
    assert_eq!(direction, Some(&TraitValue::Direction(Direction::RTL)));
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the extension logic (e.g., adding a new `Calendar` or `Collation` override):
1.  **Red Phase**: Add a failing test case in `orthography.rs` using a locale tag with the new extension (e.g., `en-US-u-ca-hebrew`).
2.  **Implementation**: Update the parsing logic to recognize the new subtag key and map it to a corresponding `TraitValue`.
3.  **Audit**: Ensure the new trait key is defined in `src/models/traits.rs` to maintain the "Golden Set" integrity.
4.  **Verification**: Run `just verify-all` to ensure the sub-millisecond resolution budget is maintained.

### 2. Troubleshooting & Common Failures
* **Malformed Subtags**: If a BCP 47 tag is malformed (e.g., `ar-u-`), the parser should log a `WARN` and fall back to mechanical defaults rather than returning an error.
* **Bidi Misidentification**: Ensure the `has_bidi` flag in the `LocaleProfile` is respected, as it triggers different layout logic in consuming UI engines.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When analyzing this module, prioritize the **Phase 3 Override** logic. Ensure that mechanical defaults from the script (Orthography) are correctly reconciled with user-specific BCP 47 extensions. Do not suggest implementations that block the pipeline on malformed extensions; the system must favor "Graceful Fallback" to ensure the **< 1ms** resolution budget is met.
