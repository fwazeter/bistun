// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! # Atomic Reference Swap Engine (Optimized)
//! Ref: [010-LMS-MEM]
//! Location: `src/data/swap.rs`
//!
//! **Why**: This module provides wait-free, lock-free access to the Flyweight registry. It allows background threads to atomically swap in new definitions without ever blocking active HTTP readers.
//! **Impact**: Guarantees that concurrent threads never contend for locks, keeping the p99 latency budget strictly < 1ms under extreme load.
//!
//! ### Glossary
//! * **ArcSwap**: A wait-free atomic pointer wrapper optimized for read-heavy workloads.
//! * **Hot-Swap**: The process of replacing the entire underlying data store via a single atomic pointer update.

use crate::data::store::{LocaleProfile, RegistryStore};
use arc_swap::ArcSwap;
use std::sync::Arc;

/// Interface for reading from the active linguistic registry.
/// Enables Dependency Inversion for hermetic testing across the capability engine.
pub trait IRegistryState: Send + Sync {
    /// Safely and wait-freely fetches a locale profile for a reader.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Perform a wait-free load of the atomic pointer to the active store.
    /// 2. Delegate the query to the underlying `RegistryStore`.
    /// 3. Return the `Arc<LocaleProfile>` if found.
    ///
    /// # Arguments
    /// * `id` (&str): The canonical BCP 47 locale ID to retrieve.
    ///
    /// # Returns
    /// * `Option<Arc<LocaleProfile>>`: An atomic reference to the immutable profile, or `None` if absent.
    ///
    /// # Golden I/O
    /// * **Input**: `"ar-EG"`
    /// * **Output**: `Some(Arc<LocaleProfile { ... }>)`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None. Wait-free reads cannot panic.
    /// * **Safety**: Safe concurrent execution.
    fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>>;
}

/// Manages thread-safe, wait-free access to the active linguistic registry.
///
/// Time: O(1) pointer load | Space: O(1)
#[derive(Debug, Clone)]
pub struct RegistryState {
    /// The active registry, protected by a wait-free atomic pointer.
    /// Wrapped in an Arc so `RegistryState` can be cheaply cloned across HTTP workers
    /// while pointing to the exact same atomic variable.
    active_store: Arc<ArcSwap<RegistryStore>>,
}

impl Default for RegistryState {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryState {
    /// Initializes a new, empty RegistryState.
    ///
    /// Time: O(1) | Space: O(1) map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default, empty `RegistryStore`.
    /// 2. Wraps the raw store in an `ArcSwap` via `from_pointee` to initialize the atomic pointer.
    /// 3. Wraps the `ArcSwap` in an `Arc` to allow cheap struct cloning across web workers.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::data::swap::RegistryState;
    /// let state = RegistryState::new();
    /// ```
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `Self`: A prepared wait-free registry state.
    ///
    /// # Golden I/O
    /// * **Input**: `()`
    /// * **Output**: `RegistryState { active_store: ... }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Fully safe synchronous initialization.
    pub fn new() -> Self {
        // [STEP 1]: Instantiate default store. (No Arc yet)
        let store = RegistryStore::new();
        // [STEP 2 & 3]: from_pointee accepts `T` and creates `ArcSwap<T>`
        Self { active_store: Arc::new(ArcSwap::from_pointee(store)) }
    }

    /// Atomically hot-swaps the current registry with a newly hydrated one.
    ///
    /// Time: O(1) pointer swap | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Perform an atomic pointer swap to the newly provided `RegistryStore`.
    /// 2. Active readers safely finish using the old memory allocation until they drop it.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::data::swap::RegistryState;
    /// use bistun::data::store::RegistryStore;
    /// let state = RegistryState::new();
    /// state.swap_registry(RegistryStore::new());
    /// ```
    ///
    /// # Arguments
    /// * `new_store` (RegistryStore): The newly hydrated data store to hot-swap into active memory.
    ///
    /// # Returns
    /// * `()`: Side-effect function.
    ///
    /// # Golden I/O
    /// * **Input**: `RegistryStore { ... }`
    /// * **Output**: `()`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None. Wait-free pointer swap.
    /// * **Safety**: Safe concurrent execution.
    pub fn swap_registry(&self, new_store: RegistryStore) {
        // [STEP 1 & 2]: Atomic pointer swap. Lock-free and wait-free.
        self.active_store.store(Arc::new(new_store));
    }
}

impl IRegistryState for RegistryState {
    fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>> {
        // [STEP 1]: Load the pointer (Wait-free, no locks acquired).
        // `load()` returns a `Guard` which implements `Deref`.
        let store = self.active_store.load();

        // [STEP 2]: Query the store.
        store.get_profile(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::traits::{Direction, MorphType, SegType};

    /// Helper to generate a mock profile and resolve linter code duplication warnings.
    fn create_mock_profile() -> LocaleProfile {
        LocaleProfile {
            id: "ar-EG".to_string(),
            morph: MorphType::TEMPLATIC,
            base_seg: SegType::SPACE,
            alt_seg: None,
            direction: Direction::RTL,
            has_bidi: true,
            requires_shaping: true,
            plurals: vec!["other".to_string()],
            required_resource: None,
        }
    }

    #[test]
    fn test_registry_wait_free_hot_swap() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create state and verify it's empty.
        let state = RegistryState::new();
        assert!(state.get_profile("ar-EG").is_none());

        // [STEP 2]: Execute: Build a new store offline, insert a stub, and swap it.
        let mut new_store = RegistryStore::new();
        new_store.insert_stub(create_mock_profile());

        // Verify cloning the state respects the atomic pointer (Simulating web workers)
        let worker_state = state.clone();

        // The Wait-Free Hot-Swap
        state.swap_registry(new_store);

        // [STEP 3]: Assert: Verify the new data is instantly accessible across cloned states.
        let profile = worker_state.get_profile("ar-EG").expect("Swap failed on cloned state");
        assert_eq!(profile.direction, Direction::RTL);
    }
}
