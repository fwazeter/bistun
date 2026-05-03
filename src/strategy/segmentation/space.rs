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

//! # Space-Based Segmentation Strategy
//! Ref: [009-LMS-STRAT]
//! Location: `src/strategy/segmentation/space.rs`
//!
//! **Why**: This module provides the concrete execution logic for locales that separate tokens using standard Unicode whitespace (e.g., English, Spanish).
//! **Impact**: If this algorithm fails, NLP pipelines for Latin, Cyrillic, and similar scripts will receive unsegmented blocks of text, causing search indexers and syntax highlighters to fail.
//!
//! ### Glossary
//! * **Whitespace Segmentation**: The process of dividing text into lexical tokens using space characters as boundaries.
//! * **Delimiter**: The character used to join the resulting tokens for string-based transport (we use `|`).

use crate::models::{CapabilityManifest, LmsError, TraitKey, TraitValue};
use crate::strategy::ILinguisticStrategy;

/// Implements standard whitespace segmentation for applicable locales.
pub struct SpaceSegmenter;

impl ILinguisticStrategy for SpaceSegmenter {
    /// Executes the whitespace segmentation strategy on the input text.
    ///
    /// Time: O(N) | Space: O(N) where N is the length of the input string.
    ///
    /// # Logic Trace (Internal)
    /// 1. **Validation**: Extract the `SEGMENTATION_STRATEGY` from the manifest.
    /// 2. **Integrity Check**: Ensure the manifest actually mandates `SPACE` segmentation. If not, return an error.
    /// 3. **Core Transformation**: Split the input string by Unicode whitespace.
    /// 4. **Return**: Collect the tokens and join them with a `|` delimiter for safe transport.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::strategy::ILinguisticStrategy;
    /// use bistun::strategy::segmentation::SpaceSegmenter;
    /// use bistun::models::{CapabilityManifest, TraitKey, TraitValue, SegType};
    ///
    /// let segmenter = SpaceSegmenter;
    /// let mut manifest = CapabilityManifest::new("en-US".to_string());
    /// manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
    ///
    /// let result = segmenter.execute("Hello world", &manifest).unwrap();
    /// assert_eq!(result, "Hello|world");
    /// ```
    ///
    /// # Arguments
    /// * `input` (&str): The raw sentence or paragraph to be tokenized.
    /// * `context` (&CapabilityManifest): The resolved linguistic DNA verifying this is the correct strategy.
    ///
    /// # Returns
    /// * `Result<String, LmsError>`: A pipe-delimited string of tokens.
    ///
    /// # Golden I/O
    /// * **Input**: "The quick brown fox"
    /// * **Output**: `Ok("The|quick|brown|fox")`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**:
    ///   * [`LmsError::MissingTrait`] if the `SEGMENTATION_STRATEGY` is missing.
    ///   * [`LmsError::StrategyExecutionFailure`] if the manifest mandates a strategy other than `SPACE`.
    /// * **Panics**: None. Safe UTF-8 handling guaranteed by Rust standard library.
    /// * **Safety**: Safe.
    fn execute(&self, input: &str, context: &CapabilityManifest) -> Result<String, LmsError> {
        // [STEP 1 & 2]: Verify the manifest mandates SPACE segmentation.
        match context.traits.get(&TraitKey::SegmentationStrategy) {
            Some(TraitValue::SegType(seg_type)) => {
                if *seg_type != crate::models::SegType::SPACE {
                    return Err(LmsError::StrategyExecutionFailure {
                        pipeline_step: "Phase 4: Execution".to_string(),
                        context: context.resolved_locale.clone(),
                        reason: format!(
                            "SpaceSegmenter invoked but manifest requires {:?}",
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

        // [STEP 3]: Split by standard whitespace and filter out empty strings (handling multiple spaces).
        let tokens: Vec<&str> = input.split_whitespace().collect();

        // [STEP 4]: Join and return.
        Ok(tokens.join("|"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Removed Direction and MorphType to satisfy clippy
    use crate::models::SegType;

    /// Helper to create a Golden Set English manifest for hermetic testing.
    fn mock_english_manifest() -> CapabilityManifest {
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
        manifest
    }

    #[test]
    fn test_space_segmentation_golden_path() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate segmenter and English manifest.
        // [STEP 2]: Execute: Pass a standard English sentence.
        // [STEP 3]: Assert: Verify tokens are pipe-delimited correctly.
        let segmenter = SpaceSegmenter;
        let manifest = mock_english_manifest();

        let result = segmenter.execute("The quick brown fox", &manifest).unwrap();
        assert_eq!(result, "The|quick|brown|fox");
    }

    #[test]
    fn test_space_segmentation_handles_multiple_spaces() {
        let segmenter = SpaceSegmenter;
        let manifest = mock_english_manifest();

        let result = segmenter.execute("Hello    world\t\nRust", &manifest).unwrap();
        assert_eq!(result, "Hello|world|Rust");
    }

    #[test]
    fn test_fails_on_invalid_manifest_strategy() {
        // Prove that the strategy rejects manifests meant for other algorithms (e.g., Thai Dictionary).
        let segmenter = SpaceSegmenter;
        let mut manifest = CapabilityManifest::new("th-TH".to_string());
        manifest
            .traits
            .insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::DICTIONARY));

        let result = segmenter.execute("สวัสดีชาวโลก", &manifest);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("SpaceSegmenter invoked but manifest requires DICTIONARY"));
    }
}
