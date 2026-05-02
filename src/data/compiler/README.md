# MODULE-README: WORM Compiler & Ingestion Engine

![Blueprint](https://img.shields.io/badge/Blueprint-005--LMS--INGEST-blue)
![Blueprint](https://img.shields.io/badge/Blueprint-002--LMS--DATA-blue)
![Domain](https://img.shields.io/badge/Domain-Taxonomy%20|%20Typology%20|%20Orthography-green)
![Location](https://img.shields.io/badge/Location-src%2Fdata%2Fcompiler-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module processes raw data from international standards (ISO/CLDR), applies the deterministic **Truth Hierarchy**, and compiles the finalized, cryptographically signed WORM snapshot. It serves as a standalone build-time domain that ensures only verified linguistic DNA enters the production registry.

### 2. System Impact
As the primary "Ingestion Gate," failure in this module results in malformed or contradictory data reaching the runtime memory pools. This can lead to "Linguistic Chimeras"—locales with impossible trait combinations—causing catastrophic UI layout failures or NLP engine crashes in downstream services.

### 3. Design Patterns
* **WORM (Write-Once, Read-Many)**: A storage philosophy ensuring that data is never edited in place; new versions are created to provide an immutable audit trail.
* **Facade Pattern**: The `RegistryCompiler` provides a simplified interface to the complex multi-standard ingestion and merging logic.
* **Truth Hierarchy**: A deterministic conflict resolution strategy (Tier 1: Manual > Tier 2: CLDR > Tier 3: ISO) used to unify disparate data sources.

### 4. Local Glossary
* **WORM**: Write-Once, Read-Many. Immutability standard for the linguistic registry.
* **Truth Hierarchy**: The ranking system used to resolve data conflicts between international standards and manual overrides.
* **Linguistic Chimera**: A locale profile containing contradictory traits (e.g., a language marked as `ISOLATING` but requiring `STEMMING`).

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method               | Input                             | Output                                 | Purpose                                             |
|:------------------------------|:----------------------------------|:---------------------------------------|:----------------------------------------------------|
| `RegistryCompiler::compile()` | Internal state (ISO/CLDR strings) | `Result<Vec<LocaleProfile>, LmsError>` | Merges standards and applies the Truth Hierarchy.   |
| `validate_profile()`          | `&LocaleProfile`                  | `Result<(), CompilerError>`            | Enforces typological integrity checks (DNA Linter). |

### 2. Side Effects & SLIs
* **Telemetry**: Records compiler throughput and linter rejection rates to track the health of automated ingestion pipelines.
* **Performance**: As an offline tool, performance targets focus on **Integrity** rather than latency. However, validation must remain efficient enough to support thousands of locales.
* **Dependencies**: Relies on `serde_json` for standard parsing and `hashbrown` for profile indexing during the merge phase.

### 3. Quirks & Invariants
* **Manual Override Primacy**: Any field manually curated (Tier 1) is immutable to the automated scrapers (Tier 2/3).
* **Invariant**: A `LocaleProfile` must pass the `DNA Linter` before it can be appended to a WORM snapshot.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::data::compiler::ingest::RegistryCompiler;

fn main() {
    // Initialize the compiler facade
    let compiler = RegistryCompiler::new();
    
    // Explicit use to satisfy compiler; verifies empty state before ingestion
    assert!(compiler.is_empty());
}
```

### 2. The "Golden Path" Example
```rust
use crate::data::compiler::ingest::RegistryCompiler;
use crate::data::compiler::linter::validate_profile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Ingest standards via the Truth Hierarchy
    let profiles = RegistryCompiler::new()
        .with_iso_639_3("id\t...\tChinese")
        .with_cldr_scripts(r#"{"scriptMetadata": {...}}"#)
        .compile()?;

    // 2. Validate a specific profile against the DNA Linter
    if let Some(zh) = profiles.first() {
        validate_profile(zh)?;
        println!("Profile {} passed typological integrity.", zh.id);
    }
    
    Ok(())
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the compiler logic (e.g., adding a new `ISO 15924` scraper or a new linter rule):
1.  **Red Phase**: Add a failing test case in `ingest.rs` (for new data) or `linter.rs` (for a new rule).
2.  **Implementation**: Update the `RegistryCompiler` builder or the `validate_profile` contradiction matrix.
3.  **Trace**: Mirror the `# Logic Trace` with `// [STEP X]` comments to ensure the update to the System of Record is narrated.
4.  **Verification**: Run `just verify-all` to ensure the compilation results match the "Golden Set".

### 2. Troubleshooting & Common Failures
* **Typological Contradiction**: Occurs when raw data implies an impossible state (e.g., TTB direction with SPACE-based segmentation). The linter will block the build.
* **Integrity Violation**: Triggered if the compiler attempts to output a registry missing critical "Golden Set" locales.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When modifying the compiler or linter, you MUST NOT relax the **Typological Integrity** rules to "fix" a build failure. Any contradiction detected by the DNA Linter represents a fundamental error in the source data or manual curation. Correct the **Truth Hierarchy** source instead of the validation logic.
