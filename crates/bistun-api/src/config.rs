//! # Environment Configuration Loader
//! Ref: [001-LMS-CORE], [002-LMS-DATA]
//! Location: `crates/bistun-api/src/config.rs`
//!
//! **Why**: Centralizes runtime variables from .env to drive the provider selection logic.
//! **Impact**: If this fails, the SDK defaults to the "Simulated" safety state.

use serde::Deserialize;
use std::env;

/// Primary configuration container for the Bistun Ecosystem.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// The environment mode (development, production, test)
    pub lms_env: String,
    /// URL for the remote WORM registry snapshot
    pub lms_registry_url: Option<String>,
    /// Database connection string for the API service
    pub database_url: Option<String>,
}

impl AppConfig {
    /// Loads configuration from the environment and .env file.
    ///
    /// # Logic Trace
    /// 1. Attempt to load the .env file using `dotenvy`.
    /// 2. Deserialize environment variables into the AppConfig struct.
    /// 3. Provide safe defaults if non-critical variables are missing.
    pub fn load() -> Self {
        // [STEP 1]: Load .env
        let _ = dotenvy::dotenv();

        // [STEP 2]: Construct from Env
        Self {
            lms_env: env::var("LMS_ENV").unwrap_or_else(|_| "development".to_string()),
            lms_registry_url: env::var("LMS_REGISTRY_URL").ok(),
            database_url: env::var("DATABASE_URL").ok(),
        }
    }
}
