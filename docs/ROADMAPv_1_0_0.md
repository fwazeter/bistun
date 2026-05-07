# Roadmap: LMS v1.0.0 Hardening & Completeness

## Phase A: Authoritative Identity (Identity Layer)
**Goal**: Ensure every `CapabilityManifest` contains a verifiable audit trail linked to the specific registry build.
* **A.1: Snapshot Header Implementation**: Upgrade `WormPayload` to include a mandatory `metadata` block (version, build_date, checksum).
* **A.2: Dynamic Versioning**: Refactor `telemetry::record_metrics` to pull `registry_version` from the active memory pool instead of a hardcoded string.
* **A.3: Cryptographic Signature Foundation**: Prepare the `verifier.rs` to validate the header-payload-signature triplet.

## Phase B: Aggregation Depth (Typology Layer)
**Goal**: Complete the "Golden Set" of traits required for full rendering and NLP automation.
* **B.1: Plural Category Mapping**: Hydrate `TraitKey::PluralCategories` from the `LocaleProfile` during Phase 2.
* **B.2: Unicode Block Discovery**: Implement logic to populate `TraitKey::UnicodePreloadBlocks` to drive automated font-loading.
* **B.3: Default Mechanics**: Add support for `NormalizationType` and `TransliterationType` defaults in the `LocaleProfile`.

## Phase C: Resource Bridge (Resource Layer)
**Goal**: Transform "dead" Resource IDs into actionable physical paths for ICU4X data.
* **C.1: Resource Resolver**: Create a centralized service to map logical IDs (e.g., `icu_arab`) to physical URLs or filesystem paths.
* **C.2: Manifest Injection**: Ensure the manifest includes the resolved path for immediate client-side resource fetching.

## Phase D: Operational Visibility (Management Layer)
**Goal**: Provide full transparency into the health of the background synchronization worker.
* **D.1: Sync Telemetry**: Update `LinguisticManager` to track `last_successful_sync` and `sync_error_count`.
* **D.2: Health Check Endpoint**: Expose manager status and sync health for Kubernetes/Orchestrator monitoring.
