# ROADMAP: Bistun LMS Implementation Path

> **Version:** 0.1.4  
> **Status:** Tactical Execution Plan

---

## I. Overview

This document outlines the tactical execution strategy for the Bistun Linguistic Metadata Service (LMS). Following a **Testing-First** and **Narrative-Code** philosophy, we prioritize components by their "Dependency Gravity"—starting with the core models and moving toward the high-performance orchestration layers.

---

## II. Priority Chunks (Dependency Gravity)

1.  **Foundation**: Models, Enums, and DTOs.
2.  **Taxonomy**: The Logical Engine and **Taxonomic** Resolver Chain.
3.  **Typology**: **Typological** Trait Aggregator and High-Water Mark logic.
4.  **Optimization**: Flyweight instance pools and Atomic Reference Swaps.
5.  **Orchestration**: SDK State Machine and the 5-Phase Pipeline.

---

## III. Versioned Milestones

### v0.1.0: The Static Foundation (Current Milestone)
*Objective: Build a working in-memory capability engine with a hardcoded "Golden Set" of 5 languages.*

- [X] **Standards Verification**: Ensure CI gates for `LMS-DOC` and `LMS-TEST` are active.
- [X] **Core Models**: Implement `TraitKey`, `Direction`, `SegType`, and `MorphType` enums in `src/models/traits.rs`.
- [X] **Manifest DTO**: Define the immutable `CapabilityManifest` and `Metadata` structures.
- [X] **Taxonomic Resolver**: Implement the `ExactMatchResolver` and `DefaultFallbackResolver`.
- [X] **Typology Aggregator MVP**: Implement the `TraitAggregator` with `HighWaterMark` logic for segmentation.
- [X] **Orchestrator MVP**: A simplified `LinguisticManager` coordinating a 3-phase pipeline (Resolve -> Aggregate -> Telemetry).

### v0.2.0: The Performance Engine
*Objective: Achieve 80% memory reduction and sub-millisecond resolution via Flyweight patterns.*

- [ ] **Flyweight Pools**: Implement immutable instance pools in `src/data/store.rs`.
- [ ] **Truncation Logic**: Implement the RFC 4647 `TruncationResolver`.
- [ ] **Atomic Swap**: Implement background shadow-registry hydration and atomic pointer swaps.
- [ ] **Benchmarks**: Add performance unit tests to verify the $< 1\text{ms}$ budget.

### v0.5.0: The Secured Sidecar
*Objective: Production-ready SDK with cryptographic signing and full state management.*

- [ ] **JWS Verification**: Implement SDK-side signature checking against the LMS Public Key.
- [ ] **Full State Machine**: Transitions for **BOOTSTRAPPING**, **READY**, and **DEGRADED**.
- [ ] **Circuit Breaker**: Graceful fallback to "System Default" on synchronization failure.
- [ ] **Unicode Overrides**: Implement the `-u-` extension mapper logic.
- [ ] **Ops Integration**: Full SLI/SLO tracking for p99 resolution latency.

### v0.8.0: The Global Registry
*Objective: Scale to 7,000+ languages via automated ingestion pipelines.*

- [ ] **ISO Scrapers**: Automated scripts for ISO 639-3 and ISO 15924 data ingestion.
- [ ] **CLDR Importer**: Synchronization with Unicode CLDR for plural rules and directionality.
- [ ] **Truth Hierarchy**: Implementation of Tier 1-4 collision and drift management.
- [ ] **DNA Linter**: Strict **Typological Integrity** pre-persistence validation in the repository.

### v1.0.0: System of Record
*Objective: Final production stability and administrative curation tools.*

- [ ] **QA Verification**: Pass regression tests for the 50 "Edge Case Locales".
- [ ] **Persistence Architecture**: Finalize PostgreSQL/Redis WORM storage.
- [ ] **Curator UI**: Deploy the RBAC-protected management dashboard for linguists.
- [ ] **Code Narrative Audit**: Final verification of 100% path coverage and narrative documentation.

---

## IV. Weekly Development Checklist (Per Feature)

- [ ] Initialize source file from `TEMPLATE.rs`.
- [ ] Define internal `//!` narrative and reference the Blueprint ID.
- [ ] Write failing unit test (Red phase) for every documented `# Errors` case.
- [ ] Implement logic with `// [STEP X]` inline comments mapping to the `# Logic Trace`.
- [ ] Verify complexity matches the performance budget for the target phase.
- [ ] Pass `cargo test`, `cargo clippy`, and `cargo doc`.

---

**Author**: Francis Xavier Wazeter IV  
**License**: GNU GPL v3  
**Date Created**: 04/29/2026  
**Date Updated**: 04/30/2026