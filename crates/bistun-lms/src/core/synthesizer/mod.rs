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

//! # Synthesizer Domain
//! Crate: `bistun-lms`
//! Ref: [013-LMS-RULE]
//! Location: `crates/bistun-lms/src/core/synthesizer/mod.rs`
//!
//! **Why**: This domain encapsulates the "Logic Bridge" (Phase 2.5) of the resolution pipeline, responsible for translating structural typological traits into actionable software rules.
//! **Impact**: If this domain is compromised or decoupled, the `CapabilityManifest` will lack operational directives, forcing consuming services to guess how to execute linguistic algorithms, leading to system-wide crashes for complex scripts.
//!
//! ### Glossary
//! * **Logic Bridge**: The conceptual phase where static metadata (Traits) is mapped to dynamic capabilities (Rules).
//! * **Synthesis**: The $O(1)$ derivation of a functional command from a static ISO 639-3 trait.

pub mod rules;
