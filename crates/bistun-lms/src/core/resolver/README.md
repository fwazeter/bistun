# MODULE-README: Taxonomic Resolver Engine

![Blueprint](https://img.shields.io/badge/Blueprint-012--LMS--ENG-blue)
![Domain](https://img.shields.io/badge/Domain-Taxonomy-green)
![Location](https://img.shields.io/badge/Location-src%2Fcore%2Fresolver-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module implements Phase 1 (Resolve) of the capability pipeline. It is responsible for mapping non-canonical or broad BCP 47 language tags into a precise `LocaleEntry` using a deterministic fallback strategy.

### 2. System Impact
As the system's "Identity Gate," any failure here prevents the capability engine from locating the correct Linguistic DNA. Compromise results in the system failing-open to generic `en-US` defaults, leading to significant cultural data loss in downstream rendering.

### 3. Design Patterns
* **Chain of Responsibility**: Resolution logic is partitioned into discrete handlers (Exact Match, Alias, Truncation) that are evaluated sequentially until a match is found.
* **Strategy Pattern**: Individual resolvers encapsulate specific fallback algorithms, allowing the engine to be extended with project-specific overrides without modifying the core orchestrator.

### 4. Local Glossary
* **Resolver**: A discrete operational unit in the fallback chain responsible for a single matching algorithm.
* **Canonicalization**: The process of mapping deprecated or broad tags (e.g., `in` -> `id`) to operational identities.
* **Truncation**: Iteratively stripping subtags from right-to-left (RFC 4647) to find the nearest supported parent locale.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method | Input                                             | Output                | Purpose                                                      |
|:----------------|:--------------------------------------------------|:----------------------|:-------------------------------------------------------------|
| `resolve()`     | `&str`, `&dyn IRegistryState`, `&mut Vec<String>` | `Option<LocaleEntry>` | Executes the resolution chain to locate a canonical profile. |
| `set_next()`    | `Box<dyn IResolver>`                              | `()`                  | Appends a new handler to the Chain of Responsibility.        |

### 2. Side Effects & SLIs
* **Telemetry**: Every "hop" in the resolution process is appended to the `path` vector, which is later injected into the `CapabilityManifest` metadata for production auditability.
* **Performance**: Target latency: **< 0.1ms** per hop. Complexity is **O(N)** where N is the number of subtags in the input string.
* **Dependencies**: Relies on `IRegistryState` for thread-safe access to the active Flyweight pools.

### 3. Quirks & Invariants
* **Macrolanguage Priority**: Resolution prioritizes alias hits for macrolanguages (e.g., `no` -> `nb`) before falling back to generic language truncation.
* **Invariant**: The resolver must never return a `LocaleEntry` belonging to a different registry version than the provided `IRegistryState`.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::core::resolver::IResolver;
use crate::core::resolver::bcp47::ExactMatchResolver;

fn main() {
    // Construct a resolver unit
    let resolver = ExactMatchResolver::new();
    
    // Explicit use to satisfy compiler; verifies the resolver is not terminal by default
    assert!(!resolver.is_terminal());
}
```

### 2. The "Golden Path" Example
```rust
use crate::core::resolver::bcp47::resolve;
use crate::data::swap::MockRegistryState; // Assuming mock from LMS-TEST setup

fn main() {
    let state = MockRegistryState::new();
    let mut resolution_path = Vec::new();
    
    // Resolving a tag requiring truncation (e.g., en-AU-u-nu-latn -> en-AU)
    let entry = resolve("en-AU-u-nu-latn", &state, &mut resolution_path);
    
    if let Some(locale) = entry {
        println!("Resolved to canonical ID: {}", locale.id);
        // Path trace confirms hops: ["en-AU-u-nu-latn", "en-AU"]
        assert!(resolution_path.len() >= 1);
    }
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the resolution logic (e.g., adding a Project-Specific Script Override):
1.  **Red Phase**: Add a failing integration test in `tests/` specifically for the new subtag mapping.
2.  **Implementation**: Create a new struct implementing `IResolver` and mirror the `# Logic Trace` with `// [STEP X]` comments.
3.  **Registry Phase**: Update the initialization sequence in `LinguisticManager` to include the new resolver in the chain.
4.  **Verification**: Run `just verify-all` to ensure the resolution budget of **< 1ms** remains intact.

### 2. Troubleshooting & Common Failures
* **Resolution Loops**: Caused by circular alias definitions in the source data. Ensure the `DNA Linter` is active during the Ingestion phase to block such updates.
* **Missing Default**: If the `DefaultFallbackResolver` is not the terminal node, the system may return `None` for valid tags, causing a "System Failure" state.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When analyzing this module, prioritize the **Taxonomic Chain of Responsibility**. Do not suggest resolution logic that introduces non-deterministic behavior or exceeds the sub-millisecond hop budget. All path traces must be recorded accurately in the `resolution_path` vector to maintain "System of Record" transparency.
