# LMS-TYPOLOGY: Trait Aggregator & High-Water Mark Logic

> **Version:** 0.1.4

---

## I. Overview

The Trait Aggregator is the computational core of the Capability Engine. Its purpose is to merge the **Typological** traits of a language with the **Orthographic** mechanics of its associated scripts into a single, decoupled DTO. This process utilizes specific logical filters to ensure the resulting manifest accurately reflects the technical requirements of complex, multi-script locales.

---

## II. The `TraitAggregator` Class

This class processes the **Typological Identity** (Language Definition) and the **Orthographic Mechanics** (Script Manifest) retrieved by the Logical Engine.

### 1. Functional Input

* **`language`**: The immutable **Typological** record (e.g., `Morphology_Type`, `Synthesis_Degree`).
* **`scripts`**: A priority-ordered list of **Orthographic** definitions (e.g., `[Hani, Hira, Kana]`).

### 2. The `Aggregate()` Workflow

The aggregator executes a three-tier logic gate to populate the `traits` map:

1.  **Direct Mapping**: One-to-one transfer of typology-specific traits.
2.  **Positional Priority**: Selection of UI-critical traits from the primary script.
3.  **Cumulative Union**: Aggregation of rendering requirements across all scripts.

---

## III. Aggregation Tiers

### Tier 1: Positional Priority (The Primary Context)

For UI rendering, the engine must determine a single base direction.

* **Logic**: The `PRIMARY_DIRECTION` key is populated using the `Directionality` value from `scripts[0]`.
* **Impact**: In a manifest for Japanese (`[Hani, Hira, Kana, Latn]`), if `Hani` (LTR) is first, the UI remains LTR regardless of subsequent script properties.

### Tier 2: Cumulative Union (Global Flags)

Flags that impact the entire rendering pipeline or search indexer use a "Boolean OR" logic.

* **`HAS_BIDI_ELEMENTS`**: Set to `true` if *any* script in the manifest is `RTL` or `BIDI`.
* **`REQUIRES_SHAPING`**: Set to `true` if *any* script in the manifest requires contextual shaping (e.g., Arabic, Syriac).
* **`UNICODE_PRELOAD_BLOCKS`**: A mathematical union of all `Unicode_Registry_Ref` values from the script list, de-duplicated to create a final font-loading array.

---

## IV. The High-Water Mark Strategy (Complexity Ranking)

To resolve conflicts in multi-script locales, the Aggregator uses a **Complexity Hierarchy** for the `SEGMENTATION_STRATEGY` trait.

### 1. Complexity Ranking Table (Lowest to Highest)
1.  **`NONE`**: Scriptio continua (e.g., Ancient Greek).
2.  **`SPACE`**: Standard whitespace (e.g., Latin/Cyrillic).
3.  **`CHARACTER`**: Grapheme cluster boundaries (e.g., CJK).
4.  **`DICTIONARY`**: Statistical/Lexical analysis (e.g., Thai/Lao).

### 2. Resolution Logic
The Aggregator iterates through the `Script_Manifest` and selects the strategy with the **highest ordinal rank**. This ensures that if a locale contains *any* script requiring dictionary-based segmentation, the entire manifest supports it to prevent text corruption.

---

## V. Final Trait Synthesis

Once tiers are processed, the aggregator merges the `Morphology_Type` and `Synthesis_Degree` from the **Typological record** to complete the "Golden Set" of keys defined in the API contract.

---

**Author**: Francis Xavier Wazeter IV  
**License**: GNU GPL v3  
**Date Created**: 04/29/2026  
**Date Updated**: 04/30/2026