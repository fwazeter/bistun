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

#![cfg(test)] // Strictly limits this file to the testing compiler pass

//! # Resolver Test Utilities
//! Crate: `bistun-lms`
//! Ref: [LMS-TEST], [012-LMS-ENG]
//! Location: `crates/bistun-lms/src/core/resolver/test_utils.rs`
//!
//! **Why**: Provides a unified set of mocks and generators to ensure hermetic testing of the Taxonomic Engine.
//! **Impact**: Prevents "Mock Drift" across the resolver chain, ensuring that `Alias`, `Exact`, and `Truncation` nodes all test against the same V2.0.0 data structures.
//!
//! ### Glossary
//! * **Mock**: A simulated object that mimics the behavior of real components for isolated testing.
//! * **Stub**: A simple data object used to satisfy type requirements during execution.

use super::{IResolver, orchestrator::LocaleEntry};
use crate::data::store::LocaleProfile;
use crate::data::swap::IRegistryState;
use bistun_core::manifest::TraitValue;
use bistun_core::traits::{Direction, LmsRule, MorphType, NormRule, SegType, TraitKey};
use hashbrown::HashMap;
use mockall::mock;
use std::sync::Arc;

// ---------------------------------------------------------
// UNIFIED MOCKS
// ---------------------------------------------------------

mock! {
    /// Unified Mock for the `RegistryStore` memory pool.
    pub RegistryState {}
    impl IRegistryState for RegistryState {
        fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>>;
        fn resolve_alias(&self, tag: &str) -> Option<String>;
        fn get_version(&self) -> String;
        fn get_base_resource_uri(&self) -> String;
    }
}

mock! {
    /// Unified Mock for the Chain of Responsibility delegation.
    pub NextResolver {}
    impl IResolver for NextResolver {
        fn resolve(&self, tag: &str, state: &dyn IRegistryState, path: &mut Vec<String>) -> Option<LocaleEntry>;
        fn set_next(&mut self, next: Box<dyn IResolver>);
    }
}

// ---------------------------------------------------------
// STUB GENERATORS
// ---------------------------------------------------------

/// Generates a "Golden" [`LocaleProfile`] stub for hermetic resolver testing.
///
/// Time: `O(1)` | Space: `O(1)`
///
/// # Logic Trace (Internal)
/// 1. Initialize empty `traits`, `rules`, and `resources` maps to align with V2.0.0 standards.
/// 2. Inject default "Linguistic DNA" (LTR, Space Segmentation, Fusional Morphology).
/// 3. Wrap the resulting [`LocaleProfile`] in an `Arc` for thread-safe Flyweight simulation.
///
/// # Examples
/// ```rust
/// # use crate::bistun_lms::core::resolver::test_utils::create_stub;
/// let profile = create_stub("en-US");
/// assert_eq!(profile.id, "en-US");
/// ```
///
/// # Arguments
/// * `id` (&str): The canonical `BCP 47` identifier for the stub.
///
/// # Returns
/// * `Arc<LocaleProfile>`: An immutable, reference-counted linguistic profile.
///
/// # Golden I/O
/// * **Input**: `"en-US"`
/// * **Output**: `Arc<LocaleProfile { id: "en-US", traits: {...}, ... }>`
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous initialization.
#[must_use]
pub fn create_stub(id: &str) -> Arc<LocaleProfile> {
    let mut traits = HashMap::new();

    // Inject V2.0.0 DNA Traits
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));

    // Inject Mandatory Execution Directives
    let mut rules = HashMap::new();
    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));

    Arc::new(LocaleProfile { id: id.to_string(), traits, rules, resources: HashMap::new() })
}
