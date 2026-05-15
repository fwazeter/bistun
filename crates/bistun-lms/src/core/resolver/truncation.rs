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

//! # Truncation Resolver
//! Crate: `bistun-lms`
//! Ref: [012-LMS-ENG]
//! Location: `crates/bistun-lms/src/core/resolver/truncation.rs`
//!
//! **Why**: Implements the `RFC 4647` fallback algorithm, iteratively stripping subtags from right to left to find the nearest supported parent locale.
//! **Impact**: Prevents high-specificity tags (e.g., `ar-EG-u-nu-latn`) from failing into generic system defaults by correctly identifying the closest regional data (`ar-EG`).
//!
//! ### Glossary
//! * **RFC 4647**: The `IETF` standard defining the "Lookup" and "Fallback" mechanisms for `BCP 47` language tags.
//! * **Truncation**: The mechanical process of dropping the rightmost subtag following a hyphen (`-`).

use crate::core::resolver::{IResolver, orchestrator::LocaleEntry};
use crate::data::swap::IRegistryState;

/// Evaluates `BCP 47` tags by iteratively truncating the rightmost subtag.
#[derive(Default)]
pub struct TruncationResolver {
    /// The successor node in the resolution Chain of Responsibility.
    next: Option<Box<dyn IResolver>>,
}

impl TruncationResolver {
    /// Constructs a new [`TruncationResolver`] with no initial successor.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IResolver for TruncationResolver {
    /// Executes the Truncation resolution strategy.
    ///
    /// Time: `O(N)` where N is the number of subtags | Space: `O(1)` inside the loop
    ///
    /// # Logic Trace (Internal)
    /// 1. Initialize the working tag string to the exact requested input.
    /// 2. Enter an iterative loop, stripping subtags from right-to-left using `-` as the delimiter.
    /// 3. Perform a wait-free registry check for each truncated prefix via the active `RegistryState`.
    /// 4. If a match is found, record telemetry in the `path` and return the [`LocaleEntry`].
    /// 5. If the loop exhausts the string without a match, delegate the original tag to the next resolver.
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
    /// * **Input**: `"en-AU-u-nu-latn"`, `RegistryState`, `[]`
    /// * **Output**: `Some(LocaleEntry { id: "en-AU", resolution_path: ["en-AU"] })`
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
        // [STEP 1]: Initialize the working tag string.
        let mut current_tag = tag;

        // [STEP 2]: Iteratively strip subtags right-to-left using '-' as the delimiter.
        while let Some((prefix, _suffix)) = current_tag.rsplit_once('-') {
            current_tag = prefix;

            // [STEP 3]: Perform wait-free registry check for truncated candidate.
            if state.get_profile(current_tag).is_some() {
                // [STEP 4]: Match found; record telemetry and return.
                path.push(current_tag.to_string());
                return Some(LocaleEntry {
                    id: current_tag.to_string(),
                    resolution_path: path.clone(),
                });
            }
        }

        // [STEP 5]: Delegate to next if truncation exhausts parentage.
        // We pass the *original* tag down in case the DefaultFallbackResolver needs it for context.
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
    fn test_truncation_resolves_parent_tag() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: We query "en-AU-u-nu-latn", but only "en-AU" exists in the registry.
        let mut mock_state = MockRegistryState::new();
        mock_state
            .expect_get_profile()
            .with(mockall::predicate::eq("en-AU-u-nu"))
            .returning(|_| None);
        mock_state.expect_get_profile().with(mockall::predicate::eq("en-AU-u")).returning(|_| None);
        mock_state
            .expect_get_profile()
            .with(mockall::predicate::eq("en-AU"))
            .returning(|_| Some(create_stub("en-AU")));

        let resolver = TruncationResolver::new();
        let mut path = Vec::new();

        // [STEP 2]: Execute.
        let entry = resolver
            .resolve("en-AU-u-nu-latn", &mock_state, &mut path)
            .expect("LMS-TEST: Truncation failed to resolve parent for valid child tag");

        // [STEP 3]: Assert: Tag successfully truncated down to "en-AU".
        assert_eq!(entry.id, "en-AU");
        assert_eq!(entry.resolution_path.len(), 1);
        assert_eq!(entry.resolution_path[0], "en-AU");
    }

    #[test]
    fn test_truncation_delegates_on_complete_exhaustion() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: A tag where neither the full tag nor its parents exist.
        let mut mock_state = MockRegistryState::new();
        mock_state.expect_get_profile().returning(|_| None); // Always miss

        // Setup the next resolver to catch the delegation
        let mut next_resolver = MockNextResolver::new();
        next_resolver
            .expect_resolve()
            .withf(|tag, _, _| tag == "xx-YY-ZZ") // Must delegate the original tag
            .returning(|_, _, _| {
                Some(LocaleEntry {
                    id: "en-US".to_string(),
                    resolution_path: vec!["en-US".to_string()],
                })
            });

        let mut resolver = TruncationResolver::new();
        resolver.set_next(Box::new(next_resolver));
        let mut path = Vec::new();

        // [STEP 2 & 3]: Execute and Assert delegation occurred correctly.
        let entry = resolver
            .resolve("xx-YY-ZZ", &mock_state, &mut path)
            .expect("LMS-TEST: Truncation failed to delegate correctly after exhaustion");
        assert_eq!(entry.id, "en-US");
    }
}
