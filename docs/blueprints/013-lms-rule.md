# BLUEPRINT: Rule Synthesis Engine

![Blueprint](https://img.shields.io/badge/Blueprint-013--LMS--RULE-blue)
![Domain](https://img.shields.io/badge/Domain-Typology%20|%20Orthography-green)
![Status](https://img.shields.io/badge/Status-Production-yellow)

---

## I. Strategic Overview

### 1. The "Why"
The Rule Synthesis Engine provides a standardized mechanism for the LMS to deliver logical instructions (e.g., default transliteration targets, normalization forms) to consuming services during Phase 2 (Aggregation) of the pipeline. It bridges the gap between static **Linguistic DNA** (traits) and the **Strategy Pattern** (execution) by selecting the most appropriate rule set for a resolved locale.

### 2. System Impact
If rule synthesis fails or is structurally malformed, downstream microservices (Search, UI, NLP) will lack the necessary directives to handle complex scripts correctly. This results in text corruption, defaulting to incorrect transliteration, or broken pluralization logic for multi-script environments.

### 3. Design Patterns
* **Strategy Pattern**: Rules act as the configuration "keys" to unlock specific algorithmic strategies defined in downstream NLP services.
* **High-Water Mark Strategy**: Conflicting rules (e.g., different plural models in a multi-script locale) are resolved by selecting the rule with the highest complexity requirement.

---

## II. Technical Specification

### 1. Primary Interface / Data Schema
Integrated into Phase 2 (Aggregation), this logic populates the `rules` map in the `CapabilityManifest` by cloning the pre-computed directives from the Flyweight `LocaleProfile`.

| Method / Key | Input / Type | Output / Value | Purpose                                                        |
|:-------------|:-------------|:---------------|:---------------------------------------------------------------|
| `rules` map  | `String`     | `LmsRule`      | Determines the authoritative algorithmic rules for the locale. |

### 2. Golden Set & Standard Enums
The system strictly types execution logic under the untagged `LmsRule` enum.

* **`TransRule`**: Transliterator targets (`NONE`, `ROMANIZATION`, `PHONETIC`, `ICU_TRANSFORM`).
* **`NormRule`**: Unicode normalization standard (`NFC`, `NFD`, `NFKC`, `NFKD`).
* **`PluralRule`**: Algorithmic logic required (`CARDINAL_ONLY`, `ORDINAL_SUPPORT`, `MULTIPLE_CATEGORIES`).
* **`CasingRule`**: Collation sorting logic (`CASE_SENSITIVE`, `CASE_INSENSITIVE`, `UNICODE_SPECIAL`).

### 3. Logic & Algorithms (The Workflow)
1. **Phase 1: Ingestion**: Retrieve the base rules defined in the canonical `LocaleProfile`.
2. **Phase 2: Conflict Detection**: Identify rules derived from different scripts in multi-script manifests.
3. **Phase 3: High-Water Mark Resolution**: If conflicting `PluralRules` exist, select the one requiring the most categories.
4. **Phase 4: Injection**: Populate the `rules` map in the `CapabilityManifest` and validate against the `MORPHOLOGY_TYPE`.

---

## III. Execution & Integrity

### 1. Performance Budget
* **Target Latency**: < 0.2ms (subset of the 1ms total resolution budget).
* **Complexity**: O(1) map cloning from the `Arc` wrapped `LocaleProfile`.

### 2. Enforcement & Validation (LMS-VAL)
* **Level A (Strict)**: Rules must be verified for compatibility during registry curation (e.g., a `TEMPLATIC` language cannot use a `NONE` transliteration rule).
* **Level C (Runtime)**: The `integrity.rs` orchestrator ensures that every manifest strictly includes at least one `NORMALIZATION_DEFAULT` rule before returning the payload.

### 3. Observability (LMS-OPS)
* **Telemetry Keys**: `rule_synthesis_count`, `rule_conflict_resolved_total`.
* **Alert Thresholds**: Any resolution time $> 2\text{ms}$ triggers a critical SLI alert.

---

## IV. Implementation Example

```json
{
  "rules": {
    "NORMALIZATION_DEFAULT": "NFC",
    "TRANSLITERATION_DEFAULT": "NONE",
    "PLURAL_LOGIC": "MULTIPLE_CATEGORIES",
    "CASING_STRATEGY": "CASE_SENSITIVE"
  }
}

```

---

## V. Metadata

* **Author**: Francis Xavier Wazeter IV
* **Version**: 2.0.0
* **Blueprint Ref**: [[013-LMS-RULE]]
* **Last Updated**: 2026-05-12
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents

When analyzing this blueprint, prioritize the **System of Record** philosophy. Every change must be narrated as a permanent update to a linguistic truth. Do not suggest implementations that breach the p99 resolution target or lack a clear **# Logic Trace**.