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

//! # Character-Based Segmentation Strategy
//! Ref: [009-LMS-STRAT]
//! Location: `src/strategy/segmentation/character.rs`
//!
//! **Why**: This module provides execution logic for scripts where text processing algorithms require evaluation character-by-character rather than by lexical words.
//! **Impact**: If this algorithm fails, NLP pipelines expecting discrete character tokens will process unsegmented strings, causing morphological analysis to fail.
//!
//! ### Glossary
//! * **Character Segmentation**: The process of dividing text into individual Unicode characters (scalars).

use crate::models::{CapabilityManifest, LmsError, TraitKey, TraitValue};
use crate::strategy::ILinguisticStrategy;

/// Implements character-by-character segmentation.
pub struct CharacterSegmenter;

impl ILinguisticStrategy for CharacterSegmenter {
    /// Executes the character segmentation strategy on the input text.
    ///
    /// Time: O(N) | Space: O(N) where N is the length of the input string.
    ///
    /// # Logic Trace (Internal)
    /// 1. **Validation**: Extract the `SEGMENTATION_STRATEGY` from the manifest.
    /// 2. **Integrity Check**: Ensure the manifest mandates `CHARACTER` segmentation.
    /// 3. **Core Transformation**: Iterate over the string's Unicode scalar values.
    /// 4. **Return**: Collect the characters and join them with a `|` delimiter.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::strategy::ILinguisticStrategy;
    /// use bistun::strategy::segmentation::CharacterSegmenter;
    /// use bistun::models::{CapabilityManifest, TraitKey, TraitValue, SegType};
    ///
    /// let segmenter = CharacterSegmenter;
    /// let mut manifest = CapabilityManifest::new("zh-Hant".to_string());
    /// manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::CHARACTER));
    ///
    /// let result = segmenter.execute("測試", &manifest).unwrap();
    /// assert_eq!(result, "測|試");
    /// ```
    ///
    /// # Arguments
    /// * `input` (&str): The raw text to be tokenized.
    /// * `context` (&CapabilityManifest): The resolved linguistic DNA verifying this is the correct strategy.
    ///
    /// # Returns
    /// * `Result<String, LmsError>`: A pipe-delimited string of characters.
    ///
    /// # Golden I/O
    /// * **Input**: "Test"
    /// * **Output**: `Ok("T|e|s|t")`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**:
    ///   * [`LmsError::MissingTrait`] if the strategy is missing.
    ///   * [`LmsError::StrategyExecutionFailure`] if the manifest mandates a non-CHARACTER strategy.
    fn execute(&self, input: &str, context: &CapabilityManifest) -> Result<String, LmsError> {
        // [STEP 1 & 2]: Verify the manifest mandates CHARACTER segmentation.
        match context.traits.get(&TraitKey::SegmentationStrategy) {
            Some(TraitValue::SegType(seg_type)) => {
                if *seg_type != crate::models::SegType::CHARACTER {
                    return Err(LmsError::StrategyExecutionFailure {
                        pipeline_step: "Phase 4: Execution".to_string(),
                        context: context.resolved_locale.clone(),
                        reason: format!(
                            "CharacterSegmenter invoked but manifest requires {:?}",
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

        // [STEP 3]: Extract characters.
        let tokens: Vec<String> = input.chars().map(|c| c.to_string()).collect();

        // [STEP 4]: Join and return.
        Ok(tokens.join("|"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SegType;

    fn mock_character_manifest() -> CapabilityManifest {
        let mut manifest = CapabilityManifest::new("zh-Hant".to_string());
        manifest
            .traits
            .insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::CHARACTER));
        manifest
    }

    #[test]
    fn test_character_segmentation_golden_path() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate segmenter and manifest.
        // [STEP 2]: Execute: Pass a string.
        // [STEP 3]: Assert: Verify tokens are single characters.
        let segmenter = CharacterSegmenter;
        let manifest = mock_character_manifest();

        let result = segmenter.execute("測試", &manifest).unwrap();
        assert_eq!(result, "測|試");
    }

    #[test]
    fn test_character_fails_on_space_manifest() {
        let segmenter = CharacterSegmenter;
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));

        let result = segmenter.execute("Test", &manifest);
        assert!(result.is_err());
    }
}
