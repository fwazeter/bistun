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

//! # The Capability Engine Coordinator
//! Ref: [001-LMS-CORE]
//!
//! **Why**: This module serves as the Pipeline Coordinator. It wires together the 5-Phase execution sequence to transform a BCP 47 string into an immutable `CapabilityManifest`.
//! **Impact**: If this module fails, the entire capability engine is disconnected, and the service will return empty or un-resolved instructions to all clients.
//!
//! ### Glossary
//! * **Pipeline Coordinator**: The orchestrator that executes Resolution, Aggregation, Override, Integrity, and Telemetry in a strict, sequential order.
//! * **In-Place Hydration**: The process of passing a mutable reference of the manifest down the pipeline to avoid costly memory re-allocations.

pub mod aggregator;
pub mod extension;
pub mod resolver;

use crate::core::resolver::bcp47::{self, LmsError};
use crate::models::manifest::CapabilityManifest;
// Note: In our current sandbox, we import from the submodules we just built.
use self::aggregator::typology;
use self::extension::orthography;

/// Orchestrates the 5-Phase capability pipeline to generate a manifest.
///
/// Time: O(N) where N is tag truncation length | Space: O(1) map allocations
///
/// # Logic Trace (Internal)
/// 1. **Phase 1 (Resolve)**: Pass the raw tag to the Taxonomic resolver to get a canonical `LocaleEntry`.
/// 2. **Instantiation**: Create a fresh `CapabilityManifest` using the resolved locale ID.
/// 3. **Phase 2 (Aggregate)**: Pass a mutable reference of the manifest to the Typology aggregator to bind structural genetics.
/// 4. **Phase 3 (Override)**: Pass the mutable reference to the Orthography mapper to bind rendering mechanics and `-u-` extensions.
/// 5. **Phase 4 & 5 (Integrity & Telemetry)**: (Stubbed) Run validation algorithms and log execution times.
/// 6. **Return**: Yield the fully hydrated and immutable manifest.
///
/// # Examples
/// ```rust
/// use bistun::core::generate_manifest;
///
/// let manifest = generate_manifest("ar-EG").unwrap();
/// assert_eq!(manifest.resolved_locale, "ar-EG");
/// assert!(!manifest.traits.is_empty());
/// ```
///
/// # Golden I/O
/// * **Input**: "zh-Hant-TW"
/// * **Output**: `CapabilityManifest { resolved_locale: "zh-Hant", traits: { ... } }`
///
/// # Errors
/// * Returns [`LmsError::InvalidTag`] if the initial string is entirely malformed.
/// * Returns [`LmsError::ResolutionFailed`] if no registry match can be found during truncation.
pub fn generate_manifest(raw_tag: &str) -> Result<CapabilityManifest, LmsError> {
    // Phase 1: Resolve (Taxonomy)
    let locale = bcp47::resolve(raw_tag)?;

    // Instantiation
    let mut manifest = CapabilityManifest::new(locale.id.clone());

    // Phase 2: Aggregate (Typology)
    typology::aggregate(&mut manifest, &locale)?;

    // Phase 3: Override (Orthography)
    orthography::apply_rendering_traits(&mut manifest, &locale)?;

    // Phase 4: Integrity Check [STUB - Ref: 003-LMS-VAL]
    // validation::integrity::verify(&manifest)?;

    // Phase 5: Telemetry [STUB - Ref: 007-LMS-OPS]
    // ops::telemetry::record_resolution(&manifest);

    // Return the immutable manifest
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::manifest::TraitValue;
    use crate::models::traits::{Direction, MorphType, SegType, TraitKey};

    #[test]
    fn test_e2e_pipeline_success() {
        // [Logic Trace Mapping]
        // 1. Setup/Execute: Pass a standard raw tag that requires standard resolution.
        // 2. Assert: Verify the returned manifest has run through all 3 active phases successfully.

        let manifest = generate_manifest("th-TH-u-nu-thai").expect("Pipeline failed on valid tag");

        // Phase 1 verification
        assert_eq!(manifest.resolved_locale, "th-TH");

        // Phase 2 verification (Typology)
        let morph = manifest.traits.get(&TraitKey::MorphologyType);
        assert_eq!(morph, Some(&TraitValue::MorphType(MorphType::ISOLATING)));
        let seg = manifest.traits.get(&TraitKey::SegmentationStrategy);
        assert_eq!(seg, Some(&TraitValue::SegType(SegType::DICTIONARY)));

        // Phase 3 verification (Orthography)
        let dir = manifest.traits.get(&TraitKey::PrimaryDirection);
        assert_eq!(dir, Some(&TraitValue::Direction(Direction::LTR)));
    }

    #[test]
    fn test_e2e_pipeline_truncation() {
        // [Logic Trace Mapping]
        // 1. Setup/Execute: Pass a complex tag that requires right-to-left truncation.
        // 2. Assert: Verify the pipeline successfully maps it to the base locale "ar-EG".

        let manifest =
            generate_manifest("ar-EG-u-ca-islamic").expect("Pipeline failed on truncation");

        assert_eq!(manifest.resolved_locale, "ar-EG");
        let bidi = manifest.traits.get(&TraitKey::HasBidiElements);
        assert_eq!(bidi, Some(&TraitValue::Boolean(true)));
    }

    #[test]
    fn test_e2e_pipeline_fallback_on_invalid_input() {
        // [Logic Trace Mapping]
        // 1. Setup/Execute: Pass garbage input.
        // 2. Assert: Verify the pipeline triggers the Safety Net and returns an en-US manifest.
        let manifest =
            generate_manifest("invalid-tag-xyz").expect("Pipeline should succeed via fallback");

        assert_eq!(manifest.resolved_locale, "en-US");
        assert_eq!(
            manifest.traits.get(&TraitKey::PrimaryDirection),
            Some(&TraitValue::Direction(Direction::LTR))
        );
    }
}
