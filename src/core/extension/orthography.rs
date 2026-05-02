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

//! # Orthographic Extension Mapper
//! Ref: [004-LMS-EXT]
//! Location: `src/core/extension/orthography.rs`
//!
//! **Why**: This module serves as Phase 3 (Override/Extension) of the pipeline. It maps the mechanical rendering requirements (Directionality, Bidi) of a script into the manifest, applying any necessary Unicode (-u-) overrides.
//! **Impact**: If this module fails, UIs will render text in the wrong direction (e.g., Arabic rendering LTR), causing catastrophic unreadability for RTL and TTB languages.
//!
//! ### Glossary
//! * **Bidi (Bidirectional)**: Text that contains both LTR and RTL scripts natively, requiring complex shaping and layout algorithms.

use crate::core::resolver::bcp47::LmsError;
use crate::data::store::LocaleProfile;
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::TraitKey;

/// Enriches and overrides the `CapabilityManifest` with Orthographic traits and Unicode extensions.
///
/// Time: O(N) string traversal | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Insert mechanical default rendering traits (`PrimaryDirection`, `HasBidiElements`, `RequiresShaping`) from the Flyweight profile.
/// 2. Scan the `raw_tag` for the `-u-` singleton sequence.
/// 3. If `-u-nu` (Numbers) or `-u-ca-` (Calendar) are found, extract their associated values via iterator and inject them into the manifest.
/// 4. Return successful completion.
///
/// # Examples
/// ```rust
///  let mut manifest = CapabilityManifest::new("en-US".to_string());
///  apply_rendering_traits(&mut manifest, &profile, "en-US-u-nu-latn").unwrap();
/// ```
///
/// # Arguments
/// * `manifest` (&mut CapabilityManifest): The mutable DTO being hydrated through the pipeline.
/// * `profile` (&LocaleProfile): The read-only Flyweight profile containing typological/orthographic defaults.
/// * `raw_tag` (&str): The raw BCP 47 language tag requested, parsed for Unicode extensions.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful hydration of the manifest.
///
/// # Golden I/O
/// * **Input**: `manifest`, `profile`, `"en-US-u-nu-latn-ca-gregory"`
/// * **Output**: `Ok(())` (Manifest updated with `NumberingSystem: "latn"` and `Calendar: "gregory"`)
///
/// # Errors, Panics, & Safety
/// * **Errors**: Conforms to pipeline signature returning `Result`, but currently infallible.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution with zero heap allocations for tag parsing.
pub fn apply_rendering_traits(
    manifest: &mut CapabilityManifest,
    profile: &LocaleProfile,
    raw_tag: &str,
) -> Result<(), LmsError> {
    // [STEP 1]: Base hydration of default rendering traits
    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(profile.direction));
    manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(profile.has_bidi));
    manifest
        .traits
        .insert(TraitKey::RequiresShaping, TraitValue::Boolean(profile.requires_shaping));

    // [STEP 2]: Extension Parsing (BCP 47 `-u-`)
    if let Some(u_ext_start) = raw_tag.find("-u-") {
        let extension_subtag = &raw_tag[u_ext_start + 3..];

        // Zero-allocation iterator parsing to protect the < 1ms budget
        let mut iter = extension_subtag.split('-');

        // [STEP 3]: Override Execution
        while let Some(part) = iter.next() {
            // If we hit another BCP 47 singleton (e.g., -t- or -x-), the -u- block is over.
            if part.len() == 1 {
                break;
            }

            match part {
                "nu" => {
                    if let Some(val) = iter.next() {
                        manifest
                            .traits
                            .insert(TraitKey::NumberingSystem, TraitValue::String(val.to_string()));
                    }
                }
                "ca" => {
                    if let Some(val) = iter.next() {
                        manifest
                            .traits
                            .insert(TraitKey::Calendar, TraitValue::String(val.to_string()));
                    }
                }
                _ => continue,
            }
        }
    }

    // [STEP 4]: Return
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::traits::{Direction, MorphType, SegType};

    /// Internal helper to generate a mock Flyweight profile for hermetic testing.
    fn create_mock_profile(
        direction: Direction,
        has_bidi: bool,
        requires_shaping: bool,
    ) -> LocaleProfile {
        LocaleProfile {
            id: "test".to_string(),
            morph: MorphType::ISOLATING,
            base_seg: SegType::SPACE,
            alt_seg: None,
            direction,
            has_bidi,
            requires_shaping,
            plurals: vec!["other".to_string()],
        }
    }

    #[test]
    fn test_apply_unicode_overrides() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create manifest and LTR mock profile.
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        let profile = create_mock_profile(Direction::LTR, false, false);

        // [STEP 2]: Execute: Pass a tag with Number (-nu-) and Calendar (-ca-) extensions.
        assert!(
            apply_rendering_traits(&mut manifest, &profile, "en-US-u-nu-latn-ca-gregory").is_ok()
        );

        // [STEP 3]: Assert: Verify the override execution mapped correctly.
        assert_eq!(
            manifest.traits.get(&TraitKey::NumberingSystem),
            Some(&TraitValue::String("latn".to_string()))
        );
        assert_eq!(
            manifest.traits.get(&TraitKey::Calendar),
            Some(&TraitValue::String("gregory".to_string()))
        );
    }

    #[test]
    fn test_apply_rtl_bidi_traits() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create manifest and RTL mock profile.
        // [STEP 2]: Execute: Run orthographic mapper.
        // [STEP 3]: Assert: Verify RTL and Bidi flags are correctly inserted dynamically.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let profile = create_mock_profile(Direction::RTL, true, true);

        assert!(apply_rendering_traits(&mut manifest, &profile, "ar-EG").is_ok());

        let dir = manifest.traits.get(&TraitKey::PrimaryDirection);
        assert_eq!(dir, Some(&TraitValue::Direction(Direction::RTL)));

        let bidi = manifest.traits.get(&TraitKey::HasBidiElements);
        assert_eq!(bidi, Some(&TraitValue::Boolean(true)));
    }

    #[test]
    fn test_apply_ttb_traits() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create manifest for Traditional Chinese mock profile.
        // [STEP 2]: Execute: Run orthographic mapper.
        // [STEP 3]: Assert: Verify Top-To-Bottom directionality is assigned dynamically.
        let mut manifest = CapabilityManifest::new("zh-Hant".to_string());
        let profile = create_mock_profile(Direction::TTB, false, false);

        assert!(apply_rendering_traits(&mut manifest, &profile, "zh-Hant").is_ok());

        let dir = manifest.traits.get(&TraitKey::PrimaryDirection);
        assert_eq!(dir, Some(&TraitValue::Direction(Direction::TTB)));
    }
}
