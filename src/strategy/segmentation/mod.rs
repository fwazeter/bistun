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

//! # Segmentation Strategies
//! Ref: [009-LMS-STRAT]
//! Location: `src/strategy/segmentation/mod.rs`
//!
//! **Why**: This module serves as the registry boundary for all orthographic segmentation algorithms (e.g., Space, Dictionary, Character).
//! **Impact**: Without this module, the `SegmentationProvider` cannot route linguistic metadata to concrete execution logic, breaking NLP text processing capabilities.
//!
//! ### Glossary
//! * **Module Tree**: Rust's hierarchical structure for organizing code. This file explicitly exports the localized segmentation algorithms to the broader strategy factory.

pub mod character;
pub mod dictionary;
pub mod none;
pub mod space;
