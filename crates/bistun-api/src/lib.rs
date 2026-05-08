// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV
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

//! # API Sidecar Library
//! Ref: [001-LMS-CORE]
//! Location: `crates/bistun-api/src/lib.rs`
//!
//! **Why**: This module serves as the primary integration boundary for the Bistun API sidecar. It exposes modularized logic for environment configuration, error mapping, and HTTP route handling to support both binary execution and integration testing.
//! **Impact**: If this library root is compromised, the sidecar will fail to compile or expose its public interface, preventing external services from accessing linguistic capabilities.
//!
//! ### Glossary
//! * **Sidecar**: A service pattern where this API runs alongside a primary application to provide specialized capability resolution.
//! * **Modularized Logic**: The practice of separating concerns into discrete sub-modules (Handlers, Routes, Config) to prevent monolith bloat.

pub mod config;
pub mod error;
pub mod handlers;
pub mod routes;
