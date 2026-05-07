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

use crate::models::traits::{Direction, MorphType, NormType, SegType, TransType};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Authoritative identity and audit metadata for a specific registry build.
///
/// Time: O(1) | Space: O(1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegistryMetadata {
    /// Semantic version of the data (e.g., "v1.2.3").
    pub version: String,
    /// ISO 8601 timestamp of when the curator compiled this snapshot.
    pub build_date: String,
    /// SHA-256 hash or similar unique identifier for data integrity.
    pub checksum: String,
}

impl Default for RegistryMetadata {
    /// Provides a safe "Phase 0" default identity for bootstrapping.
    ///
    /// Time: O(1) | Space: O(1)
    fn default() -> Self {
        Self {
            version: "v0.0.0-default".to_string(),
            build_date: "1970-01-01T00:00:00Z".to_string(),
            checksum: "00000000000000000000000000000000".to_string(),
        }
    }
}

/// An aggregated representation of a locale's Typological and Orthographic rules.
/// Kept immutable to allow safe, lock-free reading across threads.
/// `required_resource` points to the heavy binary asset required by a given script (e.g., "icu_arab")
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
    pub unicode_blocks: Vec<String>,
    pub normalization: NormType,
    pub transliteration: TransType,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub required_resource: Option<String>,
}

/// The high-performance, in-memory data store for linguistic profiles and dynamic routing mappings.
///
/// Time: O(1) reads | Space: O(N) where N is the number of distinct locales + aliases
#[derive(Debug, Default)]
pub struct RegistryStore {
    /// Authoritative identity of this specific data set.
    pub metadata: Arc<RegistryMetadata>,
    /// Environment-specific base URI for resolving binary data blobs (e.g. ICU4X data).
    pub base_resource_uri: Arc<String>,
    /// Maps a canonical Locale ID (e.g., "ar-EG") to its Flyweight profile.
    pools: HashMap<String, Arc<LocaleProfile>>,
    /// Maps a deprecated or macrolanguage tag (e.g., "in") to its canonical ID (e.g., "id").
    aliases: HashMap<String, String>,
}

impl RegistryStore {
    /// Initializes a new, empty RegistryStore.
    ///
    /// Time: O(1) | Space: O(1) map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiate the high-throughput `hashbrown` HashMaps for pools and aliases.
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
        // [STEP 1]: Instantiate the HashMaps & default metadata.
        // [STEP 2]: Return the empty structure.
        Self {
            metadata: Arc::new(RegistryMetadata::default()),
            // Default to a safe localhost URI until configured by the Sidecar
            base_resource_uri: Arc::new("http://localhost:8080/assets/".to_string()),
            pools: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    /// Sets the authoritative metadata for this store.
    ///
    /// Time: O(1) | Space: O(1) pointer swap
    ///
    /// # Logic Trace (Internal)
    /// 1. Wrap the raw metadata struct in a thread-safe `Arc`.
    /// 2. Update the internal metadata pointer.
    ///
    /// # Arguments
    /// * `metadata` (RegistryMetadata): The new identity header extracted during hydration.
    pub fn set_metadata(&mut self, metadata: RegistryMetadata) {
        self.metadata = Arc::new(metadata);
    }

    /// Sets the environment-specific base URI for resource resolution.
    ///
    /// Time: O(1) | Space: O(1) pointer swap
    ///
    /// # Arguments
    /// * `uri` (String): The fully qualified base URI (e.g., `https://cdn.example.com/v1/icu/`).
    pub fn set_base_resource_uri(&mut self, uri: String) {
        self.base_resource_uri = Arc::new(uri);
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

    /// Retrieves the canonical target for a known alias or macrolanguage tag.
    ///
    /// Time: O(1) | Space: O(1) string clone
    ///
    /// # Logic Trace (Internal)
    /// 1. Query the internal `aliases` map using the provided tag.
    /// 2. If an alias is found, clone the canonical string and return it.
    /// 3. If missing, return `None`.
    pub fn resolve_alias(&self, tag: &str) -> Option<String> {
        self.aliases.get(tag).cloned()
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

    /// Inserts an alias mapping directly into the dynamic routing table.
    ///
    /// Note: Utilized by `repository.rs` during WORM hydration.
    pub fn insert_alias(&mut self, alias: String, canonical: String) {
        self.aliases.insert(alias, canonical);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_store_with_stub() -> RegistryStore {
        let mut store = RegistryStore::new();
        store.set_base_resource_uri("https://dev.cdn.bistun.io/assets/".to_string());
        store.insert_stub(LocaleProfile {
            id: "th-TH".to_string(),
            morph: MorphType::ISOLATING,
            base_seg: SegType::DICTIONARY,
            alt_seg: None,
            direction: Direction::RTL,
            has_bidi: false,
            requires_shaping: true,
            plurals: vec!["other".to_string()],
            unicode_blocks: vec![],
            normalization: NormType::NFC,
            transliteration: TransType::NONE,
            required_resource: Some("tri_thai".to_string()),
        });
        store.insert_alias("in".to_string(), "id".to_string());
        store
    }

    #[test]
    fn test_store_retrieves_flyweight_reference() {
        let store = setup_store_with_stub();
        let req1 = store.get_profile("th-TH").expect("Profile should exist");
        let req2 = store.get_profile("th-TH").expect("Profile should exist");

        // Ensure they point to the exact same memory allocation (Flyweight pattern)
        assert!(Arc::ptr_eq(&req1, &req2));
    }

    #[test]
    fn test_store_returns_none_for_missing_profile() {
        let store = setup_store_with_stub();
        assert!(store.get_profile("xx-YY").is_none());
    }

    #[test]
    fn test_store_resolves_dynamic_alias() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Initialize store with the "in" -> "id" alias stub.
        let store = setup_store_with_stub();

        // [STEP 2]: Execute & Assert: Retrieve the dynamic alias mapping.
        assert_eq!(store.resolve_alias("in"), Some("id".to_string()));
        assert_eq!(store.resolve_alias("en-US"), None);
    }

    #[test]
    fn test_store_manages_base_resource_uri() {
        let store = setup_store_with_stub();
        assert_eq!(*store.base_resource_uri, "https://dev.cdn.bistun.io/assets/");
    }
}
