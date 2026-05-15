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
//! Crate: `bistun-lms`
//! Ref: [010-LMS-MEM]
//! Location: `crates/bistun-lms/src/data/swap.rs`
//!
//! **Why**: This module provides wait-free, lock-free access to the Flyweight registry. It allows background threads to atomically swap in new definitions without ever blocking active `HTTP` readers.
//! **Impact**: If this module fails, the system cannot safely handle concurrent updates to the memory pool, resulting in data races, stale resolution, or service panics under high contention.
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
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Logic Trace (Internal)
    /// 1. Perform a wait-free load of the atomic pointer to the active store.
    /// 2. Delegate the query to the underlying [`RegistryStore`].
    /// 3. Return the `Arc<LocaleProfile>` if found.
    ///
    /// # Arguments
    /// * `id` (&str): The canonical `BCP 47` locale ID to retrieve.
    ///
    /// # Returns
    /// * `Option<Arc<LocaleProfile>>`: An atomic reference to the immutable profile, or `None` if absent.
    fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>>;

    /// Safely and wait-freely resolves a dynamic alias mapping.
    ///
    /// Time: `O(1)` | Space: `O(1)` string clone
    ///
    /// # Logic Trace (Internal)
    /// 1. Perform a wait-free load of the atomic pointer to the active store.
    /// 2. Delegate the query to the underlying [`RegistryStore`].
    /// 3. Return the mapped canonical `ID` string if found.
    ///
    /// # Arguments
    /// * `tag` (&str): The deprecated or macrolanguage `BCP 47` tag.
    ///
    /// # Returns
    /// * `Option<String>`: The mapped canonical locale string if an alias exists.
    fn resolve_alias(&self, tag: &str) -> Option<String>;

    /// Returns the semantic version of the active data set.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    fn get_version(&self) -> String;

    /// Safely and wait-freely fetches the configured base resource URI.
    ///
    /// Time: `O(1)` | Space: `O(1)` string clone
    ///
    /// # Logic Trace (Internal)
    /// 1. Perform a wait-free load of the atomic pointer to the active store.
    /// 2. Access and clone the configured `base_resource_uri`.
    ///
    /// # Returns
    /// * `String`: The fully qualified base `URI` (e.g., `https://cdn.example.com/v1/icu/`).
    fn get_base_resource_uri(&self) -> String;
}

/// Manages thread-safe, wait-free access to the active linguistic registry.
///
/// Time: `O(1)` pointer load | Space: `O(1)`
#[derive(Debug, Clone)]
pub struct RegistryState {
    /// The active registry, protected by a wait-free atomic pointer.
    active_store: Arc<ArcSwap<RegistryStore>>,
}

impl Default for RegistryState {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryState {
    /// Initializes a new, empty [`RegistryState`].
    ///
    /// Time: `O(1)` | Space: `O(1)` map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default, empty [`RegistryStore`].
    /// 2. Wraps the raw store in an [`ArcSwap`] via `from_pointee` to initialize the atomic pointer.
    /// 3. Wraps the [`ArcSwap`] in an `Arc` to allow cheap struct cloning across web workers.
    ///
    /// # Examples
    /// ```rust
    /// use bistun_lms::data::swap::RegistryState;
    /// let state = RegistryState::new();
    /// ```
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
    #[must_use]
    pub fn new() -> Self {
        // [STEP 1]: Instantiate default store. (No Arc yet)
        let store = RegistryStore::new();
        // [STEP 2 & 3]: from_pointee accepts `T` and creates `ArcSwap<T>`
        Self { active_store: Arc::new(ArcSwap::from_pointee(store)) }
    }

    /// Atomically hot-swaps the current registry with a newly hydrated one.
    ///
    /// Time: `O(1)` pointer swap | Space: `O(1)`
    ///
    /// # Logic Trace (Internal)
    /// 1. Perform an atomic pointer swap to the newly provided [`RegistryStore`].
    /// 2. Active readers safely finish using the old memory allocation until they drop it.
    ///
    /// # Examples
    /// ```rust
    /// use bistun_lms::data::swap::RegistryState;
    /// use bistun_lms::data::store::RegistryStore;
    /// let state = RegistryState::new();
    /// state.swap_registry(RegistryStore::new());
    /// ```
    ///
    /// # Arguments
    /// * `new_store` ([`RegistryStore`]): The newly hydrated data store to hot-swap into active memory.
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

    fn resolve_alias(&self, tag: &str) -> Option<String> {
        // [STEP 1]: Load the pointer (Wait-free, no locks acquired).
        let store = self.active_store.load();
        // [STEP 2]: Query the store.
        store.resolve_alias(tag)
    }

    fn get_version(&self) -> String {
        // Wait-free read of the metadata header
        self.active_store.load().metadata.version.clone()
    }

    fn get_base_resource_uri(&self) -> String {
        // Wait-free read of the configured base CDN URI
        self.active_store.load().base_resource_uri.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bistun_core::manifest::TraitValue;
    use bistun_core::traits::{Direction, MorphType, SegType, TraitKey};
    use hashbrown::HashMap;

    /// Helper to generate a mock profile for V2.0.0 architectural alignment.
    fn create_mock_profile() -> LocaleProfile {
        let mut traits = HashMap::new();
        traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::TEMPLATIC));
        traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
        traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));

        LocaleProfile {
            id: "ar-EG".to_string(),
            traits,
            rules: HashMap::new(),
            resources: HashMap::new(),
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
        new_store.insert_alias("in".to_string(), "id".to_string());
        new_store.set_base_resource_uri("https://test.cdn.bistun.io/".to_string());

        // Verify cloning the state respects the atomic pointer (Simulating web workers)
        let worker_state = state.clone();

        // The Wait-Free Hot-Swap
        state.swap_registry(new_store);

        // [STEP 3]: Assert: Verify the new data is instantly accessible across cloned states.
        let profile =
            worker_state.get_profile("ar-EG").expect("LMS-TEST: Swap failed on cloned state");

        // Assert DNA Traits (V2.0.0 structure)
        let direction =
            profile.traits.get(&TraitKey::PrimaryDirection).expect("LMS-TEST: Missing trait");
        assert_eq!(*direction, TraitValue::Direction(Direction::RTL));

        assert_eq!(worker_state.resolve_alias("in"), Some("id".to_string()));
        assert_eq!(worker_state.get_base_resource_uri(), "https://test.cdn.bistun.io/");
    }
}
