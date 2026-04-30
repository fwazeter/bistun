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

//! # The Linguistic Manager (SDK Orchestrator)
//! Ref: [001-LMS-CORE], [010-LMS-MEM]
//!
//! **Why**: This module serves as the primary SDK interface for external consumers. It abstracts away the complex 5-phase resolution pipeline and manages the thread-safe dynamic memory pool.
//! **Impact**: If this module fails, external services cannot interface with the capability engine or will fail to boot due to hydration errors.
//!
//! ### Glossary
//! * **SDK Orchestrator**: The public-facing state manager holding in-memory registry caches and security keys.
//! * **Hydration on Boot**: The process of loading the WORM snapshot into memory the moment the manager is instantiated.

use crate::core::pipeline::generate_manifest;
use crate::core::resolver::bcp47::LmsError;
use crate::data::repository::hydrate_snapshot;
use crate::data::swap::RegistryState;
use crate::models::manifest::CapabilityManifest;

/// The primary interface for generating Linguistic Capability Manifests.
///
/// Time: O(1) reads | Space: O(1) pointer allocations
#[derive(Debug, Clone)]
pub struct LinguisticManager {
    /// The thread-safe dynamic memory state protecting the Flyweight pool.
    state: RegistryState,
}

impl Default for LinguisticManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LinguisticManager {
    /// Initializes a new instance and immediately hydrates the WORM snapshot.
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default, empty `RegistryState`.
    /// 2. Triggers the background repository hydrator (`hydrate_snapshot`).
    /// 3. If hydration succeeds, hot-swaps the active memory pointer to the populated pool.
    /// 4. Returns the manager ready for execution.
    pub fn new() -> Self {
        let state = RegistryState::new();

        // Hydrate from WORM repository on boot
        if let Ok(store) = hydrate_snapshot() {
            state.swap_registry(store);
        }

        Self { state }
    }

    /// Resolves a raw BCP 47 tag dynamically into an immutable CapabilityManifest.
    ///
    /// Time: O(N) based on tag truncation length | Space: O(1) beyond returned map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. **Ingestion**: Accepts a borrowed string slice of a user-provided locale.
    /// 2. **Delegation**: Hands the tag and the internal dynamic state down to the `pipeline::generate_manifest` engine.
    /// 3. **Return**: Yields the generated DTO or bubbles up the architectural `LmsError`.
    pub fn resolve_capabilities(&self, tag: &str) -> Result<CapabilityManifest, LmsError> {
        generate_manifest(tag, &self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_delegates_to_dynamic_pipeline_successfully() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate the manager (which auto-hydrates the mock repository).
        // 2. Execute: Pass a valid tag.
        // 3. Assert: Verify the pipeline resolves dynamically and returns a populated DTO.
        let manager = LinguisticManager::new();
        let manifest = manager.resolve_capabilities("th-TH").expect("SDK delegation failed");

        assert_eq!(manifest.resolved_locale, "th-TH");
        assert!(!manifest.traits.is_empty());
    }

    #[test]
    fn test_manager_falls_back_on_unknown() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate the manager.
        // 2. Execute: Pass a completely unknown/garbage tag.
        // 3. Assert: Verify the default fallback resolver ("en-US") catches it safely.
        let manager = LinguisticManager::new();
        let manifest =
            manager.resolve_capabilities("invalid-tag-here").expect("Should fallback cleanly");

        assert_eq!(manifest.resolved_locale, "en-US");
    }
}
