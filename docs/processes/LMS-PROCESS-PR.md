# LMS-PROCESS-PR: Pull Request & Narrative Mapping

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/29/2026  
> **Status:** Operational Process Guide

---

## I. Overview

In the Bistun LMS project, a Pull Request (PR) is not just a code submission; it is a proposed update to the **System of Record**. To maintain absolute architectural integrity, every PR must narrate its changes using the **Logic Trace Mapping** system. This process ensures that any developer—now or three years from now—can understand the exact "thought process" behind a code change without reading the implementation line-by-line.

---

## II. The "Logic Trace Mapping" Philosophy

Following the **LMS-DOC** standard, all public functions must contain an internal `# Logic Trace`. The PR process extends this requirement to the commit level.

### 1. Architectural Alignment
Every PR must reference a **Blueprint ID** (e.g., `Ref: [008-LMS-DNA]`) and a **ROADMAP Milestone** (e.g., `v0.1.0 Chunk 3`). This ensures that we are building the system according to the prioritized dependency gravity.

### 2. The Narrative Check
A PR will be rejected if the description merely says "fixed a bug" or "implemented aggregator." You must provide the play-by-play steps as defined in the **PR Template**.

---

## III. Comparative Examples

### Example A: Poor Submission (Rejected)
> **Title**: Update traits  
> **Description**: I added some logic to the aggregator to handle RTL scripts. Tests pass.

* **Why it fails**: No Logic Trace. No reference to blueprints. No mention of the performance budget.

### Example B: Elite Submission (Approved)
> **Title**: [v0.1.0] Implement Positional Priority Logic  
> **Ref**: [008-LMS-DNA]  
> **Chunk**: v0.1.0 Chunk 3
>
> **Logic Trace Mapping**:
> 1.  [Step 1]: Ingest the priority-ordered `Script_Manifest` array.
> 2.  [Step 2]: Extract the `Directionality` trait from index `[0]` to set the `PRIMARY_DIRECTION`.
> 3.  [Step 3]: Validate that the resulting enum matches the `Direction` standard in `traits.rs`.
>
> **Performance**: Complexity is $O(1)$ as we only check the first element. Well within the 1ms budget.

---

## IV. The PR Checklist Standards

Before marking a PR as "Ready for Review," the contributor must verify the following:

* **Hermeticity**: Are all unit tests mocked using the `mockall` patterns defined in **LMS-TEST**?
* **Executable Specs**: Does the `# Examples` section in the code serve as a functional "Golden Path" test?
* **Narrative Flow**: Does the module-level `//!` documentation explain the **System Impact** if this code were deleted?

---

## V. Placement Logic

This guide live in `docs/processes/`. It serves as the primary reference for code reviewers and new contributors to ensure that our **v0.1.4** engineering standards are applied uniformly across the entire repository.
