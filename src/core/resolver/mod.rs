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

//! # Resolver Core Engine
//! Ref: [012-LMS-ENG]
//! Location: `src/core/resolver/mod.rs`
//!
//! **Why**: This module defines the Chain of Responsibility interface for Taxonomic Resolution. It serves as the abstract foundation for Phase 1 of the capability pipeline.
//! **Impact**: Compromising this interface breaks the ability to dynamically route BCP 47 tags to their linguistic definitions, causing the system to fail open to defaults and potentially misidentifying critical script traits.
//!
//! ### Glossary
//! * **Resolver**: A discrete operational unit in the fallback chain (e.g., ExactMatch, Truncation).
//! * **Chain of Responsibility**: A behavioral design pattern where a request is passed along a chain of handlers until one processes it.

pub mod bcp47;

use crate::core::resolver::bcp47::LocaleEntry;
use crate::data::swap::IRegistryState;

/// The standard contract for all Taxonomic Resolvers in the pipeline.
///
/// Time: O(1) definition | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Accept a BCP 47 tag and a reference to the active registry state.
/// 2. If the current resolver can satisfy the request, return a `LocaleEntry`.
/// 3. If not, delegate the request to the `next` resolver in the chain.
pub trait IResolver: Send + Sync {
    /// Attempts to resolve the given tag or defers to the next resolver in the chain.
    ///
    /// # Arguments
    /// * `tag` (&str): The current BCP 47 string being evaluated.
    /// * `state` (&dyn IRegistryState): The thread-safe active Flyweight pool, abstracted via dynamic dispatch for object safety.
    /// * `path` (&mut `Vec<String>`): The accumulated resolution path for telemetry.
    ///
    /// # Returns
    /// * `Option<LocaleEntry>`: The matched entry, or `None` if the chain exhausts without a match.
    ///
    /// # Golden I/O
    /// * **Input**: `"en-US"`, `IRegistryState`, `[]`
    /// * **Output**: `Some(LocaleEntry { id: "en-US", resolution_path: ["en-US"] })`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: May panic if the underlying registry lock is poisoned.
    /// * **Safety**: Safe synchronous execution.
    fn resolve(
        &self,
        tag: &str,
        state: &dyn IRegistryState,
        path: &mut Vec<String>,
    ) -> Option<LocaleEntry>;

    /// Appends the next resolver in the Chain of Responsibility.
    ///
    /// # Arguments
    /// * `next` (`Box<dyn IResolver>`): The next strategy to execute on failure.
    ///
    /// # Returns
    /// * `()`: Side-effect function to build the execution chain.
    ///
    /// # Golden I/O
    /// * **Input**: `Box<TruncationResolver>`
    /// * **Output**: `()`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Fully safe synchronous configuration.
    fn set_next(&mut self, next: Box<dyn IResolver>);
}
