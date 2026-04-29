# LMS-STRAT: The Strategy Pattern Registry

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026

**Status:** Implementation Reference

---

## I. Overview

The Strategy Pattern Registry allows the LMS SDK to decouple **Linguistic Metadata** (the "What") from **Algorithmic
Execution** (the "How"). Instead of large, hardcoded conditional blocks, the SDK maintains a registry of interchangeable
strategy objects. When a `CapabilityManifest` is processed, the engine selects the appropriate strategy based on the
manifest's traits.



---

## II. Interface Definition: `ILinguisticStrategy`

To ensure program-agnostic implementation, all linguistic algorithms must implement a common interface.

### 1. The `Execute` Method

* **Signature**: `Execute(String input, Manifest context) -> Result`.
* **Purpose**: Performs a specific linguistic operation (e.g., stemming, normalization) using the provided input and the
  traits found in the manifest.

---

## III. Strategy Registries

The SDK organizes strategies into functional "Providers." These providers act as factories that return the correct
algorithm for a given trait value.

### 1. `StemmingProvider` (Morphological Logic)

Maps the `MORPHOLOGY_TYPE` trait to a specific stemming algorithm.

| Trait Value     | Concrete Strategy         | Algorithm Description                                             |
|:----------------|:--------------------------|:------------------------------------------------------------------|
| `TEMPLATIC`     | `RootExtractionStrategy`  | Extracts triliteral or quadriliteral roots (e.g., Hebrew/Arabic). |
| `AGGLUTINATIVE` | `SuffixStrippingStrategy` | Iteratively removes bound morphemes (e.g., Turkish/Finnish).      |
| `ISOLATING`     | `IdentityStrategy`        | Returns the input unchanged, as words do not change form.         |

### 2. `SegmentationProvider` (Boundary Logic)

Maps the `SEGMENTATION_STRATEGY` (derived via the High-Water Mark logic) to a boundary detection engine.

* **`SpaceStrategy`**: Standard whitespace-based tokenization.
* **`DictionaryStrategy`**: Uses a lexical trie or statistical model (HMM/LSTM) to find boundaries in scriptio
  continua (e.g., Thai).
* **`CharacterStrategy`**: Tokenizes by individual Unicode grapheme clusters (e.g., CJK).

---

## IV. Execution Flow: The "Logic Injection"

When a consuming service (like Search or UI Rendering) needs to process text, the workflow is as follows:

1. **Request Manifest**: The service calls `LinguisticManager.GetManifest(locale)`.
2. **Retrieve Strategy**: The service passes the manifest to a Provider (e.g., `StemmingProvider.Get(manifest)`).
3. **Execute**: The service calls `.Execute(text)` on the returned strategy.

---

## V. Extensibility and Maintenance

* **Zero-Code Scaling**: To support a newly discovered language type (e.g., `POLYSYNTHETIC`), a developer only needs to
  implement the `ILinguisticStrategy` interface and register it with the `StemmingProvider`.
* **Project Overrides**: Applications can swap out standard strategies for high-performance versions (e.g., replacing a
  dictionary-based Thai segmenter with a machine-learning model) without altering the `LinguisticManager`.
