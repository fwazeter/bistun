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
//! Ref: [011-LMS-DTO]
//!
//! **Why**: This module defines the shared vocabulary (TraitKeys and Enums) used to encapsulate the Typological and Orthographic properties of a locale.
//! **Impact**: If this module is compromised, the `CapabilityManifest` cannot be constructed, breaking the capability engine and causing downstream services to fail.
//!
//! ### Glossary
//! * **Typology**: The structural properties of a language (e.g., morphology).
//! * **Orthography**: The mechanical rendering requirements of a script (e.g., directionality).

use serde::{Deserialize, Serialize};

/// The "Golden Set" of trait keys used in the CapabilityManifest.
///
/// Time: O(1) | Space: O(1) (Stack allocated)
///
/// # Logic Trace (Internal)
/// 1. Represents standard keys for the DTO `traits` map.
/// 2. Utilizes `SCREAMING_SNAKE_CASE` serialization to match the DTO standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraitKey {
    PrimaryDirection,
    HasBidiElements,
    RequiresShaping,
    SegmentationStrategy,
    MorphologyType,
    PluralCategories,
    UnicodePreloadBlocks,
    NumberingSystem,
    Calendar,
}

/// The UI rendering direction derived from Orthographic mechanics.
///
/// Time: O(1) | Space: O(1)
///
/// # Examples
/// ```rust
/// use bistun::models::traits::Direction;
///
/// let dir = Direction::RTL;
/// assert_eq!(dir, Direction::RTL);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Direction {
    LTR,
    RTL,
    TTB,
    BIDI,
}

/// The boundary detection logic (Segmentation) required by the script.
/// Ordered from the lowest complexity to highest to support the High-Water Mark strategy.
///
/// Time: O(1) | Space: O(1)
///
/// # Examples
/// ```rust
/// use bistun::models::traits::SegType;
///
/// // Demonstrating High-Water Mark ordinal comparison
/// assert!(SegType::DICTIONARY > SegType::SPACE);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SegType {
    NONE,
    SPACE,
    CHARACTER,
    DICTIONARY,
}

/// The Typological structure of a language's word formation.
///
/// Time: O(1) | Space: O(1)
///
/// # Examples
/// ```rust
/// use bistun::models::traits::MorphType;
///
/// let morph = MorphType::AGGLUTINATIVE;
/// assert_eq!(morph, MorphType::AGGLUTINATIVE);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MorphType {
    ISOLATING,
    AGGLUTINATIVE,
    FUSIONAL,
    TEMPLATIC,
    POLYSYNTHETIC,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segtype_high_water_mark_ordering() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate SegType variants.
        // 2. Execute: Compare using Ord trait.
        // 3. Assert: Verify DICTIONARY ranks higher than SPACE.
        assert!(SegType::DICTIONARY > SegType::SPACE);
        assert!(SegType::CHARACTER > SegType::SPACE);
        assert!(SegType::SPACE > SegType::NONE);
    }

    #[test]
    fn test_traitkey_serialization() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate a TraitKey.
        // 2. Execute: Serialize to JSON string.
        // 3. Assert: Verify it serializes to SCREAMING_SNAKE_CASE per 011-LMS-DTO.
        let serialized = serde_json::to_string(&TraitKey::SegmentationStrategy).unwrap();
        assert_eq!(serialized, "\"SEGMENTATION_STRATEGY\"");
    }
}
