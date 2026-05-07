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
use crate::data::repository::{ISnapshotProvider, hydrate_snapshot};
use crate::data::swap::RegistryState;
use crate::models::error::LmsError;
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::{Direction, MorphType, SegType, TraitKey};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time;
use tracing::{error, info};

/// Represents the operational health and readiness of the SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SdkState {
    /// The manager is initializing and attempting to load data.
    Bootstrapping,
    /// The manager is fully hydrated and operating normally.
    Ready,
    /// The manager failed to hydrate and is running in Circuit Breaker mode.
    Degraded,
}

/// Tracks the operational health and synchronization history of the capability engine.
///
/// Time: O(1) | Space: O(1)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncMetrics {
    /// Unix timestamp of the last time the worker attempted to fetch a snapshot.
    pub last_attempted_sync: u64,
    /// Unix timestamp of the last time the worker successfully hot-swapped a valid snapshot.
    pub last_successful_sync: u64,
    /// The cumulative number of failed hydration attempts since boot.
    pub sync_error_count: u64,
}

/// The primary interface for generating Linguistic Capability Manifests.
///
/// Time: O(1) reads | Space: O(1) pointer allocations
#[derive(Debug, Clone)]
pub struct LinguisticManager {
    /// The thread-safe dynamic memory state protecting the Flyweight pool.
    state: RegistryState,
    /// The current operational status of the engine. Protected by an RwLock
    /// so the background worker can update it across threads.
    status: Arc<RwLock<SdkState>>,
    /// Thread-safe tracking of background sync health and errors.
    pub metrics: Arc<RwLock<SyncMetrics>>,
}

