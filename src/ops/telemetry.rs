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

//! # Telemetry & Observability Engine
//! Ref: [007-LMS-OPS], [011-LMS-DTO]
//! Location: `src/ops/telemetry.rs`
//!
//! **Why**: This module serves as Phase 5 (Telemetry) of the pipeline. It stops the execution timer and embeds performance metrics directly into the manifest.
//! **Impact**: Critical for proving compliance with the < 1ms architecture budget. If it fails, manifests are returned without an audit trail.
//!
//! ### Glossary
//! * **SLI (Service Level Indicator)**: The actual measured metric (e.g., latency in ms) used to determine if the system is meeting its goals.

use crate::models::manifest::CapabilityManifest;
use std::time::Instant;

/// Injects high-precision performance metrics and system data into the manifest metadata.
///
/// Time: O(1) map insertions | Space: O(1) beyond string allocations
///
/// # Logic Trace (Internal)
/// 1. Calculate elapsed time in milliseconds since the `start_time`.
/// 2. Join the vector of attempted tags into a comma-separated string to serialize the path.
/// 3. Insert `registry_version`, `resolution_time_ms`, and `resolution_path` into the manifest's metadata map.
/// 4. Return seamlessly (in-place mutation).
///
/// # Examples
/// ```rust
///   let mut manifest = CapabilityManifest::new("ar-EG".to_string());
///   let start = Instant::now();
///   record_metrics(&mut manifest, start, &vec!["ar-EG-u-nu".to_string(), "ar-EG".to_string()], "1.0.0");
/// ```
///
/// # Arguments
/// * `manifest` (&mut CapabilityManifest): The mutable DTO receiving the telemetry metadata.
/// * `start_time` (Instant): The timestamp captured at the very beginning of the 5-Phase pipeline.
/// * `resolution_path` (&[String]): The sequence of BCP 47 tags evaluated during the Taxonomic Resolution phase.
/// * `registry_version` (&str): The semantic version of the active `RegistryStore` providing the data.
///
/// # Returns
/// * `()`: Side-effect function that mutates the manifest in-place.
///
/// # Golden I/O
/// * **Input**: `manifest`, `Instant::now()`, `["en-US-x-test", "en-US"]`, `"1.0.0"`
/// * **Output**: `()` (Manifest metadata map populated with telemetry).
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn record_metrics(
    manifest: &mut CapabilityManifest,
    start_time: Instant,
    resolution_path: &[String],
    registry_version: &str,
) {
    // [STEP 1]: Calculate Elapsed Time (in fractional milliseconds)
    let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;

    // [STEP 2]: Serialize the Taxonomic Path
    let path_str = resolution_path.join(" -> ");

    // [STEP 3]: Hydrate Metadata
    manifest.metadata.insert("registry_version".to_string(), registry_version.to_string());
    manifest.metadata.insert("resolution_time_ms".to_string(), format!("{:.4}", elapsed_ms));
    manifest.metadata.insert("resolution_path".to_string(), path_str);

    // [STEP 4]: Return (Implicit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_record_metrics_hydrates_metadata() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate manifest, start clock, and simulate processing time.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let start = Instant::now();

        // Simulate minor artificial processing delay (acceptable for latency validation)
        thread::sleep(Duration::from_micros(50));

        let path = vec!["ar-EG-u-ext".to_string(), "ar-EG".to_string()];

        // [STEP 2]: Execute: Run the telemetry injector.
        record_metrics(&mut manifest, start, &path, "1.0.0");

        // [STEP 3]: Assert: Verify metadata is properly structured and formatted.
        assert_eq!(manifest.metadata.get("registry_version").unwrap(), "1.0.0");
        assert_eq!(manifest.metadata.get("resolution_path").unwrap(), "ar-EG-u-ext -> ar-EG");
        assert!(manifest.metadata.contains_key("resolution_time_ms"));
    }
}
