# LMS-TEST: Testing First Philosophy & Standards

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026
<br> **Status:** Engineering Standard

---

## I. Overview: Testing-First Philosophy

The Bistun LMS project treats tests as **executable specifications** of the "Global Truth." To ensure our system remains a "System of Record" that is 100% reliable, we adopt a **Testing-First Philosophy**. Code that is not tested is "broken by design."

### Core Principles
1.  **Red-Green-Refactor**: No feature logic is written until a failing test case exists.
2.  **Hermeticity (Total Isolation)**: Every module must be testable in a vacuum. If a module requires another module to function, that dependency **MUST** be mocked.
3.  **Documentation as Proof**: Public APIs must provide executable examples (Doctests) that prove the implementation matches the documentation.
4.  **Path Exhaustion**: Tests are not written for "functions"; they are written for **code paths**. Every `if`, `match`, and `loop` variant must have a dedicated test case.

---

## II. The Testing Hierarchy

We utilize Rust's three-tier testing architecture to maintain speed and correctness.

| Tier | Location | Scope | Responsibility |
| :--- | :--- | :--- | :--- |
| **Unit Tests** | `src/**` (Internal) | Private & Public Logic | Verifies internal state and isolated logic paths. |
| **Integration Tests** | `tests/**` (External) | Public API Only | Verifies the "Physics" of the full system integration. |
| **Documentation Tests** | `src/**` (`///`) | Example Usage | Proves that API usage documentation is correct. |

---

## III. Hermetic Testing & Mocking Standards

To achieve world-class isolation, the LMS mandates the use of **Trait-Based Dependencies** and the `mockall` crate.

### 1. The Isolation Rule
No module shall hit the real file system, network, or another module's concrete implementation during a unit test.

### 2. Mocking Strategy
* **Decoupling**: Implement shared behavior via `traits`.
* **Injection**: Pass dependencies as generic trait objects or via Dependency Injection (DI) during `Initialize()`.
* **Automation**: Use `#[mockall::automock]` on internal traits to generate test doubles automatically.

---

## IV. Implementation Template (Rust Standards)

Every module must follow this pattern to be considered "Merge Ready."

### 1. The Internal Test Module
Place unit tests at the bottom of the source file to allow testing of private functions.

```rust
// Logic Trace: [STEP 1] ... [STEP 2] ...
pub fn resolve_locale(tag: &str) -> Result<Locale, Error> {
    // Implementation...
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_resolve_exact_match() {
        // [1] Set up Mock
        // [2] Execute
        // [3] Assert Equation
        assert_eq!(resolve_locale("en-US").unwrap().id, "en-US");
    }

    #[test]
    #[should_panic(expected = "Invalid UTF-8")]
    fn test_panic_on_malformed_input() {
        // Verifies the "Safety" and "Panics" documentation in LMS-DOC
        resolve_locale("\u{FFFD}");
    }
}
```

### 2. Executable API Specs (Doctests)
Documentation MUST include `# Examples` which serve as tests.

```rust
/// Returns the segmentation strategy for a script.
/// 
/// # Examples
/// ```
/// let strategy = get_seg_strategy("Latn");
/// assert_eq!(strategy, SegType::SPACE);
/// ```
pub fn get_seg_strategy(script: &str) -> SegType { ... }
```

---

## V. Novice-Friendly Test Narratives

Following the **LMS-DOC** standard, every test must explain **What is being proven**.

* **Test Name**: Use descriptive snake_case names (e.g., `test_fails_if_registry_signature_mismatch`).
* **Logic Comments**: Use comments inside tests to map back to the **Logic Trace** of the function under test.
* **Golden Inputs**: Use real-world examples from the "Golden Set" (e.g., `ar-EG`, `zh-Hant`) rather than "foo/bar".

---

## VI. Quality Assurance Gates

1.  **Mandatory Mocking**: PRs will be rejected if a unit test hits the database or sidecar service directly.
2.  **100% Coverage**: New modules must demonstrate coverage of all error variants defined in the `# Errors` section of their docs.
3.  **Linter Check**: `cargo test` must pass all doctests; failing examples are considered critical bugs.
4.  **Big O Verification**: For performance-critical code ($< 1\text{ms}$), performance unit tests (benchmarks) must be included to verify Time Complexity.
