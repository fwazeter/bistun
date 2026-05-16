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
//! Crate: `bistun-lms`
//! Ref: [007-LMS-OPS], [011-LMS-DTO]
//! Location: `crates/bistun-lms/src/ops/telemetry.rs`
//!
//! **Why**: This module serves as Phase 5 (Telemetry) of the pipeline. It stops the execution timer and embeds performance metrics directly into the manifest.
//! **Impact**: Critical for proving compliance with the `< 1ms` architecture budget. If it fails, manifests are returned without an audit trail, blinding observability systems.
//!
//! ### Glossary
//! * **SLI (Service Level Indicator)**: The actual measured metric (e.g., latency in ms) used to determine if the system is meeting its goals.
//! * **Circuit Breaker**: An operational mode indicating the manifest was resolved using fallback/stale data due to a core failure.

use bistun_core::manifest::CapabilityManifest;
use std::time::Instant;

/// Injects high-precision performance metrics and system data into the manifest metadata.
///
/// Time: `O(1)` map insertions | Space: `O(1)` beyond string allocations
///
/// # Logic Trace (Internal)
/// 1. Calculate elapsed time in milliseconds since the `start_time`.
/// 2. Join the vector of attempted tags into a comma-separated string to serialize the path.
/// 3. Insert `registry_version`, `resolution_time_ms`, and `resolution_path` into the manifest's metadata map.
/// 4. If `circuit_breaker` is true, inject the degradation flag.
/// 5. Return seamlessly (in-place mutation).
///
/// # Examples
/// ```rust
/// # use std::time::Instant;
/// # use bistun_core::manifest::CapabilityManifest;
/// # use bistun_lms::ops::telemetry::record_metrics;
/// # let mut manifest = CapabilityManifest::new("ar-EG".to_string());
///   let start = Instant::now();
///   record_metrics(&mut manifest, start, &vec!["ar-EG-u-nu".to_string(), "ar-EG".to_string()], "2.0.0", false);
/// ```
///
/// # Arguments
/// * `manifest` (&mut [`CapabilityManifest`]): The mutable `DTO` receiving the telemetry metadata.
/// * `start_time` (`Instant`): The timestamp captured at the very beginning of the 5-Phase pipeline.
/// * `resolution_path` (`&[String]`): The sequence of `BCP 47` tags evaluated during the Taxonomic Resolution phase.
/// * `registry_version` (&str): The semantic version of the active `RegistryStore` providing the data.
/// * `circuit_breaker` (bool): Flag indicating if the system is running in degraded mode.
///
/// # Returns
/// * `()`: Side-effect function that mutates the manifest in-place.
///
/// # Golden I/O
/// * **Input**: `manifest`, `Instant::now()`, `["en-US-x-test", "en-US"]`, `"2.0.0"`, `true`
/// * **Output**: `()` (Manifest metadata populated with telemetry and `circuit_breaker: "true"`).
///
/// # Errors
/// * None. This function is infallible.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Safe synchronous execution.
///
/// # Side Effects
/// * Mutates the `manifest` in-place by injecting latency strings and taxonomic evaluation path data.
pub fn record_metrics(
    manifest: &mut CapabilityManifest,
    start_time: Instant,
    resolution_path: &[String],
    registry_version: &str,
    circuit_breaker: bool,
) {
    // [STEP 1]: Calculate Elapsed Time (in fractional milliseconds)
    let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;

    // [STEP 2]: Serialize the Taxonomic Path
    let path_str = resolution_path.join(" -> ");

    // [STEP 3]: Hydrate Metadata
    manifest.metadata.insert("registry_version".to_string(), registry_version.to_string());
    manifest.metadata.insert("resolution_time_ms".to_string(), format!("{elapsed_ms:.4}"));
    manifest.metadata.insert("resolution_path".to_string(), path_str);

    // [STEP 4]: Inject Circuit Breaker state if degraded
    if circuit_breaker {
        manifest.metadata.insert("circuit_breaker".to_string(), "true".to_string());
    }

    // [STEP 5]: Return (Implicit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_record_metrics_hydrates_metadata_healthy() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Instantiate manifest, start clock, and simulate processing time.
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let start = Instant::now();

        // Simulate minor artificial processing delay (acceptable for latency validation)
        thread::sleep(Duration::from_micros(50));

        let path = vec!["ar-EG-u-ext".to_string(), "ar-EG".to_string()];

        // [STEP 2]: Execute: Run the telemetry injector (circuit_breaker = false).
        record_metrics(&mut manifest, start, &path, "2.0.0", false);

        // [STEP 3]: Assert: Verify metadata is properly structured and formatted.
        assert_eq!(
            manifest
                .metadata
                .get("registry_version")
                .expect("LMS-TEST: Missing registry_version in metadata"),
            "2.0.0"
        );
        assert_eq!(
            manifest
                .metadata
                .get("resolution_path")
                .expect("LMS-TEST: Missing resolution_path in metadata"),
            "ar-EG-u-ext -> ar-EG"
        );
        assert!(manifest.metadata.contains_key("resolution_time_ms"));
        assert!(!manifest.metadata.contains_key("circuit_breaker"));
    }

    #[test]
    fn test_record_metrics_hydrates_metadata_degraded() {
        let mut manifest = CapabilityManifest::new("th-TH".to_string());
        let start = Instant::now();
        let path = vec!["th-TH".to_string()];

        // Execute with circuit breaker engaged
        record_metrics(&mut manifest, start, &path, "v2.0.0-stale", true);

        assert_eq!(
            manifest
                .metadata
                .get("circuit_breaker")
                .expect("LMS-TEST: Missing circuit_breaker in metadata"),
            "true"
        );
    }
}
