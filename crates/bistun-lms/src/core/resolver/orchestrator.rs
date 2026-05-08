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

//! # Taxonomic Orchestrator
//! Ref: [012-LMS-ENG]
//! Location: `src/core/resolver/orchestrator.rs`
//!
//! **Why**: This module serves as the primary entry point for Phase 1 (Resolve) of the pipeline. It constructs the Chain of Responsibility and defines the shared resolution data types.
//! **Impact**: If the orchestrator miswires the chain, the system may skip vital resolution steps like Aliasing, causing valid tags to fall through to the system default resulting in cultural data loss.
//!
//! ### Glossary
//! * **LocaleEntry**: The deterministic result of a successful resolution, containing the target ID and the diagnostic path taken to find it.

use crate::core::resolver::{
    IResolver, alias::AliasResolver, exact::ExactMatchResolver, fallback::DefaultFallbackResolver,
    truncation::TruncationResolver,
};
use crate::data::swap::IRegistryState;
use bistun_core::LmsError;

/// Represents the canonical linguistic profile resolved from the Taxonomy engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleEntry {
    pub id: String,
    pub resolution_path: Vec<String>,
}

/// Resolves a BCP 47 string to a [`LocaleEntry`] using the Chain of Responsibility.
///
/// Time: O(N) where N is the number of subtags | Space: O(N) for path tracking.
///
/// # Logic Trace (Internal)
/// 1. Sanitize the input string to ensure it is not empty.
/// 2. Construct and link the resolver chain (`Exact Match` -> `Alias` -> `Truncation` -> `Default`).
/// 3. Execute the chain, collecting telemetry in the resolution path.
///
/// # Examples
/// ```text
/// // Note: This requires the concrete resolvers and a mock registry to execute.
/// // See the `tests` module below for the hermetic Golden Path validation.
/// ```
///
/// # Arguments
/// * `tag` (&str): The raw BCP 47 language tag requested by the consuming application.
/// * `state` (&dyn IRegistryState): The thread-safe active Flyweight pool, abstracted via dynamic dispatch for object safety.
///
/// # Returns
/// * `Result<LocaleEntry, LmsError>`: The resolved canonical ID and its diagnostic audit path.
///
/// # Golden I/O
/// * **Input**: `"ar-EG"`, `RegistryState`
/// * **Output**: `Ok(LocaleEntry { id: "ar-EG", resolution_path: ["ar-EG"] })`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns `LmsError::InvalidTag` if the input is whitespace or empty. Returns `LmsError::ResolutionFailed` if the chain exhausts (structurally impossible if the Default Fallback is wired).
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn resolve(tag: &str, state: &dyn IRegistryState) -> Result<LocaleEntry, LmsError> {
    // [STEP 1]: Sanitization
    let trimmed = tag.trim();
    if trimmed.is_empty() {
        return Err(LmsError::InvalidTag {
            pipeline_step: "Phase 1: Taxonomic Resolution".to_string(),
            tag: tag.to_string(),
            reason: "Provided tag is empty or entirely whitespace".to_string(),
        });
    }

    // [STEP 2]: Chain Construction [Ref: 012-LMS-ENG]
    let mut exact_resolver = ExactMatchResolver::new();
    let mut alias_resolver = AliasResolver::new();
    let mut trunc_resolver = TruncationResolver::new();
    let default_resolver = DefaultFallbackResolver::new();

    // Link the chain: Exact -> Alias -> Truncation -> Default
    trunc_resolver.set_next(Box::new(default_resolver));
    alias_resolver.set_next(Box::new(trunc_resolver));
    exact_resolver.set_next(Box::new(alias_resolver));

    // [STEP 3]: Execution
    // Pre-allocate capacity to avoid mid-resolution heap reallocations
    let mut resolution_path = Vec::with_capacity(4);

    exact_resolver.resolve(trimmed, state, &mut resolution_path).ok_or_else(|| {
        LmsError::ResolutionFailed {
            pipeline_step: "Phase 1: Taxonomic Resolution".to_string(),
            tag: trimmed.to_string(),
            reason: "Fallback chain exhausted without hitting system default".to_string(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resolver::test_utils::*;

    #[test]
    fn test_orchestrator_rejects_empty_tags() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate an empty mock state and a whitespace tag.
        let mock_state = MockRegistryState::new();

        // [STEP 2]: Execute.
        let result = resolve("   ", &mock_state);

        // [STEP 3]: Assert: Ensure sanitization catches the empty string.
        assert_eq!(
            result,
            Err(LmsError::InvalidTag {
                pipeline_step: "Phase 1: Taxonomic Resolution".to_string(),
                tag: "   ".to_string(),
                reason: "Provided tag is empty or entirely whitespace".to_string(),
            })
        );
    }

    #[test]
    fn test_orchestrator_wires_chain_correctly() {
        let mut mock_state = MockRegistryState::new();
        mock_state
            .expect_get_profile()
            .with(mockall::predicate::eq("th-TH"))
            .returning(|_| Some(create_stub("th-TH")));

        let entry = resolve("th-TH", &mock_state).unwrap();

        assert_eq!(entry.id, "th-TH");
        assert_eq!(entry.resolution_path.len(), 1);
    }
}
