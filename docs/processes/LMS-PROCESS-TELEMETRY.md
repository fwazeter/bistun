# LMS-PROCESS-TELEMETRY: Observability & Logic Spans

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/29/2026  
> **Status:** Operational Process Guide

---

## I. Overview

In the Bistun LMS, telemetry is the window into the **Linguistic DNA** resolution process. To maintain our **$<1\text{ms}$** performance budget, we require a telemetry system that is high-performance, structured, and directly mapped to our **5-Phase Pipeline**.

**Location**: `docs/processes/LMS-PROCESS-TELEMETRY.md`

---

## II. Framework Selection: `tracing`

We utilize the `tracing` ecosystem (maintained by the Tokio team) for all project observability.

* **Why `tracing`?**: Unlike standard logging, `tracing` is centered around **spans** (intervals of time) rather than just discrete events. This allows us to measure the exact latency of each "Step" in a function's **Logic Trace**.
* **Zero-Cost Goal**: When no subscribers are active, `tracing` macros have near-zero overhead, ensuring we do not breach our resolution SLIs.

---

## III. The "Logic Trace" Span Standard

Every phase of the resolution pipeline must be wrapped in a diagnostic span that includes the `locale_tag` as context.

### 1. Level Standards
* **`ERROR`**: System-critical failures (e.g., Registry Hot-Swap failure).
* **`WARN`**: Phase-level anomalies (e.g., malformed `-u-` extension in Phase 3).
* **`INFO`**: High-level resolution results (e.g., "Resolved 'en-AU' to 'en-GB'").
* **`DEBUG`**: Internal logic trace steps (e.g., "[STEP 2] Checking Truncation Resolver").
* **`TRACE`**: Exhaustive variable-level dumps (e.g., raw byte sequences from JWS bundles).

---

## IV. Implementation Narrative

Following the **LMS-DOC** standard, spans must narrate the execution flow.

### 1. Span Creation
Always include the target locale in the instrumented span to allow for distributed tracing correlation.

```rust
impl LinguisticManager {
    /// Use the 'instrument' macro to automatically create a span for the function.
    #[tracing::instrument(skip(self), fields(locale = %tag))]
    pub fn get_manifest(&self, tag: &str) -> Result<CapabilityManifest, LmsError> {
        // [STEP 1]: Logic Trace Step (Resolution Phase)
        tracing::debug!(step = 1, "Initiating locale resolution chain");
        let entry = self.resolver.resolve(tag)?; // Automatically bubbles up ResolutionError

        // [STEP 2]: Aggregation Phase
        let span = tracing::info_span!("aggregation_phase");
        let _enter = span.enter();

        // ... [Implementation logic to build the manifest] ...

        // Final Step: Return the successfully constructed manifest
        let manifest = CapabilityManifest::new(entry);
        Ok(manifest)
    }
}
```

---

## V. SLI Correlation

The telemetry recorded via `tracing` must feed directly into the metadata map of the `CapabilityManifest` DTO.

* **Latency Field**: The `resolution_time_ms` reported in the DTO must be derived from the root `get_manifest` span duration.
* **Path Trace**: The `resolution_path` must record each hit in the `ResolverChain` as a discrete `tracing::event!`.

---

## VI. Quality Assurance Gates

* **Instrument Verification**: PRs that implement new pipeline logic must include `tracing::instrument` or explicit spans for each step in their **Logic Trace Mapping**.
* **Zero-Allocation**: Events in the critical $<1\text{ms}$ path must avoid heap allocations (e.g., avoid `format!()` in log messages) to preserve performance.
