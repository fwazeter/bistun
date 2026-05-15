# BLUEPRINT: Resource Bridge & Physical Mapping

---

## I. Strategic Overview

### 1. The "Why"

The Resource Bridge serves as Phase 2.5 of the resolution pipeline, translating abstract resource IDs (e.g., `icu_arab`) into actionable, environment-specific physical URIs. It ensures that consuming applications can dynamically locate and download the binary data blobs (such as ICU4X data) required for complex rendering and segmentation.

### 2. System Impact

If the Resource Bridge fails, downstream clients are unable to fetch physical assets, rendering capabilities like dictionary-based segmentation or script-specific transliteration inoperable despite a valid manifest.

### 3. Design Patterns

* **Pointer Pattern**: Instead of streaming heavy binary files through the LMS service, the engine provides a synthesized URL string for the client to resolve externally.
* **Decoupled Mapping**: Logical IDs remain stable in the registry while physical paths can change based on the deployment environment (CDN, local filesystem, or cloud storage).

---

## II. Technical Specification

### 1. Primary Interface: `ResourceResolver`

This logic is injected between Phase 2 (Aggregation) and Phase 3 (Override) of the pipeline.

| Method / Key        | Input / Type                      | Output / Value         | Purpose                                                                   |
|---------------------|-----------------------------------|------------------------|---------------------------------------------------------------------------|
| `resolve_resources` | `&mut CapabilityManifest`, `&str` | `Result<(), LmsError>` | Maps a `ResourceId` trait to a physical URI and injects it into metadata. |

### 2. Golden Set & Standard Enums

* **`TraitKey::ResourceId`**: The abstract identifier for a required linguistic asset.
* **`metadata::resource_uri`**: The fully qualified destination where the asset is hosted.

### 3. Logic & Algorithms (The Workflow)

Following the `# Logic Trace` standard defined in `LMS-DOC.md`:

1. **Phase 1: Identification**: Check if the `CapabilityManifest` contains a `TraitKey::ResourceId`.
2. **Phase 2: Extraction**: Retrieve the abstract string ID (e.g., `tri_thai`) from the manifest traits.
3. **Phase 3: Path Normalization**: Format the environment-specific `base_uri` to ensure correct slash-delimited concatenation.
4. **Phase 4: Synthesis**: Concatenate the base URI with the ID, appending the standard binary extension (e.g., `.dat` or `.postcard`).
5. **Phase 5: Metadata Injection**: Store the fully resolved URI in the manifest's metadata map.

---

## III. Execution & Integrity

### 1. Performance Budget

* **Target Latency**: < 0.1ms (O(1) map lookup and string concatenation).
* **Complexity**: O(1).

### 2. Enforcement & Validation (LMS-VAL)

* **Level A (Strict)**: Registry curation must verify that every `ResourceId` referenced in a `LocaleProfile` has a corresponding physical asset in the deployment bucket.
* **Level C (Runtime)**: The resolver must handle missing trailing slashes in the `base_uri` to prevent malformed URLs.

### 3. Observability (LMS-OPS)

* **Telemetry Keys**: `resource_resolution_success`, `base_uri_env`.
* **Alert Thresholds**: Any resolution failure for a locale requiring `DICTIONARY` segmentation is a critical event.

---

## IV. Implementation Example

**Linguistic Bridge Mapping:**

```json
{
  "traits": {
    "RESOURCE_ID": "icu_arab"
  },
  "metadata": {
    "resource_uri": "https://cdn.bistun.io/assets/v1/icu_arab.dat"
  }
}

```

---

## V. Metadata

* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.1.0
* **Blueprint Ref**: [[014-LMS-BRDG]]
* **Last Updated**: 2026-05-11
* **License**: GNU GPL v3
