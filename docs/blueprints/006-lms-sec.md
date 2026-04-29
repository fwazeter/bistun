# LMS-SEC: Security & Registry Integrity

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026

**Status:** Implementation Reference

---

## I. Overview

As the **System of Record** for Linguistic DNA, the LMS must protect the integrity of the registry from "Linguistic
Poisoning"—the injection of malicious or incorrect traits that could break downstream search indexing or UI rendering.
This specification defines the security protocols for data at rest, data in transit, and the verification steps required
during the **Atomic Hot-Swap**.

---

## II. Authentication & Authorization

Access to the LMS API and registry synchronization must be strictly controlled to prevent unauthorized data exfiltration
or spoofing.

### 1. API Consumer Authentication (Sidecar SDK)

* **Bearer Token / JWT**: Every call to `GET /v1/registry/sync` must include a valid JWT issued by the organization's
  Identity Provider (IdP).
* **Machine-to-Machine (M2M)**: Sidecar instances use client credentials to obtain short-lived tokens for
  synchronization.

### 2. Curator UI RBAC (Role-Based Access Control)

* **Linguist Role**: Permission to propose changes and view all traits.
* **Administrator Role**: Permission to approve snapshots and trigger a global `RegistryBundle` promotion.
* **Audit Logs**: The `ILinguisticRepository` must record the Identity ID for every Tier 1 manual override.

---

## III. Data Integrity & Registry Signing

To ensure the registry has not been tampered with between the Repository and the SDK, the LMS employs cryptographic
signing.

### 1. Registry Bundle Signing

* **Signature Generation**: Upon creating a `SaveRegistrySnapshot`, the server generates a SHA-256 hash of the
  `RegistryBundle` and signs it using a private key.
* **JWS Packaging**: The bundle is delivered as a JSON Web Signature (JWS) where the payload is the registry data and
  the protected header contains the signature.

### 2. SDK-Side Verification

* **Public Key Pinning**: The Sidecar SDK is configured with the LMS Public Key.
* **Pre-Swap Check**: Before the **Atomic Reference Swap**, the SDK must verify the signature of the downloaded
  `RegistryBundle`. If verification fails, the swap is aborted, a critical error is logged, and the SDK continues using
  the previous pinned version.

---

## IV. Transport Security

* **TLS 1.3**: All communication between the Sidecar SDK and the LMS API must be conducted over TLS 1.3 or higher.
* **Mutual TLS (mTLS)**: In high-security environments, mTLS is recommended to ensure that only verified sidecar
  containers can connect to the LMS backend.

---

## V. Threat Mitigation Matrix

| Threat                   | Mitigation Strategy                                      | Reference |
|:-------------------------|:---------------------------------------------------------|:----------|
| **Linguistic Poisoning** | Cryptographic Signing + Linguistic Linter (`LMS-VAL-01`) |           |
| **Man-in-the-Middle**    | TLS 1.3 + Public Key Pinning                             |           |
| **Partial State Error**  | Atomic Reference Swap                                    |           |
| **Unauthorized Sync**    | JWT Authentication (M2M)                                 |           |
