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

//! # Validation Domain
//! Ref: [003-LMS-VAL]
//! Location: `src/validation/mod.rs`
//!
//! **Why**: This module acts as the QA gatekeeper for the capability engine, organizing validators into performance-tiered levels (Level A vs Level C).
//! **Impact**: Prevents corrupted or mechanically impossible trait combinations from reaching downstream rendering systems, ensuring the integrity of the "Linguistic DNA" model.
//!
//! ### Glossary
//! * **Tiered Enforcement**: The strategy of applying strict validation during ingestion (Level A) and lightweight checks during runtime resolution (Level C).
//! * **Linter**: The static analysis component used to verify typological consistency in snapshots.

// Level C: Runtime lightweight checks (Hot-path optimized)
pub mod integrity;

// Level A: Strict pre-persistence checks (Offline/Ingestion optimized)
// Note: DNA validation is currently handled via the compiler linter to preserve the < 1ms budget.
pub use crate::data::compiler::linter as dna;
