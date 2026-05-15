# ROADMAP: Bistun LMS v1.0.0 (Rule-Centric)

## Phase 1: Foundation & Identity (Hardening)
*Objective: Solidify the 'System of Record' authority.*
- [ ] **A.1 Snapshot Headers**: Implement mandatory metadata (checksums, build dates) in WORM payloads.
- [ ] **A.2 Verification Logic**: Finalize the Ed25519 verifier gate in the security module.
- [ ] **A.3 Dependency Hygiene**: Implement granular feature flags for WASM-readiness.

## Phase 2: The Logic Bridge (Option C Priority)
*Objective: Enable microservices to consume linguistic rules, not just data.*
- [ ] **B.1 Rule Synthesis**: Implement the `rules` map in `CapabilityManifest`.
- [ ] **B.2 Resource Resolver**: Map Logical IDs to Physical Paths for ICU4X hydration.
- [ ] **B.3 Default Mechanics**: Add support for default Transliteration/Normalization types.

## Phase 3: Operational Excellence
*Objective: Professional-grade monitoring and stability.*
- [ ] **C.1 Sync Telemetry**: Track success/error rates for background workers.
- [ ] **C.2 Health Endpoints**: Expose SDK state (READY/DEGRADED) for Kubernetes probes.
- [ ] **C.3 Performance Proof**: Scientific verification of <1ms latency with rule injection.

## Phase 4: Release & Curation
*Objective: Final v1.0.0 stabilization.*
- [ ] **D.1 Narrative Audit**: 100% Logic Trace coverage for all public APIs.
- [ ] **D.2 Curator CLI**: Finalize the 'curator' tool for administrative registry signing.