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
//! Crate: `bistun-lms`
//! Ref: [002-LMS-DATA]
//! Location: `crates/bistun-lms/src/data/providers.rs`
//!
//! **Why**: This module implements the concrete `I/O` transport mechanisms (Disk, Network) for fetching `WORM` registry payloads. It ensures the operational engine can ingest authoritative data from varied persistence layers.
//! **Impact**: If these providers fail, the system cannot hydrate new linguistic updates from external sources, stranding the service on stale data and preventing the deployment of critical linguistic fixes.
//!
//! ### Glossary
//! * **Payload**: The combined data of the `WORM` `JSON` registry and its cryptographic `JWS` signature.

#[cfg(any(feature = "fs", feature = "network"))]
use crate::data::repository::{ISnapshotProvider, PayloadFuture};
#[cfg(any(feature = "fs", feature = "network"))]
use bistun_core::error::LmsError;

// -----------------------------------------------------------------------------
// File-Based Provider (Gated by 'fs' feature)
// -----------------------------------------------------------------------------

#[cfg(feature = "fs")]
/// A concrete provider that reads `WORM` snapshots from the local filesystem.
#[derive(Debug, Clone)]
pub struct FileSnapshotProvider {
    /// The absolute or relative path to the `.json` snapshot.
    pub json_path: String,
    /// The absolute or relative path to the `.sig` signature file.
    pub sig_path: String,
}

#[cfg(feature = "fs")]
impl FileSnapshotProvider {
    /// Instantiates a new [`FileSnapshotProvider`].
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Logic Trace (Internal)
    /// 1. Map the provided `json_path` and `sig_path` to the internal struct.
    /// 2. Return the configured instance.
    ///
    /// # Examples
    /// ```rust
    /// # use bistun_lms::data::providers::FileSnapshotProvider;
    /// let provider = FileSnapshotProvider::new("data.json".into(), "data.sig".into());
    /// ```
    ///
    /// # Arguments
    /// * `json_path` (`String`): The absolute or relative path to the `.json` snapshot.
    /// * `sig_path` (`String`): The absolute or relative path to the `.sig` signature file.
    ///
    /// # Returns
    /// * `Self`: The configured provider ready for `WORM` hydration.
    ///
    /// # Golden I/O
    /// * **Input**: `"data.json"`, `"data.sig"`
    /// * **Output**: `FileSnapshotProvider { json_path: "data.json", sig_path: "data.sig" }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None during instantiation.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous initialization.
    #[must_use]
    pub fn new(json_path: String, sig_path: String) -> Self {
        // [STEP 1]: Initialize and return.
        Self { json_path, sig_path }
    }
}

