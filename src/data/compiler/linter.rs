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

//! # The DNA Linter (Pre-Persistence Validator)
//! Ref: [002-LMS-DATA], [003-LMS-VAL]
//! Location: `src/data/compiler/linter.rs`
//!
//! **Why**: Enforces strict Typological and Orthographic rules on linguistic profiles *before* they are written to the database.
//! **Impact**: Acts as the QA gatekeeper. Prevents human curation errors or upstream ISO/CLDR scraping bugs from corrupting the System of Record.
//!
//! ### Glossary
//! * **Linguistic Chimera**: A locale profile that contains contradictory typological or orthographic traits that cannot exist in reality.

use crate::data::store::LocaleProfile;
use crate::models::traits::{Direction, MorphType, SegType};
use std::fmt;

/// Errors that can occur during the WORM compilation process.
#[derive(Debug, PartialEq, Eq)]
pub enum CompilerError {
    TypologicalContradiction(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::TypologicalContradiction(msg) => write!(f, "DNA Linter Fault: {}", msg),
        }
    }
}

/// Evaluates a `LocaleProfile` for linguistic and mechanical impossibilities.
///
/// Time: O(1) condition checks | Space: O(1) string allocation only on error
///
/// # Logic Trace (Internal)
/// 1. **Bidi Constraints**: Ensure RTL languages inherently flag bidirectional rendering.
/// 2. **Orthographic Sanity**: Ensure Top-To-Bottom (TTB) scripts do not declare incompatible segmentation strategies.
/// 3. **Morphological Sanity**: Ensure Templatic languages do not declare continuous Character-based segmentation.
/// 4. **Return**: Yield success if the profile is linguistically sound.
///
/// # Examples
/// ```rust
///   let profile = create_valid_profile();
///   assert!(validate_profile(&profile).is_ok());
/// ```
///
/// # Arguments
/// * `profile` (&LocaleProfile): The raw profile candidate to be evaluated before WORM ingestion.
///
/// # Returns
/// * `Result<(), CompilerError>`: `Ok(())` if the profile passes all integrity checks.
///
/// # Golden I/O
/// * **Input**: `LocaleProfile { id: "ar-EG", direction: Direction::RTL, has_bidi: false, ... }`
/// * **Output**: `Err(CompilerError::TypologicalContradiction("..."))`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns [`CompilerError::TypologicalContradiction`] if the constraints matrix is violated.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn validate_profile(profile: &LocaleProfile) -> Result<(), CompilerError> {
    // [STEP 1]: Bidi / Shaping Constraints
    if profile.direction == Direction::RTL && !profile.has_bidi {
        return Err(CompilerError::TypologicalContradiction(format!(
            "[{}] is RTL but does not flag has_bidi. RTL implicitly requires Bidi context.",
            profile.id
        )));
    }

    // [STEP 2]: Orthographic Sanity
    if profile.direction == Direction::TTB && profile.base_seg == SegType::SPACE {
        return Err(CompilerError::TypologicalContradiction(format!(
            "[{}] is Top-To-Bottom but uses SPACE segmentation. TTB scripts generally require CHARACTER or DICTIONARY segmentation.",
            profile.id
        )));
    }

    // [STEP 3]: Morphological Sanity
    if profile.morph == MorphType::TEMPLATIC && profile.base_seg == SegType::CHARACTER {
        return Err(CompilerError::TypologicalContradiction(format!(
            "[{}] is TEMPLATIC but uses CHARACTER segmentation. Templatic morphology requires distinct word boundaries to extract triliteral roots.",
            profile.id
        )));
    }

    // [STEP 4]: Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_profile() -> LocaleProfile {
        LocaleProfile {
            id: "ar-EG".to_string(),
            morph: MorphType::TEMPLATIC,
            base_seg: SegType::SPACE,
            alt_seg: None,
            direction: Direction::RTL,
            has_bidi: true,
            requires_shaping: true,
            plurals: vec![
                "zero".to_string(),
                "one".to_string(),
                "two".to_string(),
                "few".to_string(),
                "many".to_string(),
                "other".to_string(),
            ],
        }
    }

    #[test]
    fn test_linter_passes_valid_profile() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create a mathematically sound linguistic profile.
        let profile = create_valid_profile();

        // [STEP 2] & [STEP 3]: Execute & Assert: Verify it passes validation.
        assert!(validate_profile(&profile).is_ok());
    }

    #[test]
    fn test_linter_catches_rtl_without_bidi() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create a profile with an RTL / no-bidi contradiction.
        let mut profile = create_valid_profile();
        profile.has_bidi = false;

        // [STEP 2]: Execute: Run validation.
        let err = validate_profile(&profile).unwrap_err();

        // [STEP 3]: Assert: Confirm it blocks compilation.
        assert!(matches!(err, CompilerError::TypologicalContradiction(_)));
    }

    #[test]
    fn test_linter_catches_ttb_space_contradiction() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create a profile with a TTB / SPACE contradiction.
        let mut profile = create_valid_profile();
        profile.id = "zh-Hant".to_string();
        profile.direction = Direction::TTB;
        profile.base_seg = SegType::SPACE;

        // [STEP 2]: Execute: Run validation.
        let err = validate_profile(&profile).unwrap_err();

        // [STEP 3]: Assert: Confirm it blocks compilation.
        assert!(matches!(err, CompilerError::TypologicalContradiction(_)));
    }

    #[test]
    fn test_linter_catches_templatic_character_contradiction() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create a profile with a Templatic / Character contradiction.
        let mut profile = create_valid_profile();
        profile.id = "invalid-templatic".to_string();
        profile.morph = MorphType::TEMPLATIC;
        profile.base_seg = SegType::CHARACTER;

        // [STEP 2]: Execute: Run validation.
        let err = validate_profile(&profile).unwrap_err();

        // [STEP 3]: Assert: Confirm it blocks compilation.
        assert!(matches!(err, CompilerError::TypologicalContradiction(_)));
    }
}
