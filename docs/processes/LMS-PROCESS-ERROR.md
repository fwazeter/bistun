# LMS-PROCESS-ERROR: Error Handling Philosophy & Standards

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/29/2026  
> **Status:** Operational Process Guide

---

## I. Overview

As a **System of Record** for Linguistic DNA, the Bistun LMS must treat failures with the same narrative rigor as successful operations. This guide establishes a standardized philosophy for error propagation, categorization, and narration, ensuring that even a novice can diagnose a failure in the 5-phase pipeline without deep-diving into the source code.

**Location**: `docs/processes/LMS-PROCESS-ERROR.md`

---

## II. Framework Selection: `thiserror`

We utilize the `thiserror` crate for library-level error definitions.

* **Logic**: `thiserror` allows us to derive descriptive, strongly-typed errors that maintain their context across module boundaries while remaining lightweight.
* **Requirement**: All public-facing modules (Core, Data, Strategy) must return a `Result<T, LmsError>`.

---

## III. The "Phase-Linked" Error Standard

Every error variant must explicitly link back to one of the **5-Phase Pipeline** stages or the **SDK State Machine**.

### 1. Error Categories
* **`ResolutionError`**: Triggered during Phase 1 (Resolve) when a BCP 47 tag cannot be mapped to a registry entry.
* **`AggregationError`**: Triggered during Phase 2 (Aggregate) if the "Physics" of scripts create a logical contradiction (e.g., conflicting High-Water Marks).
* **`OverrideError`**: Triggered during Phase 3 (Override) if a `-u-` extension contains malformed or unsupported values.
* **`IntegrityError`**: Triggered during Phase 4 (Integrity Check) if the resulting DTO violates the Golden Set schema.
* **`SyncError`**: Triggered during the SDK **BOOTSTRAPPING** or **SYNCING** states if JWS verification fails.

---

## IV. The Failure Narrative (LMS-DOC Alignment)

Following the **LMS-DOC** standard, every error message must answer three questions for the novice:
1.  **What failed?** (The specific trait or key).
2.  **Where did it fail?** (The pipeline step).
3.  **Why did it fail?** (The logic violation).

### Example Error Definition
```rust
#[derive(thiserror::Error, Debug)]
pub enum LmsError {
    #[error("[PHASE 1: RESOLVE] Failed to resolve tag '{tag}': {reason}")]
    ResolutionError { tag: String, reason: String },

    #[error("[PHASE 4: INTEGRITY] Manifest for '{locale}' is missing Golden Key '{key}'")]
    IntegrityCheckFailed { locale: String, key: String },
}
```

---

## V. Failure Logic Trace

When implementing a function, use the following "Logic Trace" for failure states:
1.  **Step 1**: Detect the anomaly (e.g., a null trait).
2.  **Step 2**: Wrap the anomaly in the corresponding `LmsError` variant.
3.  **Step 3**: Annotate with the current pipeline context (e.g., the input locale tag).
4.  **Step 4**: Bubble the error up to the `LinguisticManager` for telemetry recording.

---

## VI. Quality Assurance Gates

* **Exhaustive Testing**: Every error variant defined in a module must have a corresponding unit test that triggers it.
* **Narrative Documentation**: The `# Errors` section in a function's documentation must link to the specific `LmsError` variants it returns.
