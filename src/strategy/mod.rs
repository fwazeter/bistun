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

//! # Strategy Pattern Registry
//! Ref: [009-LMS-STRAT]
//! Location: `src/strategy/mod.rs`
//!
//! **Why**: This module decouples linguistic metadata (the "What") from algorithmic execution (the "How"). It allows the SDK to swap stemming, segmentation, and rendering logic at runtime based on the 5-phase `CapabilityManifest`.
//! **Impact**: If this module fails, downstream consuming services (like Search or UI rendering) will receive metadata but will have no executable algorithms to process text, halting core linguistic capabilities.
//!
//! ### Glossary
//! * **ILinguisticStrategy**: The universal interface for all linguistic algorithms.
//! * **Provider**: A factory that inspects a `CapabilityManifest` and returns the appropriate strategy (e.g., `StemmingProvider`).

pub mod segmentation;
pub mod stemming;

// Re-export concrete strategies for external use if needed
pub use segmentation::space::SpaceSegmenter;
pub use stemming::isolating::IsolatingStemmer;

use crate::models::{CapabilityManifest, LmsError, SegType, TraitKey, TraitValue};

use crate::strategy::segmentation::character::CharacterSegmenter;
use crate::strategy::segmentation::dictionary::DictionarySegmenter;
use crate::strategy::segmentation::none::NoneSegmenter;
#[cfg(test)]
use mockall::{automock, predicate::*};

/// Universal interface for all executable linguistic algorithms.
///
/// Time: O(1) (Interface Definition) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Ingest the raw input string and the `CapabilityManifest` context.
/// 2. Apply the algorithm (e.g., suffix-stripping, dictionary lookup) to the input string.
/// 3. Return the transformed string or an appropriate `LmsError`.
#[cfg_attr(test, automock)]
pub trait ILinguisticStrategy {
    /// Executes the linguistic transformation on the input string based on the provided manifest traits.
    ///
    /// # Arguments
    /// * `input` (&str): The raw text string requiring linguistic processing (e.g., stemming or segmentation).
    /// * `context` (&CapabilityManifest): The resolved linguistic DNA dictating *how* the text should be processed.
    ///
    /// # Returns
    /// * `Result<String, LmsError>`: The transformed string (e.g., the stem or segmented tokens).
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: Returns specific variants of [`LmsError`] (e.g., `LmsError::StrategyExecutionFailure`) if the algorithm fails to process the input.
    /// * **Panics**: Must never panic; all internal parsing errors must be bubbled up as `LmsError`.
    /// * **Safety**: Safe. No unsafe blocks permitted in strategy execution.
    fn execute(&self, input: &str, context: &CapabilityManifest) -> Result<String, LmsError>;
}

/// Factory for retrieving the correct Stemming Strategy based on a Locale's Morphology.
///
/// Time: O(1) | Space: O(1)
pub struct StemmingProvider;

// Find the StemmingProvider and update it:
impl StemmingProvider {
    /// Retrieves the appropriate stemming strategy.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::strategy::StemmingProvider;
    /// use bistun::models::{CapabilityManifest, TraitKey, TraitValue, MorphType};
    ///
    /// let mut manifest = CapabilityManifest::new("vi-VN".to_string());
    /// manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::ISOLATING));
    ///
    /// let strategy = StemmingProvider::get(&manifest).unwrap();
    /// let stem = strategy.execute("sách", &manifest).unwrap();
    /// assert_eq!(stem, "sách");
    /// ```
    ///
    /// # Arguments
    /// * `context` (&CapabilityManifest): The resolved manifest containing the `MORPHOLOGY_TYPE`.
    ///
    /// # Returns
    /// * `Result<Box<dyn ILinguisticStrategy>, LmsError>`: A boxed, executable strategy.
    ///
    /// # Golden I/O
    /// * **Input**: Manifest with `MORPHOLOGY_TYPE` = `ISOLATING`
    /// * **Output**: `Ok(Box<IsolatingStemmer>)`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: Returns [`LmsError::MissingTrait`] if the manifest lacks a morphology type.
    pub fn get(context: &CapabilityManifest) -> Result<Box<dyn ILinguisticStrategy>, LmsError> {
        // [STEP 1]: Inspect the morphology type from the manifest.
        let morph_type = match context.traits.get(&TraitKey::MorphologyType) {
            Some(TraitValue::MorphType(mt)) => mt,
            _ => {
                return Err(LmsError::MissingTrait {
                    pipeline_step: "Phase 4: Execution".to_string(),
                    trait_key: "MORPHOLOGY_TYPE".to_string(),
                    reason: "Manifest lacks a valid MorphType for stemming routing".to_string(),
                });
            }
        };

        // [STEP 2]: Map the type to a strategy.
        // [STEP 3]: Return the strategy.
        match morph_type {
            crate::models::MorphType::ISOLATING => {
                Ok(Box::new(stemming::isolating::IsolatingStemmer))
            }
            crate::models::MorphType::AGGLUTINATIVE => {
                unimplemented!("Agglutinative stemmer not yet implemented")
            }
            crate::models::MorphType::FUSIONAL => {
                unimplemented!("Fusional stemmer not yet implemented")
            }
            crate::models::MorphType::TEMPLATIC => {
                unimplemented!("Templatic stemmer not yet implemented")
            }
            crate::models::MorphType::POLYSYNTHETIC => {
                unimplemented!("Polysynthetic stemmer not yet implemented")
            }
        }
    }
}

