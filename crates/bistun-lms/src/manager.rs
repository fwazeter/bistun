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
//! Crate: `bistun-lms`
//! Ref: [001-LMS-CORE], [010-LMS-MEM], [007-LMS-OPS]
//! Location: `crates/bistun-lms/src/manager.rs`
//!
//! **Why**: This module serves as the primary ``SDK`` interface for external consumers. It abstracts away the complex ``5-Phase`` resolution pipeline and manages the thread-safe dynamic memory pool.
//! **Impact**: If this module fails, external services cannot interface with the capability engine or will fail to boot due to hydration errors.
//!
//! ### Glossary
//! * **Circuit Breaker**: A design pattern that prevents the system from executing a failing operation (like querying an empty memory pool), instead instantly returning a safe default.
//! * **SdkState**: The operational health of the manager (``Bootstrapping``, ``Ready``, ``Degraded``).

use crate::core::pipeline::generate_manifest;
use crate::data::repository::{ISnapshotProvider, hydrate_snapshot};
use crate::data::swap::RegistryState;
use bistun_core::{
    CapabilityManifest, Direction, LmsError, LmsRule, MorphType, NormRule, ResolutionMetrics,
    SdkState, SegType, SyncMetrics, TraitKey, TraitValue, TransRule,
};
use std::sync::{Arc, RwLock};
use web_time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "async-worker")]
use std::time::Duration;
#[cfg(feature = "async-worker")]
use tokio::time;

use tracing::{error, info};

/// The primary interface for generating Linguistic Capability Manifests.
///
/// Time: `O(1)` reads | Space: `O(1)` pointer allocations
#[derive(Debug, Clone)]
pub struct LinguisticManager {
    /// The thread-safe dynamic memory state protecting the Flyweight pool.
    state: RegistryState,
    /// The current operational status of the engine. Protected by an ``RwLock``
    /// so the background worker can update it across threads.
    status: Arc<RwLock<SdkState>>,
    /// Thread-safe tracking of background sync health and errors.
    pub metrics: Arc<RwLock<SyncMetrics>>,
    /// Thread-safe tracking of atomic runtime execution telemetry (``V2.0.0``).
    pub resolution_metrics: Arc<RwLock<ResolutionMetrics>>,
}

