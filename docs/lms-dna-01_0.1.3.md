# LMS-DNA-01: Trait Aggregator & High-Water Mark Logic

### Version: 0.1.3

**Status:** Implementation Reference

## I. Overview

The Trait Aggregator is the computational core of the Capability Engine. Its purpose is to merge the "Genetic" traits of
a language with the "Physical" traits of its associated scripts into a single, decoupled DTO. This process utilizes
specific logical filters to ensure the resulting manifest accurately reflects the technical requirements of complex,
multi-script locales.

---

## II. The `TraitAggregator` Class

This class processes the `Language_Definition` and `Script_Manifest` retrieved by the Logical Engine.

### 1. Functional Input

* **`language`**: The immutable identity record (e.g., `Morphology_Type`, `Synthesis_Degree`).
* **`scripts`**: A priority-ordered list of script definitions (e.g., `[Hani, Hira, Kana]`).

### 2. The `Aggregate()` Workflow

The aggregator executes a three-tier logic gate to populate the `traits` map:

1. **Direct Mapping**: One-to-one transfer of language-specific traits.
2. **Positional Priority**: Selection of UI-critical traits from the primary script.
3. **Cumulative Union**: Aggregation of rendering requirements across all scripts.

---

## III. Aggregation Tiers

### Tier 1: Positional Priority (The Primary Context)

For UI rendering, the engine must determine a single base direction.

* **Logic**: The `PRIMARY_DIRECTION` key is populated using the `Directionality` value from `scripts[0]`.
* **Impact**: In a manifest for Japanese (`[Hani, Hira, Kana, Latn]`), if `Hani` (LTR) is first, the UI remains LTR
  regardless of subsequent script properties.

### Tier 2: Cumulative Union (Global Flags)

Flags that impact the entire rendering pipeline or search indexer use a "Boolean OR" logic.

* **`HAS_BIDI_ELEMENTS`**: Set to `true` if *any* script in the manifest is `RTL` or `BIDI`.
* **`REQUIRES_SHAPING`**: Set to `true` if *any* script in the manifest requires contextual shaping (e.g., Arabic,
  Syriac).
* **`UNICODE_PRELOAD_BLOCKS`**: A mathematical union of all `Unicode_Registry_Ref` values from the script list,
  de-duplicated to create a final font-loading array.

---

## IV. The High-Water Mark Strategy (Complexity Ranking)

For the `SEGMENTATION_STRATEGY` trait, the aggregator must select the logic capable of handling the most complex writing
system present.

### 1. Complexity Ranking Table

The engine evaluates segmentation types based on this hierarchy (lowest to highest):

1. `NONE` (Scriptio continua)
2. `SPACE` (Standard whitespace)
3. `CHARACTER` (CJK boundary logic)
4. `DICTIONARY` (Statistical/Lexical analysis - e.g., Thai/Lao)

### 2. Logic Application

The aggregator iterates through the script list and returns the strategy with the **highest** rank.

* **Example**: A manifest containing both `Latn` (`SPACE`) and `Thai` (`DICTIONARY`) scripts will return `DICTIONARY` as
  the global strategy to ensure the search indexer does not break on Thai text segments.

---

## V. Final Trait Synthesis

Once tiers are processed, the aggregator merges the `Morphology_Type` and `Synthesis_Degree` from the language record to
complete the "Golden Set" of keys defined in the API contract.