#[cfg(feature = "fs")]
impl ISnapshotProvider for FileSnapshotProvider {
    /// Fetches the `JSON` payload and signature from the local disk asynchronously.
    ///
    /// Time: `O(N)` (File Read) | Space: `O(N)` (Memory Buffer)
    ///
    /// # Logic Trace (Internal)
    /// 1. Attempt to read the `JSON` file from disk using `tokio::fs`.
    /// 2. Attempt to read the signature file from disk using `tokio::fs`.
    /// 3. Wrap file `I/O` errors into [`LmsError::PersistenceFault`].
    /// 4. Return the paired payload tuple.
    ///
    /// # Errors
    /// * Returns [`LmsError::PersistenceFault`] if the files are unreachable or unreadable.
    fn fetch_payload(&self) -> PayloadFuture<'_> {
        Box::pin(async move {
            // [STEP 1]: Attempt to read the JSON file from disk.
            let json_payload = tokio::fs::read_to_string(&self.json_path).await.map_err(|e| {
                LmsError::PersistenceFault {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "FileSnapshotProvider".to_string(),
                    reason: format!("Failed to read JSON snapshot: {e}"),
                }
            })?;

            // [STEP 2]: Attempt to read the Signature file from disk.
            let signature = tokio::fs::read_to_string(&self.sig_path).await.map_err(|e| {
                LmsError::PersistenceFault {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "FileSnapshotProvider".to_string(),
                    reason: format!("Failed to read snapshot signature: {e}"),
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

#[cfg(feature = "network")]
/// A concrete provider that fetches `WORM` snapshots from a remote `HTTP` server.
#[derive(Debug, Clone)]
pub struct HttpSnapshotProvider {
    /// The `URL` prefix where the `snapshot.json` and `snapshot.sig` are hosted.
    pub base_url: String,
}

#[cfg(feature = "network")]
impl HttpSnapshotProvider {
    /// Instantiates a new [`HttpSnapshotProvider`].
    ///
    /// Time: `O(1)` | Space: `O(1)`
    ///
    /// # Logic Trace (Internal)
    /// 1. Map the provided base `URL` to the internal struct.
    /// 2. Return the configured instance.
    ///
    /// # Examples
    /// ```rust
    /// # use bistun_lms::data::providers::HttpSnapshotProvider;
    /// let provider = HttpSnapshotProvider::new("[https://cdn.lms.local/v1](https://cdn.lms.local/v1)".into());
    /// ```
    ///
    /// # Arguments
    /// * `base_url` (`String`): The `URL` prefix (e.g., `https://registry.example.com`).
    ///
    /// # Returns
    /// * `Self`: The configured provider ready for remote hydration.
    ///
    /// # Golden I/O
    /// * **Input**: `"https://registry.example.com"`
    /// * **Output**: `HttpSnapshotProvider { base_url: "https://registry.example.com" }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None during instantiation.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous initialization.
    #[must_use]
    pub fn new(base_url: String) -> Self {
        // [STEP 1]: Initialize and return.
        Self { base_url }
    }
}

#[cfg(feature = "network")]
impl ISnapshotProvider for HttpSnapshotProvider {
    /// Fetches the `JSON` payload and signature from a remote server asynchronously.
    ///
    /// Time: `O(N)` (Network Transfer) | Space: `O(N)` (Memory Buffer)
    ///
    /// # Logic Trace (Internal)
    /// 1. Construct target `URLs` for `snapshot.json` and `snapshot.sig`.
    /// 2. Fetch the `JSON` payload using `reqwest`.
    /// 3. Fetch the signature payload using `reqwest`.
    /// 4. Map network or status errors into [`LmsError::PersistenceFault`].
    /// 5. Return the paired payload tuple.
    ///
    /// # Errors
    /// * Returns [`LmsError::PersistenceFault`] if the network is unreachable or the server returns an error status.
    fn fetch_payload(&self) -> PayloadFuture<'_> {
        Box::pin(async move {
            // [STEP 1]: Construct URLs
            let json_url = format!("{}/snapshot.json", self.base_url);
            let sig_url = format!("{}/snapshot.sig", self.base_url);

            // [STEP 2]: Fetch JSON payload asynchronously
            let json_resp = reqwest::get(&json_url)
                .await
                .map_err(|e| LmsError::PersistenceFault {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP request failed for JSON: {e}"),
                })?
                .error_for_status()
                .map_err(|e| LmsError::PersistenceFault {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP status error for JSON: {e}"),
                })?;

            let json_payload = json_resp.text().await.map_err(|e| LmsError::PersistenceFault {
                pipeline_step: "Phase 0: WORM Hydration".to_string(),
                context: "HttpSnapshotProvider".to_string(),
                reason: format!("Failed to extract JSON text: {e}"),
            })?;

            // [STEP 3]: Fetch Signature payload asynchronously
            let sig_resp = reqwest::get(&sig_url)
                .await
                .map_err(|e| LmsError::PersistenceFault {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP request failed for Signature: {e}"),
                })?
                .error_for_status()
                .map_err(|e| LmsError::PersistenceFault {
                    pipeline_step: "Phase 0: WORM Hydration".to_string(),
                    context: "HttpSnapshotProvider".to_string(),
                    reason: format!("HTTP status error for Signature: {e}"),
                })?;

            let signature = sig_resp.text().await.map_err(|e| LmsError::PersistenceFault {
                pipeline_step: "Phase 0: WORM Hydration".to_string(),
                context: "HttpSnapshotProvider".to_string(),
                reason: format!("Failed to extract Signature text: {e}"),
            })?;

            // [STEP 4]: Return the paired payload.
            Ok((json_payload, signature))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "fs")]
    mod fs_tests {
        use super::*;
        use std::io::Write;
        use tempfile::NamedTempFile;

        #[tokio::test]
        async fn test_file_provider_fetches_payload() {
            // [Logic Trace Mapping]
            // [STEP 1]: Setup: Create a temporary file and write a valid JSON stub.
            let mut json_file =
                NamedTempFile::new().expect("LMS-TEST: Failed to create temp JSON file");
            let sig_file = NamedTempFile::new().expect("LMS-TEST: Failed to create temp SIG file");

            writeln!(json_file, "[{{\"id\": \"ar-EG\"}}]")
                .expect("LMS-TEST: Failed to write to temp file");

            let json_path = json_file.path().to_str().expect("LMS-TEST: Invalid path").to_string();
            let sig_path = sig_file.path().to_str().expect("LMS-TEST: Invalid path").to_string();

            // [STEP 2]: Execute: Instantiate provider and await fetch.
            let provider = FileSnapshotProvider::new(json_path, sig_path);
            let result = provider.fetch_payload().await;

            // [STEP 3]: Assert: Verify the payload was read correctly.
            assert!(result.is_ok());
            let (payload, _) = result.expect("LMS-TEST: Provider failed to fetch payload");
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

            // [STEP 3]: Assert: Verify it returns a PersistenceFault.
            assert!(matches!(result, Err(LmsError::PersistenceFault { .. })));
        }
    }
}
