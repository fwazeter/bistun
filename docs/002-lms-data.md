# LMS-DATA-01: Persistence & Repository Pattern

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026

**Status:** Implementation Reference

---

## I. Overview

The `LinguisticRepository` abstracts the underlying storage technology—whether relational (SQL), document-oriented (
NoSQL), or distributed—from the core Capability Engine. It is responsible for the hydration of the `RegistryStore` and
ensuring the integrity of the "Linguistic DNA" at rest. By utilizing a "Write-Once, Read-Many" (WORM) approach, it
guarantees that every version of the registry is immutable and reproducible.

---

## II. Primary Interface: `ILinguisticRepository`

To maintain program-agnostic interoperability, the repository provides a unified set of methods for fetching the three
pillars of the LMS: Languages, Scripts, and Locales.

### 1. Version & Aggregate Management

* **`GetRegistryManifest(version)`**: Returns the high-level metadata and checksum for a specific `SemVer`.
* **`ExportFullRegistry(version)`**: Returns a `RegistryBundle` containing all `LanguagePool` and `ScriptPool` data
  required for a full SDK synchronization.

### 2. Entity-Level Retrieval

| Method                     | Returns              | Purpose                                                           |
|:---------------------------|:---------------------|:------------------------------------------------------------------|
| `GetLanguage(id, version)` | `LanguageDefinition` | Fetches specific morphological/syntactic traits.                  |
| `GetScript(id, version)`   | `ScriptDefinition`   | Fetches technical rendering requirements (Direction, Shaping).    |
| `GetLocale(tag, version)`  | `LocaleEntry`        | Fetches the mapping between a BCP 47 tag and its constituent IDs. |

---

## III. Data Access Strategy: The "Snapshot Pattern"

To support the **Atomic Hot-Swap** mechanism in high-traffic environments, the repository eschews standard CRUD
operations in favor of versioned snapshots.

### 1. Version Pinning

Every query to the repository must include a `SemVer`. This ensures that a `CapabilityManifest` generated at $T_1$
remains consistent and reproducible even if the global registry is updated at $T_2$.

### 2. Batch Hydration

To minimize I/O overhead during the resolution budget of $< 1\text{ms}$, the `ExportFullRegistry` method performs a
unified fetch (e.g., a SQL JOIN or multi-document aggregation) to return the entire `LanguagePool` and `ScriptPool` in a
single operation.

---

## IV. Repository Architecture

The implementation is divided into three distinct layers to ensure scalability and observability.

* **L3: Persistence Layer**: The physical storage (e.g., PostgreSQL). The schema enforces referential integrity between
  `Locale_Registry.Lang_Ref` and `Language_Definition.Language_ID`.
* **L2: Cache Layer**: A distributed cache (e.g., Redis) that stores the `RegistryBundle` for the "Active Production"
  version. This prevents `Sync()` calls from 1,000+ sidecar instances from saturating the primary database.
* **L1: Mapping Layer**: Responsible for converting database-specific records into the immutable DTOs defined in *
  *LMS-DTO-01**.

---

## V. Interaction Workflow: Sync & Hydration

The lifecycle of a registry update follows a strict sequence to prevent partial-state errors:

1. **Snapshot**: An administrator promotes a new version via a Curator UI; the `LinguisticManager` triggers
   `SaveRegistrySnapshot`.
2. **Invalidation**: The Repository invalidates the L2 `RegistryBundle` cache.
3. **SDK Sync**: Consuming Sidecar SDKs call `GET /v1/registry/sync`.
4. **Delta-check**: The SDK uses a "Conditional GET" (via ETag or SemVer) to determine if a download is necessary.
5. **Atomic Swap**: The SDK performs the **Atomic Reference Swap** to point the `LinguisticManager` to the new
   `RegistryStore`.

---

## VI. Performance & Integrity

* **Audit Trail**: Every change to a trait must result in a new registry version rather than an in-place update.
* **Validation**: A pre-persistence validator must ensure that manually curated traits do not violate ISO standards or
  create circular resolution aliases.
