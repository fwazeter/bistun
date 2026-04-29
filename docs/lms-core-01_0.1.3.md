# LMS-CORE-01: The Core SDK Interface Specification

### Version: 0.1.3

**Status:** Implementation Reference

## I. Overview

This specification defines the primary entry point for the Linguistic Metadata Service (LMS) Sidecar SDK. The
`LinguisticManager` is responsible for registry synchronization, memory management, and orchestration of the Capability
Engine to return a `CapabilityManifest`.

---

## II. Primary Interface: `LinguisticManager`

The `LinguisticManager` acts as a singleton within the consuming application's lifecycle to ensure a consistent,
thread-safe view of linguistic data.

### 1. Initialization and Lifecycle

* **`Initialize(Config options)`**:
    * Accepts a configuration object containing the sidecar service URL, default registry version, and sync interval.
    * Triggers the initial bootstrap sequence to populate the local cache.
* **`Shutdown()`**:
    * Gracefully terminates sync observers and clears in-memory registries to prevent memory leaks.

### 2. Registry Synchronization

* **`Sync()`**:
    * Executes a call to `GET /v1/registry/sync`.
    * Validates the incoming payload against the local schema.
    * Invokes the **Atomic Hot-Swap** mechanism to update the in-memory `RegistryStore` without interrupting active
      requests.

### 3. Capability Resolution

* **`GetManifest(String localeTag)`**:
    * The primary method for developers to retrieve linguistic metadata.
    * Input must be a valid BCP 47 tag.
    * Returns a `CapabilityManifest` object containing resolved traits and metadata.

---

## III. Configuration Schema (`Config`)

The SDK requires specific parameters to maintain its "System of Record" status while meeting performance targets.

| Parameter                | Type     | Default                 | Purpose                                            |
|:-------------------------|:---------|:------------------------|:---------------------------------------------------|
| `SidecarEndpoint`        | String   | `http://localhost:8080` | Location of the LMS Sidecar service.               |
| `DefaultRegistryVersion` | SemVer   | `1.0.0`                 | The version to lock to if a sync is unavailable.   |
| `EnableAutoSync`         | Boolean  | `true`                  | Enables the Pub/Sub observer for registry updates. |
| `PerformanceBudget`      | Duration | `1ms`                   | Targeted latency for manifest resolution.          |

---

## IV. Core Workflow Logic

The internal execution of `GetManifest` follows a strict four-phase pipeline to ensure "Maximum DRY" scalability:

1. **Resolution Phase**: The `ResolverChain` processes the `localeTag` to find the correct `LocaleEntry`.
2. **Aggregation Phase**: The `TraitAggregator` merges properties from the language and script manifests, applying the *
   *High-Water Mark Strategy** for complex scripts.
3. **Override Phase**: Any `-u-` extensions in the original tag are parsed and injected as atomic overrides.
4. **Manifest Creation**: The final trait map is packaged into the DTO with resolution metadata and returned.

---

## V. Observability and Error Handling

* **Registry Cache Miss**: If a tag cannot be resolved even by the `DefaultResolver`, the SDK must log a critical
  warning but fall back to the "System Default" defined in the registry to prevent application crashes.
* **Latency Tracking**: Each manifest resolution must populate the `X-LMS-Resolution-Time` metadata field for
  monitoring.

---
