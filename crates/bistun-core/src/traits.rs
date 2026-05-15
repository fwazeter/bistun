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

//! # Traits Dictionary & Enumerations
//! Crate: bistun-core
//! Ref: [011-LMS-DTO]
//! Location: `crates/bistun-core/src/traits.rs`
//!
//! **Why**: This module defines the shared vocabulary (`TraitKeys` and Enums) used to encapsulate the Typological and Orthographic properties of a locale.
//! **Impact**: If this module is compromised, the `CapabilityManifest` cannot be constructed, breaking the capability engine and causing downstream services to fail.
//!
//! ### Glossary
//! * **Typology**: The structural properties of a language (e.g., morphology).
//! * **Orthography**: The mechanical rendering requirements of a script (e.g., directionality).

use serde::{Deserialize, Serialize};

/// The "Golden Set" of trait keys used in the `CapabilityManifest`.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Represents standard keys for the DTO `traits` map.
/// 2. Utilizes `SCREAMING_SNAKE_CASE` serialization to match the DTO standard.
///
/// # Examples
/// ```rust
/// use crate::bistun_core::traits::TraitKey;
/// let key = TraitKey::SegmentationStrategy;
/// assert_eq!(key, TraitKey::SegmentationStrategy);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraitKey {
    // --- Rendering & Orthography ---
    /// The primary layout direction (e.g., LTR, RTL).
    PrimaryDirection,
    /// Indicates if the text naturally contains bidirectional elements.
    HasBidiElements,
    /// Indicates if the script requires complex shaping (e.g., Arabic).
    RequiresShaping,
    /// Unicode blocks to preload for rendering.
    UnicodePreloadBlocks,

    // --- Segmentation & Morphology ---
    /// Strategy used for word and sentence boundary detection.
    SegmentationStrategy,
    /// Typological classification of word formation.
    MorphologyType,
    /// Plural category logic required for the locale.
    PluralCategories,

    // --- Cultural Defaults ---
    /// Default numeric system (e.g., latn, arab).
    DefaultNumberingSystem,
    /// Default calendar system (e.g., gregory, islamic).
    DefaultCalendar,
}

/// The UI rendering direction derived from Orthographic mechanics.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Represents the text layout requirements for a specific script.
/// 2. Utilizes `UPPERCASE` serialization for cross-system compatibility.
///
/// # Examples
/// ```rust
/// use crate::bistun_core::traits::Direction;
/// let dir = Direction::RTL;
/// assert_eq!(dir, Direction::RTL);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// Left-to-Right layout.
    LTR,
    /// Right-to-Left layout.
    RTL,
    /// Top-to-Bottom layout.
    TTB,
    /// Native bidirectional layout.
    BIDI,
}

/// The boundary detection logic (Segmentation) required by the script.
///
/// Ordered from the lowest complexity to highest to support the High-Water Mark strategy.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Ordered explicitly to allow `Ord` trait derivation to rank complexity automatically.
/// 2. Permits `TraitAggregator` to resolve conflicts seamlessly.
///
/// # Examples
/// ```rust
/// use crate::bistun_core::traits::SegType;
/// // Demonstrating High-Water Mark ordinal comparison
/// assert!(SegType::DICTIONARY > SegType::SPACE);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SegType {
    /// No segmentation required.
    NONE,
    /// Segmentation based on whitespace.
    SPACE,
    /// Segmentation based on individual characters/syllables.
    CHARACTER,
    /// Dictionary-based complex segmentation.
    DICTIONARY,
}

/// The Typological structure of a language's word formation.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Maps language identity to execution strategies for NLP operations (e.g., stemming).
///
/// # Examples
/// ```rust
/// use crate::bistun_core::traits::MorphType;
/// let morph = MorphType::AGGLUTINATIVE;
/// assert_eq!(morph, MorphType::AGGLUTINATIVE);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MorphType {
    /// Words are invariant (e.g., Chinese).
    ISOLATING,
    /// Words are formed by stringing together discrete morphemes (e.g., Turkish).
    AGGLUTINATIVE,
    /// Morphemes are fused together in complex ways (e.g., Spanish).
    FUSIONAL,
    /// Words are formed using root consonants and vowel templates (e.g., Arabic).
    TEMPLATIC,
    /// Complex multi-morpheme words acting as entire sentences (e.g., Inuktitut).
    POLYSYNTHETIC,
}

// =====================================================================
// V2.0.0 Rule Engine Directives
// =====================================================================

/// Represents the standard algorithmic directives for the Rule Synthesis Engine.
/// Note: this does not actually appear in the JSON, only the inner variants appear.
/// Ref: [013-LMS-RULE]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LmsRule {
    /// Transliteration rule directive.
    Trans(TransRule),
    /// Pluralization rule directive.
    Plural(PluralRule),
    /// Casing rule directive.
    Casing(CasingRule),
    /// Normalization rule directive.
    Norm(NormRule),
}

/// Directives for transliteration and phonetic rendering strategies.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransRule {
    /// No transliteration required.
    NONE,
    /// Standard romanization transformation.
    ROMANIZATION,
    /// Phonetic spelling transformation.
    PHONETIC,
    /// ICU4X algorithmic transform capability.
    ICU_TRANSFORM,
}

/// Directives for Unicode normalization logic.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormRule {
    /// Normalization Form C.
    NFC,
    /// Normalization Form D.
    NFD,
    /// Normalization Form KC.
    NFKC,
    /// Normalization Form KD.
    NFKD,
}

/// Directives for morphological plural category mapping.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluralRule {
    /// Only cardinal numbers are supported.
    CARDINAL_ONLY,
    /// Ordinal and cardinal numbers are supported.
    ORDINAL_SUPPORT,
    /// Multiple plural categories required (few, many, other, etc.).
    MULTIPLE_CATEGORIES,
}

/// Directives for typographic casing mechanics.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CasingRule {
    /// Strict case sensitivity.
    CASE_SENSITIVE,
    /// Case-insensitive matching.
    CASE_INSENSITIVE,
    /// Special Unicode casing rules (e.g., Turkish dotless i).
    UNICODE_SPECIAL,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seg_type_high_water_mark_ordering() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate SegType variants via Ord trait checking.
        // [STEP 2]: Execute: Compare using the derived Ord logic.
        // [STEP 3]: Assert: Verify DICTIONARY ranks higher than SPACE, etc.
        assert!(SegType::DICTIONARY > SegType::SPACE);
        assert!(SegType::CHARACTER > SegType::SPACE);
        assert!(SegType::SPACE > SegType::NONE);
    }

    #[test]
    fn test_trait_key_serialization() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate TraitKeys.
        // [STEP 2]: Execute: Serialize to JSON strings.
        // [STEP 3]: Assert: Verify SCREAMING_SNAKE_CASE serialization.
        let key_dir = TraitKey::PrimaryDirection;
        let key_num = TraitKey::DefaultNumberingSystem;

        let json_dir = serde_json::to_string(&key_dir).expect("Failed to serialize trait key");
        let json_num = serde_json::to_string(&key_num).expect("Failed to serialize trait key");

        assert_eq!(json_dir, r#""PRIMARY_DIRECTION""#);
        assert_eq!(json_num, r#""DEFAULT_NUMBERING_SYSTEM""#);
    }
}
