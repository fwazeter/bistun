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

//! # Shared Data Models (DTOs) & Foundation
//! Crate: bistun-core
//! Ref: [011-LMS-DTO]
//! Location: `crates/bistun-core/src/lib.rs`
//!
//! **Why**: This crate serves as the authoritative central hub for the system's Data Transfer Objects (DTOs) and shared vocabulary. It flattens the internal module hierarchy to provide a clean, ergonomic Public API for all consuming crates and sidecars.
//! **Impact**: This module defines the "Contract Layer" of the entire Bistun ecosystem; any breaking changes here will instantly propagate across the service boundary, potentially corrupting serialization logic in all downstream SDKs.
//!
//! ### Glossary
//! * **Re-export**: A technique to provide a more ergonomic API by exposing items from submodules at the root level.
//! * **Contract Layer**: The set of immutable data structures that define the communication protocol between the LMS engine and its clients.

pub mod error;
pub mod manifest;
pub mod traits;

// [Persistence Domain]: Opt-in for Engine and Persistence tools
#[cfg(feature = "persistence")]
pub mod registry;

// [Operations Domain]: Opt-in for Observability and API tools
#[cfg(feature = "ops")]
pub mod ops;

// [Testing Domain]: Opt-in for QA and Simulation
#[cfg(feature = "testing")]
pub mod simulation;

// Re-export core DTO for ergonomic API usage
pub use manifest::{CapabilityManifest, TraitValue};

// Re-export the shared vocabulary (Updated for v2.0.0 Logic Provider)
pub use traits::{
    CasingRule, Direction, LmsRule, MorphType, NormRule, PluralRule, SegType, TraitKey, TransRule,
};

// Re-export Errors
pub use error::LmsError;

// Re-export Persistence Models
#[cfg(feature = "persistence")]
pub use registry::{LocaleProfile, RegistryMetadata, RegistryStore, WormPayload};

// Re-export Operational Models
#[cfg(feature = "ops")]
pub use ops::{ResolutionMetrics, SdkState, SyncMetrics};

// Testing Re-exports
#[cfg(feature = "testing")]
pub use simulation::*;
