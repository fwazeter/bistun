# LMS-GLOSSARY: The Linguistic & Engineering Dictionary

> **Version:** 0.1.4
<br> **Status:** Engineering Standard

---

## I. Overview

This glossary serves as the authoritative definition of terms used within the Bistun Linguistic Metadata Service (LMS). It ensures that contributors (human) and reasoning engines (AI) share a consistent mental model of the "System of Record".

---

## II. Architectural Context (System Design)

These terms define the high-level philosophy and design patterns of the LMS.

* **System of Record**: The authoritative, single source of truth for linguistic data, ensuring immutability and reproducibility through versioned snapshots.
* **Atomic Capability Provider**: The service's role in delivering functional "Linguistic DNA" (traits) rather than just static text, enabling software to adapt to any language class.
* **Taxonomy**: The scientific classification of locales and language tags (BCP 47), handled primarily in the Resolution phase of the pipeline.
* **Typology**: The study of structural language traits (ISO 639-3), such as morphology and synthesis, handled during the Aggregation phase.
* **Orthography**: The technical mechanics of writing systems (ISO 15924), including rendering requirements like directionality and shaping.
* **5-Phase Pipeline**: The standard execution sequence for resolving a manifest: Resolve → Aggregate → Override → Integrity → Telemetry.
* **Chain of Responsibility**: A design pattern where a request (locale resolution) is passed through a chain of resolvers (Exact, Truncation, Alias, Default) until handled.
* **Strategy Pattern**: An architectural pattern that allows the SDK to swap linguistic algorithms (e.g., stemming) at runtime based on the manifest's traits.
* **Flyweight Pattern**: A memory optimization technique where shared linguistic definitions are stored once and referenced by multiple locales, reducing memory footprint by $>80\%$.
* **Atomic Reference Swap**: A thread-safe mechanism that hot-reloads the entire in-memory registry by swapping a single pointer to a new "Shadow Registry".
* **High-Water Mark**: A conflict resolution strategy that selects the most complex requirement (e.g., `DICTIONARY` segmentation) when merging multi-script locales.
* **Circuit Breaker**: A safety state (`DEGRADED`) that prevents application failure by falling back to hardcoded system defaults if synchronization fails.
* **WORM (Write-Once, Read-Many)**: A storage philosophy where data is never edited in place; instead, new versions are created to ensure an immutable audit trail.

---

## III. Linguistic Context (The Science)

These terms define the scientific standards and traits used to categorize languages.

* **BCP 47**: The international standard for language tags used to identify locales in software (e.g., `en-AU`).
* **ISO 639-3**: A set of 3-letter codes providing a comprehensive list of all known natural languages (The "Identity").
* **ISO 15924**: The standard for 4-letter codes representing writing systems/scripts (The "Tool").
* **Morphology Type**: A classification of how a language forms words (e.g., `ISOLATING`, `AGGLUTINATIVE`, `TEMPLATIC`).
* **Synthesis Degree**: A numerical scale (1–10) used to indicate the complexity of a language's word-formation for stemming logic.
* **Segmentation**: The process of determining boundaries in text (e.g., `SPACE`, `CHARACTER`, or `DICTIONARY` for scripts like Thai).
* **Contextual Shaping**: A rendering requirement where character glyphs change shape based on their position or surrounding characters (e.g., Arabic).
* **Unicode Extension (-u-)**: A BCP 47 subtag allowing users to specify preferences like numbering systems or calendars that override default traits.

---

## IV. Engineering Context (The Workflow)

These terms define the rigorous engineering standards for development and quality.

* **LMS-DOC**: The "Narrative Code" standard requiring module headers, glossaries, and step-by-step logic traces in all documentation.
* **LMS-TEST**: The "Testing-First" standard enforcing hermetic isolation, trait-based mocking, and path exhaustion.
* **LMS-AI**: The standard for aligning AI coding agents with the project's architectural reasoning and constraints.
* **Logic Trace**: A numbered, step-by-step walkthrough of a function's internal algorithm, documented both in comments and PR summaries.
* **Golden I/O**: The authoritative example of an input and its expected output used for visual and automated verification.
* **Hermetic Testing**: Tests performed in total isolation where all external dependencies (I/O, other modules) are replaced with mocks.
* **LMS-GATE**: The automated suite of quality checks (Fmt, Test, Lint, Doc) triggered locally via `just verify-all`.

---

**Author**: Francis Xavier Wazeter IV  
**License**: GNU GPL v3  
**Date Created**: 04/30/2026  
**Date Updated**: 04/30/2026