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
//! Location: `src/data/store.rs`
//!
//! **Why**: This module implements an in-memory Flyweight cache to hold linguistic profiles. It guarantees that thousands of concurrent requests can read locale data without duplicating heap allocations.
//! **Impact**: If this module is compromised, the pipeline will either fail to find necessary rendering traits or crash the server via Out-Of-Memory (OOM) heap exhaustion.
//!
//! ### Glossary
//! * **Flyweight Pattern**: A memory optimization where a single immutable instance of an object is shared among multiple contexts.
//! * **Arc (Atomic Reference Count)**: A thread-safe pointer that allows multiple readers to own a reference to the same data without copying it.

use crate::models::traits::{Direction, MorphType, SegType};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// An aggregated representation of a locale's Typological and Orthographic rules.
/// Kept immutable to allow safe, lock-free reading across threads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleProfile {
    pub id: String,
    pub morph: MorphType,
    pub base_seg: SegType,
    pub alt_seg: Option<SegType>,
    pub direction: Direction,
    pub has_bidi: bool,
    pub requires_shaping: bool,
    pub plurals: Vec<String>,
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
    /// Time: O(1) | Space: O(1) map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiate the high-throughput `hashbrown` HashMap.
    /// 2. Return the empty store structure.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::data::store::RegistryStore;
    /// let store = RegistryStore::new();
    /// ```
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `Self`: A newly instantiated, empty `RegistryStore`.
    ///
    /// # Golden I/O
    /// * **Input**: `()`
    /// * **Output**: `RegistryStore { pools: {} }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Fully safe synchronous initialization.
    pub fn new() -> Self {
        // [STEP 1]: Instantiate the HashMap.
        // [STEP 2]: Return the empty structure.
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
    /// use bistun::data::store::RegistryStore;
    /// let store = RegistryStore::new();
    /// assert!(store.get_profile("ar-EG").is_none());
    /// ```
    ///
    /// # Arguments
    /// * `id` (&str): The canonical BCP 47 locale ID to retrieve.
    ///
    /// # Returns
    /// * `Option<Arc<LocaleProfile>>`: An atomic reference to the immutable profile, or `None` if absent.
    ///
    /// # Golden I/O
    /// * **Input**: `"ar-EG"`
    /// * **Output**: `None` (if empty) or `Some(Arc<LocaleProfile { ... }>)`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous read access.
    pub fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>> {
        // [STEP 1]: Query the internal map.
        // [STEP 2] & [STEP 3]: Clone the Arc if found, else return None.
        self.pools.get(id).cloned()
    }

    /// Inserts a profile directly into the pool.
    ///
    /// Note: In Phase 8, this will be utilized by `repository.rs` during WORM hydration.
    ///
    /// Time: O(1) amortized | Space: O(1) reference insertion
    ///
    /// # Logic Trace (Internal)
    /// 1. Wrap the owned `LocaleProfile` in an `Arc` to establish the Flyweight pattern.
    /// 2. Insert the `Arc` into the `pools` map, keyed by the profile's ID.
    ///
    /// # Arguments
    /// * `profile` (LocaleProfile): The fully hydrated linguistic profile.
    ///
    /// # Returns
    /// * `()`: Side-effect function.
    ///
    /// # Golden I/O
    /// * **Input**: `LocaleProfile { id: "th-TH", ... }`
    /// * **Output**: `()`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous write access.
    pub fn insert_stub(&mut self, profile: LocaleProfile) {
        // [STEP 1] & [STEP 2]: Wrap in Arc and insert.
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
            direction: Direction::RTL,
            has_bidi: false,
            requires_shaping: true,
            plurals: vec!["other".to_string()],
        });
        store
    }

    #[test]
    fn test_store_retrieves_flyweight_reference() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Initialize store with "th-TH" stub.
        let store = setup_store_with_stub();

        // [STEP 2]: Execute: Retrieve the profile twice.
        let req1 = store.get_profile("th-TH").expect("Profile should exist");
        let req2 = store.get_profile("th-TH").expect("Profile should exist");

        // [STEP 3]: Assert: Verify both retrievals point to the same underlying data via Arc pointer math.
        assert_eq!(req1.id, "th-TH");
        assert_eq!(req1.morph, MorphType::ISOLATING);

        // Ensure they point to the exact same memory allocation (Flyweight pattern)
        assert!(Arc::ptr_eq(&req1, &req2));
    }

    #[test]
    fn test_store_returns_none_for_missing_profile() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Initialize store.
        let store = setup_store_with_stub();

        // [STEP 2]: Execute: Request an unknown locale.
        // [STEP 3]: Assert: Verify O(1) fallback to None without panicking.
        assert!(store.get_profile("xx-YY").is_none());
    }
}
