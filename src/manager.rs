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
//! Location: `src/manager.rs`
//!
//! **Why**: This module serves as the primary SDK interface for external consumers. It abstracts away the complex 5-phase resolution pipeline and manages the thread-safe dynamic memory pool.
//! **Impact**: If this module fails, external services cannot interface with the capability engine or will fail to boot due to hydration errors.
//!
//! ### Glossary
//! * **Circuit Breaker**: A design pattern that prevents the system from executing a failing operation (like querying an empty memory pool), instead instantly returning a safe default.
//! * **SdkState**: The operational health of the manager (`Bootstrapping`, `Ready`, `Degraded`).

use crate::core::pipeline::generate_manifest;
use crate::core::resolver::bcp47::LmsError;
use crate::data::repository::{SimulatedSnapshotProvider, hydrate_snapshot};
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
    /// Time: O(M) hydration | Space: O(M) memory pool allocation
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default `RegistryState` and sets status to `Bootstrapping`.
    /// 2. Initializes the `SimulatedSnapshotProvider`.
    /// 3. Triggers the repository hydrator with the provider.
    /// 4. On success, hot-swaps the registry and sets status to `Ready`.
    /// 5. On failure, activates Circuit Breaker mode (`Degraded`).
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `Self`: A prepared manager instance.
    ///
    /// # Golden I/O
    /// * **Input**: `()`
    /// * **Output**: `LinguisticManager { status: Ready, ... }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: Internal hydration errors are caught to prevent boot failure.
    /// * **Panics**: None.
    /// * **Safety**: Thread-safe initialization.
    pub fn new() -> Self {
        // [STEP 1]: Initialize empty state.
        let state = RegistryState::new();
        // [STEP 2]: Setup default provider.
        let provider = SimulatedSnapshotProvider::new();

        // [STEP 3]: Attempt hydration.
        let status = match hydrate_snapshot(&provider) {
            Ok(store) => {
                // [STEP 4]: Perform atomic hot-swap.
                state.swap_registry(store);
                SdkState::Ready
            }
            Err(_) => {
                // [STEP 5]: Fail-safe transition.
                SdkState::Degraded
            }
        };

        Self { state, status }
    }

    /// Returns the current health status of the SDK.
    ///
    /// # Returns
    /// * `SdkState`: Current operational readiness.
    pub fn status(&self) -> SdkState {
        self.status
    }

    /// Resolves a raw BCP 47 tag dynamically into an immutable CapabilityManifest.
    ///
    /// Time: O(N) based on tag truncation length | Space: O(1) beyond returned map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Check health status; if `Degraded`, yield the hardcoded fallback.
    /// 2. Delegate valid requests to the 5-Phase pipeline.
    /// 3. Return the generated manifest or bubble up architectural errors.
    ///
    /// # Arguments
    /// * `tag` (&str): The BCP 47 string requested by the client.
    ///
    /// # Returns
    /// * `Result<CapabilityManifest, LmsError>`: The resolved capability payload.
    ///
    /// # Golden I/O
    /// * **Input**: `"ar-EG"`
    /// * **Output**: `Ok(CapabilityManifest { resolved_locale: "ar-EG", .. })`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: Returns `LmsError` variants from Phase 1-4.
    /// * **Panics**: None.
    /// * **Safety**: Lock-free read access for active requests.
    pub fn resolve_capabilities(&self, tag: &str) -> Result<CapabilityManifest, LmsError> {
        // [STEP 1]: Circuit Breaker Check.
        if self.status == SdkState::Degraded {
            return Ok(Self::generate_circuit_breaker_manifest());
        }

        // [STEP 2] & [STEP 3]: Pipeline Delegation.
        generate_manifest(tag, &self.state)
    }

    /// Generates a guaranteed-safe fallback manifest when the memory pool is unreachable.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiate a default "en-US" manifest.
    /// 2. Inject Tier 3 baseline traits.
    /// 3. Inject Circuit Breaker telemetry metadata.
    fn generate_circuit_breaker_manifest() -> CapabilityManifest {
        // [STEP 1]: Instantiate en-US.
        let mut manifest = CapabilityManifest::new("en-US".to_string());

        // [STEP 2]: Inject traits.
        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));

        // [STEP 3]: Inject fallback metadata.
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
        // [Logic Trace Mapping]
        // [STEP 1]: Instantiate manager.
        let manager = LinguisticManager::new();
        // [STEP 2]: Assert readiness.
        assert_eq!(manager.status(), SdkState::Ready);
    }

    #[test]
    fn test_manager_delegates_to_dynamic_pipeline() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup manager.
        let manager = LinguisticManager::new();
        // [STEP 2]: Execute resolution.
        let manifest = manager.resolve_capabilities("th-TH").expect("SDK delegation failed");

        // [STEP 3]: Assert hydration.
        assert_eq!(manifest.resolved_locale, "th-TH");
        assert!(!manifest.traits.is_empty());
    }

    #[test]
    fn test_circuit_breaker_intercepts_requests() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate and force Degraded state.
        let mut manager = LinguisticManager::new();
        manager.status = SdkState::Degraded;

        // [STEP 2]: Execute: Request "ar-EG".
        let manifest = manager.resolve_capabilities("ar-EG").expect("Circuit breaker failed");

        // [STEP 3]: Assert: Verify fallback to en-US.
        assert_eq!(manifest.resolved_locale, "en-US");
        assert_eq!(manifest.metadata.get("registry_version").unwrap(), "CIRCUIT_BREAKER");
    }
}
