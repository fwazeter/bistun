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

//! # No-Op Segmentation Strategy
//! Ref: [009-LMS-STRAT]
//! Location: `src/strategy/segmentation/none.rs`
//!
//! **Why**: This module provides execution logic for locales or specific pipeline configurations that explicitly require text to remain unsegmented (a No-Op).
//! **Impact**: If this algorithm fails, downstream systems expecting a raw, unaltered string might receive incorrectly tokenized data, breaking their internal parsers.
//!
//! ### Glossary
//! * **No-Op (No Operation)**: A design pattern where the function executes successfully but intentionally makes no changes to the underlying data.

use crate::models::{CapabilityManifest, LmsError, TraitKey, TraitValue};
use crate::strategy::ILinguisticStrategy;

/// Implements a bypass (No-Op) segmentation strategy.
pub struct NoneSegmenter;

impl ILinguisticStrategy for NoneSegmenter {
    /// Executes the No-Op segmentation strategy on the input text.
    ///
    /// Time: O(N) | Space: O(N) where N is the length of the input string due to String allocation.
    ///
    /// # Logic Trace (Internal)
    /// 1. **Validation**: Extract the `SEGMENTATION_STRATEGY` from the manifest.
    /// 2. **Integrity Check**: Ensure the manifest explicitly mandates `NONE` segmentation.
    /// 3. **Core Transformation**: Bypass transformation.
    /// 4. **Return**: Return the input string unaltered.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::strategy::ILinguisticStrategy;
    /// use bistun::strategy::segmentation::NoneSegmenter;
    /// use bistun::models::{CapabilityManifest, TraitKey, TraitValue, SegType};
    ///
    /// let segmenter = NoneSegmenter;
    /// let mut manifest = CapabilityManifest::new("und".to_string());
    /// manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::NONE));
    ///
    /// let result = segmenter.execute("Unsegmented text block", &manifest).unwrap();
    /// assert_eq!(result, "Unsegmented text block");
    /// ```
    ///
    /// # Arguments
    /// * `input` (&str): The raw text to be bypassed.
    /// * `context` (&CapabilityManifest): The resolved linguistic DNA verifying this is the correct strategy.
    ///
    /// # Returns
    /// * `Result<String, LmsError>`: The exact string that was provided.
    ///
    /// # Golden I/O
    /// * **Input**: "Hello World"
    /// * **Output**: `Ok("Hello World")`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**:
    ///   * [`LmsError::MissingTrait`] if the strategy is missing.
    ///   * [`LmsError::StrategyExecutionFailure`] if the manifest mandates a non-NONE strategy.
    fn execute(&self, input: &str, context: &CapabilityManifest) -> Result<String, LmsError> {
        // [STEP 1 & 2]: Verify the manifest mandates NONE segmentation.
        match context.traits.get(&TraitKey::SegmentationStrategy) {
            Some(TraitValue::SegType(seg_type)) => {
                if *seg_type != crate::models::SegType::NONE {
                    return Err(LmsError::StrategyExecutionFailure {
                        pipeline_step: "Phase 4: Execution".to_string(),
                        context: context.resolved_locale.clone(),
                        reason: format!(
                            "NoneSegmenter invoked but manifest requires {:?}",
                            seg_type
                        ),
                    });
                }
            }
            _ => {
                return Err(LmsError::MissingTrait {
                    pipeline_step: "Phase 4: Execution".to_string(),
                    trait_key: "SEGMENTATION_STRATEGY".to_string(),
                    reason: "Trait missing or invalid type in manifest".to_string(),
                });
            }
        }

        // [STEP 3 & 4]: Return unaltered string.
        Ok(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SegType;

    fn mock_none_manifest() -> CapabilityManifest {
        let mut manifest = CapabilityManifest::new("und".to_string());
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::NONE));
        manifest
    }

    #[test]
    fn test_none_segmentation_golden_path() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate segmenter and manifest.
        // [STEP 2]: Execute: Pass a string.
        // [STEP 3]: Assert: Verify tokens are completely unaltered.
        let segmenter = NoneSegmenter;
        let manifest = mock_none_manifest();

        let result = segmenter.execute("Do not touch this text.", &manifest).unwrap();
        assert_eq!(result, "Do not touch this text.");
    }

    #[test]
    fn test_none_fails_on_character_manifest() {
        let segmenter = NoneSegmenter;
        let mut manifest = CapabilityManifest::new("zh-Hant".to_string());
        manifest
            .traits
            .insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::CHARACTER));

        let result = segmenter.execute("Test", &manifest);
        assert!(result.is_err());
    }
}
