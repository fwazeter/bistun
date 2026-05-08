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

//! # Orthographic Extension Domain
//! Ref: [004-LMS-EXT]
//! Location: `src/core/extension/mod.rs`
//!
//! **Why**: This module coordinates Phase 3 (Override/Extension) of the 5-Phase pipeline. It parses requested Unicode extensions (e.g., `-u-nu-latn`) and merges them with the baseline typological profile.
//! **Impact**: If this domain is bypassed or fails, specific client-side rendering preferences (like numbering systems or calendars) will be ignored, and critical script-direction overrides will not be applied.
//!
//! ### Glossary
//! * **Extension Parsing**: The process of extracting specific Unicode BCP 47 extension subtags to modify the behavioral manifest.
//! * **Mechanical Defaults**: The baseline rendering traits (Direction, Bidi) defined by the script before overrides are applied.

/// Handles the extraction of Unicode extensions and mechanical rendering traits.
pub mod orthography;
