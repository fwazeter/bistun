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

//! # Alias Resolver
//! Ref: [012-LMS-ENG]
//! Location: `src/core/resolver/alias.rs`
//!
//! **Why**: Maps deprecated, regional, or macrolanguage tags to their operational equivalents before passing them down the Chain of Responsibility.
//! **Impact**: If this fails, valid legacy tags (like `in` for Indonesian) will fail to resolve to existing definitions, causing the engine to incorrectly fall back to the system default.
//!
//! ### Glossary
//! * **Canonicalization**: The mapping of an alternate or deprecated locale code to its primary, standard identity.

use crate::core::resolver::{IResolver, orchestrator::LocaleEntry};
use crate::data::swap::IRegistryState;

/// Evaluates BCP 47 tags against a known list of aliases and macrolanguages.
#[derive(Default)]
pub struct AliasResolver {
    next: Option<Box<dyn IResolver>>,
}

impl AliasResolver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IResolver for AliasResolver {
    /// Executes the Alias resolution strategy.
    ///
    /// Time: O(1) | Space: O(1) (excluding path telemetry allocation)
    ///
    /// # Logic Trace (Internal)
    /// 1. Identify known aliases using an O(1) match constraint.
    /// 2. If the tag was aliased, check the registry for the canonical ID.
    /// 3. Delegate to the next resolver using the canonicalized tag to prevent double-work in downstream nodes.
    ///
    /// # Examples
    /// ```text
    /// // See internal `tests` module for hermetic execution.
    /// ```
    ///
    /// # Arguments
    /// * `tag` (&str): The current BCP 47 string being evaluated.
    /// * `state` (&dyn IRegistryState): The thread-safe active Flyweight pool.
    /// * `path` (&mut `Vec<String>`): The accumulated resolution path for telemetry.
    ///
    /// # Returns
    /// * `Option<LocaleEntry>`: The matched entry, or `None` if the chain exhausts.
    ///
    /// # Golden I/O
    /// * **Input**: `"in"`, `RegistryState`, `[]`
    /// * **Output**: `Some(LocaleEntry { id: "id", resolution_path: ["alias:in->id", "id"] })`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous execution.
    fn resolve(
        &self,
        tag: &str,
        state: &dyn IRegistryState,
        path: &mut Vec<String>,
    ) -> Option<LocaleEntry> {
        // [STEP 1]: Query the dynamic routing state for an alias.
        if let Some(canonical_id) = state.resolve_alias(tag) {
            // [STEP 2]: If aliased, record telemetry and check the registry.
            path.push(format!("alias:{}->{}", tag, canonical_id));

            if state.get_profile(&canonical_id).is_some() {
                path.push(canonical_id.clone());
                return Some(LocaleEntry { id: canonical_id, resolution_path: path.clone() });
            }

            // [STEP 3]: Delegate canonical
            // Pass `canonical_id` down to prevent double work in downstream nodes.
            self.next.as_ref().and_then(|n| n.resolve(&canonical_id, state, path))
        } else {
            // [STEP 3]: Delegate original. Not an alias; pass the original tag down the chain.
            self.next.as_ref().and_then(|n| n.resolve(tag, state, path))
        }
    }

    fn set_next(&mut self, next: Box<dyn IResolver>) {
        self.next = Some(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resolver::test_utils::*;

    #[test]
    fn test_alias_resolver_matches_and_returns() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Indonesian 'in' aliases to 'id'.
        let mut mock_state = MockRegistryState::new();
        mock_state
            .expect_resolve_alias()
            .with(mockall::predicate::eq("in"))
            .returning(|_| Some("id".to_string()));

        mock_state
            .expect_get_profile()
            .with(mockall::predicate::eq("id"))
            .returning(|_| Some(create_stub("id")));

        let resolver = AliasResolver::new();
        let mut path = Vec::new();

        // [STEP 2]: Execute.
        let entry = resolver.resolve("in", &mock_state, &mut path).unwrap();

        // [STEP 3]: Assert: Tag caught and mapped before delegation.
        assert_eq!(entry.id, "id");
        assert_eq!(entry.resolution_path.len(), 2);
        assert_eq!(entry.resolution_path[0], "alias:in->id");
        assert_eq!(entry.resolution_path[1], "id");
    }

    #[test]
    fn test_alias_resolver_delegates_non_aliases() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: 'en-US' is not an alias.
        let mut mock_state = MockRegistryState::new();

        mock_state.expect_resolve_alias().with(mockall::predicate::eq("en-US")).returning(|_| None);

        let mut next_resolver = MockNextResolver::new();
        next_resolver.expect_resolve().withf(|tag, _, _| tag == "en-US").returning(|_, _, _| {
            Some(LocaleEntry {
                id: "en-US".to_string(),
                resolution_path: vec!["en-US".to_string()],
            })
        });

        let mut resolver = AliasResolver::new();
        resolver.set_next(Box::new(next_resolver));
        let mut path = Vec::new();

        // [STEP 2 & 3]: Execute and Assert delegation bypassed step 2.
        let entry = resolver.resolve("en-US", &mock_state, &mut path).unwrap();
        assert_eq!(entry.id, "en-US");
    }
}
