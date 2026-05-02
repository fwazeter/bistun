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

//! # The Core Engine Domain
//! Ref: [001-LMS-CORE]
//! Location: `src/core/mod.rs`
//!
//! **Why**: This module acts as the primary orchestrator for the 5-Phase Capability Pipeline. It routes requests through the Taxonomic Resolver, Typological Aggregator, and Orthographic Extension mapper to synthesize a finalized manifest.
//! **Impact**: If this module or its internal routing is misconfigured, the service boundaries will fail to communicate, preventing the transformation of raw BCP 47 tags into actionable rendering instructions.
//!
//! ### Glossary
//! * **Capability Pipeline**: The sequential execution of 5 distinct phases (Resolve, Aggregate, Override, Integrity, Telemetry) to hydrate a manifest.
//! * **Orchestration**: The process of coordinating sub-engine execution without embedding specific business logic in the dispatcher.

/// Phase 2: Handles the merging of typological traits from the Flyweight pool.
pub mod aggregator;

/// Phase 3: Manages Unicode extensions and mechanical script overrides.
pub mod extension;

/// The central coordinator executing the full 5-Phase pipeline.
pub mod pipeline;

/// Phase 1: Resolves BCP 47 tags via the Chain of Responsibility.
pub mod resolver;
