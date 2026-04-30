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

use crate::core::resolver::bcp47::{LmsError, LocaleEntry};
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::{Direction, TraitKey};

/// Enriches and overrides the `CapabilityManifest` with Orthographic rendering mechanics.
///
/// Time: O(1) per locale lookup | Space: O(1) (In-place mutation)
///
/// # Logic Trace (Internal)
/// 1. **Ingestion**: Accept a mutable `manifest` and the canonical `locale`.
/// 2. **Script Mechanics Lookup**: Query the internal registry stub for base directional traits.
/// 3. **Extension Override**: (Future) Check the BCP 47 tag for explicit `-u-` rendering overrides and apply them if present.
/// 4. **Manifest Hydration**: Insert `PrimaryDirection` and `HasBidiElements` directly into the manifest.
/// 5. **Return**: Yield success, or bubble up an error for unsupported rendering profiles.
///
/// # Examples
/// ```rust
/// use bistun::core::extension::orthography::apply_rendering_traits;
/// use bistun::models::manifest::CapabilityManifest;
/// use bistun::resolvers::bcp47::LocaleEntry;
///
/// let mut manifest = CapabilityManifest::new("ar-EG".to_string());
/// let locale = LocaleEntry { id: "ar-EG".to_string() };
///
/// apply_rendering_traits(&mut manifest, &locale).unwrap();
/// // Manifest now contains RTL directionality and Bidi support requirements.
/// ```
///
/// # Golden I/O
/// * **Input**: `LocaleEntry { id: "ar-EG" }`
/// * **Output**: (Mutated Manifest) `PRIMARY_DIRECTION: RTL, HAS_BIDI_ELEMENTS: true`
///
/// # Errors
/// * Returns [`LmsError::ResolutionFailed`] if the locale ID is missing from the Orthography registry.
pub fn apply_rendering_traits(
    manifest: &mut CapabilityManifest,
    locale: &LocaleEntry,
) -> Result<(), LmsError> {
    // 1 & 2. Ingestion & Registry Lookup
    let (direction, has_bidi, requires_shaping) = match locale.id.as_str() {
        "ar-EG" => (Direction::RTL, true, true),
        "th-TH" => (Direction::LTR, false, true), // Requires complex grapheme cluster shaping
        "zh-Hant" => (Direction::TTB, false, false), // Supports Top-To-Bottom natively
        "ja-JP" => (Direction::LTR, false, false),
        "en-US" | "en-AU" => (Direction::LTR, false, false),
        _ => return Err(LmsError::ResolutionFailed(locale.id.clone())),
    };

    // 3. Extension Override (Stub for [004-LMS-EXT])
    // If the raw tag included `-u-` extensions that override standard orthography,
    // they would be processed and applied here before hydration.

    // 4. Manifest Hydration
    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(direction));

    manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(has_bidi));

    manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(requires_shaping));

    // 5. Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_rtl_bidi_traits() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest and RTL locale (ar-EG).
        // 2. Execute: Run orthographic mapper.
        // 3. Assert: Verify RTL and Bidi flags are correctly inserted.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let locale = LocaleEntry { id: "ar-EG".to_string() };

        assert!(apply_rendering_traits(&mut manifest, &locale).is_ok());

        let dir = manifest.traits.get(&TraitKey::PrimaryDirection);
        assert_eq!(dir, Some(&TraitValue::Direction(Direction::RTL)));

        let bidi = manifest.traits.get(&TraitKey::HasBidiElements);
        assert_eq!(bidi, Some(&TraitValue::Boolean(true)));
    }

    #[test]
    fn test_apply_ttb_traits() {
        // [Logic Trace Mapping]
        // 1. Setup: Create manifest for Traditional Chinese (zh-Hant).
        // 2. Execute: Run orthographic mapper.
        // 3. Assert: Verify Top-To-Bottom directionality is assigned.
        let mut manifest = CapabilityManifest::new("zh-Hant".to_string());
        let locale = LocaleEntry { id: "zh-Hant".to_string() };

        assert!(apply_rendering_traits(&mut manifest, &locale).is_ok());

        let dir = manifest.traits.get(&TraitKey::PrimaryDirection);
        assert_eq!(dir, Some(&TraitValue::Direction(Direction::TTB)));
    }

    #[test]
    fn test_orthography_fails_on_unknown() {
        // [Logic Trace Mapping]
        // 1. Setup: Pass an unsupported locale ID.
        // 2. Execute & Assert: Verify it correctly bubbles the error up the chain.
        let mut manifest = CapabilityManifest::new("xx-YY".to_string());
        let locale = LocaleEntry { id: "xx-YY".to_string() };

        let result = apply_rendering_traits(&mut manifest, &locale);
        assert_eq!(result, Err(LmsError::ResolutionFailed("xx-YY".to_string())));
    }
}
