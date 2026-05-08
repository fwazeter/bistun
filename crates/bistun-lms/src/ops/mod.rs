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

//! # Operations & Telemetry Domain
//! Ref: [007-LMS-OPS]
//! Location: `src/ops/mod.rs`
//!
//! **Why**: This module handles system observability, performance metrics, and manifest metadata injection for the 5-phase pipeline.
//! **Impact**: If this module fails, we lose visibility into system latency and resolution paths, breaching our Service Level Indicators (SLIs) and making performance regressions impossible to audit.
//!
//! ### Glossary
//! * **SLI (Service Level Indicator)**: A quantitative measure of some aspect of the level of service that is provided (e.g., resolution latency)[cite: 24].
//! * **Span**: An interval of time representing a discrete logical operation within the pipeline.

/// Submodule responsible for injecting latency and resolution metadata into the manifest.
pub mod telemetry;
