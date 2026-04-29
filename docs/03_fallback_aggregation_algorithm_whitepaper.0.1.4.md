# The Fallback & Aggregation Algorithm Whitepaper

### Version: 0.1.4

This whitepaper details the internal logic of the **Capability Engine**, specifically how it transforms raw registry
data into the decoupled **CapabilityManifest DTO**. The algorithm ensures that regardless of locale complexity,
consuming services receive a predictable set of functional traits.

---

### 3.1 Phase 1: Locale Resolution (Chain of Responsibility)

The engine utilizes a **Chain of Responsibility** pattern to resolve a `Locale_Tag` into a specific `Locale_Registry`
entry. Each resolver in the chain attempts a match before passing the request to the next link:

1. **Exact Match Resolver**: Checks for the literal BCP 47 string.
2. **Truncation Resolver (RFC 4647)**: Progressively strips subtags to find the most specific broader match.
3. **Script Alias Resolver**: Maps regional or legacy aliases (e.g., `zh-TW` to `zh-Hant`) to canonical standards.
4. **Default Fallback Resolver**: Returns the "System Default" (e.g., `en-US`) to ensure a manifest is always returned.

---

### 3.2 Phase 2: Attribute Aggregation (Trait Merging)

Once a locale is resolved, the engine fetches the **Language_Definition** and the associated **Script_Definitions**. The
engine applies a multi-tier merging logic to populate the **Trait Container**:

* **Tier 1: Positional Priority (Primary Context)**: The `PRIMARY_DIRECTION` trait is derived exclusively from the *
  *first script** index in the `Script_Manifest`.
* **Tier 2: Boolean OR Aggregation (Global Capabilities)**:
    * `HAS_BIDI_ELEMENTS`: Set to `true` if **any** script in the manifest is `RTL`.
    * `REQUIRES_SHAPING`: Set to `true` if **any** script in the manifest requires contextual shaping.
* **Tier 3: Array Union (Resource Optimization)**: `UNICODE_PRELOAD_BLOCKS` merges all HEX ranges from every script into
  a single, de-duplicated set.

---

### 3.3 Phase 3: Strategy Selection (Logic Mapping)

The engine uses the **Strategy Pattern** to map linguistic "DNA" to functional software logic.

| Capability        | Trait Input       | Logic Strategy Selection                                                                                             |
|:------------------|:------------------|:---------------------------------------------------------------------------------------------------------------------|
| **Stemming**      | `Morphology_Type` | Maps types (e.g., `TEMPLATIC`) to specific algorithms (e.g., `ROOT_EXTRACTION`).                                     |
| **Segmentation**  | `Segmentation`    | **High-Water Mark Strategy**: Selects the most complex strategy (e.g., `DICTIONARY`) required by any script present. |
| **Normalization** | `Normalization`   | Defaults to the language-specific Unicode recommendation (NFC/NFD).                                                  |

---

### 3.4 Phase 4: Atomic Extension Overrides

The engine processes BCP 47 `-u-` (Unicode) extensions as an **Atomic Override** layer via the **Unicode Extension
Mapper**.

1. **Parse**: The engine extracts extension keys and values (e.g., `-u-nu-latn`).
2. **Translate**: Extension keys (e.g., `nu`) are matched to LMS `TraitKey` targets (e.g., `NUMBERING_SYSTEM`).
3. **Inject**: Resulting values are injected into the `traits` Map, overriding values derived from the DNA registry.

---

### 3.5 Phase 5: Validation Gate (Linguistic Linter)

The final resulting manifest is passed through the **Linguistic Linter** to ensure DNA consistency.

* **Consistency Check**: Enforces compatibility between `Morphology_Type` and selected strategies (e.g., preventing a
  `RootExtraction` strategy on an `ISOLATING` language).
* **Structural Integrity**: Validates that all IDs follow ISO and BCP standards.
* **Complexity Check**: Re-verifies that High-Water Mark constraints for segmentation and BIDI directionality are
  upheld.