impl Default for LinguisticManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LinguisticManager {
    /// Initializes a new, empty manager instance in the ``Bootstrapping`` state.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default [`RegistryState`].
    /// 2. Wraps the [`SdkState::Bootstrapping`] enum in a thread-safe ``Arc<RwLock>``.
    /// 3. Initializes metrics containers for sync and resolution telemetry.
    /// 4. Returns the prepared struct.
    ///
    /// # Returns
    /// * `Self`: A newly instantiated [`LinguisticManager`] ready for async initialization.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: RegistryState::new(),
            status: Arc::new(RwLock::new(SdkState::Bootstrapping)),
            metrics: Arc::new(RwLock::new(SyncMetrics::default())),
            resolution_metrics: Arc::new(RwLock::new(ResolutionMetrics::default())),
        }
    }

    /// Internal helper to get the current ``Unix`` timestamp in seconds safely.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Panics
    /// * **Panics**: Panics if the system clock moved backwards from the ``UNIX_EPOCH``.
    #[must_use]
    fn now_secs() -> u64 {
        // web_time acts as a drop-in replacement.
        // On native targets, it uses std::time. On WASM, it uses JS Date.now().
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("LMS-OPS: System clock moved backwards")
            .as_secs()
    }

    /// Performs the initial async ``WORM`` snapshot hydration via Dependency Injection.
    ///
    /// Time: `O(M)` where M is the number of locales | Space: `O(M)` memory pool allocation
    ///
    /// # Logic Trace (Internal)
    /// 1. Record the start of the sync attempt in [`SyncMetrics`].
    /// 2. Await the repository hydrator with the injected ``provider`` and ``public_key_b64``.
    /// 3. On success, hot-swap the registry and set status to ``Ready``.
    /// 4. On failure, activate Circuit Breaker mode (``Degraded``).
    ///
    /// # Arguments
    /// * `provider` (&impl [`ISnapshotProvider`]): The authoritative data source.
    /// * `public_key_b64` (&str): The ``Base64`` encoded public key for signature verification.
    ///
    /// # Panics
    /// * Panics if the internal ``metrics`` or ``status`` locks are poisoned.
    pub async fn initialize(&self, provider: &impl ISnapshotProvider, public_key_b64: &str) {
        // [STEP 1]: Record the start of the sync attempt.
        self.metrics.write().expect("LMS-OPS: Sync metrics lock poisoned").last_attempted_sync =
            Self::now_secs();

        // [STEP 2]: Await the repository hydrator.
        match hydrate_snapshot(provider, public_key_b64).await {
            Ok(store) => {
                // [STEP 3a]: On success, hot-swap the registry and set status to Ready.
                self.state.swap_registry(store);
                *self.status.write().expect("LMS-OPS: Status lock poisoned") = SdkState::Ready;
                self.metrics
                    .write()
                    .expect("LMS-OPS: Sync metrics lock poisoned")
                    .last_successful_sync = Self::now_secs();
                info!("LinguisticManager initialized successfully.");
            }
            Err(e) => {
                // [STEP 3b]: On failure, activate Circuit Breaker mode.
                *self.status.write().expect("LMS-OPS: Status lock poisoned") = SdkState::Degraded;
                self.metrics
                    .write()
                    .expect("LMS-OPS: Sync metrics lock poisoned")
                    .sync_error_count += 1;
                error!(
                    "LinguisticManager failed to initialize. Triggering Circuit Breaker. Reason: {e}"
                );
            }
        }
    }

    /// Spawns a ``Tokio`` background worker that periodically polls for registry updates.
    ///
    /// # Logic Trace (Internal)
    /// 1. Clone atomic state and locks for the background thread.
    /// 2. Spawn the detached ``Tokio`` task.
    /// 3. Loop at the specified ``interval_secs``, attempting to fetch and hydrate a new snapshot.
    /// 4. Hot-swap the registry on success, or increment error counts on failure.
    ///
    /// # Arguments
    /// * `interval_secs` (u64): The polling frequency for the background worker.
    /// * `provider` (P): The snapshot provider implementation.
    /// * `public_key_b64` (String): The ``Base64`` public key for validation.
    ///
    /// # Panics
    /// * Panics if the internal ``status`` or ``metrics`` locks are poisoned during background execution.
    #[cfg(feature = "async-worker")]
    pub fn spawn_background_sync<P>(&self, interval_secs: u64, provider: P, public_key_b64: String)
    where
        P: ISnapshotProvider + 'static,
    {
        // [STEP 1]: Clone atomic state and locks for the background thread.
        let state = self.state.clone();
        let status = self.status.clone();
        let metrics = self.metrics.clone();

        // [STEP 2]: Spawn the detached Tokio task
        tokio::spawn(async move {
            info!("Background sync worker started (Interval: {interval_secs}s)");
            let mut interval_timer = time::interval(Duration::from_secs(interval_secs));

            loop {
                // [STEP 3]: Sleep for the requested interval.
                interval_timer.tick().await;

                // [STEP 4]: Attempt to fetch and hydrate a new snapshot.
                match hydrate_snapshot(&provider, &public_key_b64).await {
                    Ok(store) => {
                        state.swap_registry(store);
                        let mut s =
                            status.write().expect("LMS-OPS: Background status lock poisoned");
                        if *s != SdkState::Ready {
                            *s = SdkState::Ready;
                        }
                        info!("Background sync successful. Registry hot-swapped.");
                    }
                    Err(e) => {
                        // CRITICAL: We do NOT set SdkState::Degraded here.
                        // If a background update fails, we keep using the last known good memory pool!
                        metrics
                            .write()
                            .expect("LMS-OPS: Background metrics lock poisoned")
                            .sync_error_count += 1;
                        error!(
                            "Background sync failed. Retaining current registry state. Reason: {e}"
                        );
                    }
                }
            }
        });
    }

    /// Returns the current health status of the ``SDK``.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Returns
    /// * [`SdkState`]: The current operational health status.
    ///
    /// # Panics
    /// * **Panics**: Panics if the internal ``status`` lock is poisoned.
    #[must_use]
    pub fn status(&self) -> SdkState {
        *self.status.read().expect("LMS-OPS: Status lock poisoned")
    }

    /// Returns a copy of the current sync metrics.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Returns
    /// * [`SyncMetrics`]: Thread-safe copy of background sync telemetry.
    ///
    /// # Panics
    /// * **Panics**: Panics if the internal ``metrics`` lock is poisoned.
    #[must_use]
    pub fn metrics(&self) -> SyncMetrics {
        self.metrics.read().expect("LMS-OPS: Metrics lock poisoned").clone()
    }

    /// Returns a copy of the current resolution metrics.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Returns
    /// * [`ResolutionMetrics`]: Thread-safe copy of runtime resolution telemetry.
    ///
    /// # Panics
    /// * **Panics**: Panics if the internal ``resolution_metrics`` lock is poisoned.
    #[must_use]
    pub fn resolution_metrics(&self) -> ResolutionMetrics {
        self.resolution_metrics.read().expect("LMS-OPS: Resolution metrics lock poisoned").clone()
    }

    /// Resolves a raw ``BCP 47`` tag dynamically into an immutable [`CapabilityManifest`].
    ///
    /// Time: `O(N)` based on tag truncation length | Space: `O(1)` beyond returned allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Increment the atomic global metrics counter.
    /// 2. Check health status; if ``Degraded``, yield the hardcoded ``V2.0.0`` fallback.
    /// 3. Delegate valid requests to the ``5-Phase`` pipeline via [`generate_manifest`].
    /// 4. Return the generated manifest or bubble up architectural errors.
    ///
    /// # Arguments
    /// * `tag` (&str): The raw language tag to resolve.
    ///
    /// # Returns
    /// * `Result<CapabilityManifest, LmsError>`: The resolved manifest or a resolution error.
    ///
    /// # Errors
    /// * Returns [`LmsError`] if the pipeline fails during resolution steps.
    ///
    /// # Panics
    /// * Panics if the internal ``resolution_metrics`` lock is poisoned.
    pub fn resolve_capabilities(&self, tag: &str) -> Result<CapabilityManifest, LmsError> {
        // [STEP 1]: Increment operational telemetry
        self.resolution_metrics
            .write()
            .expect("LMS-OPS: Resolution metrics lock poisoned")
            .total_manifests_resolved += 1;

        // [STEP 2]: Check health status; trigger Circuit Breaker if Degraded.
        if self.status() == SdkState::Degraded {
            return Ok(Self::generate_circuit_breaker_manifest());
        }

        // [STEP 3 & 4]: Delegate to the 5-Phase pipeline and return.
        generate_manifest(tag, &self.state)
    }

    /// Generates a guaranteed-safe fallback manifest when the memory pool is unreachable.
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiate a default ``en-US`` manifest.
    /// 2. Inject immutable Linguistic ``DNA`` traits.
    /// 3. Inject algorithmic execution rules (Mandatory for Phase 4 Integrity).
    /// 4. Inject telemetry metadata including the ``circuit_breaker: "true"`` flag.
    /// 5. Return the hardcoded fallback manifest.
    ///
    /// # Returns
    /// * [`CapabilityManifest`]: The hardcoded system default manifest.
    #[must_use]
    fn generate_circuit_breaker_manifest() -> CapabilityManifest {
        // [STEP 1]: Instantiate default "en-US" manifest.
        let mut manifest = CapabilityManifest::new("en-US".to_string());

        // [STEP 2]: Inject V2.0.0 Domain 1 (Traits)
        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
        manifest.traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
        manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));
        manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));

        // [STEP 3]: Inject V2.0.0 Domain 2 (Rules) - Required by Phase 4 Integrity!
        manifest.rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
        manifest
            .rules
            .insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::NONE));

        // [STEP 4]: Inject telemetry metadata.
        manifest.metadata.insert("registry_version".to_string(), "CIRCUIT_BREAKER".to_string());
        manifest.metadata.insert("resolution_path".to_string(), "DEGRADED_FALLBACK".to_string());
        manifest.metadata.insert("resolution_time_ms".to_string(), "0.0000".to_string());
        manifest.metadata.insert("circuit_breaker".to_string(), "true".to_string());

        // [STEP 5]: Return fallback manifest.
        manifest
    }
}

