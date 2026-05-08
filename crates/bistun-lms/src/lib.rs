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

//! # Capability Engine Root
//! Ref: [012-LMS-ENG]
//! Location: `crates/bistun-lms/src/lib.rs`
//!
//! **Why**: This crate root centralizes the 5-phase capability engine's operational modules and exposes the primary SDK entry point. It serves as the orchestrator for resolving BCP 47 tags into actionable Linguistic DNA.
//! **Impact**: If this root is compromised or misconfigured, external consumers cannot access the `LinguisticManager`, rendering the entire capability resolution service unreachable and breaking downstream UI/NLP integrations.
//!
//! ### Glossary
//! * **Orchestration**: The coordination of multiple sub-engines (Taxonomy, Typology, etc.) into a unified result.
//! * **SDK (Software Development Kit)**: The public interface providing tools and libraries for developers to integrate Bistun capabilities.

/// The 5-phase resolution pipeline logic.
pub mod core;

/// WORM hydration and memory pool management.
pub mod data;

/// The primary SDK interface for external consumers.
pub mod manager;

/// System observability and performance telemetry.
pub mod ops;

/// Cryptographic verification and JWS gates.
pub mod security;

/// Runtime and pre-persistence integrity checks.
pub mod validation;

// [Logic Trace: Re-exports]
// To maintain a "Living Document" and ergonomic API, we surface the primary
// operational models at the crate root.

// Re-export the operational status enum for SDK health monitoring.
pub use bistun_core::ops::SdkState;

// Re-export the primary orchestrator for capability resolution.
pub use manager::LinguisticManager;
