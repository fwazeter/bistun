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
//! * **Circuit Breaker**: A design pattern that prevents the system from executing a failing operation (like querying an empty memory pool), instead instantly returning a safe default.
//! * **SdkState**: The operational health of the manager (`Bootstrapping`, `Ready`, `Degraded`).

use crate::core::pipeline::generate_manifest;
use crate::core::resolver::bcp47::LmsError;
use crate::data::repository::hydrate_snapshot;
use crate::data::swap::RegistryState;
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::{Direction, MorphType, SegType, TraitKey};

/// Represents the operational health and readiness of the SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkState {
    /// The manager is initializing and attempting to load data.
    Bootstrapping,
    /// The manager is fully hydrated and operating normally.
    Ready,
    /// The manager failed to hydrate and is running in Circuit Breaker mode.
    Degraded,
}

/// The primary interface for generating Linguistic Capability Manifests.
///
/// Time: O(1) reads | Space: O(1) pointer allocations
#[derive(Debug, Clone)]
pub struct LinguisticManager {
    /// The thread-safe dynamic memory state protecting the Flyweight pool.
    state: RegistryState,
    /// The current operational status of the engine.
    status: SdkState,
}

impl Default for LinguisticManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LinguisticManager {
    /// Initializes a new instance and attempts to hydrate the WORM snapshot.
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default `RegistryState` and sets status to `Bootstrapping`.
    /// 2. Triggers the background repository hydrator (`hydrate_snapshot`).
    /// 3. If hydration succeeds, hot-swaps the active memory pointer and sets status to `Ready`.
    /// 4. If hydration fails, sets status to `Degraded` (activating the Circuit Breaker).
    pub fn new() -> Self {
        let state = RegistryState::new();

        // Hydrate from WORM repository on boot
        let status = match hydrate_snapshot() {
            Ok(store) => {
                state.swap_registry(store);
                SdkState::Ready
            }
            Err(_) => SdkState::Degraded,
        };

        Self { state, status }
    }

    /// Returns the current health status of the SDK.
    pub fn status(&self) -> SdkState {
        self.status
    }

    /// Resolves a raw BCP 47 tag dynamically into an immutable CapabilityManifest.
    ///
    /// Time: O(N) based on tag truncation length | Space: O(1) beyond returned map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. **Circuit Breaker Check**: If `status` is `Degraded`, bypass the pipeline entirely and yield the hardcoded fallback manifest.
    /// 2. **Delegation**: If `Ready`, hand the tag and internal dynamic state down to the `pipeline::generate_manifest` engine.
    /// 3. **Return**: Yield the generated DTO or bubble up the architectural `LmsError`.
    pub fn resolve_capabilities(&self, tag: &str) -> Result<CapabilityManifest, LmsError> {
        if self.status == SdkState::Degraded {
            return Ok(Self::generate_circuit_breaker_manifest());
        }

        generate_manifest(tag, &self.state)
    }

    /// Generates a guaranteed-safe fallback manifest when the memory pool is unreachable.
    ///
    /// Time: O(1) | Space: O(1)
    fn generate_circuit_breaker_manifest() -> CapabilityManifest {
        let mut manifest = CapabilityManifest::new("en-US".to_string());

        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));

        manifest.metadata.insert("registry_version".to_string(), "CIRCUIT_BREAKER".to_string());
        manifest.metadata.insert("resolution_path".to_string(), "DEGRADED_FALLBACK".to_string());
        manifest.metadata.insert("resolution_time_ms".to_string(), "0.0000".to_string());

        manifest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_boots_into_ready_state() {
        let manager = LinguisticManager::new();
        assert_eq!(manager.status(), SdkState::Ready);
    }

    #[test]
    fn test_manager_delegates_to_dynamic_pipeline() {
        let manager = LinguisticManager::new();
        let manifest = manager.resolve_capabilities("th-TH").expect("SDK delegation failed");

        assert_eq!(manifest.resolved_locale, "th-TH");
        assert!(!manifest.traits.is_empty());
    }

    #[test]
    fn test_circuit_breaker_intercepts_requests() {
        // [Logic Trace Mapping]
        // 1. Setup: Instantiate a manager and manually force it into a Degraded state to simulate hydration failure.
        // 2. Execute: Request a valid locale ("ar-EG").
        // 3. Assert: Verify the Circuit Breaker intercepts the request and returns the safe "en-US" fallback instead.
        let mut manager = LinguisticManager::new();
        manager.status = SdkState::Degraded; // Force Degraded state

        let manifest = manager.resolve_capabilities("ar-EG").expect("Circuit breaker failed");

        assert_eq!(manifest.resolved_locale, "en-US");
        assert_eq!(manifest.metadata.get("registry_version").unwrap(), "CIRCUIT_BREAKER");
    }
}
