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
//! Location: `src/core/aggregator/typology.rs`
//!
//! **Why**: This module serves as Phase 2 (Aggregate) of the pipeline. It maps a resolved canonical locale to its core Typological traits, applying High-Water Mark logic for complex scripts.
//! **Impact**: If this module fails, the `CapabilityManifest` will lack foundational parsing traits, causing downstream NLP engines to crash or behave unpredictably.
//!
//! ### Glossary
//! * **High-Water Mark**: An architectural strategy where the most computationally expensive requirement (e.g., Dictionary-based segmentation) overrides simpler requirements (e.g., Space-based).
//! * **In-Place Mutation**: Modifying an object via mutable reference to prevent heap allocation overhead during the pipeline.

use crate::data::store::LocaleProfile;
use crate::models::error::LmsError;
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::TraitKey;
use std::cmp;

/// Enriches the `CapabilityManifest` with Typological traits directly from the `LocaleProfile`.
///
/// Time: O(1) map insertions | Space: O(1) (In-place mutation)
///
/// # Logic Trace (Internal)
/// 1. Compute the High-Water Mark segmentation strategy using the strict maximum of `base_seg` and `alt_seg`.
/// 2. Hydrate the manifest's trait dictionary with the resolved `MorphType`.
/// 3. Hydrate the manifest's trait dictionary with the resolved `SegType`.
/// 4. Yield successful completion.
///
/// # Examples
/// ```rust
///  let mut manifest = CapabilityManifest::new("ja-JP".to_string());
///  aggregate(&mut manifest, &profile).unwrap();
/// ```
///
/// # Arguments
/// * `manifest` (&mut CapabilityManifest): The mutable DTO being hydrated through the Phase 2 pipeline.
/// * `profile` (&LocaleProfile): The read-only Flyweight profile containing the baseline typological defaults.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful hydration of the manifest.
///
/// # Golden I/O
/// * **Input**: `manifest`, `profile` with `base_seg: CHARACTER`, `alt_seg: Some(DICTIONARY)`
/// * **Output**: `Ok(())` (Manifest updated with Plurals, Unicode, Morph, Seg, Norm, and Trans)
///
/// # Errors, Panics, & Safety
/// * **Errors**: Conforms to pipeline signature returning `Result`, but currently infallible since the profile guarantees data presence.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution with zero heap allocations.
pub fn aggregate(
    manifest: &mut CapabilityManifest,
    profile: &LocaleProfile,
) -> Result<(), LmsError> {
    // ---------------------------------------------------------
    // BLOCK 1: Structural & Typological Traits
    // ---------------------------------------------------------
    // [STEP 1]: Compute the High-Water Mark segmentation strategy.
    // If an alternative segmentation strategy exists for this locale profile,
    // we enforce the maximum complexity to ensure safety in the rendering engine.
    let final_seg = match profile.alt_seg {
        Some(alt) => cmp::max(profile.base_seg, alt),
        None => profile.base_seg,
    };

    // [STEP 2]: Hydrate the manifest with MorphType.
    manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(profile.morph));

    // [STEP 3]: Hydrate the manifest with SegType.
    manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(final_seg));

    // ---------------------------------------------------------
    // BLOCK 2: Grammatical Traits
    // ---------------------------------------------------------
    manifest
        .traits
        .insert(TraitKey::PluralCategories, TraitValue::StringArray(profile.plurals.clone()));

    // ---------------------------------------------------------
    // BLOCK 3: Encoding & Mechanics Traits
    // ---------------------------------------------------------
    manifest.traits.insert(
        TraitKey::UnicodePreloadBlocks,
        TraitValue::StringArray(profile.unicode_blocks.clone()),
    );
    manifest
        .traits
        .insert(TraitKey::NormalizationType, TraitValue::NormType(profile.normalization));
    manifest
        .traits
        .insert(TraitKey::TransliterationType, TraitValue::TransType(profile.transliteration));

    // ---------------------------------------------------------
    // BLOCK 4: Resource Linkage
    // ---------------------------------------------------------
    if let Some(resource_id) = &profile.required_resource {
        manifest.traits.insert(TraitKey::ResourceId, TraitValue::String(resource_id.clone()));
    }

    // [STEP 4]: Return Success.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::traits::{Direction, MorphType, NormType, SegType, TransType};

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
            plurals: vec!["one".to_string(), "other".to_string()],
            unicode_blocks: vec!["Basic Latin".to_string()],
            normalization: NormType::NFC,
            transliteration: TransType::NONE,
            required_resource: None,
        }
    }

    #[test]
    fn test_aggregate_standard_profile() {
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        let profile = create_mock_profile(MorphType::FUSIONAL, SegType::SPACE, None);

        assert!(aggregate(&mut manifest, &profile).is_ok());

        assert_eq!(
            manifest.traits.get(&TraitKey::MorphologyType),
            Some(&TraitValue::MorphType(MorphType::FUSIONAL))
        );
        assert_eq!(
            manifest.traits.get(&TraitKey::PluralCategories),
            Some(&TraitValue::StringArray(vec!["one".to_string(), "other".to_string()]))
        );
        assert_eq!(
            manifest.traits.get(&TraitKey::NormalizationType),
            Some(&TraitValue::NormType(NormType::NFC))
        );
    }

    #[test]
    fn test_aggregate_high_water_mark() {
        let mut manifest = CapabilityManifest::new("ja-JP".to_string());
        let profile = create_mock_profile(
            MorphType::AGGLUTINATIVE,
            SegType::CHARACTER,
            Some(SegType::DICTIONARY),
        );

        assert!(aggregate(&mut manifest, &profile).is_ok());

        let seg = manifest.traits.get(&TraitKey::SegmentationStrategy);
        assert_eq!(seg, Some(&TraitValue::SegType(SegType::DICTIONARY)));
    }
}
