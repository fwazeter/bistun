# LMS-CRATE-README-TEMPLATE.md

```markdown
# [CRATE NAME]: [Simple One-Line Subtitle]

[![Domain: [Taxonomy | Typology | Orthography]](https://img.shields.io/badge/Domain-[DOMAIN]-green.svg)](#)
[![Status: [Draft | Engineering Standard | Production]](https://img.shields.io/badge/Status-[STATUS]-yellow.svg)](#)
[![Version: [X.Y.Z]](https://img.shields.io/badge/Version-[VERSION]-blue.svg)](#)

---

## 💡 Elevator Pitch
**What is this?** Imagine you are building a global app and need to know exactly how a specific language (like Thai or Arabic) behaves—how it's written, how words are separated, and how it should look on a screen. 

This crate acts like a **Linguistic DNA Reader**. It takes a simple language code and gives you a "instruction manual" (a manifest) that tells your software exactly how to handle that language's unique rules without you having to be a linguist.

---

## I. Strategic Overview

### 1. The "Why"
[2-sentence explanation of why this crate exists as a standalone workspace member and its specific responsibility in the 5-phase resolution pipeline (Resolve → Aggregate → Override → Integrity → Telemetry)].

### 2. System Impact
[Description of what system functionality fails if this specific crate is compromised. List the downstream crates or services that depend on this output].

### 3. Domain Alignment
This crate operates primarily within the **[Taxonomy | Typology | Orthography]** domain of the Bistun ecosystem.

---

## 🏗️ Technical Architecture

### 1. Internal Logic Flow
The following diagram illustrates how data moves through this specific crate:

```mermaid
graph TD
    A[Input Data] --> B{[Process Name]}
    B -->|Success| C[Transformed Object]
    B -->|Failure| D[LmsError Narrative]
    C --> E[Crate Output/Public API]
```

### 2. Component Relationship
```mermaid
classDiagram
    class [PrimaryStruct] {
        +FieldType field_name
        +execute(input) Result
    }
    class [SupportingEnum] {
        <<enumeration>>
        VARIANT_A
        VARIANT_B
    }
    [PrimaryStruct] *-- [SupportingEnum]
```

---

## 📚 Technical Interface

### 1. Primary API / Data Schema
[Use a table to show the core functions or the fields in the primary data object].

| Function/Field | Input Type | Output/Type | Purpose |
| :--- | :--- | :--- | :--- |
| `function_name()` | `Type` | `Result<T, LmsError>` | Narrative description of role. |
| `field_name` | `N/A` | `Type` | Narrative description of value. |

### 2. Side Effects & SLIs
* **Performance**: Target latency: `< [X]ms`. Complexity: `O(?)`.
* **Observability**: Records spans via `tracing` per **007-LMS-OPS**.

---

## 🚀 Usage & Implementation

### 1. The "Golden Path" Example
```rust
use [crate_name]::{[RequiredTypes]};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // [STEP 1]: Initialize the component
    let component = [PrimaryStruct]::new();

    // [STEP 2]: Execute the core logic
    let result = component.execute("input-tag")?;

    // [STEP 3]: Use the result
    println!("Resolved Capability: {:?}", result);
    
    Ok(())
}
```

---

## 🛠️ Development & Contribution

### 1. Building and Testing
To ensure this crate maintains its "System of Record" integrity, use the following commands:
* **Check Logic**: `cargo test -p [crate-name]`
* **Verify Docs**: `cargo doc -p [crate-name] --open`

### 2. Extension Guide
To add a new feature to this crate:
1. **Red Phase**: Add a failing test case in `tests/` or the internal `mod tests`.
2. **Logic Trace**: Document your proposed implementation steps using the **LMS-DOC** `# Logic Trace` format.
3. **Implementation**: Mirror the trace with `// [STEP X]` comments in the code.

---

## V. Metadata
* **Author**: [Original Author]
* **Version**: [Current Version]
* **License**: GNU GPL v3
* **Date Created**: [YYYY-MM-DD]
* **Date Updated**: [YYYY-MM-DD]

---

### Comparison & Logic Trace
1.  **Elevator Pitch**: Added a non-technical introduction to explain the "Physics" of the crate in simple terms (e.g., comparing a language manifest to an "instruction manual").
2.  **Mermaid Integration**: Included placeholders for both a **Logic Flow** (flowchart) and a **Object Model** (class diagram) to fulfill the visual tech requirement.
3.  **Interface Tables**: Implemented a flexible table structure that can be used for either API methods or data object fields depending on the crate's role (Logic-heavy vs. Data-heavy).
4.  **Contributor-First**: Preserved the "How to Build it Up" section from the module template to ensure new developers follow the **Testing-First** and **Narrative-Code** standards.
