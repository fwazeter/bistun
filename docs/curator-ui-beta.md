# Functional Specification: Registry Curator UI

## I. Overview

The Registry Curator UI is a centralized management dashboard used by linguists to maintain the immutable properties of
languages, the technical requirements of scripts, and the implementation mappings of locales. It serves as the
administrative frontend for the **Atomic Capability Provider**.



---

## II. Governance & Security (RBAC)

To protect the integrity of the linguistic data, the UI implements **Role-Based Access Control (RBAC)**:

* **Viewer**: Can browse and search all registry entries but cannot make changes.
* **Linguist**: Can create and edit `Language_Definition` and `Script_Definition` entries.
* **Administrator**: Can manage `Locale_Registry` mappings, approve registry version releases, and manage user
  permissions.

---

## III. Core Functional Modules

### 1. Language Management (Identity)

This module allows for the curation of "Genetic" language traits.

* **ISO 639-3 Registry**: A searchable list of language IDs (3-8 characters).
* **Trait Editor**: Form-based editing for `Morphology_Type` (Enum), `Synthesis_Degree` (1–10), and `Normalization`
  requirements.
* **Historical Timeline**: Selection for the language `Period` (Living, Ancient, etc.).

### 2. Script Management (Physics)

This module manages the technical "physics" of writing systems.

* **ISO 15924 Registry**: Management of 4-letter script codes (e.g., `Hebr`).
* **Rendering Configuration**: Toggle for `Shaping_Req` and dropdowns for `Directionality` and `Segmentation`.
* **Unicode Block Manager**: An interface to map shared hex ranges from the `Unicode_Registry` to specific scripts.

### 3. Locale Registry (Mapping)

The implementation layer where languages are paired with scripts for specific regions.

* **BCP 47 Constructor**: A builder for creating valid locale tags.
* **Manifest Orchestrator**: An interface to define the priority-ordered `Script_Manifest` for a locale (e.g., placing
  `Hani` before `Latn` for Japanese).
* **Localization Rules**: Dropdowns to assign specific `Plural_Rules` and ICU collation IDs.

---

## IV. Registry Versioning & Deployment

The UI acts as the staging area for new registry versions.

* **Staging vs. Production**: Changes are saved to a draft state before being bundled into a new **Semantic Version (
  SemVer)**.
* **The "Publish" Gate**: Administrators must trigger a "Sync Update" which notifies the Sidecar SDKs via Pub/Sub to
  perform an **Atomic Reference Swap**.
* **Audit Log**: A permanent, read-only record of who changed which trait and when, essential for the "System of Record"
  status.

---

## V. Quality Assurance Integration

Before a new version is published, the Curator UI runs the **Golden Set QA** suite.

* **Automated Validation**: Checks for "orphan" scripts (scripts not used by any locale) or invalid ISO codes.
* **Regression Testing**: High-priority locales (the "Edge Case Locales") are automatically validated against the new
  traits to ensure no breaking rendering changes.


