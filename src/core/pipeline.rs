// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # The Capability Engine Coordinator
//! Ref: [001-LMS-CORE]
//!
//! **Why**: This module executes the 5-Phase pipeline to transform a BCP 47 string into an immutable `CapabilityManifest` using the dynamic memory pool.
//! **Impact**: If this module fails, the entire capability engine is disconnected.

use crate::core::aggregator::typology;
use crate::core::extension::orthography;
use crate::core::resolver::bcp47::{self, LmsError};
use crate::data::swap::RegistryState;
use crate::models::manifest::CapabilityManifest;
use crate::ops::telemetry;
use crate::validation::integrity;
use std::time::Instant;

/// Orchestrates the 5-Phase capability pipeline.
///
/// Time: O(N) where N is tag truncation length | Space: O(1) map allocations
///
/// # Logic Trace (Internal)
/// 1. **Phase 1 (Resolve)**: Pass the raw tag and state to the Taxonomic resolver.
/// 2. **Fetch Profile**: Retrieve the exact `LocaleProfile` from the dynamic Flyweight pool.
/// 3. **Phase 2 (Aggregate)**: Hydrate Typology traits directly from the `LocaleProfile`.
/// 4. **Phase 3 (Override)**: Hydrate Orthography mechanics directly from the `LocaleProfile`.
/// 5. **Phase 4 (Integrity)**: Verify the fully aggregated manifest for mechanical contradictions.
/// 6. **Phase 5 (Telemetry)**: Record the pipeline duration and resolution path.
/// 7. **Return**: Yield the hydrated and validated manifest.
pub fn generate_manifest(
    raw_tag: &str,
    state: &RegistryState,
) -> Result<CapabilityManifest, LmsError> {
    // START THE CLOCK
    let start_time = Instant::now();

    // Phase 1: Resolve (Taxonomy) uses dynamic state to find the truncation bound
    let locale = bcp47::resolve(raw_tag, state)?;

    // Fetch the Flyweight Profile using the resolved canonical ID
    let profile = state
        .get_profile(&locale.id)
        .ok_or_else(|| LmsError::ResolutionFailed(locale.id.clone()))?;

    // Instantiation
    let mut manifest = CapabilityManifest::new(locale.id.clone());

    // Phase 2: Aggregate (Typology) - No more stubs!
    typology::aggregate(&mut manifest, &profile)?;

    // Phase 3: Override (Orthography) - No more stubs!
    orthography::apply_rendering_traits(&mut manifest, &profile, raw_tag)?;

    // Phase 4: Integrity Check [Ref: 003-LMS-VAL]
    integrity::verify(&manifest)?;

    // Phase 5: Telemetry [Ref: 007-LMS-OPS]
    // Note: In Phase 8, "v0.2.0" will be dynamically pulled from the registry snapshot.
    telemetry::record_metrics(&mut manifest, start_time, &locale.resolution_path, "v0.2.0");

    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::repository;

    #[test]
    fn test_dynamic_pipeline_resolution_and_telemetry() {
        let state = RegistryState::new();
        state.swap_registry(repository::hydrate_snapshot().unwrap());

        // Resolves ar-EG-u-extension dynamically down to ar-EG
        let manifest = generate_manifest("ar-EG-u-ca-islamic", &state).unwrap();
        assert_eq!(manifest.resolved_locale, "ar-EG");
        assert!(manifest.metadata.contains_key("resolution_time_ms"));
        assert_eq!(
            manifest.metadata.get("resolution_path").unwrap(),
            "ar-EG-u-ca-islamic -> ar-EG-u-ca -> ar-EG-u -> ar-EG"
        );
    }
}
