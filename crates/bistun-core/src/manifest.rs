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

//! # Capability Manifest DTO
//! Crate: bistun-core
//! Ref: [011-LMS-DTO]
//! Location: `crates/bistun-core/src/manifest.rs`
//!
//! **Why**: This module defines the primary Data Transfer Object (DTO) generated at the end of the 5-phase pipeline.
//! **Impact**: If this module or its serialization is compromised, downstream software components will receive malformed rendering/processing instructions, causing widespread UI and algorithmic failures.
//!
//! ### Glossary
//! * **Manifest**: An immutable, resolved package of linguistic instructions (traits) tailored to a specific runtime environment.
//! * **Untagged Serialization**: A Serde configuration where enums are serialized as their underlying value rather than explicitly wrapping them in their variant name.

use crate::traits::{Direction, LmsRule, MorphType, SegType, TraitKey};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Strongly typed wrapper for the heterogeneous values in the traits dictionary.
/// Using `untagged` allows us to serialize standard JSON types while maintaining Rust's exhaustiveness checks.
///
/// Time: O(1) | Space: O(N) (For String allocations in arrays)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TraitValue {
    // --- Orthographic Enums ---
    /// Text directionality layout directive.
    Direction(Direction),
    /// Word and sentence boundary detection strategy.
    SegType(SegType),

    // --- Typological Enums ---
    /// Linguistic morphology classification.
    MorphType(MorphType),

    // -- Primitives ---
    /// A boolean flag (e.g., true/false).
    Boolean(bool),
    /// An array of string values (e.g., `["one", "other"]`).
    StringArray(Vec<String>),
    /// A catch-all string value (e.g., "arab").
    /// Note: Must be last to prevent greedy matching over specialized enums.
    String(String),
}

/// The immutable `CapabilityManifest` DTO delivered to consuming services.
///
/// Time: O(1) instantiation | Space: O(N) based on map sizes
///
/// # Logic Trace (Internal)
/// 1. Stores the fully resolved BCP 47 string (`resolved_locale`).
/// 2. Maintains a `hashbrown::HashMap` of linguistic traits to instruct rendering engines.
/// 3. Maintains a `metadata` dictionary for telemetry and observability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityManifest {
    // --- Identity ---
    /// The canonical BCP 47 tag resulting from taxonomic resolution.
    pub resolved_locale: String,

    // --- Linguistic Domains ---
    /// The dictionary of "Linguistic DNA" traits (Typology & Orthography).
    pub traits: HashMap<TraitKey, TraitValue>,
    /// Logical directives for algorithmic execution (Rule Synthesis).
    pub rules: HashMap<String, LmsRule>,
    /// Mappings of Logical Resource IDs to Physical Paths.
    pub resources: HashMap<String, String>,

    // --- Overrides & Observability
    /// User-requested BCP 47 extensions (e.g., `-u-nu-latn`).
    pub extensions: HashMap<String, String>,
    /// Observability data (registry version, SLI latency).
    pub metadata: HashMap<String, String>,
}

impl CapabilityManifest {
    /// Constructs a new, empty `CapabilityManifest`.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Ingest `resolved_locale` as the authoritative BCP 47 tag.
    /// 2. Initialize high-performance `hashbrown` `HashMaps` for traits and metadata.
    /// 3. Return the populated instance.
    ///
    /// # Examples
    /// ```rust
    /// use crate::bistun_core::manifest::CapabilityManifest;
    ///
    /// let manifest = CapabilityManifest::new("ar-EG".to_string());
    /// assert_eq!(manifest.resolved_locale, "ar-EG");
    /// ```
    ///
    /// # Arguments
    /// * `resolved_locale` (String): The fully resolved and validated BCP 47 language tag (e.g., "ar-EG").
    ///
    /// # Returns
    /// * `Self`: An empty `CapabilityManifest`, ready to be hydrated by the `TraitAggregator`.
    ///
    /// # Golden I/O
    /// * **Input**: `"ar-EG".to_string()`
    /// * **Output**: `CapabilityManifest { resolved_locale: "ar-EG", traits: {}, metadata: {} }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Fully safe synchronous initialization.
    #[must_use]
    pub fn new(resolved_locale: String) -> Self {
        // [STEP 1]: Ingest resolved_locale.
        // [STEP 2]: Initialize hashbrown maps.
        // [STEP 3]: Return the populated instance.
        Self {
            resolved_locale,
            traits: HashMap::new(),
            rules: HashMap::new(),
            resources: HashMap::new(),
            extensions: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate a manifest for Egyptian Arabic with Latin numeral override.
        // [STEP 2]: Execute: Populate the categorized maps with specific v2.0.0 test data.
        // [STEP 3]: Assert: Verify that Serde correctly flattens the untagged enums into the JSON output.

        let mut manifest = CapabilityManifest::new("ar-EG-u-nu-latn".to_string());

        // Inject Phase 2 Aggregation Traits (Immutable DNA)
        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::TEMPLATIC));
        manifest
            .traits
            .insert(TraitKey::DefaultNumberingSystem, TraitValue::String("arab".to_string()));

        // Inject Phase 2 Synthesis Rules (Algorithmic Logic)
        manifest.rules.insert(
            "TRANSLITERATION_DEFAULT".to_string(),
            LmsRule::Trans(crate::traits::TransRule::ICU_TRANSFORM),
        );

        // Inject Phase 2.5 Resource Paths
        manifest.resources.insert(
            "icu_arab".to_string(),
            "https://cdn.bistun.io/v1/data/icu_arab.postcard".to_string(),
        );

        // Inject Phase 3 Overrides
        manifest.extensions.insert("nu".to_string(), "latn".to_string());

        // Inject Phase 5 Telemetry
        manifest.metadata.insert("registry_version".to_string(), "2.0.0".to_string());

        let json_output =
            serde_json::to_string(&manifest).expect("LMS-TEST: Failed to serialize manifest");

        // Parse the raw string back into a dynamic JSON Value for structural assertions
        let parsed: serde_json::Value =
            serde_json::from_str(&json_output).expect("LMS-TEST: Failed to parse JSON");

        // Asserts ensure untagged serialization maps correctly to raw JSON primitives
        assert_eq!(parsed["resolved_locale"], "ar-EG-u-nu-latn");
        assert_eq!(parsed["traits"]["PRIMARY_DIRECTION"], "RTL");
        assert_eq!(parsed["traits"]["DEFAULT_NUMBERING_SYSTEM"], "arab");
        assert_eq!(parsed["rules"]["TRANSLITERATION_DEFAULT"], "ICU_TRANSFORM");
        assert_eq!(
            parsed["resources"]["icu_arab"],
            "https://cdn.bistun.io/v1/data/icu_arab.postcard"
        );
        assert_eq!(parsed["extensions"]["nu"], "latn");
        assert_eq!(parsed["metadata"]["registry_version"], "2.0.0");
    }
}
