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

//! # WORM Compiler & Ingestion Engine
//! Ref: [005-LMS-INGEST], [002-LMS-DATA]
//! Location: `src/data/compiler/mod.rs`
//!
//! **Why**: This module processes raw data (ISO/CLDR), applies the Truth Hierarchy, and compiles the finalized, cryptographically signed WORM snapshot.
//! **Impact**: This is a standalone build tool logic domain. It prevents malformed data from ever reaching the runtime memory pools by enforcing typological and orthographic integrity.
//!
//! ### Glossary
//! * **WORM**: Write-Once, Read-Many. A storage philosophy where data is never edited in place; instead, new versions are created to ensure an immutable audit trail.
//! * **Truth Hierarchy**: The deterministic conflict resolution engine (Tier 1: Manual > Tier 2: CLDR > Tier 3: ISO).

pub mod ingest;
pub mod linter;
