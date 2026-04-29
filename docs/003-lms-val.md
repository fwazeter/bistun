# LMS-VAL: Linguistic Validation Rules (The Linter)

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026

**Status:** Implementation Reference

---

## I. Overview

The **Linguistic Linter** is the core validation engine responsible for maintaining the integrity of the "Linguistic DNA". It serves as the pre-persistence validator within the `LinguisticRepository`, with the primary goal of preventing "Linguistic Chimera" data—logical contradictions where a language's genetic traits do not align with its technical implementation or historical standards.

---

## II. The Consistency Matrix (Genetic vs. Logic)

The system enforces compatibility between a language's `Morphology_Type` and its executable software strategies. Entries that violate these pairings must be rejected.

| Morphology Type | Allowed Stemming Strategy | Invalid Strategy (Automatic Fail) |
| :--- | :--- | :--- |
| `ISOLATING` | `IdentityStrategy` | `RootExtraction`, `SuffixStripping` |
| `AGGLUTINATIVE` | `SuffixStrippingStrategy` | `RootExtraction` |
| `TEMPLATIC` | `RootExtractionStrategy` | `SuffixStripping` |
| `FUSIONAL` | `Identity` or `CustomFusional` | `RootExtraction` |



---

## III. Structural Integrity Rules

These rules ensure that the metadata follows the international standards defined in the Master Plan.

1.  **Identity Uniqueness**: A `Language_ID` must strictly follow ISO 639-3 (3-letter lowercase) or the private-use extension `qaa-qtz`.
2.  **Script Format**: `Script_ID` must be exactly 4 letters (Title Case) as per ISO 15924 (e.g., `Hebr`, `Latn`).
3.  **Locale Canonicalization**: A `Locale_Tag` must be a valid BCP 47 string. If an alias is used, the resolver must have a corresponding entry in the `ScriptAliasResolver`.

---

## IV. The "High-Water Mark" Constraints

Validation logic for the `TraitAggregator` to ensure multi-script locales do not have conflicting render logic.

* **BIDI Consistency**: If any `Script_Definition` in the `Script_Manifest` has a `Directionality` of `RTL`, the resulting `CapabilityManifest` **MUST** have `HAS_BIDI_ELEMENTS: true`.
* **Segmentation Hierarchy**: If a locale contains a script requiring `DICTIONARY` segmentation (e.g., Thai), the `Linter` prevents the manifest from being saved with a lower-rank strategy like `SPACE`.

---

## V. Enforcement Lifecycle (Refined)

The Linter operates at different "Strictness Levels" depending on the pipeline phase to balance absolute integrity with high-performance requirements.

### 1. Level A: DNA Validation (Strict)
* **When**: Ingestion Phase & Curator UI Submission.
* **Checks**: Consistency Matrix (e.g., Agglutinative vs. SuffixStripping), ISO 639-3 format, and BIDI consistency.
* **Action**: Hard Failure. Blocks registry version creation.

### 2. Level B: Sync Validation (Integrity)
* **When**: Sidecar SDK `Sync()`.
* **Checks**: JWS Signature verification and JSON Schema structural integrity.
* **Action**: Abort Swap. Revert to previous version.

### 3. Level C: Runtime Validation (Lightweight)
* **When**: `GetManifest()` request.
* **Checks**: Ensures the `resolved_locale` exists and the `traits` map contains "Golden Set" keys.
* **Action**: Log warning; return manifest (Optimized for $< 1\text{ms}$ path).

---

## VI. Critical Error Policy

Any record failing a **Level A (Consistency Matrix)** check results in a hard failure: the registry update is blocked, and the existing "Active Production" version remains pinned to ensure zero-downtime stability.