/// Factory for retrieving the correct Segmentation Strategy based on a Locale's script mechanics.
///
/// Time: O(1) | Space: O(1) (Returns Boxed allocation; targeted for Flyweight optimization in [010-LMS-MEM])
///
/// # Logic Trace (Internal)
/// 1. Inspect the `SEGMENTATION_STRATEGY` trait within the `CapabilityManifest`.
/// 2. Map the type (e.g., `SPACE`, `DICTIONARY`) to the appropriate strategy implementation.
/// 3. Return a boxed instance of the strategy for execution.
pub struct SegmentationProvider;

impl SegmentationProvider {
    /// Retrieves the appropriate segmentation strategy.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::strategy::SegmentationProvider;
    /// use bistun::models::{CapabilityManifest, TraitKey, TraitValue, SegType};
    ///
    /// let mut manifest = CapabilityManifest::new("en-US".to_string());
    /// manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
    ///
    /// let strategy = SegmentationProvider::get(&manifest).unwrap();
    /// let tokens = strategy.execute("Hello world", &manifest).unwrap();
    ///
    /// assert_eq!(tokens, "Hello|world");
    /// ```
    ///
    /// # Arguments
    /// * `context` (&CapabilityManifest): The resolved manifest containing the `SEGMENTATION_STRATEGY`.
    ///
    /// # Returns
    /// * `Result<Box<dyn ILinguisticStrategy>, LmsError>`: A boxed, executable strategy.
    ///
    /// # Golden I/O
    /// * **Input**: Manifest with `SEGMENTATION_STRATEGY` = `SPACE`
    /// * **Output**: `Ok(Box<SpaceSegmenter>)`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: Returns [`LmsError::MissingTrait`] if the manifest lacks a segmentation strategy.
    pub fn get(context: &CapabilityManifest) -> Result<Box<dyn ILinguisticStrategy>, LmsError> {
        // [STEP 1]: Inspect the segmentation type from the manifest.
        let seg_type = match context.traits.get(&TraitKey::SegmentationStrategy) {
            Some(TraitValue::SegType(st)) => st,
            _ => {
                return Err(LmsError::MissingTrait {
                    pipeline_step: "Phase 4: Execution".to_string(),
                    trait_key: "SEGMENTATION_STRATEGY".to_string(),
                    reason: "Manifest lacks a valid SegType for segmentation routing".to_string(),
                });
            }
        };

        // [STEP 2]: Map the type to a strategy.
        // [STEP 3]: Return the strategy.
        match seg_type {
            SegType::SPACE => Ok(Box::new(SpaceSegmenter)),
            SegType::NONE => Ok(Box::new(NoneSegmenter)),
            SegType::CHARACTER => Ok(Box::new(CharacterSegmenter)),
            SegType::DICTIONARY => Ok(Box::new(DictionarySegmenter)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CapabilityManifest, SegType, TraitKey, TraitValue};

    #[test]
    fn test_mock_linguistic_strategy_execution() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate the mock strategy and define its strict expectations.
        // [STEP 2]: Execute: Call the mocked method with the expected parameters.
        // [STEP 3]: Assert: Verify the mock returns the expected "run" string.

        let mut mock_strategy = MockILinguisticStrategy::new();

        mock_strategy
            .expect_execute()
            .with(eq("running"), always())
            .times(1)
            .returning(|_, _| Ok("run".to_string()));

        // We must actually trigger the execution to satisfy `times(1)`
        let dummy_manifest = CapabilityManifest::new("en-US".to_string());
        let result = mock_strategy.execute("running", &dummy_manifest).unwrap();

        assert_eq!(result, "run");
    }

    #[test]
    fn test_segmentation_provider_routes_space_correctly() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate a manifest mandating SPACE segmentation.
        // [STEP 2]: Execute: Retrieve the strategy via SegmentationProvider.
        // [STEP 3]: Assert: Verify the returned strategy correctly splits whitespace.

        let mut manifest = CapabilityManifest::new("en-US".to_string());
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));

        let strategy = SegmentationProvider::get(&manifest).unwrap();

        let result = strategy.execute("A test string", &manifest).unwrap();
        assert_eq!(result, "A|test|string");
    }

    #[test]
    fn test_segmentation_provider_fails_on_missing_trait() {
        // Prove that an incomplete manifest correctly triggers an LmsError.
        let manifest = CapabilityManifest::new("und".to_string()); // No traits inserted

        let result = SegmentationProvider::get(&manifest);

        // We use a match statement here to extract the error safely because
        // calling .unwrap_err() requires the success type (Box<dyn ILinguisticStrategy>)
        // to implement Debug, which it does not.
        match result {
            Err(e) => {
                assert!(e.to_string().contains("Missing Trait (SEGMENTATION_STRATEGY)"));
            }
            Ok(_) => panic!("Expected a MissingTrait error, but provider returned Ok"),
        }
    }
}
