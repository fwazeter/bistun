# LMS-EXT: Unicode Extension Mapping

> **Version:** 0.1.4
<br> **Author:** Francis Xavier Wazeter IV
<br> **Date:** 04/29/2026

**Status:** Implementation Reference

---

## I. Overview

The **Unicode Extension Mapper** is responsible for the "Atomic Override Phase" of the capability resolution pipeline.
It parses the `-u-` (Unicode) subtag of a BCP 47 locale tag and injects the resulting values directly into the `traits`
Map of the `CapabilityManifest`. This ensures that specific user or application preferences (e.g., Latin numerals in an
Arabic locale) take precedence over the default "Linguistic DNA" stored in the registry.

---

## II. Mapping Table: Extensions to TraitKeys

The following table defines the official translation layer between standard BCP 47 Unicode keys and the LMS `TraitKeys`.

| BCP 47 Key | Extension Name      | LMS TraitKey Target       | Expected Value Examples                    |
|:-----------|:--------------------|:--------------------------|:-------------------------------------------|
| `nu`       | Numbering System    | `NUMBERING_SYSTEM`        | `latn`, `arab`, `deva`, `fullwide`         |
| `ca`       | Calendar System     | `CALENDAR_SYSTEM`         | `gregory`, `islamic`, `buddhist`, `hebrew` |
| `kf`       | Case First          | `COLLATION_CASE_PRIORITY` | `upper`, `lower`, `false`                  |
| `kb`       | Backwards Collation | `COLLATION_BACKWARDS`     | `true`, `false`                            |
| `co`       | Collation Type      | `COLLATION_STRATEGY`      | `phonebk`, `pinyin`, `standard`            |

---

## III. Execution Logic (Atomic Override)

The `LinguisticManager` invokes this logic after the `TraitAggregator` has completed the base manifest synthesis.

1. **Parse**: Extract the Unicode extension string from the input `localeTag` (e.g., `-u-nu-latn-ca-gregory`).
2. **Key Translation**: Match the BCP 47 keys (`nu`, `ca`) to their corresponding `TraitKey` constants.
3. **Injection**: Insert the translated pairs into the `traits` Map.
4. **Override**: If a key already exists (e.g., the language definition specifies `nu-arab`), the extension value (
   `latn`) **MUST** overwrite the existing value.

---

## IV. Conflict Resolution & Validation

To maintain the integrity of the service, the extension values are subject to the following constraints:

* **Linter Integration**: Overridden values are passed through the **Linguistic Linter** to ensure they don't create
  technical impossibilities (e.g., an RTL script attempting to use a TTB-only collation strategy).
* **Malformed Tags**: If a subtag key is recognized but the value is malformed or not supported by the current registry
  version, the override is ignored, and a warning is logged in the `metadata` map of the DTO.

---

## V. Example: Mixed-DNA Manifest

**Input Locale**: `ar-EG-u-nu-latn`

1. **Registry Resolution**: Resolves to Egyptian Arabic.
2. **Base Aggregation**: Sets `PRIMARY_DIRECTION: RTL` and `NUMBERING_SYSTEM: arab`.
3. **Extension Override**: Detects `nu-latn`.
4. **Final Manifest**:
    * `PRIMARY_DIRECTION: RTL`
    * `NUMBERING_SYSTEM: latn` (Overridden)
