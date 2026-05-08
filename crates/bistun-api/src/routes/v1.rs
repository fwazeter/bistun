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

//! # API v1 Route Definitions
//! Ref: [001-LMS-CORE]
//! Location: `crates/bistun-api/src/routes/v1.rs`
//!
//! **Why**: Defines the versioned endpoint map for the capability engine's public interface.
//! **Impact**: If this module is compromised, external consumers cannot reach the resolution handlers, breaking the capability delivery pipeline.
//!
//! ### Glossary
//! * **Route**: A URL pattern mapped to a specific executable handler.

use crate::handlers::capability::resolve_handler;
use axum::{Router, routing::get};
use bistun_lms::LinguisticManager;

/// Constructs the scoped v1 capability router.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Initialize a new Axum `Router` instance.
/// 2. Register the `GET` method for the `/manifest/:locale` path.
/// 3. Map the path to the `resolve_handler` for 5-phase pipeline execution.
/// 4. Return the configured router to the master orchestrator.
///
/// # Examples
/// ```rust
/// // Internal assembly within mod.rs
///    use crate::bistun_api::routes::v1;
///
///    let v1_router = v1::router();
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `Router<LinguisticManager>`: A router specialized for the `LinguisticManager` state, containing v1 endpoints.
///
/// # Golden I/O
/// * **Input**: `()`
/// * **Output**: `Router` (v1 endpoints registered)
///
/// # Errors, Panics, & Safety
/// * **Errors**: Infallible static registration.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn router() -> Router<LinguisticManager> {
    // [STEP 1, 2 & 3]: Initialize and Map
    Router::new().route("/manifest/:locale", get(resolve_handler))
}
