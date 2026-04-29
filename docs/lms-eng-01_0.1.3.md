# LMS-ENG-01: Logical Engine & Resolver Hierarchy

### Version: 0.1.3

**Status:** Implementation Reference

## I. Overview

The Logical Engine is responsible for the first phase of the capability resolution pipeline: **Locale Resolution**. It
ensures that any input BCP 47 tag is mapped to the most relevant available entry in the `Locale_Registry`. By utilizing
a modular chain, the engine remains extensible to new resolution logic (such as project-specific overrides) without
modifying the core SDK architecture.

---

## II. Interface Definition: `IResolver`

Every resolver in the chain must adhere to a standard interface to maintain program-agnostic interoperability.

### 1. The `Resolve` Method

* **Signature**: `Resolve(String tag, Registry cache, IResolver next)`.
* **Logic**:
    1. The resolver attempts to find a match for the provided `tag` within the local `cache`.
    2. If a match is found, it returns the associated `LocaleEntry`.
    3. If no match is found, it calls the `next` resolver in the chain.

---

## III. The Resolver Chain

The standard implementation includes four primary resolvers, executed in sequence.

### 1. `ExactMatchResolver`

* **Purpose**: Performs a high-performance, $O(1)$ lookup for the literal string.
* **Example**: Input `sr-Cyrl-RS` matches key `sr-Cyrl-RS`.

### 2. `TruncationResolver` (RFC 4647)

* **Purpose**: Iteratively strips the rightmost subtag to find the most specific broader match.
* **Logic**:
    * Input: `sr-Cyrl-RS`.
    * Step 1: Check `sr-Cyrl-RS` (Fail).
    * Step 2: Check `sr-Cyrl` (Match).

### 3. `ScriptAliasResolver`

* **Purpose**: Maps regional, legacy, or non-standard aliases to their canonical equivalents.
* **Implementation**: Uses a lookup table (e.g., `zh-TW` $\rightarrow$ `zh-Hant`) before passing the canonical tag back
  to the `TruncationResolver` or `ExactMatchResolver`.

### 4. `DefaultFallbackResolver`

* **Purpose**: The final "Safety Net".
* **Logic**: Returns the "System Default" locale (e.g., `en-US`) if all other links in the chain fail, ensuring the
  `LinguisticManager` never returns a null value to the calling application.

---

## IV. The `ResolverChain` Orchestrator

The Orchestrator manages the instantiation and ordering of the resolvers.

* **Chain Initialization**: At SDK startup, resolvers are registered in priority order.
* **Execution**: When `GetManifest(tag)` is called, the Orchestrator initiates the first resolver.
* **Extensibility**: Third-party developers can inject custom resolvers (e.g., a `UserPreferenceResolver` that checks
  local session data) at the head of the chain.

---

## V. Operational Constraints

* **Maximum Depth**: To prevent infinite loops (particularly with aliasing), the engine should enforce a maximum
  resolution depth (typically 5 iterations).
* **Performance**: Because resolution occurs on the critical path for rendering/search, the combined latency of the
  entire chain must remain within the `PerformanceBudget` of $< 1\text{ms}$.
