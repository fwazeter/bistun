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

//! # Flyweight Definition Pools
//! Ref: [010-LMS-MEM]
//!
//! **Why**: This module implements an in-memory Flyweight cache to hold linguistic profiles. It guarantees that thousands of concurrent requests can read locale data without duplicating heap allocations.
//! **Impact**: If this module is compromised, the pipeline will either fail to find necessary rendering traits or crash the server via Out-Of-Memory (OOM) heap exhaustion.
//!
//! ### Glossary
//! * **Flyweight Pattern**: A memory optimization where a single immutable instance of an object is shared among multiple contexts.
//! * **Arc (Atomic Reference Count)**: A thread-safe pointer that allows multiple readers to own a reference to the same data without copying it.

use crate::models::traits::{Direction, MorphType, SegType};
use hashbrown::HashMap;
use std::sync::Arc;

/// An aggregated representation of a locale's Typological and Orthographic rules.
/// Kept immutable to allow safe, lock-free reading across threads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleProfile {
    pub id: String,
    pub morph: MorphType,
    pub base_seg: SegType,
    pub alt_seg: Option<SegType>,
    pub direction: Direction,
    pub has_bidi: bool,
    pub requires_shaping: bool,
}

/// The high-performance, in-memory data store for linguistic profiles.
///
/// Time: O(1) reads | Space: O(N) where N is the number of distinct locales
#[derive(Debug, Default)]
pub struct RegistryStore {
    /// Maps a canonical Locale ID (e.g., "ar-EG") to its Flyweight profile.
    pools: HashMap<String, Arc<LocaleProfile>>,
}

impl RegistryStore {
    /// Initializes a new, empty RegistryStore.
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiate the high-throughput `hashbrown` HashMap.
    /// 2. Return the empty store structure.
    pub fn new() -> Self {
        Self { pools: HashMap::new() }
    }

    /// Retrieves an atomic reference to a `LocaleProfile` if it exists.
    ///
    /// Time: O(1) | Space: O(1) (Only pointer allocation)
    ///
    /// # Logic Trace (Internal)
    /// 1. Query the internal `pools` map using the provided locale `id`.
    /// 2. If found, clone the `Arc` (bumping the reference count) and return it.
    /// 3. If missing, return `None`.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::data::store::{RegistryStore, LocaleProfile};
    ///
    /// let store = RegistryStore::new();
    /// assert!(store.get_profile("ar-EG").is_none());
    /// ```
    pub fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>> {
        self.pools.get(id).cloned()
    }

    /// [\STUB\]: Inserts a profile directly into the pool.
    /// In Phase 8, this will be replaced by atomic pointer swaps from the `repository.rs` WORM loader.
    ///
    /// Time: O(1) amortized | Space: O(1) reference insertion
    pub fn insert_stub(&mut self, profile: LocaleProfile) {
        self.pools.insert(profile.id.clone(), Arc::new(profile));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_store_with_stub() -> RegistryStore {
        let mut store = RegistryStore::new();
        store.insert_stub(LocaleProfile {
            id: "th-TH".to_string(),
            morph: MorphType::ISOLATING,
            base_seg: SegType::DICTIONARY,
            alt_seg: None,
            direction: Direction::LTR,
            has_bidi: false,
            requires_shaping: true,
        });
        store
    }

    #[test]
    fn test_store_retrieves_flyweight_reference() {
        // [Logic Trace Mapping]
        // 1. Setup: Initialize store with "th-TH" stub.
        // 2. Execute: Retrieve the profile twice.
        // 3. Assert: Verify both retrievals point to the same underlying data via Arc pointer math.
        let store = setup_store_with_stub();

        let req1 = store.get_profile("th-TH").expect("Profile should exist");
        let req2 = store.get_profile("th-TH").expect("Profile should exist");

        assert_eq!(req1.id, "th-TH");
        assert_eq!(req1.morph, MorphType::ISOLATING);

        // Ensure they point to the exact same memory allocation (Flyweight pattern)
        assert!(Arc::ptr_eq(&req1, &req2));
    }

    #[test]
    fn test_store_returns_none_for_missing_profile() {
        // [Logic Trace Mapping]
        // 1. Setup: Initialize store.
        // 2. Execute: Request an unknown locale.
        // 3. Assert: Verify O(1) fallback to None without panicking.
        let store = setup_store_with_stub();
        assert!(store.get_profile("xx-YY").is_none());
    }
}
