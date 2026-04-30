# LMS-PROCESS-RELEASE: Versioning & Promotion Lifecycle

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/29/2026  
> **Status:** Operational Process Guide

---

## I. Overview

As the **System of Record** for Linguistic DNA, the release of a new version is a high-integrity event. A "Release" in the Bistun LMS ecosystem involves two distinct but synchronized flows: the **SDK Software Release** (code) and the **Linguistic Registry Release** (data).

**Location**: `docs/processes/LMS-PROCESS-RELEASE.md`

---

## II. Semantic Versioning (SemVer) Standards

We strictly follow [SemVer 2.0.0](https://semver.org/) for both code and data to ensure consumer stability.

### 1. SDK Software Releases
* **MAJOR**: Breaking API changes to the `CapabilityManifest` DTO or core `LinguisticManager` methods.
* **MINOR**: New features (e.g., adding a new `StemmingStrategy`) or new pipeline phases.
* **PATCH**: Performance optimizations or bug fixes that do not change traits.

### 2. Registry Data Releases
* **MAJOR**: Structural schema changes to the `RegistryBundle`.
* **MINOR**: High-impact linguistic updates (Tier 1 Manual Overrides or major Tier 2 CLDR syncs).
* **PATCH**: Automated Tier 3 ISO scraper updates (e.g., status changes from Living to Extinct).

---

## III. The Promotion Pipeline

The transition from a draft update to a production-ready "System of Record" entry follows this strictly monitored sequence:

### 1. Snapshot Phase
* The `ILinguisticRepository` triggers `SaveRegistrySnapshot`.
* A unique SHA-256 hash is generated for the entire bundle.

### 2. Validation Gate (QA)
* The **Linguistic Linter** runs a Level A (Strict) check to ensure no DNA contradictions exist.
* **Golden Set QA**: High-priority "Edge Case Locales" are verified to ensure no rendering regressions.

### 3. Signing Phase
* The validated `RegistryBundle` is signed using the LMS Private Key.
* A **JSON Web Signature (JWS)** is produced for distribution.

### 4. Promotion (Hot-Swap)
* The new version is promoted to the L2 Cache (Redis).
* Sidecar SDKs receive a sync notification and perform the **Atomic Reference Swap**.

---

## IV. Critical Release Checklist

Before cutting a release tag, the administrator must verify:
- [ ] **Audit Trail**: Every Tier 1 change is linked to an Identity ID.
- [ ] **Performance**: The p99 resolution target of $<1\text{ms}$ is maintained under the new version.
- [ ] **Integrity**: JWS signature is verified against the public key on at least one test sidecar.

---

## V. Placement Logic

This guide is placed in `docs/processes/` to define the **Philosophy of Truth** behind our versioning. It ensures that as the system scales to 7,000+ languages, the promotion of new data is treated with the same engineering rigor as a code commit.
