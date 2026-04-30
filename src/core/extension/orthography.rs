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

/// Enriches and overrides the `CapabilityManifest` with Orthographic rendering mechanics.
///
/// Time: O(1) map insertions | Space: O(1) (In-place mutation)
///
/// # Logic Trace (Internal)
/// 1. **Ingestion**: Accept a mutable `manifest` and the dynamic Flyweight `profile`.
/// 2. **Extension Override**: (Future) Check the BCP 47 tag for explicit `-u-` rendering overrides and apply them if present.
/// 3. **Manifest Hydration**: Insert `PrimaryDirection`, `HasBidiElements`, and `RequiresShaping` directly into the manifest.
/// 4. **Return**: Yield success.
///
/// # Errors
/// * Designed to return `Result` to conform to the pipeline standard, though currently infallible since the profile guarantees data presence.
pub fn apply_rendering_traits(
    manifest: &mut CapabilityManifest,
    profile: &LocaleProfile,
) -> Result<(), LmsError> {
    // [STUB for 004-LMS-EXT]: Extension Override
    // If the raw tag included `-u-` extensions that override standard orthography,
    // they would be processed and applied here before hydration.

    // Manifest Hydration
    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(profile.direction));

    manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(profile.has_bidi));

    manifest
        .traits
        .insert(TraitKey::RequiresShaping, TraitValue::Boolean(profile.requires_shaping));

    // Return Success
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
            id: "test-locale".to_string(),
            morph: MorphType::ISOLATING,
            base_seg: SegType::SPACE,
            alt_seg: None,
            direction,
            has_bidi,
            requires_shaping,
        }
    }

    #[test]
    fn test_apply_rtl_bidi_traits() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest and RTL mock profile.
        // 2. Execute: Run orthographic mapper.
        // 3. Assert: Verify RTL and Bidi flags are correctly inserted dynamically.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let profile = create_mock_profile(Direction::RTL, true, true);

        assert!(apply_rendering_traits(&mut manifest, &profile).is_ok());

        let dir = manifest.traits.get(&TraitKey::PrimaryDirection);
        assert_eq!(dir, Some(&TraitValue::Direction(Direction::RTL)));

        let bidi = manifest.traits.get(&TraitKey::HasBidiElements);
        assert_eq!(bidi, Some(&TraitValue::Boolean(true)));
    }

    #[test]
    fn test_apply_ttb_traits() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest for Traditional Chinese mock profile.
        // 2. Execute: Run orthographic mapper.
        // 3. Assert: Verify Top-To-Bottom directionality is assigned dynamically.
        let mut manifest = CapabilityManifest::new("zh-Hant".to_string());
        let profile = create_mock_profile(Direction::TTB, false, false);

        assert!(apply_rendering_traits(&mut manifest, &profile).is_ok());

        let dir = manifest.traits.get(&TraitKey::PrimaryDirection);
        assert_eq!(dir, Some(&TraitValue::Direction(Direction::TTB)));
    }
}
