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

//! # The Capability Engine Coordinator
//! Crate: `bistun-lms`
//! Ref: [001-LMS-CORE], [011-LMS-DTO]
//! Location: `crates/bistun-lms/src/core/pipeline.rs`
//!
//! **Why**: This module executes the 5-Phase pipeline to transform a `BCP 47` string into an immutable `CapabilityManifest` using the dynamic memory pool.
//! **Impact**: If this module fails, the entire capability engine is disconnected, preventing any linguistic capability resolution across the system.
//!
//! ### Glossary
//! * **Coordinator**: The central orchestrator that manages the sequential execution of the 5-Phase pipeline without containing business logic itself.

use crate::core::aggregator::typology::aggregate; // [FIX]: Directly import the function
use crate::core::extension::orthography;
use crate::core::resolver::orchestrator;
use crate::core::resource;
use crate::data::swap::IRegistryState;
use crate::ops::telemetry;
use crate::validation::integrity;
use bistun_core::error::LmsError;
use bistun_core::manifest::CapabilityManifest;
use std::time::Instant;
use tracing::{debug, info, instrument};

/// Orchestrates the 5-Phase capability pipeline to synthesize a `CapabilityManifest`.
///
/// Time: `O(N)` where N is tag truncation length | Space: `O(1)` map allocations
///
/// # Logic Trace (Internal)
/// 1. **Phase 1 (Resolve)**: Pass the raw tag and state to the Taxonomic resolver.
/// 2. **Fetch Profile**: Retrieve the exact `LocaleProfile` from the dynamic Flyweight pool.
/// 3. **Phase 2 (Aggregate)**: Hydrate Typology, Rules, and Resources directly from the `LocaleProfile`.
/// 4. **Phase 2.5 (Resource Bridge)**: Transform abstract resource `IDs` into physical `URIs`.
/// 5. **Phase 3 (Override)**: Parse `BCP 47` extensions and inject into the `extensions` map.
/// 6. **Phase 4 (Integrity)**: Verify the fully aggregated manifest for mechanical contradictions.
/// 7. **Phase 5 (Telemetry)**: Record the pipeline duration and resolution path.
/// 8. **Return**: Yield the hydrated and validated manifest.
///
/// # Examples
/// ```rust
/// # use bistun_lms::core::pipeline::generate_manifest;
/// # // Example logic requires a mock or live IRegistryState
/// ```
///
/// # Arguments
/// * `raw_tag` (&str): The raw `BCP 47` language tag requested by the client.
/// * `state` (&dyn `IRegistryState`): The thread-safe active Flyweight pool of definitions.
///
/// # Returns
/// * `Result<CapabilityManifest, LmsError>`: The fully synthesized, immutable linguistic capability payload.
///
/// # Golden I/O
/// * **Input**: `"ar-EG-u-ca-islamic"`, `RegistryState`
/// * **Output**: `Ok(CapabilityManifest { resolved_locale: "ar-EG", extensions: {"ca": "islamic"}, ... })`
///
/// # Errors
/// * Returns [`LmsError::InvalidTag`] if the input tag is empty or whitespace.
/// * Returns [`LmsError::ResolutionFailed`] if the fallback chain exhausts or the profile is missing from the memory pool.
/// * Returns [`LmsError::IntegrityViolation`] if Phase 4 detects mechanical linguistic contradictions.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Fully safe synchronous execution.
#[instrument(level = "info", name = "pipeline_execution", skip(state), fields(tag = raw_tag))]
pub fn generate_manifest(
    raw_tag: &str,
    state: &dyn IRegistryState,
) -> Result<CapabilityManifest, LmsError> {
    let start_time = Instant::now();
    debug!("Commencing 5-Phase Pipeline for tag: {}", raw_tag);

    // [STEP 1]: Phase 1: Resolve (Taxonomy)
    let locale = orchestrator::resolve(raw_tag, state)?;
    debug!("Resolved canonical ID: {}", locale.id);

    // [STEP 2]: Fetch the Flyweight Profile
    let profile = state.get_profile(&locale.id).ok_or_else(|| LmsError::ResolutionFailed {
        pipeline_step: "Phase 1: Taxonomic Resolution".to_string(),
        tag: locale.id.clone(),
        reason:
            "Profile missing from active registry memory pool despite successful chain resolution"
                .to_string(),
    })?;

    let mut manifest = CapabilityManifest::new(locale.id.clone());

    // [STEP 3]: Phase 2: Aggregate (Typology, Rules, Resources)
    aggregate(&mut manifest, &profile)?; // [FIX]: Now calls the directly imported function

    // [STEP 4]: Phase 2.5: The Resource Bridge [Ref: 014-LMS-BRDG]
    // Fetches the environment-specific URI without locks and resolves abstract IDs
    resource::resolver::resolve_resources(&mut manifest, &state.get_base_resource_uri())?;

    // [STEP 5]: Phase 3: Override (Extensions)
    orthography::apply_extensions(&mut manifest, raw_tag)?;

    // [STEP 6]: Phase 4: Integrity Check
    integrity::verify(&manifest)?;

    // [STEP 7]: Phase 5: Telemetry
    let version = state.get_version();
    // Pass `false` for circuit_breaker as the pipeline only runs when healthy
    telemetry::record_metrics(&mut manifest, start_time, &locale.resolution_path, &version, false);

    info!("Successfully generated manifest for {}", locale.id);

    // [STEP 8]: Return
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resolver::test_utils::{MockRegistryState, create_stub};

    #[test]
    fn test_dynamic_pipeline_resolution_and_telemetry() {
        // [STEP 1]: Setup
        let mut mock_state = MockRegistryState::new();

        mock_state.expect_resolve_alias().returning(|_| None);
        mock_state.expect_get_version().returning(|| "v2.0.0-mock".to_string());

        // Inject the mock base URI
        mock_state
            .expect_get_base_resource_uri()
            .returning(|| "https://cdn.bistun.io/".to_string());

        mock_state
            .expect_get_profile()
            .returning(|id| if id == "ar-EG" { Some(create_stub("ar-EG")) } else { None });

        // [STEP 2]: Execute
        let manifest = generate_manifest("ar-EG-u-ca-islamic", &mock_state)
            .expect("LMS-TEST: Pipeline execution failed during hermetic test");

        // [STEP 3]: Assert
        assert_eq!(manifest.resolved_locale, "ar-EG");
        assert!(manifest.metadata.contains_key("resolution_time_ms"));

        // Assert V2.0.0 Architecture: Extension is placed in the extensions map!
        let calendar =
            manifest.extensions.get("ca").expect("LMS-TEST: Missing expected extension 'ca'");

        assert_eq!(calendar, "islamic");
    }
}
