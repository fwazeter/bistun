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

//! # Global Error Definitions
//! Ref: [LMS-PROCESS-ERROR]
//! Location: `src/models/error.rs`
//!
//! **Why**: This module provides a unified, strongly-typed error enumeration using the `thiserror` framework. It ensures all 5-phase pipeline failures are context-aware and bubble up gracefully to the telemetry phase.
//! **Impact**: If error contexts are lost or poorly formatted, observability sinks will lack actionable data, violating the system's operational SLAs.
//!
//! ### Glossary
//! * **thiserror**: The framework used to derive descriptive, strongly-typed errors.
//! * **Failure Narrative**: The standard ensuring every error explains What failed, Where it failed, and Why it failed.

use thiserror::Error;

/// The universal error type for the Bistun LMS pipeline.
///
/// Time: O(1) (Definition) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Associates standard pipeline failures with explicit context fields.
/// 2. Derives standard `std::error::Error` and `std::fmt::Display` via `thiserror`.
///
/// # Examples
/// ```rust
/// use bistun::models::LmsError;
///
/// let err = LmsError::MissingTrait {
///     pipeline_step: "Phase 2: Aggregation".to_string(),
///     trait_key: "MORPHOLOGY_TYPE".to_string(),
///     reason: "Golden Set baseline breached".to_string(),
/// };
///
/// assert_eq!(
///     err.to_string(),
///     "[Phase 2: Aggregation] Missing Trait (MORPHOLOGY_TYPE): Golden Set baseline breached"
/// );
/// ```
#[derive(Error, Debug)]
pub enum LmsError {
    /// Raised when a linguistic algorithm fails to process the input string.
    #[error("[{pipeline_step}] Strategy Execution Failure ({context}): {reason}")]
    StrategyExecutionFailure {
        /// Where did it fail? (e.g., "Phase 4: Stemming Strategy")
        pipeline_step: String,
        /// What failed? (e.g., the input word or language tag)
        context: String,
        /// Why did it fail? (e.g., "Invalid UTF-8 sequence")
        reason: String,
    },

    /// Raised when the `CapabilityManifest` lacks a required trait during resolution.
    #[error("[{pipeline_step}] Missing Trait ({trait_key}): {reason}")]
    MissingTrait {
        /// Where did it fail? (e.g., "Phase 2: Aggregation")
        pipeline_step: String,
        /// What failed? (e.g., "PRIMARY_DIRECTION")
        trait_key: String,
        /// Why did it fail? (e.g., "Script Definition lacks fallback")
        reason: String,
    },

    /// Raised when a BCP 47 tag is mathematically invalid or unparseable.
    #[error("[{pipeline_step}] Invalid Tag ({tag}): {reason}")]
    InvalidTag {
        /// Where did it fail? (e.g., "Phase 1: Taxonomic Resolution")
        pipeline_step: String,
        /// What failed? (The raw tag, e.g., "en-US-u-foo-bar")
        tag: String,
        /// Why did it fail? (e.g., "Malformed Unicode extension subtag")
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_narrative_formatting() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate a StrategyExecutionFailure error variant.
        // [STEP 2]: Execute: Format the error via the derived Display trait.
        // [STEP 3]: Assert: Verify the string output matches the LMS-PROCESS-ERROR "Failure Narrative" (What, Where, Why).

        let error = LmsError::StrategyExecutionFailure {
            pipeline_step: "Phase 4: Strategy Execution".to_string(),
            context: "input_word".to_string(),
            reason: "Agglutinative stemmer encountered invalid suffix".to_string(),
        };

        let output = error.to_string();

        assert_eq!(
            output,
            "[Phase 4: Strategy Execution] Strategy Execution Failure (input_word): Agglutinative stemmer encountered invalid suffix"
        );
    }
}
