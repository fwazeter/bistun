# The Definitive Linguistic Metadata Service (LMS) Master Plan

### Version: 0.1.4

## I. Executive Summary

The LMS is an **Atomic Capability Provider** designed to serve as the "System of Record" for the **Linguistic DNA** of
any given locale. By indexing languages via **ISO 639-3** and locales via **BCP 47**, the service abstracts linguistic
complexity into functional "traits" or "capabilities". This enables developers to write generic logic for classes of
languages rather than hardcoding rules for specific language names.

---

## II. The Multi-Standard Hierarchy

The architecture leverages three international standards to provide a comprehensive view of a locale:

1. **ISO 639-3 (The Identity):** 3-letter codes identifying all known natural languages.
2. **ISO 15924 (The Tool):** 4-letter codes identifying writing systems.
3. **BCP 47 (The Implementation):** The standard for language tags used in internet protocols.

---

## III. Detailed Data Architecture

### 1. Language_Definition (The "Genetic" Identity)

*Immutable properties of a language regardless of script.*

| Variable             | Type        | Description                                                             |
|:---------------------|:------------|:------------------------------------------------------------------------|
| **Language_ID**      | String(3-8) | **ISO 639-3 Code** (Supports private use extensions).                   |
| **Period**           | Enum        | Living, Historical, Ancient, Extinct, Constructed.                      |
| **Morphology_Type**  | Enum        | `ISOLATING`, `AGGLUTINATIVE`, `FUSIONAL`, `TEMPLATIC`, `POLYSYNTHETIC`. |
| **Synthesis_Degree** | Integer     | Complexity scale (1–10) for lemmatization logic.                        |
| **Normalization**    | Enum        | Recommended Unicode normalization (NFC/NFD).                            |

### 2. Script_Definition (The "Physics" of Rendering)

*Technical requirements of the writing system.*

| Variable             | Type      | Description                                 |
|:---------------------|:----------|:--------------------------------------------|
| **Script_ID**        | String(4) | **ISO 15924 Code** (e.g., `Phnx`).          |
| **Directionality**   | Enum      | `LTR`, `RTL`, `TTB`, `BIDI`.                |
| **Shaping_Req**      | Boolean   | True if script requires contextual shaping. |
| **Segmentation**     | Enum      | `SPACE`, `DICTIONARY`, `CHARACTER`, `NONE`. |
| **Unicode_Registry** | Ref       | Pointer to shared HEX ranges.               |

### 3. Locale_Registry (The "Mapping")

*Connects a language to its implementation in context.*

| Variable            | Type      | Description                                |
|:--------------------|:----------|:-------------------------------------------|
| **Locale_Tag**      | String    | **BCP 47 Tag** (Primary Key).              |
| **Lang_Ref**        | String(3) | Foreign Key to `Language_Definition`.      |
| **Script_Manifest** | List      | **Priority-ordered list** of `Script_IDs`. |
| **Plural_Rules**    | Enum      | Category set (e.g., `ZERO`, `MANY`).       |

---

## IV. The Capability Engine (Logic Flow)

The engine returns a **CapabilityManifest DTO**.

1. **Resolve Locale:** Execute BCP 47 fallback using a chain of resolvers.
2. **Aggregate Scripts:** Iterate through the `Script_Manifest` to determine direction and global traits.
3. **Process Overrides:** Apply BCP 47 `-u-` extensions via the **Unicode Extension Mapper**.
4. **Determine Logic Strategies:** Map traits like `Morphology_Type` to code strategies.

---

## V. Technical Architecture & Design Patterns

### 1. The Strategy Pattern (Algorithmic Selection)

The SDK uses a **Registry of Strategies** instead of hardcoded logic, invoking handlers based on linguistic traits.

### 2. The Chain of Responsibility (Locale Fallback)

Modular **Resolvers** handle the fallback mechanism (Exact Match $\rightarrow$ Truncation $\rightarrow$
Alias $\rightarrow$ Default).

### 3. The Flyweight Pattern (Memory Optimization)

`Script_Definition` and `Language_Definition` are treated as **Flyweights**, significantly reducing the SDK memory
footprint.

### 4. Sidecar SDK & Atomic Hot-Swap

Updates are loaded into a background buffer and swapped atomically to ensure concurrency safety.

---

## VI. Implementation Roadmap

1. **Phase 1 (Identity Foundation):** Establish the relational bridge between ISO 639-3, ISO 15924, and BCP 47.
2. **Phase 1.5 (Persistence & Validation Foundation):** Implement the `LinguisticRepository` with **Snapshotting** and
   the **Linguistic Linter** consistency matrix.
3. **Phase 2 (Capability Pivot):** Refactor consuming services to check the `CapabilityManifest`.
4. **Phase 3 (Morphological Enrichment):** Populate traits for anchor and ancient languages.
5. **Phase 4 (UI/Font Automation):** Link `Unicode_Blocks` to the UI for automated CSS/Font injection.
6. **Phase 5 (Sidecar SDK & Local Cache):** Deploy the versioned `SyncRegistry` and in-memory lookup table.
7. **Phase 5.5 (Security & Telemetry Implementation):** Integrate cryptographic signing for bundles and standard SLI/SLO
   metrics.
8. **Phase 6 (Registry Curator UI):** Launch management interface with **RBAC** for linguists.
9. **Phase 7 (Operational Playbook):** Implement health metrics and performance budgets.
10. **Phase 8 (Data Ingestion):** Build "ISO Scrapers" and CLDR mapping with a **Truth Hierarchy**.
11. **Phase 9 (Golden Set QA):** Establish 50 "Edge Case Locales" for regression testing.

---

## VII. Governance & Evolution

* **Truth Hierarchy:** Curated traits override automated imports (Manual > CLDR > ISO).
* **Trait Extension Pattern:** The `CapabilityManifestDTO` uses a **Map-based Container** to prevent breaking API
  contracts.
* **Verification Spec:** Proving fallback logic requires a robust testing framework and "Golden Set" validation.

