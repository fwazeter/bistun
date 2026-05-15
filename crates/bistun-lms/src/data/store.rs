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

//! # Registry Data Shim
//! Crate: `bistun-lms`
//! Ref: [011-LMS-DTO], [010-LMS-MEM]
//! Location: `crates/bistun-lms/src/data/store.rs`
//!
//! **Why**: This module acts as a bridge between the engine and the foundation. It re-exports the authoritative models from `bistun-core` to ensure type consistency without requiring a massive refactor of internal engine imports.
//! **Impact**: If this bridge is compromised or removed, internal engine modules (like Phase 2 Aggregation) will suffer from type-mismatch errors, breaking the resolution pipeline's ability to ingest `WORM` snapshots.
//!
//! ### Glossary
//! * **Shim**: A module that transparently intercepts or redirects an `API`, providing compatibility or a centralized access point for types.

// [FIX]: Redirect all requests for these types to the centralized DNA foundation
pub use bistun_core::registry::{LocaleProfile, RegistryMetadata, RegistryStore};
