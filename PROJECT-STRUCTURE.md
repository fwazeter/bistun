# PROJECT-STRUCTURE: Architectural Logic & Layout

> **Version:** 0.1.4
<br> **Status:** Engineering Standard

---

## I. Repository Overview

This project follows a "Separation of Writing Systems" philosophy. We decouple the **Orthography** (rendering/scripts), **Typology** (language traits), and **Taxonomy** (resolution engine) into distinct domains. This ensures the system remains a "System of Record" that can scale to 7,000+ languages without architectural debt.

---

## II. Directory Structure

```text
bistun/
├── Cargo.toml           # Project manifest and dependencies
├── Dockerfile           # Multi-stage production build
├── .dockerignore        # Build context exclusion rules
├── LICENSE              # GNU GPL v3
├── README.md            # Value proposition and quick-start
├── PROJECT_MAP.md       # Blueprint-to-file mapping table
├── PROJECT_STRUCTURE.md # This document: Architectural rationale
├── docs/                # Multi-layered documentation suite
├── scripts/             # Phase 8: ISO/CLDR data ingestion scrapers
├── tests/               # Phase 9: "Golden Set" integration testing
└── src/                 # Rust Source Tree
    ├── main.rs           # The service entry point (Executable)
    ├── lib.rs           # The public API entry point
    ├── manager.rs       # [001-LMS-CORE] SDK state and orchestration
    ├── core/            # The capability engine's algorithmic heart
    │   ├── mod.rs       # Pipeline coordinator
    │   ├── resolver/    # [012-LMS-ENG] Locale resolution logic
    │   ├── aggregator/  # [008-LMS-DNA] Trait merging & High-Water Mark
    │   └── extension/   # [004-LMS-EXT] Unicode subtag overrides
    ├── data/            # Memory management and persistence
    │   ├── mod.rs
    │   ├── repository.rs# [002-LMS-DATA] Snapshot/WORM hydration
    │   ├── store.rs     # [010-LMS-MEM] Flyweight definition pools
    │   └── swap.rs      # [010-LMS-MEM] Atomic Reference Swap logic
    ├── strategy/        # [009-LMS-STRAT] Strategy Pattern implementations
    │   ├── mod.rs       # Strategy registry factories
    │   ├── stemming.rs  # Morphological algorithms
    │   └── segmentation.rs # Boundary detection algorithms
    ├── validation/      # [003-LMS-VAL] The Linter system
    │   ├── mod.rs       # Validation dispatcher
    │   ├── dna.rs       # Level A: Strict pre-persistence checks
    │   └── integrity.rs # Level C: Runtime lightweight checks
    ├── security/        # [006-LMS-SEC] JWS verification & Key Pinning
    ├── ops/             # [007-LMS-OPS] Telemetry sinks and SLI tracking
    └── models/          # [011-LMS-DTO] Shared Data Transfer Objects
        ├── mod.rs       
        ├── manifest.rs  # CapabilityManifest immutable schema
        └── traits.rs    # TraitKey and standardized Enumerations
```

---

## III. Architectural Rationale

### 1. The Core Domain (`src/core/`)
Separates how we categorize a locale (**Taxonomy**), how we merge its data (**Typology**), and how we modify its rendering at runtime (**Orthography**). This follows the **Single Responsibility Principle**, allowing us to change resolution logic without breaking rendering logic.

### 2. The Strategy Domain (`src/strategy/`)
Uses the **Strategy Pattern** to prevent a "Giant Match Statement" in the core. Adding a new language type (e.g., `POLYSYNTHETIC`) only requires adding a file here and updating the factory, leaving the orchestrator untouched.

### 3. The Persistence & Memory Domain (`src/data/`)
Separates the **WORM (Write-Once, Read-Many)** storage logic from the in-memory **Flyweight** cache. This ensures that memory safety and atomic updates are handled in a sandbox isolated from the linguistic algorithms.

---

## IV. Documentation Hierarchy (`docs/`)

The documentation is split into five layers to serve different stakeholders (and AI agents):

1.  **Foundations (`docs/foundations/`)**: The "Executive" layer. Explains the "Global Truth" vision, implementation phases, and core algorithms. Summarized in `00-system-overview.md`.
2.  **Blueprints (`docs/blueprints/`)**: The "Implementation" layer. Highly technical, standard-linked specifications (001-012) used as "Ground Truth" during coding.
3.  **Standards (`docs/standards/`)**: The "Engineering" layer. Defines the quality of the codebase, including **LMS-DOC** narrative standards and **LMS-AI** alignment.
4.  **Interfaces (`docs/interfaces/`)**: The "Admin" layer. Specs for external UI tools like the **Curator UI**.
5.  **Processes (`docs/processes/`)**: The "Operational" layer. Detailed guides for the mechanics of CI gates, performance benchmarking, error handling, and release promotion.

---

## V. Developer Guidance

* **Intensive Documentation**: Every file in the `src/` tree must follow the **LMS-DOC** standard, providing a "Logic Trace" for every function.
* **Module Links**: Module-level documentation (`//!`) must link back to the specific **Blueprint** (e.g., `Ref: [001-LMS-CORE]`) to maintain the thread between theory and execution.

---

**Author**: Francis Xavier Wazeter IV  
**License**: GNU GPL v3  
**Date Created**: 04/29/2026  
**Date Updated**: 04/30/2026
