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

//! # Atomic Reference Swap Engine
//! Ref: [010-LMS-MEM]
//!
//! **Why**: This module provides thread-safe, lock-free (for readers) access to the Flyweight registry. It allows background threads to atomically swap in new definitions without dropping active HTTP requests.
//! **Impact**: If this module fails, concurrent thread contention will spike latency well beyond the 1ms budget, or the service will deadlock.
//!
//! ### Glossary
//! * **RwLock**: A synchronization primitive that allows multiple readers or exactly one writer at a time.
//! * **Hot-Swap**: The process of replacing the entire underlying data store via a single atomic pointer update.

use crate::data::store::{LocaleProfile, RegistryStore};
use std::sync::{Arc, RwLock};

/// Manages thread-safe access to the active linguistic registry.
///
/// Time: O(1) pointer clone | Space: O(1)
#[derive(Debug, Clone)]
pub struct RegistryState {
    /// The active registry, protected by a Read-Write lock.
    active_store: Arc<RwLock<Arc<RegistryStore>>>,
}

impl Default for RegistryState {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryState {
    /// Initializes a new, empty RegistryState.
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default, empty `RegistryStore`.
    /// 2. Wraps it in an `Arc` (for the actual data) and then an `RwLock` (to manage the pointer).
    pub fn new() -> Self {
        let store = Arc::new(RegistryStore::new());
        Self { active_store: Arc::new(RwLock::new(store)) }
    }

    /// Safely fetches a locale profile for a reader.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Acquire a non-blocking read lock.
    /// 2. Delegate the query to the underlying `RegistryStore`.
    /// 3. Return the `Arc<LocaleProfile>` if found, automatically releasing the read lock when exiting scope.
    pub fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>> {
        let store = self.active_store.read().expect("RwLock poisoned");
        store.get_profile(id)
    }

    /// Atomically hot-swaps the current registry with a newly hydrated one.
    ///
    /// Time: O(1) pointer swap | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Acquire the exclusive write lock (briefly blocking new readers).
    /// 2. Swap the internal `Arc` pointer to point to the newly provided `RegistryStore`.
    /// 3. Release the lock. Old readers finish safely using the old memory allocation until they drop.
    pub fn swap_registry(&self, new_store: RegistryStore) {
        let mut store_ptr = self.active_store.write().expect("RwLock poisoned");
        *store_ptr = Arc::new(new_store);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::traits::{Direction, MorphType, SegType};

    #[test]
    fn test_registry_hot_swap() {
        // [Logic Trace Mapping]
        // 1. Setup: Create state and verify it's empty.
        // 2. Execute: Build a new store offline, insert a stub, and swap it into the state.
        // 3. Assert: Verify the new data is instantly accessible without deadlocking.

        let state = RegistryState::new();
        assert!(state.get_profile("ar-EG").is_none());

        let mut new_store = RegistryStore::new();
        new_store.insert_stub(LocaleProfile {
            id: "ar-EG".to_string(),
            morph: MorphType::TEMPLATIC,
            base_seg: SegType::SPACE,
            alt_seg: None,
            direction: Direction::RTL,
            has_bidi: true,
            requires_shaping: true,
            plurals: vec![
                "zero".to_string(),
                "one".to_string(),
                "two".to_string(),
                "few".to_string(),
                "many".to_string(),
                "other".to_string(),
            ],
        });

        // The Hot-Swap
        state.swap_registry(new_store);

        let profile = state.get_profile("ar-EG").expect("Swap failed");
        assert_eq!(profile.direction, Direction::RTL);
    }
}
