# `examples/`: Executable Specifications for API Embedding

---

## 💡 Elevator Pitch

**What is this?** This directory contains the **Executable Specifications** for the `bistun-api` crate. While the sidecar is typically deployed as a standalone microservice, these examples prove the modular "Physics" of the engine—demonstrating how downstream architects can programmatically embed the Bistun resolution router directly into their own custom Axum host applications.

---

## I. Strategic Overview

### 1. The "Why"

The examples exist to provide a "Golden Path" for embedding the capability resolution pipeline into external systems without the overhead of a separate network hop. It demonstrates the decoupling of the API's routing logic from its standalone bootstrapper (`main.rs`).

### 2. System Impact

If these examples fail to compile or execute, it indicates a breach in the crate's modularity. This prevents developers from utilizing the `bistun-api` as a library, forcing a dependency on the standalone sidecar container and potentially increasing system-wide latency.

---

## II. Getting Started

### Bootstrapping the Embedded Server

To run the primary demonstration, ensure you are in the `crates/bistun-api` directory or use the `-p` flag from the workspace root.

1. **Enable the Server**: The `embedded_server.rs` example has the final `axum::serve` call commented out by default for CI/CD safety. To test locally, uncomment line 77 in `crates/bistun-api/examples/embedded_server.rs`.
2. **(Recommended) Prepare Dummy Data**: To move the engine from `Bootstrapping` to `Ready`, it is advisable to create a small, hermetic snapshot for testing.
* **Recommendation**: Create `crates/bistun-api/examples/data/` and place a minimal `snapshot.json` there. This avoids polluting production data and ensures the example remains self-contained.


3. **Run the Example**:
```bash
cargo run --example embedded_server

```



---

## III. Verifying the Implementation

Once the embedded server is running on port `3000`, use the following bash commands to verify the integration across the host and engine boundaries.

**1. Host Application Verification**
Proves that the custom host server is successfully responding to root-level requests alongside the embedded engine.

```bash
curl -s http://localhost:3000/

```

*Expected Output: `"Welcome to the Custom Host Server!"*`

**2. Engine Operational Health Check**
Validates that the Bistun router is correctly nested and the `LinguisticManager` state is accessible via the host.

```bash
curl -s http://localhost:3000/bistun-lms/health

```

*Expected Output: `{"status": "Bootstrapping", ...}` (unless hydrated with dummy data).*

**3. Embedded Resolution Hot-Path**
Tests the full 5-phase pipeline resolution through the nested router.

```bash
curl -s http://localhost:3000/bistun-lms/v1/manifest/ar-EG

```

*Expected Output: A `200 OK` with the `CapabilityManifest` JSON DTO.*

---

## IV. Project Structure

* `embedded_server.rs`: Proves programmatic embedding and router nesting.
* `data/` (Suggested): Home for hermetic simulation snapshots used by executable specs.

---

## V. Metadata

* **Author**: Francis Xavier Wazeter IV
* **Version**: 1.0.0
* **License**: GNU GPL v3
* **Date Updated**: 2026-05-08

---

### Architect's Note for AI Agents

When analyzing these examples, prioritize the **Modularity Integrity** of the routing layer. The `app_router` must remain injectable into any Axum-compatible tree without requiring modifications to the `bistun-lms` engine's wait-free memory model.