impl Default for LinguisticManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LinguisticManager {
    /// Initializes a new, empty manager instance in the `Bootstrapping` state.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Instantiates a default `RegistryState`.
    /// 2. Wraps the `SdkState::Bootstrapping` enum in a thread-safe `Arc<RwLock>`.
    /// 3. Returns the prepared struct.
    ///
    /// # Examples
    /// ```text
    /// // See internal `tests` module for hermetic execution.
    /// ```
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `Self`: A newly instantiated `LinguisticManager` ready for async initialization.
    ///
    /// # Golden I/O
    /// * **Input**: `()`
    /// * **Output**: `LinguisticManager { status: Bootstrapping, ... }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous initialization. Note: You must call `initialize()` after creation.
    pub fn new() -> Self {
        Self {
            state: RegistryState::new(),
            status: Arc::new(RwLock::new(SdkState::Bootstrapping)),
            metrics: Arc::new(RwLock::new(SyncMetrics::default())),
        }
    }
    /// Internal helper to get the current Unix timestamp in seconds safely.
    fn now_secs() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
    }

    /// Performs the initial async WORM snapshot hydration via Dependency Injection.
    ///
    /// Time: O(M) where M is the number of locales | Space: O(M) memory pool allocation
    ///
    /// # Logic Trace (Internal)
    /// 1. Awaits the repository hydrator with the injected `provider` and `public_key_b64`.
    /// 2. On success, hot-swaps the registry and sets status to `Ready`.
    /// 3. On failure, activates Circuit Breaker mode (`Degraded`).
    ///
    /// # Examples
    /// ```text
    /// // See internal `tests` module for hermetic execution.
    /// ```
    ///
    /// # Arguments
    /// * `provider` (&impl ISnapshotProvider): The injected provider responsible for supplying the WORM payload.
    /// * `public_key_b64` (&str): The authoritative Ed25519 public key used to verify the snapshot.
    ///
    /// # Returns
    /// * `()`: Mutates the internal state and status.
    ///
    /// # Golden I/O
    /// * **Input**: `&FileSnapshotProvider`, `"Base64_Public_Key"`
    /// * **Output**: `()`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: Internal errors are caught and logged; sets state to Degraded.
    /// * **Panics**: May panic if the internal RwLock is poisoned.
    /// * **Safety**: Safe asynchronous execution.
    ///
    /// # Side Effects
    /// * Mutates the internal `self.status` lock.
    /// * Performs heavy cryptographic operations and deserialization.
    pub async fn initialize(&self, provider: &impl ISnapshotProvider, public_key_b64: &str) {
        self.metrics.write().unwrap().last_attempted_sync = Self::now_secs();

        match hydrate_snapshot(provider, public_key_b64).await {
            Ok(store) => {
                self.state.swap_registry(store);
                *self.status.write().unwrap() = SdkState::Ready;
                self.metrics.write().unwrap().last_successful_sync = Self::now_secs();
                info!("LinguisticManager initialized successfully.");
            }
            Err(e) => {
                *self.status.write().unwrap() = SdkState::Degraded;
                self.metrics.write().unwrap().sync_error_count += 1;
                error!(
                    "LinguisticManager failed to initialize. Triggering Circuit Breaker. Reason: {}",
                    e
                );
            }
        }
    }

    /// Spawns a Tokio background worker that periodically polls for registry updates.
    ///
    /// Time: O(1) to spawn | Space: O(1) setup
    ///
    /// # Logic Trace (Internal)
    /// 1. Clones the atomic memory state, status lock, and moves the provider into the worker thread.
    /// 2. Spawns an infinite asynchronous loop using `tokio::spawn`.
    /// 3. Sleeps for the requested interval using `tokio::time::interval`.
    /// 4. Attempts to fetch and hydrate a new snapshot via the repository hydrator.
    /// 5. On success, executes a wait-free `ArcSwap` to the new data.
    /// 6. On failure, aborts the update but safely retains the old registry data.
    ///
    /// # Examples
    /// ```text
    /// // See internal `tests` module for hermetic execution.
    /// ```
    ///
    /// # Arguments
    /// * `interval_secs` (u64): The duration in seconds to wait between hydration polling attempts.
    /// * `provider` (P): The provider instance to be moved into the background worker. Must outlive the static lifetime.
    /// * `public_key_b64` (String): The authoritative Ed25519 public key used to verify the periodic snapshots.
    ///
    /// # Returns
    /// * `()`: Spawns a detached asynchronous task.
    ///
    /// # Golden I/O
    /// * **Input**: `3600`, `HttpSnapshotProvider`, `"Base64_Public_Key"`
    /// * **Output**: `()`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None returned directly. Background errors are logged via tracing.
    /// * **Panics**: May panic if the internal RwLock is poisoned.
    /// * **Safety**: Requires a running Tokio runtime context.
    ///
    /// # Side Effects
    /// * Spawns a new background thread (`tokio::spawn`).
    pub fn spawn_background_sync<P>(&self, interval_secs: u64, provider: P, public_key_b64: String)
    where
        P: ISnapshotProvider + 'static,
    {
        let state = self.state.clone();
        let status = self.status.clone();
        let metrics = self.metrics.clone();

        tokio::spawn(async move {
            info!("Background sync worker started (Interval: {}s)", interval_secs);
            let mut interval_timer = time::interval(Duration::from_secs(interval_secs));

            loop {
                interval_timer.tick().await;

                match hydrate_snapshot(&provider, &public_key_b64).await {
                    Ok(store) => {
                        state.swap_registry(store);
                        let mut s = status.write().unwrap();
                        if *s != SdkState::Ready {
                            *s = SdkState::Ready;
                        }
                        info!("Background sync successful. Registry hot-swapped.");
                    }
                    Err(e) => {
                        // CRITICAL: We do NOT set SdkState::Degraded here.
                        // If a background update fails, we keep using the last known good memory pool!
                        metrics.write().unwrap().sync_error_count += 1;
                        error!(
                            "Background sync failed. Retaining current registry state. Reason: {}",
                            e
                        );
                    }
                }
            }
        });
    }

    /// Returns the current health status of the SDK.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Acquires a read lock on the status `RwLock`.
    /// 2. Dereferences and returns a copy of the `SdkState`.
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `SdkState`: Current operational readiness (`Bootstrapping`, `Ready`, or `Degraded`).
    pub fn status(&self) -> SdkState {
        *self.status.read().unwrap()
    }

    /// Returns a copy of the current sync metrics.
    pub fn metrics(&self) -> SyncMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Resolves a raw BCP 47 tag dynamically into an immutable CapabilityManifest.
    ///
    /// Time: O(N) based on tag truncation length | Space: O(1) beyond returned map allocations
    ///
    /// # Logic Trace (Internal)
    /// 1. Check health status; if `Degraded`, yield the hardcoded fallback.
    /// 2. Delegate valid requests to the 5-Phase pipeline via `generate_manifest`.
    /// 3. Return the generated manifest or bubble up architectural errors.
    ///
    /// # Arguments
    /// * `tag` (&str): The raw BCP 47 string requested by the consuming application.
    ///
    /// # Returns
    /// * `Result<CapabilityManifest, LmsError>`: The fully resolved capability payload.
    ///
    /// # Golden I/O
    /// * **Input**: `"ar-EG"`
    /// * **Output**: `Ok(CapabilityManifest { resolved_locale: "ar-EG", ... })`
    pub fn resolve_capabilities(&self, tag: &str) -> Result<CapabilityManifest, LmsError> {
        // [STEP 1]: Circuit Breaker Check.
        if self.status() == SdkState::Degraded {
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
    /// 2. Inject Circuit Breaker traits directly into the manifest.
    /// 3. Inject telemetry metadata.
    /// 4. Return the hardcoded fallback manifest.
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

        let manifest = manager.resolve_capabilities("th-TH").expect("SDK delegation failed");

        assert_eq!(manifest.resolved_locale, "th-TH");
        assert!(!manifest.traits.is_empty());
    }

    #[tokio::test]
    async fn test_circuit_breaker_intercepts_requests() {
        let manager = LinguisticManager::new();

        // Force the degraded state manually
        *manager.status.write().unwrap() = SdkState::Degraded;

        let manifest = manager.resolve_capabilities("ar-EG").expect("Circuit breaker failed");

        // Verify fallback to en-US
        assert_eq!(manifest.resolved_locale, "en-US");
        assert_eq!(manifest.metadata.get("registry_version").unwrap(), "CIRCUIT_BREAKER");
    }
}
