# MODULE-README: Operations & Telemetry Domain

![Blueprint](https://img.shields.io/badge/Blueprint-007--LMS--OPS-blue)
![Domain](https://img.shields.io/badge/Domain-Operations-green)
![Location](https://img.shields.io/badge/Location-src%2Fops-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module handles system observability, performance metrics, and manifest metadata injection for the 5-phase pipeline. It specifically executes Phase 5 (Telemetry), stopping the execution timer and embedding diagnostic data directly into the finalized `CapabilityManifest`.

### 2. System Impact
If this module fails, the system loses all visibility into resolution latency and fallback paths. This results in a breach of our Service Level Indicators (SLIs) and makes auditing performance regressions or "Linguistic Drift" impossible in production environments.

### 3. Design Patterns
* **Metadata Injection**: Dynamically hydrates the `metadata` map of the DTO with runtime metrics without altering the core linguistic traits.
* **Observer/Telemetry Pattern**: Utilizes high-precision timers to monitor the "physics" of the resolution pipeline across multiple thread boundaries.

### 4. Local Glossary
* **SLI (Service Level Indicator)**: A quantitative measure of service level, specifically resolution latency in milliseconds.
* **Span**: An interval of time representing a discrete logical operation within the pipeline (e.g., the duration of the Aggregation phase).
* **Resolution Path**: The sequential list of locale tags attempted by the Resolver before finding a canonical match.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method    | Input                                                        | Output | Purpose                                                    |
|:-------------------|:-------------------------------------------------------------|:-------|:-----------------------------------------------------------|
| `record_metrics()` | `&mut CapabilityManifest`, `Instant`, `&Vec<String>`, `&str` | `()`   | Injects latency, path, and version data into the manifest. |

### 2. Side Effects & SLIs
* **Telemetry**: This module is the authoritative source for the `resolution_time_ms` field found in every manifest.
* **Performance**: Target latency: **< 0.01ms**. Complexity: **O(N)** relative to the length of the resolution path.
* **Dependencies**: Relies on `std::time` for precision timing and `crate::models` for the `CapabilityManifest` definition.

### 3. Quirks & Invariants
* **Precision Formatting**: Latency is recorded as a string with 4-decimal precision (`.4`) to ensure sub-microsecond visibility.
* **Invariant**: Telemetry must be the absolute final operation in the pipeline to ensure it captures the total overhead of all preceding phases.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::models::manifest::CapabilityManifest;
use crate::ops::telemetry::record_metrics;
use std::time::Instant;

fn main() {
    let mut manifest = CapabilityManifest::new("en-US".to_string());
    let start_time = Instant::now();
    let path = vec!["en-US".to_string()];
    let version = "1.0.0";

    // Inject metrics - typically called at the end of the 5-Phase pipeline
    record_metrics(&mut manifest, start_time, &path, version);
    
    // Verify metadata injection to satisfy the compiler
    assert!(manifest.metadata.contains_key("resolution_time_ms"));
}
```

### 2. The "Golden Path" Example
```rust
use crate::models::manifest::CapabilityManifest;
use crate::ops::telemetry::record_metrics;
use std::time::Instant;

fn main() {
    let mut manifest = CapabilityManifest::new("ar-EG".to_string());
    let start = Instant::now();
    
    // Simulate resolution path: User asked for ar-EG-u-ext, resolved to ar-EG
    let resolution_path = vec!["ar-EG-u-ext".to_string(), "ar-EG".to_string()];
    
    record_metrics(&mut manifest, start, &resolution_path, "v1.2.3");

    // Verify the formatted path trace
    let path_trace = manifest.metadata.get("resolution_path").unwrap();
    assert_eq!(path_trace, "ar-EG-u-ext -> ar-EG");
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the telemetry logic (e.g., adding memory utilization metrics):
1.  **Red Phase**: Add a test case in `telemetry.rs` verifying that a new metadata key (e.g., `memory_kb`) is present after recording.
2.  **Implementation**: Update `record_metrics` to ingest the new metric and insert it into the `manifest.metadata` map.
3.  **Audit**: Ensure no heavy heap allocations (like complex string formatting) are introduced, as this module sits in the critical **< 1ms** path.
4.  **Verification**: Run `just verify-all` to confirm SLI compliance.

### 2. Troubleshooting & Common Failures
* **Clock Skew**: Use `Instant::now()` for internal pipeline measurement to avoid issues with system clock adjustments during resolution.
* **Missing Path**: If the `resolution_path` is empty in the manifest, verify that the `resolver` module is correctly populating its tracking vector before Phase 5 starts.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When modifying this domain, maintain the "Zero-Allocation" goal for log events. All metadata keys must remain consistent with the **011-LMS-DTO** standard to ensure downstream dashboards can parse the telemetry without schema updates. Do not introduce metrics that require external I/O (e.g., database writes) within the `record_metrics` function.
