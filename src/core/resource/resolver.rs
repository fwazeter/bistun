// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # Resource Resolver
//! Ref: [001-LMS-CORE], [002-LMS-DATA]
//! Location: `src/core/resource/resolver.rs`
//!
//! **Why**: This module translates abstract linguistic resource IDs into actionable, environment-specific physical URIs.
//! **Impact**: If this module fails, downstream clients cannot fetch the physical ICU4X data blobs required for rendering and segmenting.
//!
//! ### Glossary
//! * **Resource URI**: The fully qualified URL or filepath where binary linguistic data is hosted.

use crate::models::error::LmsError;
use crate::models::manifest::{CapabilityManifest, TraitValue};
use crate::models::traits::TraitKey;

/// Translates an abstract resource ID into a physical URI and injects it into the manifest metadata.
///
/// Time: O(1) map lookups | Space: O(1) string allocation
///
/// # Logic Trace (Internal)
/// 1. Check if the `CapabilityManifest` contains a `TraitKey::ResourceId`.
/// 2. If present, extract the abstract string value.
/// 3. Format the environment-specific `base_uri` to guarantee proper slashes.
/// 4. Concatenate the base URI with the abstract ID (appending a `.dat` extension).
/// 5. Inject the fully resolved `resource_uri` into the manifest's `metadata` dictionary.
/// 6. Return success.
///
/// # Examples
/// ```text
/// // See internal `tests` module for hermetic execution.
/// ```
///
/// # Arguments
/// * `manifest` (&mut CapabilityManifest): The mutable DTO currently flowing through the pipeline.
/// * `base_uri` (&str): The environment-specific base URI configured in the active memory pool.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful resolution or if no resource is required.
///
/// # Golden I/O
/// * **Input**: `manifest` (with `ResourceId: "tri_thai"`), `base_uri: "https://cdn.bistun.io/"`
/// * **Output**: `Ok(())` (manifest metadata contains `"resource_uri": "https://cdn.bistun.io/tri_thai.dat"`)
///
/// # Errors, Panics, & Safety
/// * **Errors**: Conforms to pipeline signature returning `Result`, but currently infallible.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn resolve_resources(
    manifest: &mut CapabilityManifest,
    base_uri: &str,
) -> Result<(), LmsError> {
    // [STEP 1 & 2]: Check for ResourceId
    if let Some(TraitValue::String(resource_id)) = manifest.traits.get(&TraitKey::ResourceId) {
        // [STEP 3]: Ensure we format the URL correctly, preventing double slashes
        let formatted_base =
            if base_uri.ends_with('/') { base_uri.to_string() } else { format!("{}/", base_uri) };

        // [STEP 4]: Concatenate abstract ID into a physical URI
        let resolved_uri = format!("{}{}.dat", formatted_base, resource_id);

        // [STEP 5]: Inject into the metadata side-channel
        manifest.metadata.insert("resource_uri".to_string(), resolved_uri);
    }

    // [STEP 6]: Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::traits::Direction;

    #[test]
    fn test_resolver_injects_uri_when_resource_id_present() {
        let mut manifest = CapabilityManifest::new("th-TH".to_string());
        manifest.traits.insert(TraitKey::ResourceId, TraitValue::String("tri_thai".to_string()));

        let base_uri = "https://cdn.example.com/assets/";
        assert!(resolve_resources(&mut manifest, base_uri).is_ok());

        assert_eq!(
            manifest.metadata.get("resource_uri").unwrap(),
            "https://cdn.example.com/assets/tri_thai.dat"
        );
    }

    #[test]
    fn test_resolver_handles_missing_trailing_slash() {
        let mut manifest = CapabilityManifest::new("ar-EG".to_string());
        manifest.traits.insert(TraitKey::ResourceId, TraitValue::String("icu_arab".to_string()));

        let base_uri = "https://cdn.example.com/assets"; // Missing trailing slash
        assert!(resolve_resources(&mut manifest, base_uri).is_ok());

        assert_eq!(
            manifest.metadata.get("resource_uri").unwrap(),
            "https://cdn.example.com/assets/icu_arab.dat"
        );
    }

    #[test]
    fn test_resolver_skips_when_no_resource_id_required() {
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));

        let base_uri = "https://cdn.example.com/assets/";
        assert!(resolve_resources(&mut manifest, base_uri).is_ok());

        assert!(manifest.metadata.get("resource_uri").is_none());
    }
}
