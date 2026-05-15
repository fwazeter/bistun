# LMS-DOC: Code Documentation & Inline Specification

> **Version:** 1.1.0
> **Status:** Production Engineering Standard
> **Author:** Francis Xavier Wazeter IV
> **Compiler Target:** Rust 1.95+ (Strict Clippy Compliance)

---

## I. Overview

The Bistun LMS project adheres to a **"Narrative Code"** philosophy. Documentation is not "extra"—it is the **System of Record**. These standards ensure that every module is self-describing, performance-optimized, and compliant with modern Rust linting groups (`all`, `pedantic`, `nursery`).

---

## II. The Documentation Hierarchy

### 1. Module-Level Documentation (`//!`)

Every file must begin with a `//!` block.

* **Crate**: The specific crate name (e.g., `Crate: bistun-core`).
* **Blueprint Reference**: Link to the specific implementation blueprint (e.g., `Ref: [011-LMS-DTO]`).
* **Location**: The authoritative file location enclosed in backticks (e.g., `Location: crates/bistun-core/src/manifest.rs`).
* **The "Why" Header**: A 2-sentence explanation of why this file exists and its specific role in the 5-phase resolution pipeline.
* **System Impact**: A description of what system functionality fails if this specific module is compromised.
* **Local Glossary**: Definitions for domain-specific terms.

### 2. Item-Level Documentation (`///`)

All public structs, enums, variants, fields, and functions must follow the **Triple-Slash Narrative** format.

* **Variant/Field Documentation**: Every enum variant and struct field must have a documentation line to satisfy the `missing-docs` lint.
* **Intra-Doc Links**: All technical terms, types, and constants must be enclosed in backticks (e.g., ``CapabilityManifest``, ``PRIMARY_DIRECTION``).

---

## III. The Perfect Function Template

Every function must include the following sections in order:

### 1. The Summary Line

A single, concise imperative sentence. Functions that return a value without side effects **must** be decorated with `#[must_use]`.

### 2. Complexity Badges

Time and Space complexity (e.g., `Time: O(N) | Space: O(1)`).

### 3. `# Logic Trace (Internal)`

A numbered step-by-step walkthrough.

* Use `.expect("LMS-TEST: <Reason>")` instead of `.unwrap()`.
* Use `String::new()` instead of `"".to_string()`.

### 4. `# Examples`

Compilable doctests. Types used in examples must be properly imported or scoped.

### 5. `# Arguments` / `# Returns`

Bulleted list with names, types, and narrative roles.

### 6. `# Golden I/O`

A pair of common input/output values for visual verification.

### 7. `# Errors, Panics, & Safety`

List specific [`LmsError`] variants. Document why a function might panic (e.g., poisoned locks).

---

## IV. Quality Assurance Gates (Clippy 1.95+)

1. **Backtick Rule**: Any term that is not standard English (e.g., `BCP 47`, `HashMap`, `ar-EG`) must be in backticks to pass `doc-markdown`.
2. **Unwrap Forbid**: Use of `.unwrap()` is a build-breaker. Use `.expect()` with a prefix of `LMS-TEST:` or `LMS-OPS:`.
3. **Must-Use Candidate**: If a function returns a `Result` or a DTO, `#[must_use]` is mandatory.
4. **Lint Inheritance**: Child crates must use `[lints] workspace = true` to inherit global standards from the root `Cargo.toml`.

---

## V. Implementation Example

```rust
//! # Registry Resolver Engine
//! Crate: `bistun-lms`
//! Ref: [012-LMS-ENG]
//! Location: `crates/bistun-lms/src/core/resolver/orchestrator.rs`
//! 
//! **Why**: This module converts raw user strings into internal locale keys. 
//! **Impact**: If this fails, the system defaults to `en-US`, losing regional accuracy.

/// Resolves a BCP 47 string to a [`LocaleProfile`].
/// 
/// Time: O(N) | Space: O(1)
/// 
/// # Logic Trace (Internal)
/// 1. Take the `tag` and pass it to the `ExactMatchResolver`.
/// 2. If no match, check `AliasResolver` for legacy mapping.
/// 3. Return the result or an [`LmsError::ResolutionFailed`].
///
/// # Arguments
/// * `tag` (&str): The raw BCP 47 language tag to be resolved.
///
/// # Returns
/// * `Result<Arc<LocaleProfile>, LmsError>`: The resolved immutable profile. 
/// 
/// # Errors
/// * Returns [`LmsError::InvalidTag`] if the input is malformed.
#[must_use]
pub fn resolve(tag: &str) -> Result<Arc<LocaleProfile>, LmsError> {
    // [STEP 1]: Implementation...
}
```
