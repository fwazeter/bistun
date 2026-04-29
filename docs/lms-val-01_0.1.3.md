# Linguistic Validation Rules (The Linter)

### Version: 0.1.3

**Status:** Implementation Reference

## I. Overview

The **Linguistic Linter** is the core validation engine responsible for maintaining the integrity of the "Linguistic
DNA". It serves as the pre-persistence validator within the `LinguisticRepository`. Its primary goal is to prevent "
Linguistic Chimera" data—logical contradictions where a language's genetic traits do not align with its technical
implementation or historical standards.

---

## II. The Consistency Matrix (Genetic vs. Logic)

The system enforces compatibility between a language's `Morphology_Type` and its executable software strategies. Entries
that violate these pairings must be rejected by the API.

| Morphology Type | Allowed Stemming Strategy      | Invalid Strategy (Automatic Fail)   |
|:----------------|:-------------------------------|:------------------------------------|
| `ISOLATING`     | `IdentityStrategy`             | `RootExtraction`, `SuffixStripping` |
| `AGGLUTINATIVE` | `SuffixStrippingStrategy`      | `RootExtraction`                    |
| `TEMPLATIC`     | `RootExtractionStrategy`       | `SuffixStripping`                   |
| `FUSIONAL`      | `Identity` or `CustomFusional` | `RootExtraction`                    |

---

## III. Structural Integrity Rules

These rules ensure that the metadata follows the international standards defined in the Master Plan.

1. **Identity Uniqueness**: A `Language_ID` must strictly follow ISO 639-3 (3-letter lowercase) or the private-use
   extension `qaa-qtz`.
2. **Script Format**: `Script_ID` must be exactly 4 letters (Title Case) as per ISO 15924 (e.g., `Hebr`, `Latn`).
3. **Locale Canonicalization**: A `Locale_Tag` must be a valid BCP 47 string. If an alias is used, the resolver must
   have a corresponding entry in the `ScriptAliasResolver`.

---

## IV. The "High-Water Mark" Constraints

Validation logic for the `TraitAggregator` to ensure multi-script locales do not have conflicting render logic.

* **BIDI Consistency**: If any `Script_Definition` in the `Script_Manifest` has a `Directionality` of `RTL`, the
  resulting `CapabilityManifest` **MUST** have `HAS_BIDI_ELEMENTS: true`.
* **Segmentation Hierarchy**: If a locale contains a script requiring `DICTIONARY` segmentation (e.g., Thai), the
  `Linter` prevents the manifest from being saved with a lower-rank strategy like `SPACE`.

---

## V. Enforcement Lifecycle

The Linter is invoked at three specific points:

1. **Ingestion Phase**: During automated scraping (Phase 8), to flag anomalies in CLDR/ISO data.
2. **Curator Submission**: When a human linguist attempts to save or update a trait in the `Registry Curator UI`.
3. **Persistence Sync**: Before the `LinguisticRepository` finalizes a `SaveRegistrySnapshot`, ensuring the entire
   bundle is co-consistent.

---

## VI. Critical Error Policy

Any record failing a **Level 1 (Consistency Matrix)** check results in a hard failure: the registry update is blocked,
and the existing "Active Production" version remains pinned to ensure zero-downtime stability.
