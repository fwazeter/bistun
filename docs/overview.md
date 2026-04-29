# Specification Overview

### Version: 0.1.3

## LMS-CORE-01: The Core SDK Interface Specification

This is the "front door" of the LMS for any application. It handles the lifecycle of the linguistic data, from the
initial sync to the final manifest request.

### Primary Class: `LinguisticManager`

* **Method `Initialize(Config options)`**: Sets up the local cache and establishes connection parameters for the sidecar
  service.
* **Method `Sync()`**: Triggers the `GET /v1/registry/sync` call to fetch the Flyweight definitions.
* **Method `GetManifest(String localeTag)`**: The main entry point. Orchestrates the resolver chain and aggregator
  logic.

---

## LMS-ENG-01: Logical Engine & Resolver Hierarchy

This implements the **Chain of Responsibility** to ensure we never hit a dead end when looking for a locale.

### Interface: `IResolver`

* **Method `Resolve(String tag, Registry cache)`**: Returns a `LocaleEntry` or passes to the next resolver.

### Concrete Resolvers:

* **`ExactMatchResolver`**: Performs a direct O(1) lookup in the registry.
* **`TruncationResolver`**: Implements RFC 4647 logic to strip subtags until a match is found.
* **`AliasResolver`**: Checks a lookup table for legacy tags (e.g., `zh-TW` → `zh-Hant`).
* **`DefaultResolver`**: The safety net that returns the hardcoded system default (e.g., `en-US`).

---

## LMS-DNA-01: Trait Aggregator & High-Water Mark Logic

This class is the "brain" of the operation. It takes the resolved locale and its associated scripts and distills them
into a single manifest.

### Class: `TraitAggregator`

* **Function `Aggregate(Language lang, List<Script> scripts)`**: The orchestration function.
* **Logic: `PositionalPriority`**: Specifically pulls `PRIMARY_DIRECTION` from `scripts[0]`.
* **Logic: `CumulativeUnion`**: Sets `HAS_BIDI_ELEMENTS` and `REQUIRES_SHAPING` to true if *any* script in the list has
  those properties.
* **Logic: `HighWaterMark`**: Compares `Segmentation` types and returns the one with the highest computational
  complexity (e.g., `DICTIONARY` > `SPACE`).

---

## LMS-STRAT-01: The Strategy Pattern Registry

This allows the SDK to choose the right algorithm for things like stemming or normalization without having to know the
language specifically.

### Interface: `ILinguisticStrategy`

* **Method `Execute(String input)`**: Performs the specific linguistic operation.

### Registry Mapping:

* **`StemmingProvider`**: Maps `Morphology_Type` (e.g., `TEMPLATIC`) to a strategy class (e.g.,
  `RootExtractionStrategy`).
* **`SegmentationProvider`**: Maps `SEGMENTATION_STRATEGY` to the correct boundary detection logic.

---

## LMS-MEM-01: Flyweight & Atomic Swap Management

To keep the SDK light and fast (performance target: < 1ms), we handle memory with extreme care.

### Class: `RegistryStore`

* **The Flyweight Factory**: Stores unique instances of `Language_Definition` and `Script_Definition`.
* **Memory Optimization**: Reuses these shared objects across 7,000+ potential locales to reduce memory footprint by
  80%.
* **Atomic Reference Swap**: When a new registry version is synced, the SDK swaps the pointer to the `RegistryStore`
  atomically to ensure thread safety without locking.

---

## LMS-DTO-01: Formal Trait & Manifest Schema

This defines the structure of the data traveling from the SDK to the calling application.

### Class: `CapabilityManifest`

* **Field `ResolvedLocale`**: String (BCP 47).
* **Field `Traits`**: A typed `Map<TraitKey, Object>`.
* **Field `Metadata`**: Contains `RegistryVersion` and `ResolutionTime` for observability.

### Enum: `TraitKey`

* Includes `PRIMARY_DIRECTION`, `HAS_BIDI_ELEMENTS`, `REQUIRES_SHAPING`, `SEGMENTATION_STRATEGY`, `MORPHOLOGY_TYPE`, and
  `UNICODE_PRELOAD_BLOCKS`.
