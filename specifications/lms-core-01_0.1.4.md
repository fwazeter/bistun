# LMS-CORE-01: The Core SDK Interface Specification

### Version: 0.1.4

**Status:** Implementation Reference

## I. Overview

This specification defines the primary entry point for the Linguistic Metadata Service (LMS) Sidecar SDK. The
`LinguisticManager` is responsible for registry synchronization, cryptographic verification, telemetry reporting, and
orchestration of the Capability Engine to return a `CapabilityManifest`.

---

## II. Primary Interface: `LinguisticManager`

The `LinguisticManager` acts as a singleton within the consuming application's lifecycle to ensure a consistent,
thread-safe view of linguistic data.

### 1. Initialization and Lifecycle

* **`Initialize(Config options)`**: Accepts a configuration object containing the sidecar service URL, security
  credentials, and telemetry sinks.
* **`Shutdown()`**: Gracefully terminates sync observers and clears in-memory registries to prevent memory leaks.

### 2. Registry Synchronization

* **`Sync()`**:
    * Executes an authenticated call to `GET /v1/registry/sync` using the configured security credentials.
    * Validates the JWS signature of the incoming `RegistryBundle` against the LMS Public Key.
    * Upon successful verification, invokes the **Atomic Hot-Swap** mechanism to update the in-memory `RegistryStore`.

### 3. Capability Resolution

* **`GetManifest(String localeTag)`**:
    * Resolves the `localeTag` into a `CapabilityManifest` object.
    * Captures resolution metrics (e.g., latency, path depth) for telemetry export.

---

## III. Configuration Schema (`Config`)

The SDK requires specific parameters to maintain security and meet performance targets.

| Parameter           | Type     | Default                 | Purpose                                                                      |
|:--------------------|:---------|:------------------------|:-----------------------------------------------------------------------------|
| `SidecarEndpoint`   | String   | `http://localhost:8080` | Location of the LMS Sidecar service.                                         |
| `SecurityConfig`    | Object   | (Empty)                 | Contains JWT/M2M credentials and the LMS Public Key for bundle verification. |
| `TelemetryConfig`   | Object   | (Enabled)               | Defines sinks for SLI metrics (Prometheus/Grafana) and tracing headers.      |
| `EnableAutoSync`    | Boolean  | `true`                  | Enables the Pub/Sub observer for registry updates.                           |
| `PerformanceBudget` | Duration | `1ms`                   | Targeted latency for manifest resolution.                                    |

---

## IV. Core Workflow Logic

The internal execution of `GetManifest` follows a strictly monitored pipeline:

1. **Resolution Phase**: The `ResolverChain` processes the `localeTag`.
2. **Aggregation Phase**: The `TraitAggregator` merges properties using the **High-Water Mark Strategy**.
3. **Override Phase**: BCP 47 `-u-` extensions are parsed and injected via the **Unicode Extension Mapper**.
4. **Validation Phase**: The final manifest is checked against the **Linguistic Linter** for co-consistency.
5. **Telemetry Phase**: Metadata (e.g., `resolution_time_ms`) is populated and the record is exported to telemetry
   sinks.

---

## V. Observability and Error Handling

* **Verification Failure**: If a registry signature is invalid, the `Sync()` operation must fail, logging a critical
  security alert, while the manager continues to serve data from the last verified snapshot.
* **Latency Tracking**: The SDK must export p99 resolution latency to ensure adherence to the performance budget.
* **Registry Drift**: Alerts are triggered if the local registry version falls significantly behind the available
  production version.
