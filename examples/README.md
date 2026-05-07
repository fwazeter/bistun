# Bistun Linguistic Metadata Service (LMS)

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-GPLv3-blue)

The **Bistun Linguistic Metadata Service (LMS)** is a high-performance, cryptographically secure Rust microservice designed to act as the "System of Record" for linguistic and orthographic metadata. It resolves complex BCP 47 language tags into explicit rendering and processing instructions (`CapabilityManifest`) for downstream NLP engines, UI frameworks, and typesetting systems.

By utilizing an immutable Flyweight memory pool and wait-free atomic pointers, the LMS guarantees massive concurrency with a `p99` latency budget strictly `< 1ms`.

---

## Architecture Overview

The Bistun LMS operates on a **5-Phase Resolution Pipeline**:
1. **Taxonomic Resolution (Phase 1):** Cleanses, standardizes, and truncates BCP 47 tags (e.g., `en-US-u-ca-gregory` → `en-US`), handling legacy aliases (e.g., `in` → `id`).
2. **Typological Aggregation (Phase 2):** Hydrates baseline grammatical and segmentation rules from the Flyweight pool.
3. **Resource Mapping (Phase 2.5):** Translates abstract resource dependencies into environment-specific CDN pointers.
4. **Orthographic Override (Phase 3):** Dynamically applies regional script variations and `-u-` extension overrides.
5. **Integrity Validation (Phase 4):** Ensures the resulting matrix of rendering instructions contains no mechanical contradictions.
6. **Telemetry (Phase 5):** Records `resolution_path` and `resolution_time_ms` for operational visibility.

---

## Getting Started

### Bootstrapping the System of Record

1. **Generate your cryptographic keys and sign the snapshot:**
   ```bash
   cargo run --bin curator
   ```
2. **Pin your authoritative key:** Copy the generated public key from the terminal output and update the `CURATOR_PUBLIC_KEY` constant in `examples/sidecar.rs`.
3. **Run the Sidecar API:**
   ```bash
   cargo run --example sidecar
   ```

### Verifying the Installation

Once the sidecar is running on port `8080`, open a new terminal window and run the following acceptance tests to verify the pipeline. *(Pro-tip: append ` | jq` to the commands for beautifully formatted JSON output).*

**1. Operational Health Check (Phase D)**
Validates the background WORM hydration worker and cryptographic signature verification.
```bash
curl -s http://localhost:8080/health
```
*Expected Output: `"status": "Ready"` with `sync_error_count` strictly at `0`.*

**2. Standard Typology Resolution (Phases 2 & 3)**
Tests the core pipeline's ability to fetch baseline linguistic DNA for a canonical locale.
```bash
curl -s http://localhost:8080/v1/manifest/aaa
```
*Expected Output: The fully populated JSON manifest with core traits like `"MORPHOLOGY_TYPE": "FUSIONAL"`.*

**3. Dynamic BCP 47 Extension Overrides (Phase 3)**
Verifies that Unicode extensions (`-u-`) requested in the tag correctly override the baseline orthographic defaults.
```bash
curl -s http://localhost:8080/v1/manifest/aaa-u-nu-latn-ca-islamic
```
*Expected Output: The manifest dynamically injects `"CALENDAR": "islamic"` and `"NUMBERING_SYSTEM": "latn"`.*

**4. Deprecated Tag Aliasing (Phase 1)**
Tests the routing layer's ability to intercept legacy or macro-language tags and instantly map them to their modern, canonical equivalents.
```bash
curl -s http://localhost:8080/v1/manifest/in
```
*Expected Output: `"resolved_locale"` automatically snaps to `"id"`, with the telemetry block noting `"resolution_path": "alias:in->id -> id"`.*

**5. Script Truncation & Resource Bridging**
Tests taxonomic fallback (if an unsupported region is requested) and dynamic CDN asset pointer generation.
```bash
curl -s http://localhost:8080/v1/manifest/th-TH
```
*Expected Output: Safe fallback to the parent locale and injection of the physical pointer (`"resource_uri"`) for downloading required data.*

---

## Project Structure

* `src/core/`: The 5-phase capability resolution pipeline and business logic.
* `src/data/`: Memory management, WORM snapshot hydration, and wait-free atomic structures (`RegistryState`).
* `src/models/`: Domain Transfer Objects (DTOs) including the `CapabilityManifest` and trait definitions.
* `src/security/`: JWS/Ed25519 cryptographic validation ensuring data integrity.
* `src/validation/`: Pipeline integrity checks preventing contradictory rendering instructions.
* `src/ops/`: Observability, SLI metrics, and telemetry injection.
* `examples/sidecar.rs`: The production Axum web server implementation.
* `src/bin/curator.rs`: The CLI tool for signing WORM snapshots.

---

## License
Copyright (C) 2026 Francis Xavier Wazeter IV.
Licensed under the GNU General Public License v3.0.
