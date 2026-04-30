# LMS-PROCESS-CI: Continuous Integration Philosophy & Gates

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/29/2026  
> **Status:** Operational Process Guide

---

## I. Overview

The Bistun LMS Continuous Integration (CI) pipeline is not merely a build check; it is the automated enforcer of our **System of Record** integrity. Every push to the repository must survive a multi-layered verification process that aligns code execution with the **Linguistic DNA** blueprints.

The CI pipeline ensures that:
1.  **Narrative Integrity**: Documentation never "rots" or loses its link to architectural foundations.
2.  **Executable Truth**: Code examples in documentation are functional and tested.
3.  **Performance Budgets**: Resolution logic stays strictly within the **$<1\text{ms}$** p99 target.

---

## II. The Quality Gates

### 1. Documentation Integrity (LMS-DOC Enforcer)
We utilize `RUSTDOCFLAGS="-D warnings"` during the doc generation phase.

* **Logic**: This gate forces all intra-doc links to be valid. If a developer references `[001-LMS-CORE]` in a comment but the link is broken, the CI will fail.
* **Why**: This maintains the "Living Document" status of the codebase, ensuring a novice can always navigate from a function to its theoretical blueprint.

### 2. Logic Validation (LMS-TEST Enforcer)
The pipeline executes `cargo test` across three distinct tiers:

* **Unit Tests**: Verifies internal logic isolation and mock behavior.
* **Doc Tests**: Compiles and runs every code block found in `///` comments.
* **Integration Tests**: Verifies the "Physics" of the full resolution engine integration.

### 3. Best Practice Enforcement (Clippy)
We enforce `cargo clippy -- -D warnings`.

* **Logic**: Any code that violates Rust idiomatic standards or potential memory safety patterns is rejected.
* **Why**: High-performance software ($<1\text{ms}$) requires highly optimized, idiomatic code to avoid unnecessary allocations or logic overhead.

---

## III. The Performance Gate (LMS-OPS Enforcer)

The `performance-gate` job in CI runs benchmarks found in the `benches/` directory.

* **Measurement**: It evaluates the latency of the 5-phase resolution pipeline.
* **Threshold**: If the p99 resolution time exceeds **1ms**, the pipeline triggers a warning; if it exceeds **2ms**, it is considered a critical failure.
* **Interpretation**: Performance failures indicate that a change has introduced algorithmic complexity that violates the service level indicators (SLIs).

---

## IV. Interpreting Failures

When a build fails, developers must map the error back to the standards:

| CI Failure Type | Root Cause | Remediation |
| :--- | :--- | :--- |
| **Doc Warning** | Broken intra-doc link or missing section. | Fix link in `///` or follow **LMS-DOC** template. |
| **Test Failure** | Logic regression or broken `# Examples`. | Update doc examples or fix logic to match the "Golden I/O". |
| **Performance Fail** | Latency regression ($>2\text{ms}$). | Profile the code; optimize the "Logic Trace" steps. |
| **Lint Warning** | Non-idiomatic Rust code. | Run `cargo clippy --fix` and review best practices. |

---

## V. Placement Logic

This guide is stored in `docs/processes/` because it defines the **Operational Philosophy**. While the `.yml` file contains the technical "How," this document provides the "Why" for the engineering team, ensuring that automated gates are respected as core architectural requirements.
