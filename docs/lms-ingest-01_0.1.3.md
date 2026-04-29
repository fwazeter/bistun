# Ingestion & Truth Hierarchy

### Version: 0.1.3

**Status:** Implementation Reference

## I. Overview

The **LMS Ingestion Pipeline** is the automated mechanism for populating and updating the service's linguistic index. It
formalizes **Phase 8** of the Master Plan by establishing a "Truth Hierarchy" that resolves conflicts between automated
scrapers and human-curated data. This ensures the "System of Record" stays synchronized with global standards like ISO
and CLDR while maintaining local project-specific overrides.

---

## II. The Truth Hierarchy

When multiple sources provide data for the same `Language_ID` or `Script_ID`, the pipeline applies the following
priority levels (Tier 1 is absolute):

| Priority   | Source Type          | Description                                                                           |
|:-----------|:---------------------|:--------------------------------------------------------------------------------------|
| **Tier 1** | **Manual Curator**   | Manually entered traits via the Curator UI; overrides all automation.                 |
| **Tier 2** | **CLDR (Unicode)**   | Primary source for locale-specific plural rules and script mappings.                  |
| **Tier 3** | **ISO Standards**    | Fundamental identity data from ISO 639-3 and ISO 15924 scrapers.                      |
| **Tier 4** | **SIL / Ethnologue** | Supplemental morphological data (e.g., `Morphology_Type`) for rare/ancient languages. |

---

## III. Automated Source Connectors

The pipeline utilizes modular connectors to fetch data from the "Global Truth" providers:

1. **ISO-639-3 Scraper**: Fetches the UTF-8 tab-delimited files from SIL International to update language statuses (
   Living, Extinct, Ancient).
2. **Unicode CLDR Importer**: Synchronizes with the Common Locale Data Repository to update `Plural_Rules` and script
   directionality.
3. **ISO-15924 Scraper**: Pulls technical script definitions, including primary Unicode blocks.

---

## IV. Collision & Drift Management

To prevent automated updates from corrupting the registry, the pipeline follows a "Drift Detection" workflow:

* **Shadow Ingestion**: New data is first loaded into a staging environment.
* **Conflict Detection**: If a Tier 3 update contradicts a Tier 1 (Manual) entry, the automated update is blocked for
  that specific field.
* **Validation Gate**: Every ingested record must pass the **Linguistic Linter** (`LMS-VAL-01`) before it is eligible
  for a new `RegistryBundle`.
* **Audit Logging**: Every change, regardless of source, is recorded with the source URI and timestamp to support the
  WORM (Write-Once, Read-Many) architecture.

---

## V. Promotion & Snapshotting

Once ingested data is validated:

1. **Version Increment**: The system calculates the new `SemVer` (Patch for Tier 3 updates, Minor for Tier 1/2 changes).
2. **Snapshot Creation**: `ILinguisticRepository` triggers `SaveRegistrySnapshot`.
3. **Sync Availability**: The new version becomes available via the `/sync` endpoint for Sidecar SDKs to perform an *
   *Atomic Hot-Swap**.
