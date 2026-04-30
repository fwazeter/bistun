# LMS-PROCESS-HOOKS: Local Quality Enforcement

> **Version:** 0.1.4  
> **Author:** Francis Xavier Wazeter IV  
> **Date:** 04/30/2026  
> **Status:** Operational Process Guide

---

## I. Overview
To maintain a high-velocity development cycle while adhering to strict **LMS-DOC** and **LMS-TEST** standards, we utilize local Git hooks. The `pre-push` hook acts as a local "Circuit Breaker," preventing non-compliant code from reaching the remote repository.

---

## II. The Local Quality Gate
The hook executes `just verify-all` before every push.

### 1. Scope of Verification
* **Formatting**: Enforces `rustfmt.toml` rules.
* **Logic**: Runs all hermetic unit and integration tests.
* **Best Practices**: Runs Clippy with warning denial.
* **Narrative Integrity**: Validates `rustdoc` and intra-doc links.

---

## III. Installation
Developers must initialize local hooks as part of their environment setup.

### 1. Automated Setup
The `justfile` provides a command to configure Git to use our local hook directory:
```bash
just install-hooks
```

---

## IV. Bypassing (Emergencies Only)
If a push must be made without verification (e.g., a documentation-only fix that is failing a transient test), use the standard Git flag:
```bash
git push --no-verify
```
*Note: The LMS-CI pipeline will still enforce all gates on the remote side.*

---

### II. Implementing the Hook Script

**Location**: `.githooks/pre-push` (Create this directory and file)

```bash
#!/bin/bash
# Bistun LMS Pre-push hook
# Copyright (C) 2026 Francis Xavier Wazeter IV
# Ref: [LMS-PROCESS-HOOKS]

set -e

echo "--------------------------------------------------"
echo "🚀 [LMS-GATE] Running Local Quality Gate..."
echo "--------------------------------------------------"

# Execute the complete quality chain defined in the justfile
if ! just verify-all; then
    echo ""
    echo "❌ [ERROR]: Local Quality Gate failed."
    echo "Please fix formatting, tests, or documentation warnings before pushing."
    echo "Refer to docs/processes/LMS-PROCESS-ERROR.md for remediation steps."
    echo "--------------------------------------------------"
    exit 1
fi

echo ""
echo "✅ [SUCCESS]: Local Quality Gate passed. Proceeding with push."
echo "--------------------------------------------------"
exit 0
```
