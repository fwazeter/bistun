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
//! **Why**: This module serves as Phase 2 (Aggregate) of the pipeline. It maps a resolved canonical locale to its core Typological and Orthographic traits, applying High-Water Mark logic for complex scripts.
//! **Impact**: If this module fails, the `CapabilityManifest` will lack foundational rendering and parsing traits, causing downstream NLP and text layout engines to crash or behave unpredictably.
//!
//! ### Glossary
//! * **High-Water Mark**: An architectural strategy where the most computationally expensive requirement (e.g., Dictionary-based segmentation) overrides simpler requirements (e.g., Space-based).
//! * **In-Place Mutation**: Modifying an object via mutable reference to prevent heap allocation overhead during the pipeline.

use crate::core::resolver::bcp47::{LmsError, LocaleEntry};
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::{MorphType, SegType, TraitKey};
use std::cmp;

/// Enriches the `CapabilityManifest` with Typological and primary Orthographic traits.
///
/// Time: O(1) per locale lookup | Space: O(1) (In-place mutation)
///
/// # Logic Trace (Internal)
/// 1. **Ingestion**: Accept a mutable `manifest` and the resolved `locale`.
/// 2. **Registry Lookup**: Query the internal registry stub for the locale's traits.
/// 3. **High-Water Mark Aggregation**: If multiple segmentation strategies apply, compute the strict maximum using standard library `cmp::max`.
/// 4. **Manifest Hydration**: Insert the resolved `MorphType` and `SegType` directly into the manifest's trait dictionary.
/// 5. **Return**: Yield success, or bubble up an error if the locale data is completely missing from the system.
///
/// # Examples
/// ```rust
/// use bistun::aggregators::typology::aggregate;
/// use bistun::models::manifest::CapabilityManifest;
/// use bistun::resolvers::bcp47::LocaleEntry;
///
/// let mut manifest = CapabilityManifest::new("ar-EG".to_string());
/// let locale = LocaleEntry { id: "ar-EG".to_string() };
///
/// aggregate(&mut manifest, &locale).unwrap();
/// // Manifest now contains TEMPLATIC morphology and SPACE segmentation.
/// ```
///
/// # Golden I/O
/// * **Input**: `LocaleEntry { id: "th-TH" }`
/// * **Output**: (Mutated Manifest) `SEGMENTATION_STRATEGY: DICTIONARY, MORPHOLOGY_TYPE: ISOLATING`
///
/// # Errors
/// * Returns [`LmsError::ResolutionFailed`] if the locale ID is missing from the Typology registry.
pub fn aggregate(manifest: &mut CapabilityManifest, locale: &LocaleEntry) -> Result<(), LmsError> {
    // 1 & 2. Ingestion & Registry Lookup
    let (morph, base_seg, alt_seg) = match locale.id.as_str() {
        "ar-EG" => (MorphType::TEMPLATIC, SegType::SPACE, None),
        "th-TH" => (MorphType::ISOLATING, SegType::DICTIONARY, None),
        "zh-Hant" => (MorphType::ISOLATING, SegType::CHARACTER, None),
        "ja-JP" => (MorphType::AGGLUTINATIVE, SegType::CHARACTER, Some(SegType::DICTIONARY)), // Multi-script edge case
        "en-US" | "en-AU" => (MorphType::FUSIONAL, SegType::SPACE, None),
        _ => return Err(LmsError::ResolutionFailed(locale.id.clone())),
    };

    // 3. High-Water Mark Aggregation
    // If an alternative segmentation strategy exists for this locale profile,
    // we enforce the maximum complexity to ensure safety in the rendering engine.
    let final_seg = match alt_seg {
        Some(alt) => cmp::max(base_seg, alt),
        None => base_seg,
    };

    // 4. Manifest Hydration
    manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(morph));

    manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(final_seg));

    // 5. Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_standard_locale() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest and standard locale (ar-EG).
        // 2. Execute: Run aggregation.
        // 3. Assert: Verify TEMPLATIC and SPACE traits are inserted in O(1).
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let locale = LocaleEntry { id: "ar-EG".to_string() };

        assert!(aggregate(&mut manifest, &locale).is_ok());

        let morph = manifest.traits.get(&TraitKey::MorphologyType);
        assert_eq!(morph, Some(&TraitValue::MorphType(MorphType::TEMPLATIC)));

        let seg = manifest.traits.get(&TraitKey::SegmentationStrategy);
        assert_eq!(seg, Some(&TraitValue::SegType(SegType::SPACE)));
    }

    #[test]
    fn test_aggregate_high_water_mark() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest for a multi-script locale (ja-JP) known to have mixed segmentation.
        // 2. Execute: Run aggregation.
        // 3. Assert: Verify DICTIONARY overrides CHARACTER due to High-Water Mark rules.
        let mut manifest = CapabilityManifest::new("ja-JP".to_string());
        let locale = LocaleEntry { id: "ja-JP".to_string() };

        assert!(aggregate(&mut manifest, &locale).is_ok());

        let seg = manifest.traits.get(&TraitKey::SegmentationStrategy);
        assert_eq!(seg, Some(&TraitValue::SegType(SegType::DICTIONARY))); // The highest ordinal
    }

    #[test]
    fn test_aggregate_fails_on_unknown_locale() {
        // [Logic Trace Mapping]
        // 1. Setup: Pass an unsupported/invalid locale ID.
        // 2. Execute & Assert: Verify it bubbles up an LmsError and leaves the manifest untouched.
        let mut manifest = CapabilityManifest::new("xx-YY".to_string());
        let locale = LocaleEntry { id: "xx-YY".to_string() };

        let result = aggregate(&mut manifest, &locale);
        assert_eq!(result, Err(LmsError::ResolutionFailed("xx-YY".to_string())));
        assert!(manifest.traits.is_empty());
    }
}
