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
//! Ref: [001-LMS-CORE]
//! Location: `src/core/pipeline.rs`
//!
//! **Why**: This module executes the 5-Phase pipeline to transform a BCP 47 string into an immutable `CapabilityManifest` using the dynamic memory pool.
//! **Impact**: If this module fails, the entire capability engine is disconnected, preventing any linguistic capability resolution across the system.
//!
//! ### Glossary
//! * **Coordinator**: The central orchestrator that manages the sequential execution of the 5-Phase pipeline without containing business logic itself.

use crate::core::aggregator::typology;
use crate::core::extension::orthography;
use crate::core::resolver::bcp47::{self, LmsError};
use crate::data::swap::IRegistryState; // Dependency Inversion Interface
use crate::models::manifest::CapabilityManifest;
use crate::ops::telemetry;
use crate::validation::integrity;
use std::time::Instant;
use tracing::{debug, info, instrument};

/// Orchestrates the 5-Phase capability pipeline to synthesize a CapabilityManifest.
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
///
/// # Examples
/// ```rust
///  let manifest = generate_manifest("ar-EG-u-ca-islamic", &state).unwrap();
///  assert_eq!(manifest.resolved_locale, "ar-EG");
/// ```
///
/// # Arguments
/// * `raw_tag` (&str): The raw BCP 47 language tag requested by the client.
/// * `state` (&dyn IRegistryState): The thread-safe active Flyweight pool of definitions, abstracted via dynamic dispatch.
///
/// # Returns
/// * `Result<CapabilityManifest, LmsError>`: The fully synthesized, immutable linguistic capability payload.
///
/// # Golden I/O
/// * **Input**: `"ar-EG-u-ca-islamic"`, `RegistryState`
/// * **Output**: `CapabilityManifest { resolved_locale: "ar-EG", traits: { ... }, metadata: { ... } }`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns `LmsError::InvalidTag` if the input is empty, `ResolutionFailed` if the fallback chain exhausts, or `IntegrityViolation` if Phase 4 fails.
/// * **Panics**: None.
/// * **Safety**: Fully safe synchronous execution.
#[instrument(level = "info", name = "pipeline_execution", skip(state), fields(tag = raw_tag))]
pub fn generate_manifest(
    raw_tag: &str,
    state: &dyn IRegistryState, // Aligned with the dynamic dispatch requirement from bcp47.rs
) -> Result<CapabilityManifest, LmsError> {
    // START THE CLOCK for DTO injection (Tracing handles internal SLI tracking)
    let start_time = Instant::now();
    debug!("Commencing 5-Phase Pipeline for tag: {}", raw_tag);

    // [STEP 1]: Phase 1: Resolve (Taxonomy) uses dynamic state to find the truncation bound
    let locale = bcp47::resolve(raw_tag, state)?;
    debug!("Resolved canonical ID: {}", locale.id);

    // [STEP 2]: Fetch the Flyweight Profile using the resolved canonical ID
    let profile = state
        .get_profile(&locale.id)
        .ok_or_else(|| LmsError::ResolutionFailed(locale.id.clone()))?;

    // Instantiation
    let mut manifest = CapabilityManifest::new(locale.id.clone());

    // [STEP 3]: Phase 2: Aggregate (Typology)
    typology::aggregate(&mut manifest, &profile)?;

    // [STEP 4]: Phase 3: Override (Orthography)
    orthography::apply_rendering_traits(&mut manifest, &profile, raw_tag)?;

    // [STEP 5]: Phase 4: Integrity Check [Ref: 003-LMS-VAL]
    integrity::verify(&manifest)?;

    // [STEP 6]: Phase 5: Telemetry [Ref: 007-LMS-OPS]
    // Note: In Phase 8, "v0.2.0" will be dynamically pulled from the registry snapshot.
    telemetry::record_metrics(&mut manifest, start_time, &locale.resolution_path, "v0.2.0");

    info!("Successfully generated manifest for {}", locale.id);

    // [STEP 7]: Return
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::store::LocaleProfile;
    use crate::models::traits::{Direction, MorphType, SegType};
    use mockall::mock;
    use std::sync::Arc;

    // LMS-TEST: Generate the hermetic mock based on the new IRegistryState trait
    mock! {
        pub RegistryState {}
        impl IRegistryState for RegistryState {
            fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>>; // Must return Arc per the Flyweight pattern
        }
    }

    /// Internal helper to generate a mock Flyweight profile for hermetic testing.
    fn create_mock_profile() -> Arc<LocaleProfile> {
        Arc::new(LocaleProfile {
            id: "ar-EG".to_string(),
            morph: MorphType::TEMPLATIC,
            base_seg: SegType::SPACE,
            alt_seg: None,
            direction: Direction::RTL,
            has_bidi: true,
            requires_shaping: true,
            plurals: vec!["other".to_string()],
        })
    }

    #[test]
    fn test_dynamic_pipeline_resolution_and_telemetry() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Construct a purely in-memory hermetic `RegistryState` using a mock.
        let mut mock_state = MockRegistryState::new();

        // We expect the pipeline to query the profile for the canonical ID
        mock_state.expect_get_profile().returning(|id| {
            // If the resolver asks for the full tag, we return None to force truncation.
            // If it asks for the canonical ID, we return the profile.
            if id == "ar-EG" { Some(create_mock_profile()) } else { None }
        });

        // [STEP 2]: Execute: Run the pipeline orchestrator.
        let manifest = generate_manifest("ar-EG-u-ca-islamic", &mock_state).unwrap();

        // [STEP 3]: Assert: Verify the manifest is hydrated and telemetry is recorded.
        assert_eq!(manifest.resolved_locale, "ar-EG");
        assert!(manifest.metadata.contains_key("resolution_time_ms"));
    }
}
