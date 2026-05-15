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

//! # Exact Match Resolver
//! Crate: `bistun-lms`
//! Ref: [012-LMS-ENG]
//! Location: `crates/bistun-lms/src/core/resolver/exact.rs`
//!
//! **Why**: Performs the initial, highest-performance `O(1)` lookup for a `BCP 47` tag before any fallback logic is applied.
//! **Impact**: This is the fastest resolution path in the pipeline. If this logic fails, the engine will unnecessarily fall back to truncation, wasting `CPU` cycles and potentially discarding highly specific regional data.
//!
//! ### Glossary
//! * **Exact Match**: A 1:1 string comparison where the requested locale matches a registry Flyweight identity identically without any truncation or aliasing.

use crate::core::resolver::{IResolver, orchestrator::LocaleEntry};
use crate::data::swap::IRegistryState;

/// Evaluates `BCP 47` tags for a direct 1:1 match in the active registry.
#[derive(Default)]
pub struct ExactMatchResolver {
    /// The successor node in the resolution Chain of Responsibility.
    next: Option<Box<dyn IResolver>>,
}

impl ExactMatchResolver {
    /// Constructs a new [`ExactMatchResolver`] with no initial successor.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IResolver for ExactMatchResolver {
    /// Executes the Exact Match resolution strategy.
    ///
    /// Time: `O(1)` (Hash map lookup) | Space: `O(1)` (excluding path telemetry allocation)
    ///
    /// # Logic Trace (Internal)
    /// 1. Perform an exact `O(1)` lookup against the active Flyweight memory pool using the raw tag.
    /// 2. If a match is found, append the tag to the resolution path and return the [`LocaleEntry`].
    /// 3. If no match is found, delegate the unmodified tag to the next resolver in the chain.
    ///
    /// # Examples
    /// ```text
    /// // See internal `tests` module for hermetic execution.
    /// ```
    ///
    /// # Arguments
    /// * `tag` (&str): The current `BCP 47` string being evaluated.
    /// * `state` (&dyn `IRegistryState`): The thread-safe active Flyweight pool.
    /// * `path` (&mut `Vec<String>`): The accumulated resolution path for telemetry.
    ///
    /// # Returns
    /// * `Option<LocaleEntry>`: The matched entry, or `None` if the chain exhausts.
    ///
    /// # Golden I/O
    /// * **Input**: `"ar-EG"`, `RegistryState`, `[]`
    /// * **Output**: `Some(LocaleEntry { id: "ar-EG", resolution_path: ["ar-EG"] })`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous wait-free execution.
    fn resolve(
        &self,
        tag: &str,
        state: &dyn IRegistryState,
        path: &mut Vec<String>,
    ) -> Option<LocaleEntry> {
        // [STEP 1]: Perform an exact $O(1)$ lookup in the active memory pool.
        if state.get_profile(tag).is_some() {
            // [STEP 2]: Match found; record telemetry and return.
            path.push(tag.to_string());
            return Some(LocaleEntry { id: tag.to_string(), resolution_path: path.clone() });
        }

        // [STEP 3]: No match; delegate to the next resolver in the chain.
        self.next.as_ref().and_then(|n| n.resolve(tag, state, path))
    }

    /// Sets the successor node in the resolution chain.
    fn set_next(&mut self, next: Box<dyn IResolver>) {
        self.next = Some(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resolver::test_utils::*;

    #[test]
    fn test_exact_match_resolves_immediately() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: "th-TH" exists in the mocked registry.
        let mut mock_state = MockRegistryState::new();
        mock_state
            .expect_get_profile()
            .with(mockall::predicate::eq("th-TH"))
            .returning(|_| Some(create_stub("th-TH")));

        let resolver = ExactMatchResolver::new();
        let mut path = Vec::new();

        // [STEP 2]: Execute.
        let entry = resolver
            .resolve("th-TH", &mock_state, &mut path)
            .expect("LMS-TEST: Exact match failed for existing tag");

        // [STEP 3]: Assert: Tag is caught immediately.
        assert_eq!(entry.id, "th-TH");
        assert_eq!(entry.resolution_path.len(), 1);
        assert_eq!(entry.resolution_path[0], "th-TH");
    }

    #[test]
    fn test_exact_match_delegates_on_miss() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: "en-AU" does NOT exist in the registry.
        let mut mock_state = MockRegistryState::new();
        mock_state.expect_get_profile().returning(|_| None);

        // Setup the next resolver to catch the delegated request
        let mut next_resolver = MockNextResolver::new();
        next_resolver.expect_resolve().withf(|tag, _, _| tag == "en-AU").returning(|_, _, _| {
            Some(LocaleEntry {
                id: "en-GB".to_string(),
                resolution_path: vec!["en-GB".to_string()],
            })
        });

        let mut resolver = ExactMatchResolver::new();
        resolver.set_next(Box::new(next_resolver));
        let mut path = Vec::new();

        // [STEP 2 & 3]: Execute and Assert delegation occurred.
        let entry = resolver
            .resolve("en-AU", &mock_state, &mut path)
            .expect("LMS-TEST: Delegation failed on exact match miss");
        assert_eq!(entry.id, "en-GB");
    }
}
