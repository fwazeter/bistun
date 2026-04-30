# LMS-CORE: The Core SDK Interface Specification

> **Version:** 0.1.4
 <br> **Author:** Francis Xavier Wazeter IV
 <br> **Date:** 04/29/2026

**Status:** Implementation Reference

---

## I. Overview
The `LinguisticManager` is the singleton entry point for the LMS Sidecar SDK. It manages the lifecycle of the linguistic registry, ensuring thread-safe access to "Linguistic DNA".

---

## II. SDK State Machine
To ensure reliability, the `LinguisticManager` must transition through these states:
1.  **UNINITIALIZED**: Default state before `Initialize()`.
2.  **BOOTSTRAPPING**: Attempting initial `Sync()` and local cache hydration.
3.  **READY**: Registry is loaded and verified; `GetManifest()` is active.
4.  **SYNCING**: Background update in progress (Atomic Swap pending).
5.  **DEGRADED**: Sync failed; serving from stale/default data via Circuit Breaker.

---

## III. Primary Interface: `LinguisticManager`

The `LinguisticManager` acts as a singleton within the consuming application's lifecycle to ensure a consistent, thread-safe view of linguistic data.

### 1. Initialization and Lifecycle
* **`Initialize(Config options)`**:
  * Starts the **BOOTSTRAPPING** phase.
  * Accepts a configuration object containing the sidecar service URL, security credentials, and telemetry sinks.
  * **Circuit Breaker**: If the initial sync fails and no local cache exists, the manager enters **DEGRADED** state and uses the hardcoded "System Default" to prevent app startup failure.
* **`Shutdown()`**: Gracefully terminates sync observers and clears in-memory registries to prevent memory leaks.

### 2. Registry Synchronization
* **`Sync()`**:
  * Executes an authenticated call to `GET /v1/registry/sync` using the configured security credentials.
  * Validates the JWS signature of the incoming `RegistryBundle` against the LMS Public Key.
  * Upon successful verification, invokes the **Atomic Hot-Swap** mechanism to update the in-memory `RegistryStore`.

### 3. Capability Resolution
* **`GetManifest(String localeTag)`**:
  * Resolves the `localeTag` into a `CapabilityManifest` object.
  * Executes the 5-phase pipeline: Resolve -> Aggregate -> Override -> **Integrity Check** -> Telemetry.
  * **Note**: Unlike the Ingestion Linter, the Runtime Integrity Check only validates that the resulting Map is not null and follows the DTO schema.
  * Captures resolution metrics (e.g., latency, path depth) for telemetry export.

---

## IV. Configuration Schema (`Config`)

The SDK requires specific parameters to maintain security and meet performance targets.

| Parameter           | Type     | Default                 | Purpose                                                                      |
|:--------------------|:---------|:------------------------|:-----------------------------------------------------------------------------|
| `SidecarEndpoint`   | String   | `http://localhost:8080` | Location of the LMS Sidecar service.                                         |
| `SecurityConfig`    | Object   | (Empty)                 | Contains JWT/M2M credentials and the LMS Public Key for bundle verification. |
| `TelemetryConfig`   | Object   | (Enabled)               | Defines sinks for SLI metrics (Prometheus/Grafana) and tracing headers.      |
| `SyncRetryPolicy`   | Object   | {max: 3}                | Defines backoff strategy for failed synchronization attempts.                |
| `EnableAutoSync`    | Boolean  | `true`                  | Enables the Pub/Sub observer for registry updates.                           |
| `PerformanceBudget` | Duration | `1ms`                   | Hard limit for resolution; triggers "Performance Alert" SLI if exceeded.     |

---

## V. Core Workflow Logic

The internal execution of `GetManifest` follows a strictly monitored pipeline:

1.  **Resolution Phase**: The `ResolverChain` processes the `localeTag`.
2.  **Aggregation Phase**: The `TraitAggregator` merges properties using the **High-Water Mark Strategy**.
3.  **Override Phase**: BCP 47 `-u-` extensions are parsed and injected via the **Unicode Extension Mapper**.
4.  **Integrity Check Phase**: The final manifest is checked for structural integrity (Map is not null and follows DTO schema).
5.  **Telemetry Phase**: Metadata (e.g., `resolution_time_ms`) is populated and the record is exported to telemetry sinks.

---

## VI. Observability and Error Handling

* **Verification Failure**: If a registry signature is invalid, the `Sync()` operation must fail, logging a critical security alert, while the manager continues to serve data from the last verified snapshot.
* **Latency Tracking**: The SDK must export p99 resolution latency to ensure adherence to the performance budget.
* **Registry Drift**: Alerts are triggered if the local registry version falls significantly behind the available production version.
