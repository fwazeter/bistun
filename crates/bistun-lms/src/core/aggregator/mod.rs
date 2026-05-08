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

//! # Typology Aggregation Domain
//! Ref: [008-LMS-TYPOLOGY-AGGREGATOR]
//! Location: `src/core/aggregator/mod.rs`
//!
//! **Why**: This module coordinates Phase 2 (Aggregate) of the 5-Phase pipeline. It is responsible for mapping a resolved locale to its baseline structural and morphological traits.
//! **Impact**: Failure in this domain results in a `CapabilityManifest` lacking foundational parsing instructions, causing downstream NLP engines to crash or misinterpret linguistic boundaries.
//!
//! ### Glossary
//! * **High-Water Mark**: An architectural strategy used during aggregation where the most computationally expensive requirement (e.g., Dictionary-based segmentation) overrides simpler requirements.
//! * **In-Place Mutation**: The performance-critical practice of modifying the manifest via mutable reference to avoid unnecessary heap allocations during the pipeline.

/// Handles the extraction and high-water mark resolution of typological traits.
pub mod typology;
