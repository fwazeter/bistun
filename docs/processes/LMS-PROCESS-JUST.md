# LMS-PROCESS-JUST: Task Runner & Command Automation

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/29/2026  
> **Status:** Operational Process Guide

---

## I. Overview

The `justfile` serves as the unified task runner for the Bistun LMS project. It abstracts complex Rust toolchain flags into single-command interfaces, ensuring that every developer—regardless of experience level—can execute the project's strict quality and performance gates with 100% consistency.

By using `just`, we ensure that the local development environment perfectly mirrors the requirements of the **LMS-CI** pipeline.

---

## II. The Chain of Quality (`verify-all`)

The most critical command in the repository is `just verify-all`. This command executes a linear "Chain of Quality" where each step must pass for the next to begin:

1.  **`fmt`**: Enforces uniform code appearance across the "System of Record".
2.  **`test-hermetic`**: Executes unit and integration tests to ensure logic matches the blueprints.
3.  **`lint`**: Runs `clippy` with warning denial to enforce high-performance Rust idioms.
4.  **`verify-docs`**: Triggers the **LMS-DOC** enforcer to validate narrative integrity and links.

---

## III. Command Reference

### 1. Quality Gates (LMS-DOC & LMS-TEST)
* **`just fmt`**: Standardizes code formatting.
* **`just test-hermetic`**: Runs the full test suite in isolation.
* **`just lint`**: Performs deep static analysis to catch potential bugs or performance regressions.
* **`just verify-docs`**: Generates documentation while denying all warnings to ensure no broken references to blueprints exist.

### 2. Performance Gates (LMS-OPS)
* **`just bench-critical`**: Executes the statistically significant latency benchmarks defined in `benches/`.
* **Standard**: This should be run before any PR that modifies the core resolution pipeline to ensure the **$< 1\text{ms}$** budget is intact.

### 3. Developer Utilities
* **`just new-module <path>`**: Automatically bootstraps a new Rust file using the `TEMPLATE.rs` standard.
    * *Usage*: `just new-module src/core/resolver/my_new_logic.rs`.
* **`just clean`**: Wipes all build artifacts and temporary files.

---

## IV. Extending the Task Runner

When adding new commands to the `justfile`, developers must adhere to these standards:

1.  **Self-Documentation**: Every command must have a preceding comment starting with `#` so it appears in the `just --list` output.
2.  **Dependency Alignment**: Commands that represent "Quality" must be added as dependencies to the `verify-all` recipe to ensure they are included in the global check.
3.  **Cross-Platform Portability**: Ensure commands use standard `cargo` or shell utilities that function in the Linux-based CI environment.

---

## V. Placement Logic

This guide lives in `docs/processes/` to serve as the "Manual" for our automation logic. While the `justfile` in the root is the executable tool, this document explains the philosophy of **Standardized Bootstrapping** and **One-Command Verification** that defines our world-class engineering culture.
