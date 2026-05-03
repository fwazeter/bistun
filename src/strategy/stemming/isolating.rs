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

//! # Isolating Morphology Stemmer
//! Ref: [009-LMS-STRAT]
//! Location: `src/strategy/stemming/isolating.rs`
//!
//! **Why**: This module provides execution logic for locales with isolating morphology (e.g., Vietnamese, Yoruba), where words do not undergo inflectional affixation.
//! **Impact**: If this algorithm fails, search indexers might incorrectly attempt to strip characters from isolating languages, destroying the lexical meaning of the text.
//!
//! ### Glossary
//! * **Isolating Morphology**: A linguistic typology where words consist of a single morpheme and do not use affixes (prefixes/suffixes) to express grammatical relationships.
//! * **Stemming Bypass**: Since isolating words are already "stems," this strategy acts as a No-Op transformation.

use crate::models::{CapabilityManifest, LmsError, TraitKey, TraitValue};
use crate::strategy::ILinguisticStrategy;

/// Implements a bypass stemming strategy for isolating languages.
pub struct IsolatingStemmer;

impl ILinguisticStrategy for IsolatingStemmer {
    /// Executes the isolating stemming strategy on the input token.
    ///
    /// Time: O(N) | Space: O(N) where N is the length of the input token due to String allocation.
    ///
    /// # Logic Trace (Internal)
    /// 1. **Validation**: Extract the `MORPHOLOGY_TYPE` from the manifest.
    /// 2. **Integrity Check**: Ensure the manifest explicitly mandates `ISOLATING` morphology.
    /// 3. **Core Transformation**: Bypass transformation (words are already stems).
    /// 4. **Return**: Return the input string unaltered.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::strategy::ILinguisticStrategy;
    /// use bistun::strategy::stemming::IsolatingStemmer;
    /// use bistun::models::{CapabilityManifest, TraitKey, TraitValue, MorphType};
    ///
    /// let stemmer = IsolatingStemmer;
    /// let mut manifest = CapabilityManifest::new("vi-VN".to_string());
    /// manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::ISOLATING));
    ///
    /// let result = stemmer.execute("chó", &manifest).unwrap();
    /// assert_eq!(result, "chó");
    /// ```
    ///
    /// # Arguments
    /// * `input` (&str): The raw lexical token to be stemmed.
    /// * `context` (&CapabilityManifest): The resolved linguistic DNA verifying this is the correct strategy.
    ///
    /// # Returns
    /// * `Result<String, LmsError>`: The exact string that was provided.
    ///
    /// # Golden I/O
    /// * **Input**: "sách"
    /// * **Output**: `Ok("sách")`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**:
    ///   * [`LmsError::MissingTrait`] if the strategy is missing.
    ///   * [`LmsError::StrategyExecutionFailure`] if the manifest mandates a non-ISOLATING strategy.
    fn execute(&self, input: &str, context: &CapabilityManifest) -> Result<String, LmsError> {
        // [STEP 1 & 2]: Verify the manifest mandates ISOLATING morphology.
        match context.traits.get(&TraitKey::MorphologyType) {
            Some(TraitValue::MorphType(morph_type)) => {
                if *morph_type != crate::models::MorphType::ISOLATING {
                    return Err(LmsError::StrategyExecutionFailure {
                        pipeline_step: "Phase 4: Execution".to_string(),
                        context: context.resolved_locale.clone(),
                        reason: format!(
                            "IsolatingStemmer invoked but manifest requires {:?}",
                            morph_type
                        ),
                    });
                }
            }
            _ => {
                return Err(LmsError::MissingTrait {
                    pipeline_step: "Phase 4: Execution".to_string(),
                    trait_key: "MORPHOLOGY_TYPE".to_string(),
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
    use crate::models::MorphType;

    fn mock_isolating_manifest() -> CapabilityManifest {
        let mut manifest = CapabilityManifest::new("vi-VN".to_string());
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::ISOLATING));
        manifest
    }

    #[test]
    fn test_isolating_stemmer_golden_path() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate stemmer and manifest.
        // [STEP 2]: Execute: Pass an isolating language word.
        // [STEP 3]: Assert: Verify the word is returned completely unaltered.
        let stemmer = IsolatingStemmer;
        let manifest = mock_isolating_manifest();

        let result = stemmer.execute("người", &manifest).unwrap();
        assert_eq!(result, "người");
    }

    #[test]
    fn test_isolating_fails_on_agglutinative_manifest() {
        let stemmer = IsolatingStemmer;
        let mut manifest = CapabilityManifest::new("tr-TR".to_string());
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::AGGLUTINATIVE));

        let result = stemmer.execute("evler", &manifest);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("manifest requires AGGLUTINATIVE"));
    }
}
