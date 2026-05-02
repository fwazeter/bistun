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
//! Location: `src/validation/integrity.rs`
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
/// 1. Verify the traits dictionary is not completely empty.
/// 2. Assert that if `HasBidiElements` is true, `RequiresShaping` MUST also be true.
/// 3. Yield success or bubble up an `IntegrityViolation`.
///
/// # Examples
/// ```rust
///   let mut manifest = CapabilityManifest::new("ar-EG".to_string());
///   manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
///   manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
///   assert!(verify(&manifest).is_ok());
/// ```
///
/// # Arguments
/// * `manifest` (&CapabilityManifest): The fully aggregated payload evaluated just before being returned to the client.
///
/// # Returns
/// * `Result<(), LmsError>`: `Ok(())` if the manifest passes Level C lightweight checks.
///
/// # Golden I/O
/// * **Input**: `CapabilityManifest` with `HasBidiElements: true` and `RequiresShaping: false`
/// * **Output**: `Err(LmsError::IntegrityViolation("..."))`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns [`LmsError::IntegrityViolation`] if the dictionary is empty or mechanical contradictions exist.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous verification.
pub fn verify(manifest: &CapabilityManifest) -> Result<(), LmsError> {
    // [STEP 1]: Data Presence
    if manifest.traits.is_empty() {
        return Err(LmsError::IntegrityViolation(
            "Manifest traits dictionary cannot be empty".to_string(),
        ));
    }

    // [STEP 2]: Mechanical Consistency
    let has_bidi =
        manifest.traits.get(&TraitKey::HasBidiElements) == Some(&TraitValue::Boolean(true));
    let requires_shaping =
        manifest.traits.get(&TraitKey::RequiresShaping) == Some(&TraitValue::Boolean(true));

    // [STEP 3]: Contradiction check with Anomaly Exception [Ref: 003-LMS-VAL]
    // Hebrew is Bidi-capable but does not strictly require complex shaping logic.
    if has_bidi && !requires_shaping && manifest.resolved_locale != "he" {
        return Err(LmsError::IntegrityViolation(
            "Bidirectional layouts inherently require shaping algorithms, but REQUIRES_SHAPING is false".to_string(),
        ));
    }

    // [STEP 3]: Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_passes_valid_manifest() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Hydrate a valid manifest.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));

        // [STEP 2] & [STEP 3]: Execute & Assert: The check passes.
        assert!(verify(&manifest).is_ok());
    }

    #[test]
    fn test_verify_catches_empty_manifest() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate an empty manifest.
        let manifest = CapabilityManifest::new("en-US".to_string());

        // [STEP 2]: Execute: Run the verifier.
        let err = verify(&manifest).unwrap_err();

        // [STEP 3]: Assert: It throws an IntegrityViolation.
        assert!(matches!(err, LmsError::IntegrityViolation(_)));
    }

    #[test]
    fn test_verify_catches_bidi_shaping_contradiction() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Hydrate a manifest with a mechanical contradiction.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));

        // [STEP 2]: Execute: Run the verifier.
        let err = verify(&manifest).unwrap_err();

        // [STEP 3]: Assert: It throws an IntegrityViolation.
        assert!(matches!(err, LmsError::IntegrityViolation(_)));
    }
}
