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
/// 1. **Time Calculation**: Calculate elapsed time in milliseconds since the `start_time`.
/// 2. **Path Serialization**: Join the vector of attempted tags into a comma-separated string.
/// 3. **Hydration**: Insert `registry_version`, `resolution_time_ms`, and `resolution_path` into the manifest's metadata map.
/// 4. **Return**: The function mutates in-place.
pub fn record_metrics(
    manifest: &mut CapabilityManifest,
    start_time: Instant,
    resolution_path: &[String],
    registry_version: &str,
) {
    // 1. Calculate Elapsed Time (in fractional milliseconds)
    let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;

    // 2. Serialize the Taxonomic Path
    let path_str = resolution_path.join(" -> ");

    // 3. Hydrate Metadata
    manifest.metadata.insert("registry_version".to_string(), registry_version.to_string());
    manifest.metadata.insert("resolution_time_ms".to_string(), format!("{:.4}", elapsed_ms));
    manifest.metadata.insert("resolution_path".to_string(), path_str);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_record_metrics_hydrates_metadata() {
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        let start = Instant::now();

        // Simulate minor artificial processing delay
        thread::sleep(Duration::from_micros(50));

        let path = vec!["ar-EG-u-ext".to_string(), "ar-EG".to_string()];

        record_metrics(&mut manifest, start, &path, "1.0.0");

        assert_eq!(manifest.metadata.get("registry_version").unwrap(), "1.0.0");
        assert_eq!(manifest.metadata.get("resolution_path").unwrap(), "ar-EG-u-ext -> ar-EG");
        assert!(manifest.metadata.contains_key("resolution_time_ms"));
    }
}
