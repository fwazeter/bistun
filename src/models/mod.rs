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

//! # Shared Data Models (DTOs)
//! Ref: [011-LMS-DTO]
//!
//! **Why**: This module serves as the central hub for the system's Data Transfer Objects (DTOs) and shared vocabulary.
//! **Impact**: It defines the "Contract Layer" of the service; any breaking changes here will propagate to all consuming sidecars and SDKs.
//!
//! ### Glossary
//! * **Re-export**: A technique to provide a more ergonomic API by exposing items from submodules at the root level.
//! * **Flattening**: Reducing the depth of the module path required to access a type.

pub mod manifest;
pub mod traits;

// Re-export core DTO for ergonomic API usage
pub use manifest::{CapabilityManifest, TraitValue};

// Re-export the shared vocabulary
pub use traits::{Direction, MorphType, SegType, TraitKey};
