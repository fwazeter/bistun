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

//! # API Handlers Domain
//! Ref: [001-LMS-CORE]
//! Location: `crates/bistun-api/src/handlers/mod.rs`
//!
//! **Why**: This module encapsulates the isolated HTTP handlers for the sidecar API. It decouples Axum routing definitions from the actual business logic of executing engine capabilities.
//! **Impact**: Prevents `main.rs` from ballooning into an unmaintainable monolith, allowing for clean versioning (v1, v2) of the capability endpoints.
//!
//! ### Glossary
//! * **Handler**: An asynchronous function responsible for processing an incoming HTTP request and returning a response.
//! * **Sidecar**: A service pattern where an auxiliary application (this API) runs alongside a primary application to provide specialized capabilities.

pub mod capability;
pub mod health;
