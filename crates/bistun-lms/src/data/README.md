# MODULE-README: Memory Management & Persistence Domain

![Blueprint](https://img.shields.io/badge/Blueprint-010--LMS--MEM-blue)
![Blueprint](https://img.shields.io/badge/Blueprint-002--LMS--DATA-blue)
![Domain](https://img.shields.io/badge/Domain-Memory%20|%20Persistence-green)
![Location](https://img.shields.io/badge/Location-src%2Fdata-lightgrey)
![Status](https://img.shields.io/badge/Status-Standard-yellow)

---

## I. Strategic Overview

### 1. The "Why"
This module manages the in-memory cache, state transitions, and WORM hydration for all linguistic data. It provides the infrastructure required to support 7,000+ languages by decoupling raw persistence from the high-speed capability engine.

### 2. System Impact
If this module fails, the application will either serve stale linguistic data, panic due to lock poisoning, or exhaust host memory via redundant allocations. It is the foundation for the **>80% memory reduction** target.

### 3. Design Patterns
* **Flyweight Pattern**: Reuses immutable instances of `LocaleProfile` to eliminate redundant heap allocations across thousands of concurrent requests.
* **Atomic Reference Swap**: Facilitates zero-downtime registry updates by swapping the underlying memory pointer in a background thread.
* **WORM (Write-Once, Read-Many)**: Treats registry snapshots as immutable artifacts, ensuring a perfect audit trail and preventing in-place data corruption.

### 4. Local Glossary
* **Atomic Hot-Swap**: The process of replacing the entire active registry in memory without blocking active requests.
* **Hydration**: The process of inflating static WORM snapshots into operational Flyweight memory structures.
* **RwLock**: A synchronization primitive allowing multiple concurrent readers or exactly one writer to access the registry state.

---

## II. Technical Interface

### 1. Primary Capability
| Function/Method      | Input                    | Output                            | Purpose                                               |
|:---------------------|:-------------------------|:----------------------------------|:------------------------------------------------------|
| `swap_registry()`    | `RegistryStore`          | `()`                              | Atomically updates the active registry pointer.       |
| `get_profile()`      | `&str` (ID)              | `Option<Arc<LocaleProfile>>`      | Retrieves a shared reference from the Flyweight pool. |
| `hydrate_snapshot()` | `&dyn ISnapshotProvider` | `Result<RegistryStore, LmsError>` | Inflates raw data into an optimized memory store.     |

### 2. Side Effects & SLIs
* **Telemetry**: Records `registry_version` and memory pool utilization metrics to monitor the health of hot-swaps.
* **Performance**: Read-access to Flyweights is **O(1)**. Hot-swaps are **O(1)** (pointer exchange) once hydration is complete.
* **Dependencies**: Relies on `hashbrown` for high-performance indexing and `serde` for snapshot deserialization.

### 3. Quirks & Invariants
* **Arc Pointer Math**: Multiple calls to `get_profile` for the same ID return the same memory address, verified via pointer equality.
* **Invariant**: A `RegistryStore` must be fully hydrated and validated before it can be used in an `Atomic Reference Swap`.

---

## III. Usage & Implementation

### 1. Basic Instantiation
```rust
use crate::data::swap::RegistryState;
use crate::data::store::RegistryStore;

fn main() {
    // Initialize the thread-safe state container
    let state = RegistryState::new();
    
    // Verify initial empty state to satisfy the compiler
    let profile = state.get_profile("en-US");
    assert!(profile.is_none());
}
```

### 2. The "Golden Path" Example
```rust
use crate::data::repository::hydrate_snapshot;
use crate::data::providers::FileSnapshotProvider;
use crate::data::swap::RegistryState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = RegistryState::new();
    
    // 1. Setup a provider for a local WORM snapshot
    let provider = FileSnapshotProvider::new(
        "registry_v1.json".into(), 
        "registry_v1.sig".into()
    );

    // 2. Hydrate the snapshot in a background task
    let new_store = hydrate_snapshot(&provider)?;
    
    // 3. Perform the Atomic Hot-Swap
    state.swap_registry(new_store);
    
    // 4. Component is now serving new data instantly
    assert!(state.get_profile("en-US").is_some());
    
    Ok(())
}
```

---

## IV. Development & Extension Guide

### 1. How to Build it Up
To extend the persistence layer (e.g., adding an `S3SnapshotProvider`):
1.  **Red Phase**: Create a failing test in `providers.rs` attempting to fetch from the new source.
2.  **Implementation**: Implement the `ISnapshotProvider` trait for the new struct.
3.  **Trace**: Ensure the `fetch_payload` logic contains `// [STEP X]` comments mapping to the # Logic Trace.
4.  **Verification**: Run `just verify-all` to ensure the hydration overhead does not impact the system's background sync budgets.

### 2. Troubleshooting & Common Failures
* **RwLock Poisoning**: Occurs if a thread panics while holding the writer lock during a swap. The system must fail-safe and log an `ERROR` via telemetry.
* **OOM (Out-of-Memory)**: Caused by failing to clear the old registry after a swap. Ensure no "Leaked Arcs" are held by long-running capability requests.

---

## V. Metadata
* **Author**: Francis Xavier Wazeter IV
* **Version**: 0.9.7
* **File Created**: 2026-05-02
* **Last Updated**: 2026-05-02
* **License**: GNU GPL v3

---

### Architect's Note for AI Agents
When analyzing this module, prioritize **Memory Safety** and **Flyweight Integrity**. All profile lookups must utilize the `Arc` pointer exchange to maintain zero-allocation performance. Do not suggest implementations that introduce mutable global state or bypass the `RegistryState` lock-free reader path.
