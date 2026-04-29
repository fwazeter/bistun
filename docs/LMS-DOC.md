# LMS-DOC: Code Documentation & Inline Specification

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026
<br> **Status:** Engineering Standard

---

## I. Overview

The Bistun LMS project adheres to a **"Narrative Code"** philosophy. To maintain its status as a **System of Record**, the codebase must be a "Living Document" where every module, struct, and function is documented with enough detail that a novice developer can understand the internal logic without a debugger. This standard is modeled after Rust's intensive documentation patterns (`rustdoc`) but extends them with mandatory internal logic tracing and high-utility novice context.

---

## II. The Documentation Hierarchy

### 1. Module-Level Documentation (`//!`)
Every file must begin with a `//!` block that establishes the high-level context.
* **Blueprint Reference**: Link to the specific implementation blueprint (e.g., `Ref: [001-LMS-CORE]`).
* **The "Why" Header**: A 2-sentence explanation of why this file exists and its specific role in the 5-phase resolution pipeline.
* **System Impact**: A description of what system functionality fails if this specific module is compromised or deleted.
* **Local Glossary**: Definitions for domain-specific terms used within the file (e.g., "High-Water Mark", "Trait Aggregation").

### 2. Item-Level Documentation (`///`)
All public structs, enums, traits, and functions must follow the **Triple-Slash Narrative** format.

---

## III. The Perfect Function Template

Every function must include the following sections in the exact order listed below:

### 1. The Summary Line
A single, concise imperative sentence describing the transformation (e.g., "Resolves a BCP 47 tag into a canonical locale entry").

### 2. Complexity Badges
Explicitly document Time and Space complexity at the top of the function block (e.g., `Time: O(1) | Space: O(N)`).

### 3. `# Logic Trace (Internal)`
A detailed, numbered step-by-step walkthrough of the internal algorithm.
* **Step 1**: Describe variable ingestion and sanitization.
* **Step 2**: Describe the core logic/transformation (e.g., "Iterate through the script pool").
* **Step 3**: Describe validation of the intermediate result.
* **Step 4**: Describe the final state change, return value, or side-effect (Telemetry/IO).

### 4. `# Examples`
Compilable code blocks showing "Golden Path" usage.

### 5. `# Arguments & Returns`
* **Arguments**: List each parameter, its type, and its purpose.
* **Returns**: Describe the output and what it represents semantically.
* **Golden I/O**: Provide a single pair of the most common input and its resulting output for visual verification.

### 6. `# Errors, Panics, & Safety`
* **Errors**: List all specific `LmsError` variants this function can return.
* **Panics**: Document edge cases that result in a process abort or panic.
* **Safety**: If the function is `unsafe`, document the invariants the caller must uphold.

---

## IV. Markup & Visual Standards

### 1. Internal Step & Hidden Tags
To explain logic within the code without cluttering public API views:
* **Implementation Details**: Use the `/// # Implementation Detail` header for logic critical for developers but not consumers.
* **In-Code Comments**: Within the function body, use comments that map back to the `# Logic Trace` (e.g., `// [STEP 1]: Sanitize input`).

### 2. Diagram Integration
Complex state transitions (e.g., the `LinguisticManager` state machine) must be documented using **Mermaid.js** blocks:
```rust
/// ```mermaid
/// graph TD
///   A[Uninitialized] --> B[Bootstrapping]
///   B --> C{Sync Success?}
///   C -->|Yes| D[Ready]
///   C -->|No| E[Degraded]
/// ```
```

---

## V. Implementation Example (The "Standard")

```rust
//! # Registry Resolver Engine
//! Ref: [012-LMS-ENG]
//!
//! **Why**: This module converts raw user strings into internal locale keys. 
//! **Impact**: If this fails, the system defaults to 'en-US' for every request.
//!
//! ### Glossary
//! * **Truncation**: The process of stripping BCP 47 subtags from right-to-left.

/// Resolves a BCP 47 string to a [`LocaleEntry`].
/// 
/// Time: O(N) | Space: O(1)
/// 
/// # Logic Trace (Internal)
/// 1. Take the `tag` and pass it to the `ExactMatchResolver`.
/// 2. If no match, iterate via `TruncationResolver` stripping subtags.
/// 3. If no match, check the `AliasResolver` for legacy mapping.
/// 4. Fallback to `DefaultResolver` if all else fails.
///
/// # Examples
/// ```rust
/// let entry = resolver.resolve("en-AU-u-nu-latn");
/// assert_eq!(entry.id, "en-AU");
/// ```
/// 
/// # Golden I/O
/// * **Input**: "zh-TW"
/// * **Output**: `LocaleEntry { id: "zh-Hant", ... }`
/// 
/// # Errors
/// * Returns `LmsError::InvalidTag` if the input is not valid UTF-8.
pub fn resolve(tag: &str) -> Result<LocaleEntry, LmsError> {
    // [STEP 1]: Implementation here...
}
```

---

## VI. Quality Assurance Gates

1. **Intra-Doc Links**: All types must be enclosed in backticks (e.g., ``[`CapabilityManifest`]``) to ensure the documentation is fully hyperlinked.
2. **Side-Effect Callouts**: Explicitly state if a function hits a database, acquires a mutex lock, or triggers telemetry.
3. **Cargo Doc Check**: The CI pipeline must run `cargo doc --no-deps` and fail on any warnings.
4. **Mandatory Review**: Code without a `# Logic Trace` is considered "Legacy on Arrival" and will be rejected.