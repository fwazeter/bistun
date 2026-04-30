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
//! Ref: [001-LMS-CORE]
//!
//! **Why**: This module serves as the primary SDK interface for external consumers. It abstracts away the complex 5-phase resolution pipeline behind a single, ergonomic struct.
//! **Impact**: If this module fails, external services (like the Curator UI or downstream rendering engines) cannot interface with the capability engine, breaking the integration layer.
//!
//! ### Glossary
//! * **SDK Orchestrator**: The public-facing state manager that will eventually hold in-memory registry caches and security keys.

use crate::core::generate_manifest;
use crate::core::resolver::bcp47::LmsError;
use crate::models::manifest::CapabilityManifest;

/// The primary interface for generating Linguistic Capability Manifests.
///
/// Time: O(1) instantiation | Space: O(1)
#[derive(Debug, Default)]
pub struct LinguisticManager {
    // [STUB]: In future phases (e.g., [010-LMS-MEM]), this struct will hold
    // atomic references (Arc/RwLock) to the Flyweight data store and Registry.
}

impl LinguisticManager {
    /// Initializes a new instance of the LinguisticManager.
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default, stateless manager (to be expanded in Phase 4).
    ///
    /// # Examples
    /// ```rust
    /// use bistun::manager::LinguisticManager;
    ///
    /// let manager = LinguisticManager::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Resolves a raw BCP 47 tag into an immutable CapabilityManifest.
    ///
    /// Time: O(N) based on tag truncation length | Space: O(1) beyond the returned map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. **Ingestion**: Accepts a borrowed string slice of a user-provided locale.
    /// 2. **Delegation**: Hands the tag down to the `core::generate_manifest` 5-phase pipeline.
    /// 3. **Return**: Yields the generated DTO or bubbles up the architectural `LmsError`.
    ///
    /// # Examples
    /// ```rust
    /// use bistun::manager::LinguisticManager;
    ///
    /// let manager = LinguisticManager::new();
    /// let manifest = manager.resolve_capabilities("zh-Hant-TW").unwrap();
    /// assert_eq!(manifest.resolved_locale, "zh-Hant");
    /// ```
    pub fn resolve_capabilities(&self, tag: &str) -> Result<CapabilityManifest, LmsError> {
        // Delegate directly to the core algorithmic engine
        generate_manifest(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_delegates_to_core_successfully() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate the manager.
        // 2. Execute: Pass a valid tag.
        // 3. Assert: Verify the pipeline resolves and returns a populated DTO.
        let manager = LinguisticManager::new();
        let manifest = manager.resolve_capabilities("th-TH").expect("SDK delegation failed");

        assert_eq!(manifest.resolved_locale, "th-TH");
        assert!(!manifest.traits.is_empty());
    }

    /* #[test]
    fn test_manager_bubbles_errors_cleanly() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate the manager.
        // 2. Execute: Pass a garbage tag.
        // 3. Assert: Verify the LmsError propagates across the public boundary.
        let manager = LinguisticManager::new();
        let result = manager.resolve_capabilities("invalid-tag-here");

        assert!(matches!(result, Err(LmsError::ResolutionFailed(_))));
    }*/
    #[test]
    fn test_manager_provides_fallback_for_invalid_tags() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate the manager.
        // 2. Execute: Pass a garbage tag.
        // 3. Assert: Verify the manager returns the "en-US" safety manifest instead of an error.
        let manager = LinguisticManager::new();
        let manifest = manager
            .resolve_capabilities("invalid-tag-here")
            .expect("SDK should return fallback manifest");

        assert_eq!(manifest.resolved_locale, "en-US");
    }
}
