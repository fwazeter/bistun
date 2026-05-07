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

//! # WORM Snapshot Providers
//! Ref: [002-LMS-DATA]
//! Location: `src/data/providers.rs`
//!
//! **Why**: This module implements the concrete I/O transport mechanisms (Disk, Network) for fetching WORM registry payloads.
//! **Impact**: If these providers fail, the system cannot hydrate new linguistic updates from external sources, stranding the service on stale data.
//!
//! ### Glossary
//! * **Payload**: The combined data of the WORM JSON registry and its cryptographic JWS signature.

use crate::data::repository::{ISnapshotProvider, PayloadFuture};
use crate::models::error::LmsError;

// -----------------------------------------------------------------------------
// File-Based Provider
// -----------------------------------------------------------------------------

/// A concrete provider that reads WORM snapshots from the local filesystem.
#[derive(Debug, Clone)]
pub struct FileSnapshotProvider {
    json_path: String,
    sig_path: String,
}

impl FileSnapshotProvider {
    /// Instantiates a new FileSnapshotProvider.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Map the provided paths to the internal struct.
    ///
    /// # Examples
    /// ```rust
    ///   let provider = FileSnapshotProvider::new("data.json".into(), "data.sig".into());
    /// ```
    ///
    /// # Arguments
    /// * `json_path` (String): The absolute or relative path to the `.json` snapshot.
    /// * `sig_path` (String): The absolute or relative path to the `.sig` signature file.
    ///
    /// # Returns
    /// * `Self`: The configured provider.
    ///
    /// # Golden I/O
    /// * **Input**: `"data.json"`, `"data.sig"`
    /// * **Output**: `FileSnapshotProvider { json_path: "data.json", sig_path: "data.sig" }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous execution.
    pub fn new(json_path: String, sig_path: String) -> Self {
        // [STEP 1]: Initialize and return.
        Self { json_path, sig_path }
    }
}

impl ISnapshotProvider for FileSnapshotProvider {
    fn fetch_payload(&self) -> PayloadFuture<'_> {
        Box::pin(async move {
            // [STEP 1]: Attempt to read the JSON file from disk.
            let json_payload = tokio::fs::read_to_string(&self.json_path).await.map_err(|e| {
                LmsError::IntegrityViolation {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "FileSnapshotProvider".to_string(),
                    reason: format!("Failed to read JSON snapshot: {}", e),
                }
            })?;

            // [STEP 2]: Attempt to read the Signature file from disk.
            let signature = tokio::fs::read_to_string(&self.sig_path).await.map_err(|e| {
                LmsError::IntegrityViolation {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "FileSnapshotProvider".to_string(),
                    reason: format!("Failed to read snapshot signature: {}", e),
                }
            })?;

            // [STEP 3]: Return the paired payload.
            Ok((json_payload, signature))
        })
    }
}

// -----------------------------------------------------------------------------
// Network-Based Provider
// -----------------------------------------------------------------------------

/// A concrete provider that fetches WORM snapshots from a remote HTTP server.
#[derive(Debug, Clone)]
pub struct HttpSnapshotProvider {
    base_url: String,
}

impl HttpSnapshotProvider {
    /// Instantiates a new HttpSnapshotProvider.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Map the provided base URL to the internal struct.
    ///
    /// # Examples
    /// ```rust
    ///   let provider = HttpSnapshotProvider::new("[https://cdn.lms.local/v1](https://cdn.lms.local/v1)".into());
    /// ```
    ///
    /// # Arguments
    /// * `base_url` (String): The URL prefix where the `snapshot.json` and `snapshot.sig` are hosted.
    ///
    /// # Returns
    /// * `Self`: The configured provider.
    ///
    /// # Golden I/O
    /// * **Input**: `"https://registry.example.com"`
    /// * **Output**: `HttpSnapshotProvider { base_url: "https://registry.example.com" }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous execution.
    pub fn new(base_url: String) -> Self {
        // [STEP 1]: Initialize and return.
        Self { base_url }
    }
}

impl ISnapshotProvider for HttpSnapshotProvider {
    fn fetch_payload(&self) -> PayloadFuture<'_> {
        Box::pin(async move {
            // [STEP 1]: Construct URLs
            let json_url = format!("{}/snapshot.json", self.base_url);
            let sig_url = format!("{}/snapshot.sig", self.base_url);

            // [STEP 2]: Fetch JSON payload asynchronously
            let json_resp = reqwest::get(&json_url)
                .await
                .map_err(|e| LmsError::IntegrityViolation {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP request failed for JSON: {}", e),
                })?
                .error_for_status()
                .map_err(|e| LmsError::IntegrityViolation {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP status error for JSON: {}", e),
                })?;

            let json_payload =
                json_resp.text().await.map_err(|e| LmsError::IntegrityViolation {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("Failed to extract JSON text: {}", e),
                })?;

            // [STEP 3]: Fetch Signature payload asynchronously
            let sig_resp = reqwest::get(&sig_url)
                .await
                .map_err(|e| LmsError::IntegrityViolation {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP request failed for Signature: {}", e),
                })?
                .error_for_status()
                .map_err(|e| LmsError::IntegrityViolation {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP status error for Signature: {}", e),
                })?;

            let signature = sig_resp.text().await.map_err(|e| LmsError::IntegrityViolation {
                pipeline_step: "Phase 0: WORM Hydration".to_string(),
                context: "HttpSnapshotProvider".to_string(),
                reason: format!("Failed to extract Signature text: {}", e),
            })?;

            // [STEP 4]: Return the paired payload.
            Ok((json_payload, signature))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test] // NEW: Async test harness
    async fn test_file_provider_fetches_payload() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create a temporary file and write a valid JSON stub.
        let mut json_file = NamedTempFile::new().unwrap();
        let sig_file = NamedTempFile::new().unwrap();

        writeln!(json_file, "[{{\"id\": \"ar-EG\"}}]").unwrap();

        let json_path = json_file.path().to_str().unwrap().to_string();
        let sig_path = sig_file.path().to_str().unwrap().to_string();

        // [STEP 2]: Execute: Instantiate provider and await fetch.
        let provider = FileSnapshotProvider::new(json_path, sig_path);
        let result = provider.fetch_payload().await;

        // [STEP 3]: Assert: Verify the payload was read correctly.
        assert!(result.is_ok());
        let (payload, _) = result.unwrap();
        assert!(payload.contains("ar-EG"));
    }

    #[tokio::test]
    async fn test_file_provider_fails_gracefully_on_missing_file() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Point to non-existent files.
        let provider =
            FileSnapshotProvider::new("does_not_exist.json".into(), "missing.sig".into());

        // [STEP 2]: Execute: Attempt fetch.
        let result = provider.fetch_payload().await;

        // [STEP 3]: Assert: Verify it returns an IntegrityViolation.
        assert!(matches!(result, Err(LmsError::IntegrityViolation { .. })));
    }
}
