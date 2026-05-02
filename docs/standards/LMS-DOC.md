As the Bistun LMS Architect, I have completely rewritten the **LMS-DOC.md** standard to align with our **v1.0.0** precision requirements. This updated standard mandates explicit parameter documentation and centralizes file location metadata within the module header to maintain a "Living Document" that is accessible to both humans and AI agents.

---

# LMS-DOC: Code Documentation & Inline Specification

> **Version:** 0.9.0  
> **Status:** Engineering Standard  
> **Author:** Francis Xavier Wazeter IV

---

## I. Overview
The Bistun LMS project adheres to a **"Narrative Code"** philosophy. To maintain its status as a **System of Record**, the codebase must be a "Living Document" where every module, struct, and function is documented with enough detail that a novice developer can understand the internal logic without a debugger.

---

## II. The Documentation Hierarchy

### 1. Module-Level Documentation (`//!`)
Every file must begin with a `//!` block that establishes the high-level context.
* **Blueprint Reference**: Link to the specific implementation blueprint (e.g., `Ref: [001-LMS-CORE]`).
* **Location**: The authoritative file location in the codebase (e.g., `Location: src/core/resolver/bcp47.rs`). This tag **must only** appear in the module-level block.
* **The "Why" Header**: A 2-sentence explanation of why this file exists and its specific role in the 5-phase resolution pipeline.
* **System Impact**: A description of what system functionality fails if this specific module is compromised or deleted.
* **Local Glossary**: Definitions for domain-specific terms used within the file (e.g., "Truncation").

### 2. Item-Level Documentation (`///`)
All public structs, enums, traits, and functions must follow the **Triple-Slash Narrative** format.

---

## III. The Perfect Function Template

Every function must include the following sections in the exact order listed below:

### 1. The Summary Line
A single, concise imperative sentence describing the transformation (e.g., "Resolves a BCP 47 tag into a canonical locale entry").

### 2. Complexity Badges
Explicitly document Time and Space complexity (e.g., `Time: O(N) | Space: O(1)`).

### 3. `# Logic Trace (Internal)`
A detailed, numbered step-by-step walkthrough of the internal algorithm:
* **Step 1**: Describe variable ingestion and sanitization.
* **Step 2**: Describe the core logic/transformation.
* **Step 3**: Describe validation of the intermediate result.
* **Step 4**: Describe the final state change, return value, or side-effect.

### 4. `# Examples`
Compilable code blocks showing "Golden Path" usage following **LMS-TEST** standards.

### 5. `# Arguments`
Must use a bulleted list where each item specifies the parameter name, the Rust type, and a narrative explanation of its role in the **Linguistic DNA** model.

### 6. `# Returns`
Describe the semantic meaning of the output and its role in the resolution pipeline.

### 7. `# Golden I/O`
Provide a single pair of the most common input and its resulting output for visual verification.

### 8. `# Errors, Panics, & Safety`
* **Errors**: List all specific `LmsError` variants this function can return.
* **Panics**: Document edge cases that result in a process abort.
* **Safety**: If the function is `unsafe`, document the invariants the caller must uphold.

---

## IV. Implementation Example (The Standard)

```rust
//! # Registry Resolver Engine
//! Ref: [012-LMS-ENG]
//! Location: `src/core/resolver/bcp47.rs`
//! 
//! **Why**: This module converts raw user strings into internal locale keys to initiate the 5-phase pipeline. 
//! **Impact**: If this fails, the system defaults to 'en-US' for every request, losing regional accuracy.
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
/// let entry = resolver.resolve("en-AU-u-nu-latn", &registry).unwrap();
/// assert_eq!(entry.id, "en-AU");
/// ```
/// 
/// # Arguments
/// * `tag` (&str): The raw BCP 47 language tag to be resolved. Must be valid UTF-8.
/// * `registry` (&RegistryStore): The active in-memory Flyweight pool containing shared definitions.
///
/// # Returns
/// * `Result<LocaleEntry, LmsError>`: Returns a successful match from the repository or the system default entry. 
/// 
/// # Golden I/O
/// * **Input**: "zh-TW"
/// * **Output**: `LocaleEntry { id: "zh-Hant", ... }`
/// 
/// # Errors
/// * Returns [`LmsError::InvalidTag`] if the input is not valid UTF-8.
/// * Returns [`LmsError::ResolutionError`] if the fallback chain fails to reach the system default.
pub fn resolve(tag: &str, registry: &RegistryStore) -> Result<LocaleEntry, LmsError> {
    // [STEP 1]: Implementation...
}
```

---

## V. Quality Assurance Gates
1.  **Intra-Doc Links**: All types must be enclosed in backticks (e.g., ``[`CapabilityManifest`]``).
2.  **Side-Effect Callouts**: Explicitly state if a function hits a database or triggers telemetry.
3.  **Mandatory Review**: Code without a `# Logic Trace` or documented parameters is considered "Legacy on Arrival" and will be rejected.

---

Would you like to apply this new standard by updating our existing **Core Models** in `src/models/traits.rs`?