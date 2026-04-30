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

//! # Bistun Linguistic Metadata Service (LMS)
//!
//! A high-performance Linguistic System of Record. This crate transforms complex cultural variables
//! (BCP 47, ISO 639-3, ISO 15924) into functional Typological and Orthographic capabilities.
//!
//! Architectural < 1ms latency budget enforced.

pub mod core;
pub mod manager;
pub mod models;

// Public re-exports for ergonomic SDK consumption
pub use manager::LinguisticManager;
pub use models::manifest::CapabilityManifest;

// Note: To make the API ergonomic, we expose LmsError directly from the resolver layer
// until we build out `src/models/error.rs`.
pub use core::resolver::bcp47::LmsError;
