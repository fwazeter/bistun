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

//! # Environment Configuration Loader
//! Ref: [001-LMS-CORE], [002-LMS-DATA]
//! Location: `crates/bistun-api/src/config.rs`
//!
//! **Why**: This module is responsible for ingesting runtime variables from the host environment or local `.env` files. It ensures the API sidecar has the necessary keys and URLs required to hydrate the capability engine.
//! **Impact**: Failure in this module prevents the service from booting or causes it to operate in a "Degraded" state due to missing cryptographic keys or registry URLs.
//!
//! ### Glossary
//! * **Environment Variable**: A dynamic-named value that can affect the way running processes behave on a computer.
//! * **Dotenv**: A zero-dependency module that loads environment variables from a `.env` file into `process.env`.

use serde::Deserialize;
use std::env;

/// The primary configuration container for the Bistun API sidecar.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// The deployment environment identifier (e.g., "production", "development").
    pub lms_env: String,
    /// The remote URL pointing to the authoritative WORM registry snapshot.
    pub lms_registry_url: Option<String>,
    /// The connection string for the primary system database.
    pub database_url: Option<String>,
    /// The Ed25519 Public Key used to verify the integrity of the WORM payload.
    pub curator_public_key: String,
}

impl AppConfig {
    /// Loads the configuration from system environment variables and optional local files.
    ///
    /// Time: O(1) | Space: O(N) where N is the total size of configuration strings.
    ///
    /// # Logic Trace (Internal)
    /// 1. Initialize the environment by attempting to load a local `.env` file if the `dotenv` feature is enabled.
    /// 2. Ingest `LMS_ENV` with a fallback to "development".
    /// 3. Ingest optional variables `LMS_REGISTRY_URL` and `DATABASE_URL`.
    /// 4. Ingest the mandatory `CURATOR_PUBLIC_KEY`, triggering a process abort if missing.
    /// 5. Return the synthesized [`AppConfig`] struct.
    ///
    /// # Examples
    /// ```rust
    /// # use bistun_api::config::AppConfig;
    /// # // [STEP 0]: Inject dummy key to satisfy doctest environment
    /// # std::env::set_var("CURATOR_PUBLIC_KEY", "dummy_verification_key");
    /// #
    /// // Standard initialization during sidecar boot
    /// let config = AppConfig::load();
    /// assert_eq!(config.lms_env, "development");
    /// ```
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `Self`: A populated configuration instance ready for engine initialization.
    ///
    /// # Golden I/O
    /// * **Input**: `ENV { CURATOR_PUBLIC_KEY: "..." }`
    /// * **Output**: `AppConfig { curator_public_key: "...", ... }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None returned; the function favors "Fail-Fast" behavior for security.
    /// * **Panics**: Panics if `CURATOR_PUBLIC_KEY` is not present in the environment.
    /// * **Safety**: Safe synchronous execution.
    pub fn load() -> Self {
        // [STEP 1]: Conditionally load .env for local development
        #[cfg(feature = "dotenv")]
        let _ = dotenvy::dotenv();

        // [STEP 2, 3 & 4]: Construct from Env
        Self {
            lms_env: env::var("LMS_ENV").unwrap_or_else(|_| "development".to_string()),
            lms_registry_url: env::var("LMS_REGISTRY_URL").ok(),
            database_url: env::var("DATABASE_URL").ok(),
            curator_public_key: env::var("CURATOR_PUBLIC_KEY")
                .expect("CRITICAL: CURATOR_PUBLIC_KEY must be set in env to verify payloads"),
        }
    }
}
