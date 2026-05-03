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

//! # Dictionary-Based Segmentation Strategy
//! Ref: [009-LMS-STRAT]
//! Location: `src/strategy/segmentation/dictionary.rs`
//!
//! **Why**: This module provides execution logic for scripts that lack visual word boundaries (e.g., Thai, Khmer) and require lexical analysis against a known lexicon.
//! **Impact**: If this algorithm fails, these languages cannot be indexed by search engines or parsed by NLP models, as the entire paragraph will be treated as a single token.
//!
//! ### Glossary
//! * **Dictionary Segmentation**: Tokenization achieved by matching substrings against a Trie or finite-state automaton of known lexical roots.
//! * **Maximal Matching**: A greedy algorithm strategy often used in dictionary segmentation to find the longest possible valid word.

use crate::models::{CapabilityManifest, LmsError, TraitKey, TraitValue};
use crate::strategy::ILinguisticStrategy;

/// Implements dictionary-based lexical segmentation.
pub struct DictionarySegmenter;

impl ILinguisticStrategy for DictionarySegmenter {
    /// Executes the dictionary segmentation strategy on the input text.
    ///
    /// Time: O(N) | Space: O(N) (Target latency upon Flyweight hydration)
    ///
    /// # Logic Trace (Internal)
    /// 1. **Validation**: Extract the `SEGMENTATION_STRATEGY` from the manifest.
    /// 2. **Integrity Check**: Ensure the manifest explicitly mandates `DICTIONARY` segmentation.
    /// 3. **Core Transformation**: Suspend execution until `010-LMS-MEM` Flyweight pools are hydrated.
    /// 4. **Return**: Bubble up a graceful `LmsError`.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::strategy::ILinguisticStrategy;
    /// use bistun::strategy::segmentation::DictionarySegmenter;
    /// use bistun::models::{CapabilityManifest, TraitKey, TraitValue, SegType};
    ///
    /// let segmenter = DictionarySegmenter;
    /// let mut manifest = CapabilityManifest::new("th-TH".to_string());
    /// manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::DICTIONARY));
    ///
    /// // Currently returns an expected failure until the Flyweight dictionary is injected.
    /// let result = segmenter.execute("สวัสดีชาวโลก", &manifest);
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Arguments
    /// * `input` (&str): The raw continuous text block to be tokenized.
    /// * `context` (&CapabilityManifest): The resolved linguistic DNA verifying this is the correct strategy.
    ///
    /// # Returns
    /// * `Result<String, LmsError>`: A pipe-delimited string of tokens, or an execution failure.
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**:
    ///   * [`LmsError::MissingTrait`] if the strategy is missing.
    ///   * [`LmsError::StrategyExecutionFailure`] if the dictionary pool is not hydrated.
    fn execute(&self, _input: &str, context: &CapabilityManifest) -> Result<String, LmsError> {
        // [STEP 1 & 2]: Verify the manifest mandates DICTIONARY segmentation.
        match context.traits.get(&TraitKey::SegmentationStrategy) {
            Some(TraitValue::SegType(seg_type)) => {
                if *seg_type != crate::models::SegType::DICTIONARY {
                    return Err(LmsError::StrategyExecutionFailure {
                        pipeline_step: "Phase 4: Execution".to_string(),
                        context: context.resolved_locale.clone(),
                        reason: format!(
                            "DictionarySegmenter invoked but manifest requires {:?}",
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

        // [STEP 3 & 4]: Core Transformation (Pending Flyweight Hydration)
        // To maintain <1ms latency, Dictionary segmentation requires a pre-compiled
        // Trie/Automaton injected from the `RegistryStore` ([010-LMS-MEM]).
        Err(LmsError::StrategyExecutionFailure {
            pipeline_step: "Phase 4: Execution".to_string(),
            context: "Dictionary Trie".to_string(),
            reason: "Dictionary Flyweight pool not yet hydrated. Core algorithm suspended."
                .to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SegType;

    fn mock_dictionary_manifest() -> CapabilityManifest {
        let mut manifest = CapabilityManifest::new("th-TH".to_string());
        manifest
            .traits
            .insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::DICTIONARY));
        manifest
    }

    #[test]
    fn test_dictionary_fails_gracefully_without_flyweight() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate segmenter and Thai manifest.
        // [STEP 2]: Execute: Pass a Thai string.
        // [STEP 3]: Assert: Verify the execution traps gracefully, waiting for 010-LMS-MEM.
        let segmenter = DictionarySegmenter;
        let manifest = mock_dictionary_manifest();

        let result = segmenter.execute("สวัสดีชาวโลก", &manifest);

        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains("Dictionary Flyweight pool not yet hydrated")
        );
    }

    #[test]
    fn test_dictionary_fails_on_invalid_manifest() {
        let segmenter = DictionarySegmenter;
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));

        let result = segmenter.execute("Test", &manifest);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("manifest requires SPACE"));
    }
}
