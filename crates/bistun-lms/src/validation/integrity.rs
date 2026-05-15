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
//! Crate: `bistun-lms`
//! Ref: [003-LMS-VAL], [011-LMS-DTO], [013-LMS-RULE]
//! Location: `crates/bistun-lms/src/validation/integrity.rs`
//!
//! **Why**: This module serves as Phase 4 (Integrity Check) of the pipeline. It asserts that the fully aggregated manifest contains logical, non-contradictory instructions across all `V2.0.0` maps.
//! **Impact**: If this module fails to catch a contradiction (e.g., `Bidi` without Shaping) or missing mandatory rules, the downstream `UI` layout engine will likely panic or render unreadable text.

use bistun_core::{CapabilityManifest, LmsError, TraitKey, TraitValue};

/// Performs Level C runtime integrity checks on a hydrated capability manifest.
///
/// Time: `O(1)` map lookups | Space: `O(1)`
///
/// # Logic Trace (Internal)
/// 1. Verify the traits dictionary is not completely empty.
/// 2. Verify mandatory `DTO` contract fields exist (`PRIMARY_DIRECTION` in traits, `NORMALIZATION_DEFAULT` in rules).
/// 3. Assert that if `HasBidiElements` is true, `RequiresShaping` MUST also be true (with exceptions).
/// 4. Yield success or bubble up an `IntegrityViolation`.
///
/// # Examples
/// ```rust
/// # use bistun_core::{CapabilityManifest, TraitKey, TraitValue, Direction, LmsRule, NormRule};
/// # use bistun_lms::validation::integrity::verify;
///   let mut manifest = CapabilityManifest::new("ar-EG".to_string());
///   manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));
///   manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
///   manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
///   manifest.rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
///   assert!(verify(&manifest).is_ok());
/// ```
///
/// # Arguments
/// * `manifest` (&[`CapabilityManifest`]): The fully aggregated payload evaluated just before being returned to the client.
///
/// # Returns
/// * `Result<(), LmsError>`: `Ok(())` if the manifest passes Level C lightweight checks.
///
/// # Golden I/O
/// * **Input**: [`CapabilityManifest`] missing the `NORMALIZATION_DEFAULT` rule.
/// * **Output**: `Err(LmsError::IntegrityViolation("..."))`
///
/// # Errors
/// * Returns [`LmsError::IntegrityViolation`] if the dictionary is empty, mandatory keys are missing, or mechanical contradictions exist.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Safe synchronous verification.
pub fn verify(manifest: &CapabilityManifest) -> Result<(), LmsError> {
    // [STEP 1]: Data Presence
    if manifest.traits.is_empty() {
        return Err(LmsError::IntegrityViolation {
            pipeline_step: "Phase 4: Integrity Check".to_string(),
            context: manifest.resolved_locale.clone(),
            reason: "Manifest traits dictionary cannot be empty".to_string(),
        });
    }

    // [STEP 2]: Mandatory DTO Contract Verification (V2.0.0 Architecture)
    if !manifest.traits.contains_key(&TraitKey::PrimaryDirection) {
        return Err(LmsError::IntegrityViolation {
            pipeline_step: "Phase 4: Integrity Check".to_string(),
            context: manifest.resolved_locale.clone(),
            reason: "Missing mandatory trait: PRIMARY_DIRECTION".to_string(),
        });
    }

    if !manifest.rules.contains_key("NORMALIZATION_DEFAULT") {
        return Err(LmsError::IntegrityViolation {
            pipeline_step: "Phase 4: Integrity Check".to_string(),
            context: manifest.resolved_locale.clone(),
            reason: "Missing mandatory rule: NORMALIZATION_DEFAULT".to_string(),
        });
    }

    // [STEP 3]: Mechanical Consistency
    let has_bidi =
        manifest.traits.get(&TraitKey::HasBidiElements) == Some(&TraitValue::Boolean(true));
    let requires_shaping =
        manifest.traits.get(&TraitKey::RequiresShaping) == Some(&TraitValue::Boolean(true));

    // Contradiction check with Anomaly Exception [Ref: 003-LMS-VAL]
    // Hebrew is Bidi-capable but does not strictly require complex shaping logic.
    if has_bidi && !requires_shaping && manifest.resolved_locale != "he" {
        return Err(LmsError::IntegrityViolation {
            pipeline_step: "Phase 4: Integrity Check".to_string(),
            context: manifest.resolved_locale.clone(),
            reason: "Bidirectional layouts inherently require shaping algorithms, but REQUIRES_SHAPING is false".to_string(),
        });
    }

    // [STEP 4]: Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bistun_core::{Direction, LmsRule, NormRule};

    #[test]
    fn test_verify_passes_valid_manifest() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Hydrate a valid V2.0.0 manifest.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
        manifest.rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));

        // [STEP 2] & [STEP 3]: Execute & Assert: The check passes.
        assert!(verify(&manifest).is_ok());
    }

    #[test]
    fn test_verify_catches_empty_manifest() {
        let manifest = CapabilityManifest::new("en-US".to_string());
        let err = verify(&manifest)
            .expect_err("LMS-TEST: Expected IntegrityViolation for empty manifest");
        assert!(matches!(err, LmsError::IntegrityViolation { .. }));
    }

    #[test]
    fn test_verify_catches_missing_mandatory_rule() {
        // Missing NORMALIZATION_DEFAULT
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));

        let err =
            verify(&manifest).expect_err("LMS-TEST: Expected IntegrityViolation for missing rule");
        if let LmsError::IntegrityViolation { reason, .. } = err {
            assert!(reason.contains("NORMALIZATION_DEFAULT"));
        } else {
            panic!("Expected IntegrityViolation for missing rule");
        }
    }

    #[test]
    fn test_verify_catches_bidi_shaping_contradiction() {
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));
        manifest.rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));

        // Inject mechanical contradiction
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));

        let err = verify(&manifest)
            .expect_err("LMS-TEST: Expected IntegrityViolation for Bidi shaping contradiction");
        assert!(matches!(err, LmsError::IntegrityViolation { .. }));
    }
}
