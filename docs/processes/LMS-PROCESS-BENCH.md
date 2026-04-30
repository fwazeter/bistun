# LMS-PROCESS-BENCH: Performance Verification & Benchmarking

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/29/2026  
> **Status:** Operational Process Guide

---

## I. Overview

In the Bistun LMS, performance is a primary functional requirement, not an afterthought. Our Service Level Indicator (SLI) targets a **p99 resolution latency of $< 1\text{ms}$**. To scientifically prove we meet this target, we utilize **statistical benchmarking** via the `criterion` crate.

This guide ensures that our benchmarks are as rigorous as our logic, providing a "Proof of Speed" that aligns with our **LMS-TEST** and **LMS-DOC** standards.

---

## II. Benchmarking Philosophy

### 1. Mirror the Pipeline
Every benchmark must mirror the **5-Phase Resolution Pipeline** (Resolve $\rightarrow$ Aggregate $\rightarrow$ Override $\rightarrow$ Integrity Check $\rightarrow$ Telemetry). This ensures we are measuring the "Production Path," not just isolated helper functions.

### 2. Warm vs. Cold Cache
As a "System of Record" designed for high-traffic sidecar deployment, we prioritize **Warm Cache** measurements.
* **Warm Cache**: The state where `Flyweight` pools for languages and scripts are already hydrated in memory.
* **Requirement**: All benchmarks must include a "Warm-up" phase to ensure JIT optimizations and memory caches are primed.

---

## III. Implementation Guide: The "Logic Trace" Benchmark

Following the **LMS-DOC** standard, benchmarks must follow a structured logic trace.

### 1. Setup Phase
* **Mock Registry**: Use the `mockall` crate or a static "Golden Set" to hydrate a `RegistryStore`.
* **Standard Environment**: Use the default `Config` to ensure telemetry sinks are active but not blocking.

### 2. Execution Phase
Use `bencher.iter()` to wrap the critical path.
```rust
/// Example implementation for a criterion benchmark group.
fn run_sample_bench(group: &mut BenchmarkGroup, manager: &LinguisticManager) {
    // [STEP 3] from benches/performance_verification.rs
    group.bench_function("get_manifest (warm cache)", |b| {
        b.iter(|| {
            // This is the 1ms path being measured
            manager.get_manifest(black_box("ar-EG-u-nu-latn"))
        })
    });
}
```

---

## IV. Interpreting Statistical Reports

When you run `just bench-critical`, `criterion` generates an HTML report in `target/criterion/report/index.html`.

### 1. The Latency Distribution
* **Mean**: The average time per resolution.
* **p99 (The Red Line)**: Our critical metric. If p99 is $> 1\text{ms}$, the benchmark indicates a potential SLI breach.

### 2. Regression Analysis
`criterion` compares the current run against the previous "baseline."
* **Green Report**: Performance improved or stayed stable.
* **Red Report**: Performance regressed. **Action Required**: You must optimize the "Logic Trace" steps in your code to return to the performance budget.

---

## V. Placement & Maintenance

* **Location**: This guide lives in `docs/processes/` to provide developers with the "Why" behind our latency proofs.
* **Update Trigger**: Whenever a new phase is added to the resolution pipeline (e.g., v0.5.0 Security Phase), this guide and the corresponding `benches/` must be updated to include the new logic overhead.
