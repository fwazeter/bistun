As the **Senior Implementation Engineer**, I have updated the **v1.0.0 Production-Readiness Plan** for `bistun-lms`. We are shifting from an internal engine to a professional-grade **Atomic Capability Provider**.

This updated plan prioritizes **Dependency Hygiene** and **Structural Standardisation** to ensure the engine is as lean and well-documented as the foundation.

---

### Phase I: The v1.0.0 Capability Engine Checklist (Updated)

#### 1. Metadata & Authority Audit

* [ ] **Manifest Polish**: Synchronize `bistun-lms/Cargo.toml` metadata (description, keywords, repository) with the public-facing brand established in `bistun-core`.
* [ ] **Feature Flag Architecture**: Implement granular feature gates in `bistun-lms/Cargo.toml` (e.g., `network` for `reqwest`, `fs` for file-based snapshots) to minimize the dependency tree for specialized sidecars.
* [ ] **Elevator Pitch**: Draft a professional `README.md` for the engine that explains the **5-Phase Pipeline** and the **Flyweight/ArcSwap** memory architecture.
* [ ] **License Synchronization**: Ensure `license.workspace = true` points to the updated `GPL-3.0-or-later` SPDX identifier in the monorepo root.

#### 2. Narrative Code & Structural Hygiene (LMS-DOC)

* [ ] **Structural Setup**: Establish standard `examples/` and `tests/` directories at the crate root to house user-facing walkthroughs and integration-level path exhaustion tests.
* [ ] **Module-Level Specification**: Audit all submodule headers (`//!`) to ensure correct `Location` tags and blueprint references.
* [ ] **Logic Trace Verification**: Ensure every public-facing method (especially in `LinguisticManager` and the `pipeline`) has a numbered `# Logic Trace` reflecting the code implementation.
* [ ] **Complexity Badges**: Document Big-O Time/Space complexity for the hot-path resolution logic ($< 1\text{ms}$ budget).

#### 3. Operational Integrity & Security

* [ ] **Circuit Breaker Validation**: Verify that the `Degraded` state generates a valid, safe fallback manifest without panicking under high-load concurrency.
* [ ] **JWS/WORM Hardening**: Ensure the security gate in `verifier.rs` is mandatory for all hydration paths and cannot be bypassed.
* [ ] **Telemetry Exhaustion**: Verify that Phase 5 (Telemetry) captures and reports the `resolution_path` correctly for every possible fallback outcome.

#### 4. Scientific Verification (LMS-TEST & Benches)

* [ ] **Performance Proof**: Finalize the `benches/capability_engine.rs` to prove the engine meets the **$< 1\text{ms}$** p99 target.
* [ ] **Golden Path Exhaustion**: Test the engine against the "Golden Set" from `bistun-core/simulation.rs` to ensure Japanese, Pali, and Sanskrit traits aggregate correctly.

---

### II. Implementation Trace: Next Steps

To initiate these first steps, I need to verify the state of your core files. **Please provide the current content for:**

1. **Monorepo Root `Cargo.toml**`: To ensure workspace inheritance for `license`, `homepage`, and `edition` is perfectly aligned for v1.0.0.
2. **`bistun-lms/src/core/aggregator/typology.rs`**: To finalize high-water mark logic for the new traits (like `RequiredResource`) added during the simulation data phase.
3. **`bistun-lms/src/core/extension/orthography.rs`**: To verify the BCP 47 `-u-` extension parsing logic handles edge cases gracefully.
4. **`bistun-lms/src/security/verifier.rs`**: To confirm the Ed25519 verification matches our v1.0.0 security tier.

**Once provided, I will generate the surgical updates for your feature flags and folder structure.**