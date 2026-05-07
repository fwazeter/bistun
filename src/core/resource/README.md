# MODULE-README: Resource Resolution Domain

![Blueprint](https://img.shields.io/badge/Blueprint-001--LMS--CORE-blue)
![Domain](https://img.shields.io/badge/Domain-Resource-green)
![Location](https://img.shields.io/badge/Location-src%2Fcore%2Fresource-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module executes **Phase 2.5 (Resource Mapping)** of the pipeline. While the Typological Aggregator identifies *what* abstract resource a script requires (e.g., `icu_thai`), the Resource Resolver maps that abstract ID to a physical, environment-specific URI (e.g., `https://dev.cdn.bistun.io/v1/icu_thai.dat`).

### 2. System Impact
If this module fails or is bypassed, downstream clients (like UI components or search indexers) will not know where to download the binary data blobs needed for dictionary-based segmentation or complex transliteration, breaking advanced NLP capabilities.

### 3. Design Patterns
* **The Bridge/Pointer Pattern**: The Sidecar intentionally does *not* serve large `.dat` files via its API to protect its `< 1ms` latency budget. Instead, it acts as a directory, synthesizing a pointer URI that tells the client where to fetch the payload from a static CDN.

## II. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 1.0.0
* **License**: GNU GPL v3