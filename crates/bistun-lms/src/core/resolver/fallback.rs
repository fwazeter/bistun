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

//! # Default Fallback Resolver
//! Ref: [012-LMS-ENG]
//! Location: `src/core/resolver/fallback.rs`
//!
//! **Why**: Acts as the terminal node in the Chain of Responsibility, guaranteeing that the Taxonomic Engine always returns a valid `LocaleEntry`.
//! **Impact**: Prevents the capability pipeline from crashing or returning `None` when encountering entirely unsupported or alien BCP 47 tags.
//!
//! ### Glossary
//! * **Terminal Node**: The final step in a Chain of Responsibility that handles the request unconditionally, halting further delegation.
//! * **System Default**: The guaranteed baseline linguistic profile (always `"en-US"`) used when no closer match exists.

use crate::core::resolver::{IResolver, orchestrator::LocaleEntry};
use crate::data::swap::IRegistryState;

/// The terminal resolver that unconditionally provides the system default locale.
#[derive(Default)]
pub struct DefaultFallbackResolver;

impl DefaultFallbackResolver {
    pub fn new() -> Self {
        Self
    }
}

impl IResolver for DefaultFallbackResolver {
    /// Executes the final Fallback resolution strategy.
    ///
    /// Time: O(1) | Space: O(1) (excluding path telemetry allocation)
    ///
    /// # Logic Trace (Internal)
    /// 1. Unconditionally append the system default ID (`"en-US"`) to the resolution path.
    /// 2. Return the `LocaleEntry` representing the system default.
    ///
    /// # Examples
    /// ```text
    /// // See internal `tests` module for hermetic execution.
    /// ```
    ///
    /// # Arguments
    /// * `_tag` (&str): The current BCP 47 string being evaluated (ignored).
    /// * `_state` (&dyn IRegistryState): The thread-safe active Flyweight pool (ignored).
    /// * `path` (&mut `Vec<String>`): The accumulated resolution path for telemetry.
    ///
    /// # Returns
    /// * `Option<LocaleEntry>`: Unconditionally returns `Some(LocaleEntry)`.
    ///
    /// # Golden I/O
    /// * **Input**: `"xx-YY"`, `RegistryState`, `["xx-YY", "xx"]`
    /// * **Output**: `Some(LocaleEntry { id: "en-US", resolution_path: ["xx-YY", "xx", "en-US"] })`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous execution.
    fn resolve(
        &self,
        _tag: &str,
        _state: &dyn IRegistryState,
        path: &mut Vec<String>,
    ) -> Option<LocaleEntry> {
        // [STEP 1]: Append final canonical safety to the path.
        path.push("en-US".to_string());

        // [STEP 2]: Return the system default.
        Some(LocaleEntry { id: "en-US".to_string(), resolution_path: path.clone() })
    }

    fn set_next(&mut self, _next: Box<dyn IResolver>) {
        // Terminal node; delegation is structurally impossible.
        // Implementing an intentional no-op to satisfy the trait contract without panicking.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resolver::test_utils::*;

    #[test]
    fn test_fallback_unconditionally_resolves_to_en_us() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate the fallback resolver and a dummy state.
        let mock_state = MockRegistryState::new();
        let resolver = DefaultFallbackResolver::new();

        // Simulate a path that has already gone through Truncation failures
        let mut path = vec!["xx-YY".to_string(), "xx".to_string()];

        // [STEP 2]: Execute.
        let entry = resolver.resolve("xx", &mock_state, &mut path).unwrap();

        // [STEP 3]: Assert: The engine must return en-US and append it to the audit path.
        assert_eq!(entry.id, "en-US");
        assert_eq!(entry.resolution_path.len(), 3);
        assert_eq!(entry.resolution_path[2], "en-US");
    }
}
