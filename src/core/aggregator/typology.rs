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

//! # Typological Aggregator
//! Ref: [008-LMS-TYPOLOGY-AGGREGATOR]
//!
//! **Why**: This module serves as Phase 2 (Aggregate) of the pipeline. It maps a resolved canonical locale to its core Typological traits, applying High-Water Mark logic for complex scripts.
//! **Impact**: If this module fails, the `CapabilityManifest` will lack foundational parsing traits, causing downstream NLP engines to crash or behave unpredictably.
//!
//! ### Glossary
//! * **High-Water Mark**: An architectural strategy where the most computationally expensive requirement (e.g., Dictionary-based segmentation) overrides simpler requirements (e.g., Space-based).
//! * **In-Place Mutation**: Modifying an object via mutable reference to prevent heap allocation overhead during the pipeline.

use crate::core::resolver::bcp47::LmsError;
use crate::data::store::LocaleProfile;
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::TraitKey;
use std::cmp;

/// Enriches the `CapabilityManifest` with Typological traits directly from the `LocaleProfile`.
///
/// Time: O(1) map insertions | Space: O(1) (In-place mutation)
///
/// # Logic Trace (Internal)
/// 1. **Ingestion**: Accept a mutable `manifest` and the dynamic Flyweight `profile`.
/// 2. **High-Water Mark Aggregation**: If an alternative segmentation strategy exists, compute the strict maximum using standard library `cmp::max`.
/// 3. **Manifest Hydration**: Insert the resolved `MorphType` and `SegType` directly into the manifest's trait dictionary.
/// 4. **Return**: Yield success.
///
/// # Errors
/// * Designed to return `Result` to conform to the pipeline standard, though currently infallible since the profile guarantees data presence.
pub fn aggregate(
    manifest: &mut CapabilityManifest,
    profile: &LocaleProfile,
) -> Result<(), LmsError> {
    // 1 & 2. High-Water Mark Aggregation [Ref: 008-LMS-TYPOLOGY-AGGREGATOR]
    // If an alternative segmentation strategy exists for this locale profile,
    // we enforce the maximum complexity to ensure safety in the rendering engine.
    let final_seg = match profile.alt_seg {
        Some(alt) => cmp::max(profile.base_seg, alt),
        None => profile.base_seg,
    };

    // 3. Manifest Hydration
    manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(profile.morph));

    manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(final_seg));

    // 4. Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::traits::{Direction, MorphType, SegType};

    /// Internal helper to generate a mock Flyweight profile for hermetic testing.
    fn create_mock_profile(
        morph: MorphType,
        base_seg: SegType,
        alt_seg: Option<SegType>,
    ) -> LocaleProfile {
        LocaleProfile {
            id: "test-locale".to_string(),
            morph,
            base_seg,
            alt_seg,
            direction: Direction::LTR,
            has_bidi: false,
            requires_shaping: false,
            plurals: vec!["other".to_string()],
        }
    }

    #[test]
    fn test_aggregate_standard_profile() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest and a mock profile with standard traits (e.g., Templatic/Space).
        // 2. Execute: Run aggregation.
        // 3. Assert: Verify traits are inserted dynamically without hardcoded ID lookups.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let profile = create_mock_profile(MorphType::TEMPLATIC, SegType::SPACE, None);

        assert!(aggregate(&mut manifest, &profile).is_ok());

        let morph = manifest.traits.get(&TraitKey::MorphologyType);
        assert_eq!(morph, Some(&TraitValue::MorphType(MorphType::TEMPLATIC)));

        let seg = manifest.traits.get(&TraitKey::SegmentationStrategy);
        assert_eq!(seg, Some(&TraitValue::SegType(SegType::SPACE)));
    }

    #[test]
    fn test_aggregate_high_water_mark() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest and a mock profile representing a multi-script locale.
        // 2. Execute: Run aggregation.
        // 3. Assert: Verify DICTIONARY overrides CHARACTER due to the cmp::max High-Water Mark rules.
        let mut manifest = CapabilityManifest::new("ja-JP".to_string());

        let profile = create_mock_profile(
            MorphType::AGGLUTINATIVE,
            SegType::CHARACTER,
            Some(SegType::DICTIONARY), // The more complex script override
        );

        assert!(aggregate(&mut manifest, &profile).is_ok());

        let seg = manifest.traits.get(&TraitKey::SegmentationStrategy);
        assert_eq!(seg, Some(&TraitValue::SegType(SegType::DICTIONARY)));
    }
}
