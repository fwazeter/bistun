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

#![cfg(feature = "persistence")]

//! # Registry Persistence Models
//! Crate: bistun-core
//! Ref: [010-LMS-MEM], [002-LMS-DATA]
//! Location: `crates/bistun-core/src/registry.rs`
//!
//! **Why**: This module defines the "Ground Truth" data structures used to store linguistic profiles in the WORM registry and manage them in memory.
//! **Impact**: These structures represent the system's persisted state. Any changes here dictate how the WORM snapshot is deserialized and injected into the 5-Phase pipeline.
//!
//! ### Glossary
//! * **WORM (Write-Once, Read-Many)**: A storage philosophy ensuring data immutability for the registry snapshots.
//! * **Flyweight**: A memory optimization pattern where shared instances (like `LocaleProfile`) are wrapped in `Arc` for lock-free reading.

use crate::traits::LmsRule;
use crate::{TraitKey, TraitValue};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Authoritative identity and audit metadata for a specific registry build.
///
/// Time: O(1) (Definition) | Space: O(1)
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
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiate the metadata with zeroed-out default dates and checksums.
    /// 2. Return the safe bootstrap instance.
    ///
    /// # Returns
    /// * `Self`: A default metadata struct for early lifecycle states.
    fn default() -> Self {
        Self {
            version: "v0.0.0-default".to_string(),
            build_date: "1970-01-01T00:00:00Z".to_string(),
            checksum: "00000000000000000000000000000000".to_string(),
        }
    }
}

/// An aggregated representation of a locale's Typological and Orthographic rules.
///
/// Time: O(1) (Definition) | Space: O(1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleProfile {
    /// The canonical BCP 47 identifier (e.g., "ar-EG").
    #[serde(rename = "ID")]
    pub id: String,
    /// The collection of "Linguistic DNA" traits (Typology & Orthography).
    #[serde(default)]
    pub traits: HashMap<TraitKey, TraitValue>,
    /// Logical directives for algorithmic execution.
    #[serde(default)]
    pub rules: HashMap<String, LmsRule>,
    /// Mappings of Logical Resource IDs to Physical Paths.
    #[serde(default)]
    pub resources: HashMap<String, String>,
}

/// The top-level persistence contract for the entire WORM JSON registry.
///
/// Time: O(1) (Definition) | Space: O(N) based on payload size during parsing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WormPayload {
    /// Authoritative build identity.
    pub metadata: RegistryMetadata,
    /// The array of fully inflated Linguistic DNA profiles.
    pub profiles: Vec<LocaleProfile>,
    /// Deprecated or macro language tag mappings.
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

/// The high-performance, in-memory data store for linguistic profiles and dynamic routing mappings.
///
/// Time: O(1) reads | Space: O(N) where N is the number of distinct locales + aliases
#[derive(Debug, Default)]
pub struct RegistryStore {
    /// Authoritative identity of this specific data set.
    pub metadata: Arc<RegistryMetadata>,
    /// Environment-specific base URI for resolving binary data blobs.
    pub base_resource_uri: Arc<String>,
    /// Maps a canonical Locale ID (e.g., "ar-EG") to its Flyweight profile.
    pools: HashMap<String, Arc<LocaleProfile>>,
    /// Maps a deprecated or macro language tag (e.g., "in") to its canonical ID (e.g., "id").
    aliases: HashMap<String, String>,
}

impl RegistryStore {
    /// Initializes a new, empty `RegistryStore`.
    ///
    /// Time: O(1) | Space: O(1) map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiate the high-throughput `hashbrown` `HashMaps` for pools and aliases.
    /// 2. Inject default metadata and a fallback resource URI.
    /// 3. Return the empty store structure.
    ///
    /// # Examples
    /// ```rust
    /// # use bistun_core::registry::RegistryStore;
    /// let store = RegistryStore::new();
    /// ```
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `Self`: A newly instantiated, empty `RegistryStore`.
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Fully safe synchronous initialization.
    #[must_use]
    pub fn new() -> Self {
        Self {
            metadata: Arc::new(RegistryMetadata::default()),
            base_resource_uri: Arc::new(String::new()),
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
    /// Time: O(1) | Space: O(1) pointer allocation
    ///
    /// # Logic Trace (Internal)
    /// 1. Query the internal `pools` map using the provided locale `id`.
    /// 2. If found, clone the `Arc` (bumping the reference count) and return it.
    /// 3. If missing, return `None`.
    ///
    /// # Examples
    /// ```rust
    /// # use bistun_core::registry::RegistryStore;
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
    #[must_use]
    pub fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>> {
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
    ///
    /// # Arguments
    /// * `tag` (&str): The legacy or macrolanguage BCP 47 tag.
    ///
    /// # Returns
    /// * `Option<String>`: The canonical BCP 47 ID.
    #[must_use]
    pub fn resolve_alias(&self, tag: &str) -> Option<String> {
        self.aliases.get(tag).cloned()
    }

    /// Inserts a profile directly into the pool.
    ///
    /// Time: O(1) amortized | Space: O(1) reference insertion
    ///
    /// # Logic Trace (Internal)
    /// 1. Wrap the owned `LocaleProfile` in an `Arc` to establish the Flyweight pattern.
    /// 2. Insert the `Arc` into the `pools` map, keyed by the profile's ID.
    ///
    /// # Arguments
    /// * `profile` (LocaleProfile): The fully hydrated linguistic profile.
    pub fn insert_stub(&mut self, profile: LocaleProfile) {
        self.pools.insert(profile.id.clone(), Arc::new(profile));
    }

    /// Inserts an alias mapping directly into the dynamic routing table.
    ///
    /// Time: O(1) amortized | Space: O(1) node allocation
    ///
    /// # Logic Trace (Internal)
    /// 1. Insert the raw string mapping directly into the `aliases` Hashmap.
    ///
    /// # Arguments
    /// * `alias` (String): The deprecated or macrolanguage tag.
    /// * `canonical` (String): The correct operational ID.
    pub fn insert_alias(&mut self, alias: String, canonical: String) {
        self.aliases.insert(alias, canonical);
    }
}