#[cfg(all(test, feature = "simulation"))]
mod tests {
    use super::*;
    use crate::data::repository::SimulatedSnapshotProvider;

    #[test]
    fn test_manager_starts_in_bootstrapping_state() {
        let manager = LinguisticManager::new();
        assert_eq!(manager.status(), SdkState::Bootstrapping);
    }

    #[tokio::test]
    async fn test_manager_initializes_into_ready_state() {
        let manager = LinguisticManager::new();
        let provider = SimulatedSnapshotProvider::new();
        // Dynamically inject the key generated by the simulated provider
        manager.initialize(&provider, &provider.public_key).await;

        assert_eq!(manager.status(), SdkState::Ready);

        let metrics = manager.metrics();
        assert!(metrics.last_successful_sync > 0);
        assert_eq!(metrics.sync_error_count, 0);
    }

    #[tokio::test]
    async fn test_manager_delegates_to_dynamic_pipeline() {
        let manager = LinguisticManager::new();
        let provider = SimulatedSnapshotProvider::new();
        manager.initialize(&provider, &provider.public_key).await;

        let manifest =
            manager.resolve_capabilities("th-TH").expect("LMS-TEST: SDK delegation failed");

        assert_eq!(manifest.resolved_locale, "th-TH");
        assert!(!manifest.traits.is_empty());
        assert_eq!(manager.resolution_metrics().total_manifests_resolved, 1);
    }

    #[tokio::test]
    async fn test_circuit_breaker_intercepts_requests() {
        let manager = LinguisticManager::new();

        // Force the degraded state manually
        *manager.status.write().expect("LMS-TEST: Status lock poisoned") = SdkState::Degraded;

        let manifest =
            manager.resolve_capabilities("ar-EG").expect("LMS-TEST: Circuit breaker failed");

        // Verify V2.0.0 fallback structure
        assert_eq!(manifest.resolved_locale, "en-US");
        assert_eq!(
            manifest.metadata.get("registry_version").expect("LMS-TEST: Missing key"),
            "CIRCUIT_BREAKER"
        );
        assert_eq!(
            manifest.metadata.get("circuit_breaker").expect("LMS-TEST: Missing key"),
            "true"
        );

        // Verify Integrity bypass safety
        assert!(manifest.rules.contains_key("NORMALIZATION_DEFAULT"));
    }
}
