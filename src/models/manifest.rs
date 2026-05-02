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
//! Ref: [011-LMS-DTO]
//! Location: `src/models/manifest.rs`
//!
//! **Why**: This module defines the primary Data Transfer Object (DTO) generated at the end of the 5-phase pipeline.
//! **Impact**: If this module or its serialization is compromised, downstream software components will receive malformed rendering/processing instructions, causing widespread UI and algorithmic failures.
//!
//! ### Glossary
//! * **Manifest**: An immutable, resolved package of linguistic instructions (traits) tailored to a specific runtime environment.
//! * **Untagged Serialization**: A Serde configuration where enums are serialized as their underlying value rather than explicitly wrapping them in their variant name.

use crate::models::traits::{Direction, MorphType, SegType, TraitKey};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Strongly typed wrapper for the heterogeneous values in the traits dictionary.
/// Using `untagged` allows us to serialize standard JSON types while maintaining Rust's exhaustiveness checks.
///
/// Time: O(1) | Space: O(N) (For String allocations in arrays)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TraitValue {
    Boolean(bool),
    Direction(Direction),
    SegType(SegType),
    MorphType(MorphType),
    StringArray(Vec<String>),
    String(String),
}

/// The immutable CapabilityManifest DTO delivered to consuming services.
///
/// Time: O(1) instantiation | Space: O(N) based on map sizes
///
/// # Logic Trace (Internal)
/// 1. Stores the fully resolved BCP 47 string (`resolved_locale`).
/// 2. Maintains a `hashbrown::HashMap` of linguistic traits to instruct rendering engines.
/// 3. Maintains a `metadata` dictionary for telemetry and observability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityManifest {
    pub resolved_locale: String,
    pub traits: HashMap<TraitKey, TraitValue>,
    pub metadata: HashMap<String, String>,
}

impl CapabilityManifest {
    /// Constructs a new, empty CapabilityManifest.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Ingest `resolved_locale` as the authoritative BCP 47 tag.
    /// 2. Initialize high-performance `hashbrown` HashMaps for traits and metadata.
    /// 3. Return the populated instance.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::models::manifest::CapabilityManifest;
    ///
    /// let manifest = CapabilityManifest::new("ar-EG".to_string());
    /// assert_eq!(manifest.resolved_locale, "ar-EG");
    /// ```
    ///
    /// # Arguments
    /// * `resolved_locale` (String): The fully resolved and validated BCP 47 language tag (e.g., "ar-EG").
    ///
    /// # Returns
    /// * `Self`: An empty CapabilityManifest, ready to be hydrated by the `TraitAggregator`.
    ///
    /// # Golden I/O
    /// * **Input**: `"ar-EG".to_string()`
    /// * **Output**: `CapabilityManifest { resolved_locale: "ar-EG", traits: {}, metadata: {} }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Fully safe synchronous initialization.
    pub fn new(resolved_locale: String) -> Self {
        // [STEP 1]: Ingest resolved_locale.
        // [STEP 2]: Initialize hashbrown maps.
        // [STEP 3]: Return the populated instance.
        Self { resolved_locale, traits: HashMap::new(), metadata: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_golden_serialization() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate a manifest for "ar-EG".
        // [STEP 2]: Execute: Populate traits reflecting Orthography (Direction) and Typology (Morphology).
        // [STEP 3]: Execute: Serialize the entire manifest to JSON.
        // [STEP 4]: Assert: Verify the untagged Serde serialization outputs exact JSON values, proving 011-LMS-DTO compliance.

        let mut manifest = CapabilityManifest::new("ar-EG".to_string());

        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::TEMPLATIC));

        manifest.metadata.insert("registry_version".to_string(), "1.0.0".to_string());

        let json_output = serde_json::to_string(&manifest).expect("Failed to serialize manifest");

        // Asserts ensure untagged serialization maps correctly to raw JSON primitives
        assert!(json_output.contains(r#""resolved_locale":"ar-EG""#));
        assert!(json_output.contains(r#""PRIMARY_DIRECTION":"RTL""#));
        assert!(json_output.contains(r#""HAS_BIDI_ELEMENTS":true"#));
        assert!(json_output.contains(r#""MORPHOLOGY_TYPE":"TEMPLATIC""#));
        assert!(json_output.contains(r#""registry_version":"1.0.0""#));
    }
}
