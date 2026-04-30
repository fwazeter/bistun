# PROJECT-MAP: Blueprint to Implementation

> **Version:** 0.1.5
<br> **Status:** Architecture Reference

This document maps the **LMS Blueprints** to their physical locations in the Rust source tree.

| Blueprint          | Title          | Rust Module / File       | Responsibility                                                     |
|:-------------------|:---------------|:-------------------------|:-------------------------------------------------------------------|
| **001-LMS-CORE**   | SDK Interface  | `src/manager.rs`         | SDK Lifecycle, State Machine, and Orchestration.                   |
| **002-LMS-DATA**   | Persistence    | `src/data/repository.rs` | WORM storage and versioned snapshot hydration.                     |
| **003-LMS-VAL**    | Validation     | `src/validation/`        | Tiered DNA validation (Strict) and Runtime checks (Lightweight).   |
| **004-LMS-EXT**    | Extensions     | `src/core/extension/`    | Atomic BCP 47 `-u-` subtag override logic.                         |
| **005-LMS-INGEST** | Ingestion      | `scripts/`               | Automated ISO/CLDR scrapers and Truth Hierarchy.                   |
| **006-LMS-SEC**    | Security       | `src/security/`          | JWS signing/verification and public key pinning.                   |
| **007-LMS-OPS**    | Operations     | `src/ops/`               | Telemetry sinks, SLI/SLO metrics, and resolution latency tracking. |
| **008-LMS-DNA**    | Aggregator     | `src/core/aggregator/`   | Positional Priority and High-Water Mark algorithm.                 |
| **009-LMS-STRAT**  | Strategy       | `src/strategy/`          | Algorithmic logic for stemming and segmentation.                   |
| **010-LMS-MEM**    | Memory         | `src/data/store.rs`      | Flyweight instance pools and Atomic Reference Swap.                |
| **011-LMS-DTO**    | Schema         | `src/models/`            | Immutable Data Transfer Objects and standardized Enums.            |
| **012-LMS-ENG**    | Logical Engine | `src/core/resolver/`     | Chain of Responsibility for locale resolution.                     |

---

**Author**: Francis Xavier Wazeter IV  
**License**: GNU GPL v3  
**Date Created**: 04/29/2026  
**Date Updated**: 04/30/2026