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

//! # Bistun Linguistic Metadata Service (LMS)
//! Ref: [001-LMS-CORE]
//! Location: `src/lib.rs`
//!
//! **Why**: This crate serves as the primary library entry point for the Bistun Linguistic System of Record. It provides the mechanisms to transform complex cultural variables (BCP 47 tags) into functional, machine-readable Typological and Orthographic capabilities.
//!
//! **Impact**: This is the "Public Contract" of the service. Any changes to the re-exports or module visibility here directly impact the SDK stability for sidecars and downstream NLP applications.
//!
//! ### Glossary
//! * **LMS (Linguistic Metadata Service)**: A high-performance capability engine designed to resolve linguistic traits within a strictly enforced **< 1ms** latency budget.
//! * **System of Record**: The authoritative source of truth for linguistic data, ensuring consistency across all distributed systems.

// --- Internal Domains ---

/// The central resolution engine and 5-Phase pipeline coordinator.
pub mod core;

/// Memory management, Flyweight pools, and WORM persistence.
pub mod data;

/// High-level service management and operational orchestration.
pub mod manager;

/// Shared Data Transfer Objects (DTOs) and vocabulary.
pub mod models;

/// Performance telemetry and system observability.
pub mod ops;

/// Cryptographic verification and registry security.
pub mod security;

/// Multi-tier runtime and compile-time integrity checks.
pub mod validation;

pub mod strategy;

// --- Public Re-exports ---

// These exports define the ergonomic SDK boundary for the service.
pub use manager::LinguisticManager;
pub use models::manifest::CapabilityManifest;

// Note: To make the API ergonomic, we expose LmsError directly from the resolver layer
// until we build out `src/models/error.rs` in Phase 6.
pub use core::resolver::bcp47::LmsError;
