# Security Policy: Bistun LMS

## I. Overview
The Bistun Linguistic Metadata Service (LMS) is the **System of Record** for Linguistic DNA. Because we employ cryptographic verification (JWS) to protect the registry from "Linguistic Poisoning," the security of our resolution pipeline and synchronization protocols is paramount.

## II. Supported Versions
Security updates are currently provided only for the active development branch leading to v1.0.0.

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## III. Reporting a Vulnerability
If you discover a security vulnerability—specifically regarding JWS verification, Public Key pinning, or registry injection—**do not open a public issue**.

Please follow this process:
1.  **Report**: Email a detailed description to **[design@wazeter.com](mailto:design@wazeter.com)**.
2.  **Acknowledge**: We will confirm receipt within 48 hours.
3.  **Remediate**: A fix will be developed following the **LMS-TEST** hermetic standards.
4.  **Disclose**: Once the fix is deployed to the Sidecar SDKs, we will coordinate a public disclosure.

## IV. Scope
This policy covers:
* The `LinguisticManager` state machine and synchronization logic.
* The JWS signing and SDK-side verification mechanisms.
* The atomic reference swap implementation.

This policy does not cover third-party strategy implementations or the Curator UI staging environment.