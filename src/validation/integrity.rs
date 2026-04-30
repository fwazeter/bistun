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

//! # Runtime Integrity Validation
//! Ref: [003-LMS-VAL]
//!
//! **Why**: This module serves as Phase 4 (Integrity Check) of the pipeline. It asserts that the fully aggregated manifest contains logical, non-contradictory instructions.
//! **Impact**: If this module fails to catch a contradiction (e.g., Bidi without Shaping), the downstream UI layout engine will likely panic or render unreadable text.

use crate::core::resolver::bcp47::LmsError;
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::TraitKey;

/// Performs Level C runtime integrity checks on a hydrated manifest.
///
/// Time: O(1) map lookups | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. **Data Presence**: Verify the traits dictionary is not completely empty.
/// 2. **Mechanical Consistency**: Assert that if `HasBidiElements` is true, `RequiresShaping` MUST also be true (Bidirectional text layout is intrinsically complex).
/// 3. **Return**: Yield success or bubble up an `IntegrityViolation`.
pub fn verify(manifest: &CapabilityManifest) -> Result<(), LmsError> {
    // 1. Data Presence
    if manifest.traits.is_empty() {
        return Err(LmsError::IntegrityViolation(
            "Manifest traits dictionary cannot be empty".to_string(),
        ));
    }

    // 2. Mechanical Consistency
    let has_bidi =
        manifest.traits.get(&TraitKey::HasBidiElements) == Some(&TraitValue::Boolean(true));
    let requires_shaping =
        manifest.traits.get(&TraitKey::RequiresShaping) == Some(&TraitValue::Boolean(true));

    if has_bidi && !requires_shaping {
        return Err(LmsError::IntegrityViolation(
            "Bidirectional layouts inherently require shaping algorithms, but REQUIRES_SHAPING is false".to_string(),
        ));
    }

    // 3. Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_passes_valid_manifest() {
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));

        assert!(verify(&manifest).is_ok());
    }

    #[test]
    fn test_verify_catches_empty_manifest() {
        let manifest = CapabilityManifest::new("en-US".to_string());

        let err = verify(&manifest).unwrap_err();
        assert!(matches!(err, LmsError::IntegrityViolation(_)));
    }

    #[test]
    fn test_verify_catches_bidi_shaping_contradiction() {
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        // Contradiction: Claims Bidi but denies shaping
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));

        let err = verify(&manifest).unwrap_err();
        assert!(matches!(err, LmsError::IntegrityViolation(_)));
    }
}
