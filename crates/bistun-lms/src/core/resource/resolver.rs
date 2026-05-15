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

//! # Resource Resolver
//! Crate: `bistun-lms`
//! Ref: [014-LMS-BRDG], [011-LMS-DTO]
//! Location: `crates/bistun-lms/src/core/resource/resolver.rs`
//!
//! **Why**: This module translates abstract linguistic resource `IDs` into actionable, environment-specific physical `URIs`.
//! **Impact**: If this module fails, downstream clients cannot fetch the physical `ICU4X` data blobs required for rendering and segmenting, breaking complex scripts.
//!
//! ### Glossary
//! * **Resource URI**: The fully qualified `URL` or filepath where binary linguistic data is hosted.

use bistun_core::error::LmsError;
use bistun_core::manifest::CapabilityManifest;

/// Translates abstract resource `IDs` into physical `URIs`, mutating the `resources` map in-place.
///
/// Time: `O(N)` map iteration | Space: `O(1)` string allocations
///
/// # Logic Trace (Internal)
/// 1. Check if the [`CapabilityManifest`] requires any resources. If not, return early.
/// 2. Format the environment-specific `base_uri` to guarantee proper trailing slashes.
/// 3. Iterate over the abstract `IDs` (keys) in the `manifest.resources` map.
/// 4. Concatenate the base `URI` with the abstract `ID` (appending a `.dat` extension).
/// 5. Update the resource value in-place with the fully resolved `URI`.
/// 6. Return success.
///
/// # Examples
/// ```rust
/// # use bistun_core::manifest::CapabilityManifest;
/// # use bistun_lms::core::resource::resolver::resolve_resources;
/// let mut manifest = CapabilityManifest::new("th-TH".to_string());
/// manifest.resources.insert("tri_thai".to_string(), "required".to_string());
///
/// resolve_resources(&mut manifest, "https://cdn.bistun.io/").expect("LMS-TEST: Resolution failed");
/// assert_eq!(
///     manifest.resources.get("tri_thai").expect("LMS-TEST: Key missing"),
///     "https://cdn.bistun.io/tri_thai.dat"
/// );
/// ```
///
/// # Arguments
/// * `manifest` (&mut [`CapabilityManifest`]): The mutable `DTO` currently flowing through the pipeline.
/// * `base_uri` (&str): The environment-specific base `URI` configured in the active memory pool.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful resolution or if no resource is required.
///
/// # Golden I/O
/// * **Input**: `manifest` (with `resources: {"tri_thai": "required"}`), `base_uri: "https://cdn.bistun.io/"`
/// * **Output**: `Ok(())` (manifest resources mutated to `"https://cdn.bistun.io/tri_thai.dat"`)
///
/// # Errors
/// * Conforms to pipeline signature returning [`LmsError`], but currently infallible as it only performs string concatenation.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Safe synchronous execution.
pub fn resolve_resources(
    manifest: &mut CapabilityManifest,
    base_uri: &str,
) -> Result<(), LmsError> {
    // [STEP 1]: Return early if no resources are required (saves < 1ms budget)
    if manifest.resources.is_empty() {
        return Ok(());
    }

    // [STEP 2]: Ensure we format the URL correctly, preventing double slashes
    let formatted_base =
        if base_uri.ends_with('/') { base_uri.to_string() } else { format!("{base_uri}/") };

    // [STEP 3, 4 & 5]: Iterate, format, and update values in-place
    for (resource_id, uri_value) in &mut manifest.resources {
        *uri_value = format!("{formatted_base}{resource_id}.dat");
    }

    // [STEP 6]: Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver_injects_uri_when_resources_present() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create manifest with abstract resource keys
        let mut manifest = CapabilityManifest::new("th-TH".to_string());
        manifest.resources.insert("tri_thai".to_string(), "required".to_string());

        // [STEP 2]: Execute: Resolve with base URI
        let base_uri = "https://cdn.example.com/assets/";
        assert!(resolve_resources(&mut manifest, base_uri).is_ok());

        // [STEP 3]: Assert: Verify the value was mutated in-place
        assert_eq!(
            manifest.resources.get("tri_thai").expect("LMS-TEST: Resource tri_thai missing"),
            "https://cdn.example.com/assets/tri_thai.dat"
        );
    }

    #[test]
    fn test_resolver_handles_missing_trailing_slash() {
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        manifest.resources.insert("icu_arab".to_string(), String::new());

        let base_uri = "https://cdn.example.com/assets"; // Missing trailing slash
        assert!(resolve_resources(&mut manifest, base_uri).is_ok());

        assert_eq!(
            manifest.resources.get("icu_arab").expect("LMS-TEST: Resource icu_arab missing"),
            "https://cdn.example.com/assets/icu_arab.dat"
        );
    }

    #[test]
    fn test_resolver_skips_when_no_resources_required() {
        let mut manifest = CapabilityManifest::new("en-US".to_string());

        let base_uri = "https://cdn.example.com/assets/";
        assert!(resolve_resources(&mut manifest, base_uri).is_ok());

        assert!(manifest.resources.is_empty());
    }
}
