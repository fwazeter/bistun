# Technical Specification — LMS API Contract

### Version: 0.1.4

This document formalizes the interaction between the **Atomic Capability Provider** and consuming services. It defines
how the system resolves linguistic complexity into a decoupled, map-based **CapabilityManifest DTO**, ensuring 100%
alignment with the **Linguistic DNA** and security standards.

---

## I. Service Endpoints & API Contract

### 1.1 `GET /v1/manifest/{locale_tag}`

Resolves a specific BCP 47 tag into a complete set of linguistic traits.

* **Parameters**: `locale_tag` (String): A standard BCP 47 language tag (e.g., `arc-Syrc-IQ`).
* **Request Headers**:
    * `Authorization`: Mandatory Bearer JWT for M2M authentication.
* **Response Headers**:
    * `X-LMS-Registry-Version`: Current semver of the linguistic data.
    * `X-LMS-Resolution-Time`: Total latency of the resolution and aggregation pipeline (Performance
      Target: $< 1\text{ms}$).
* **Logic**:
    1. Executes the **Capability Engine** flow: resolving the locale fallback (Chain of Responsibility).
    2. Aggregates script traits via the **TraitAggregator**.
    3. Injects BCP 47 `-u-` extensions using the **Unicode Extension Mapper** logic as an atomic override.
    4. Passes the final manifest through the **Linguistic Linter** to ensure DNA consistency.

### 1.2 `GET /v1/registry/sync?version={semver}`

Allows the **Sidecar SDK** to fetch the entire registry for local in-memory caching.

* **Parameters**: `version` (String): The specific semantic version of the registry required.
* **Response**: A compressed **JWS (JSON Web Signature)** payload. The payload contains shared, immutable instances of
  `Language_Definition` and `Script_Definition`, while the protected header contains the cryptographic signature for
  integrity verification.

### 1.3 `GET /v1/capabilities/schema`

Returns the current schema of all supported **Trait Keys**, including standard DNA keys and Unicode extension mappings.

---

## II. The CapabilityManifest DTO (Trait Container)

The DTO utilizes a **Map-based Trait Container** to prevent breaking API contracts as new traits or Unicode extensions
are indexed.

### 2.1 Structure Component Table

| Field             | Type                 | Purpose                                                                                   |
|:------------------|:---------------------|:------------------------------------------------------------------------------------------|
| `resolved_locale` | String               | The actual BCP 47 tag used after fallback resolution.                                     |
| `traits`          | `Map<TraitKey, Any>` | Dynamic collection of capabilities, including base DNA and overridden Unicode extensions. |
| `metadata`        | Map                  | Registry version, resolution path, resolution time, and verification status.              |

### 2.2 Standard Trait Keys (The "Golden Set")

Consuming services should expect these keys at a minimum:

* `PRIMARY_DIRECTION`: (Enum) `LTR`, `RTL`, `TTB`.
* `HAS_BIDI_ELEMENTS`: (Boolean) True if any secondary script is `RTL`.
* `REQUIRES_SHAPING`: (Boolean) True if contextual glyph shaping is required.
* `SEGMENTATION_STRATEGY`: (Enum) `SPACE`, `DICTIONARY`, `CHARACTER`, `NONE`.
* `MORPHOLOGY_TYPE`: (Enum) `ISOLATING`, `AGGLUTINATIVE`, `FUSIONAL`, `TEMPLATIC`, `POLYSYNTHETIC`.
* `UNICODE_PRELOAD_BLOCKS`: (Array) List of hex ranges for font-loading automation.
* `NUMBERING_SYSTEM`: (String) Derived from DNA or `-u-nu-` extension override.

---

## III. Technical Implementation Patterns

### 3.1 The Flyweight Pattern

To optimize the **Sidecar SDK** memory footprint, the SDK maintains a singleton cache of `Language_Definition` and
`Script_Definition` objects, using references to minimize redundancy across 7,000+ entries.

### 3.2 Atomic Hot-Swap & Security Verification

Updates received via the `/sync` endpoint must be verified against the LMS Public Key. Once verified and fully hydrated
in a **shadow registry**, the SDK performs an **Atomic Reference Swap** to update the active linguistic data without
blocking requests.
