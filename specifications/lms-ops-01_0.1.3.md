# Telemetry & Observability

### Version: 0.1.3

**Status:** Implementation Reference

---

## I. Overview

The **LMS Operational Playbook** defines the telemetry, logging, and health-checking standards required to maintain the
service's performance budget of $< 1\text{ms}$ in high-traffic production environments. As a sidecar-deployed "System of
Record," the SDK must provide granular visibility into its resolution logic, memory efficiency, and synchronization
health without introducing significant overhead.

---

## II. Service Level Indicators (SLIs)

The following metrics are the primary indicators of system health and must be exported via the `TelemetryConfig`.

### 1. Latency Metrics

* **Resolution Time (p99)**: The time taken to resolve a locale and aggregate traits.
* **Target**: $< 1\text{ms}$ for cached registry hits.
* **Sync Latency**: The duration of the `RegistryBundle` download and hydration phase during an update.

### 2. Reliability Metrics

* **Registry Cache Misses**: Frequency of fallback to the "System Default" due to unresolvable tags.
* **Sync Success Rate**: The percentage of successful `/sync` calls vs. failures (e.g., network timeouts, signature
  verification failures).
* **Atomic Swap Success**: Counts of successful shadow-registry swaps vs. aborted attempts due to validation errors.

---

## III. Distributed Tracing & Metadata

To support cross-service debugging, the `CapabilityManifest` injects observability data into its metadata map.

* **`X-LMS-Resolution-Time`**: Injected as an HTTP header or metadata field to track processing time per request.
* **Resolution Path Analysis**: The `resolution_path` (e.g., `["en-AU", "en-GB", "en"]`) allows engineers to identify
  inefficient fallback chains or missing regional data.
* **Registry Versioning**: Every trace must log the `registry_version` to correlate linguistic behavior with specific
  data releases.

---

## IV. Resource Observability

Given the **Flyweight Pattern** used to support 7,000+ languages, memory tracking is critical.

* **Pool Utilization**: Monitoring the memory footprint of the `LanguagePool` and `ScriptPool`.
* **Redundancy Ratio**: A metric calculating the memory saved by using Flyweights versus a flat object structure (
  Target: $> 80\%$ reduction).
* **GC Impact**: Monitoring Garbage Collection spikes immediately following an **Atomic Reference Swap** as the old
  registry is cleared.

---

## V. Health Checks & Alerting Thresholds

| Metric                   | Alert Threshold (Warning)      | Alert Threshold (Critical)     |
|:-------------------------|:-------------------------------|:-------------------------------|
| **Resolution Latency**   | $> 2\text{ms}$ (Avg)           | $> 5\text{ms}$ (Avg)           |
| **Registry Version Age** | $> 48\text{h}$ since last sync | $> 72\text{h}$ since last sync |
| **Verification Failure** | N/A                            | 1+ Signature Mismatch          |
| **Cache Miss Rate**      | $> 5\%$ of total requests      | $> 15\%$ of total requests     |

