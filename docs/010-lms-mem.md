# LMS-MEM: Flyweight & Atomic Swap Management

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026

**Status:** Implementation Reference

---

## I. Overview

The LMS SDK is designed to operate in high-traffic environments where memory efficiency and zero-downtime updates are
critical. To support 7,000+ languages identified by ISO 639-3, the SDK utilizes the **Flyweight Pattern** to minimize
redundant data storage. Furthermore, it employs an **Atomic Reference Swap** to update the registry in-memory without
blocking active capability requests.

---

## II. The Flyweight Implementation

The technical "Physics" of scripts and "Genetic" identities of languages are often shared across hundreds of locales.

### 1. Shared Instance Pools

Instead of each `Locale_Registry` entry holding its own copy of script or language data, it holds a reference (ID) to a
shared, immutable instance.

* **`LanguagePool`**: Stores unique `Language_Definition` objects keyed by `Language_ID`.
* **`ScriptPool`**: Stores unique `Script_Definition` objects keyed by `Script_ID`.

### 2. Memory Optimization Logic

* **Redundancy Reduction**: By storing "Arabic Script" data once and referencing it for locales like `ar-SA`, `ar-EG`,
  and `fa-IR`, the memory footprint is reduced by over 80%.
* **Immutability**: Once an object is placed in the pool, it is immutable to ensure thread-safe read access by the
  `TraitAggregator`.

---

## III. The `RegistryStore` Container

The `RegistryStore` is the unified object that holds the current state of the linguistic universe.

* **Structure**: It contains the `LanguagePool`, `ScriptPool`, and the `Locale_Registry` map.
* **Lookups**: It provides $O(1)$ access to shared definitions during the resolution and aggregation phases.

---

## IV. Atomic Hot-Swap Mechanism

To ensure the "System of Record" remains updated without service interruption, the SDK implements a "Shadow Registry"
swap.

### 1. Update Workflow

1. **Sync Trigger**: The SDK receives a version update notification or hits the `/sync` endpoint.
2. **Shadow Hydration**: A new, temporary `RegistryStore` (the "Shadow Registry") is created and populated in a
   background thread.
3. **Atomic Swap**: Once hydration is complete, the `LinguisticManager` updates its primary internal reference to point
   to the new `RegistryStore` using an atomic pointer swap.
4. **Garbage Collection**: The old registry, no longer referenced, is naturally cleared from memory.

### 2. Thread Safety

* **No Locking**: Because the swap is atomic and the underlying data is immutable, active requests can finish processing
  using the "old" registry while new requests immediately use the "new" one.
* **Consistency**: This prevents "partial state" errors where a request might otherwise see a mix of two different
  registry versions.

---

## V. Performance Targets

* **Initialization Time**: Bootstrapping the registry should occur in the background to avoid blocking application
  startup.
* **Resolution Latency**: The architecture must maintain a resolution budget of $< 1\text{ms}$ even during a registry
  swap.
