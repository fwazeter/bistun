// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

#![cfg(feature = "ops")]

//! # Operational Models
//! Ref: [007-LMS-OPS]
//! Location: `crates/bistun-core/src/ops.rs`
//!
//! **Why**: This module centralizes the Service Level models required for SDK operational monitoring and telemetry.
//! **Impact**: Defines the standard states the system can exist in and the metrics reported to observability sinks.

use serde::{Deserialize, Serialize};

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
