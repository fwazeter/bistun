# BLUEPRINT: V2.0.0 Linguistic Data Taxonomy & DTOs

![Blueprint](https://img.shields.io/badge/Blueprint-011--LMS--DTO-blue)
![Domain](https://img.shields.io/badge/Domain-Taxonomy-green)
![Status](https://img.shields.io/badge/Status-Production-yellow)

---

## I. Strategic Overview

### 1. The "Why"
The `CapabilityManifest` and `LocaleProfile` act as the authoritative Data Transfer Objects (DTOs) for the LMS. This v2.0.0 schema enforces a strict "Separation of Domains" (Traits, Rules, Resources, Extensions, Metadata) to prevent the "God Manifest" anti-pattern. This structure allows downstream microservices to execute decoupled logic based on language classes (e.g., "Agglutinative") without parsing string spaghetti.

### 2. System Impact
If this schema is compromised or domains are mixed (e.g., placing a URL in the traits map), downstream components receive malformed instructions, causing $O(N)$ string parsing overhead and systemic failures in UI rendering and NLP pipelines.

### 3. Design Patterns
* **Untagged Serialization**: Uses Serde `#[serde(untagged)]` to serialize enums as raw primitives, ensuring API transparency for non-Rust consumers.
* **Separation of Domains**: Strictly separates physical "Truths" (`traits`) from algorithmic "Strategies" (`rules`).
* **Flyweight Reference**: DTOs are constructed from shared, immutable definitions wrapped in `Arc` memory pools.

---

## II. Technical Specification

### 1. Primary Data Schema (Rust Structures)
To ensure memory safety and zero-cost abstraction, the DTOs are defined as strongly-typed structs.

**A. Data At Rest: `LocaleProfile`** (The WORM Registry definition)
```rust
pub struct LocaleProfile {
    pub id: String, 
    pub version: String,
    pub traits: HashMap<TraitKey, TraitValue>, // Linguistic DNA
    pub rules: HashMap<String, LmsRule>,       // Algorithmic Strategy Directives
    pub resources: HashMap<String, String>,    // Physical Asset Logical IDs
}

```

**B. Data In Motion: `CapabilityManifest**` (The API Output)

```rust
pub struct CapabilityManifest {
    pub resolved_locale: String,
    pub traits: HashMap<TraitKey, TraitValue>, // Immutable defaults from Profile
    pub rules: HashMap<String, LmsRule>,       // Synthesized strategies
    pub resources: HashMap<String, String>,    // Resolved Physical URIs
    pub extensions: HashMap<String, String>,   // User BCP 47 overrides (e.g., -u-nu-latn)
    pub metadata: HashMap<String, String>,     // Telemetry (Latency, Version)
}

```

### 2. Golden Set & Standard Enums

The system classifies all data into 5 distinct domains.

**Domain 1: `traits` (The Linguistic DNA)**
Immutable physical properties defined by ISO 639-3 and ISO 15924.

* `PRIMARY_DIRECTION`: `Direction` enum (`LTR`, `RTL`, `TTB`, `BIDI`).
* `SEGMENTATION_STRATEGY`: `SegType` enum (`NONE`, `SPACE`, `CHARACTER`, `DICTIONARY`).
* `MORPHOLOGY_TYPE`: `MorphType` enum (`ISOLATING`, `AGGLUTINATIVE`, `FUSIONAL`, `TEMPLATIC`).
* `PLURAL_CATEGORIES`: `Vec<String>` (e.g., `["one", "few", "other"]`).
* `DEFAULT_NUMBERING`: `String` (e.g., `arab`, `latn`).
* `DEFAULT_CALENDAR`: `String` (e.g., `gregory`, `islamic`).

**Domain 2: `rules` (Execution Directives)**
Instructions mapping to the Strategy Pattern for text transformation.

* `TRANSLITERATION_DEFAULT`: `TransRule` (`NONE`, `ROMANIZATION`, `ICU_TRANSFORM`).
* `NORMALIZATION_DEFAULT`: `NormRule` (`NFC`, `NFD`).
* `CASING_STRATEGY`: `CasingRule` (`CASE_SENSITIVE`, `CASE_INSENSITIVE`, `UNICODE_SPECIAL`).
* `PLURAL_LOGIC`: `PluralRule` (`CARDINAL_ONLY`, `MULTIPLE_CATEGORIES`).

**Domain 3: `resources` (Physical Assets)**
Abstract IDs mapped to actionable URLs.

* `icu_arab`: `https://cdn.../arab.postcard` (ICU4X shaping blob).
* `dict_thai`: `https://cdn.../thai.trie` (Segmentation dictionary).

**Domain 4: `extensions` (User Overrides)**
Parsed from BCP 47 subtags (`-u-`, `-t-`, `-x-`).

* `nu` -> `"latn"` (Override numbering system).
* `ca` -> `"buddhist"` (Override calendar).
* `x` -> `"legal"` (Trigger proprietary private-use plugins).

**Domain 5: `metadata` (Observability)**

* `registry_version`: The active semantic version.
* `resolution_time_ms`: Pipeline latency metric.
* `circuit_breaker`: `"true"` if running in DEGRADED mode.

### 3. Logic & Algorithms (The Workflow)

1. **Phase 1: Hydration**: `LocaleProfile` is instantiated from the WORM JSON snapshot, mapping traits, rules, and resources.
2. **Phase 2: Aggregation**: `LinguisticManager` generates a `CapabilityManifest`, $O(1)$ cloning the dictionaries from the Flyweight pool.
3. **Phase 3: Override (Extensions)**: BCP 47 strings are parsed. Subtags like `-u-nu-latn` are injected directly into the `extensions` map, ensuring the `traits` map remains culturally pure.
4. **Phase 4: Synthesis**: The Resource Bridge translates logical resource keys into fully qualified URIs in the `resources` map.

---

## III. Execution & Integrity

### 1. Performance Budget

* **Target Latency**: < 1ms p99 resolution.
* **Complexity**: O(1) map cloning from the `Arc` wrapped `LocaleProfile`.

### 2. Enforcement & Validation (LMS-VAL)

* **Level A (Strict)**: The internal Consistency Matrix prevents "Linguistic Chimeras" (e.g., an `AGGLUTINATIVE` language with `TEMPLATIC` rules) from ever persisting to the database.
* **Level C (Runtime)**: The orchestrator verifies that `traits` contains `PRIMARY_DIRECTION` before returning the payload.

### 3. Observability (LMS-OPS)

* **Telemetry Keys**: `resolution_time_ms`, `registry_version`, `resolution_path`.
* **Alert Thresholds**: Any resolution time $> 2\text{ms}$ triggers a critical SLI alert.

---

## IV. Implementation Example

```json
{
  "resolved_locale": "ar-EG-u-nu-latn",
  "traits": {
    "PRIMARY_DIRECTION": "RTL",
    "HAS_BIDI_ELEMENTS": true,
    "REQUIRES_SHAPING": true,
    "SEGMENTATION_STRATEGY": "SPACE",
    "MORPHOLOGY_TYPE": "TEMPLATIC",
    "DEFAULT_NUMBERING": "arab",
    "DEFAULT_CALENDAR": "gregory"
  },
  "rules": {
    "TRANSLITERATION_DEFAULT": "ICU_TRANSFORM",
    "NORMALIZATION_DEFAULT": "NFC",
    "PLURAL_LOGIC": "MULTIPLE_CATEGORIES"
  },
  "resources": {
    "icu_arab": "[https://cdn.bistun.io/v1/data/icu_arab.postcard](https://cdn.bistun.io/v1/data/icu_arab.postcard)"
  },
  "extensions": {
    "nu": "latn"
  },
  "metadata": {
    "registry_version": "v2.0.0",
    "resolution_time_ms": "0.45",
    "resolution_path": "[\"ar-EG-u-nu-latn\", \"ar-EG\"]"
  }
}

```

---

## V. Metadata

* **Author**: Francis Xavier Wazeter IV
* **Version**: 2.0.0
* **Blueprint Ref**: [[011-LMS-DTO]]
* **Last Updated**: 2026-05-12
* **License**: GNU GPL v3
