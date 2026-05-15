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

//! # Typological Aggregator
//! Crate: `bistun-lms`
//! Ref: [008-LMS-TYPOLOGY-AGGREGATOR], [011-LMS-DTO]
//! Location: `crates/bistun-lms/src/core/aggregator/typology.rs`
//!
//! **Why**: This module serves as Phase 2 (Aggregate) of the pipeline. It performs an `O(1)` transfer of pre-computed linguistic data from the registry profile to the manifest.
//! **Impact**: If this module fails, the `CapabilityManifest` will be empty, causing the resolution engine to return default payloads (e.g., `en-US`) and breaking regional accuracy.
//!
//! ### Glossary
//! * **Aggregation**: The process of merging immutable data maps from the Flyweight `LocaleProfile` into a request-specific `CapabilityManifest`.

use crate::data::store::LocaleProfile;
use bistun_core::error::LmsError;
use bistun_core::manifest::CapabilityManifest;

/// Enriches the `CapabilityManifest` by aggregating maps from the `LocaleProfile`.
///
/// Time: `O(1)` map extensions | Space: `O(N)` (Cloning pointers/strings)
///
/// # Logic Trace (Internal)
/// 1. Extend the manifest's `traits` map with the profile's pre-computed Linguistic DNA.
/// 2. Extend the manifest's `rules` map with the profile's algorithmic directives.
/// 3. Extend the manifest's `resources` map with the profile's logical resource IDs.
/// 4. Yield successful completion.
///
/// # Examples
/// ```rust
/// # use bistun_core::manifest::CapabilityManifest;
/// # use bistun_lms::core::aggregator::typology::aggregate;
/// # use bistun_core::registry::LocaleProfile;
/// # use hashbrown::HashMap;
/// let profile = LocaleProfile {
///     id: "en-US".to_string(),
///     traits: HashMap::new(),
///     rules: HashMap::new(),
///     resources: HashMap::new()
/// };
/// let mut manifest = CapabilityManifest::new("en-US".to_string());
/// aggregate(&mut manifest, &profile).expect("LMS-TEST: Aggregation failure");
/// ```
///
/// # Arguments
/// * `manifest` (&mut [`CapabilityManifest`]): The mutable DTO being hydrated through the Phase 2 pipeline.
/// * `profile` (&[`LocaleProfile`]): The read-only Flyweight profile containing pre-computed linguistic data.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful aggregation.
///
/// # Errors
/// * While the function signature allows for an [`LmsError`], current map extension logic is infallible.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Safe synchronous execution.
pub fn aggregate(
    manifest: &mut CapabilityManifest,
    profile: &LocaleProfile,
) -> Result<(), LmsError> {
    // [STEP 1]: Aggregate Linguistic DNA (Traits)
    manifest.traits.extend(profile.traits.clone());

    // [STEP 2]: Aggregate Algorithmic Directives (Rules)
    manifest.rules.extend(profile.rules.clone());

    // [STEP 3]: Aggregate Resource IDs (Resources)
    manifest.resources.extend(profile.resources.clone());

    // [STEP 4]: Return Success.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bistun_core::manifest::TraitValue;
    use bistun_core::traits::{Direction, MorphType, SegType, TraitKey};
    use hashbrown::HashMap;

    /// Internal helper to generate a mock Flyweight profile for hermetic testing.
    fn create_mock_profile() -> LocaleProfile {
        let mut traits = HashMap::new();
        traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));
        traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
        traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));

        LocaleProfile {
            id: "test-locale".to_string(),
            traits,
            rules: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    #[test]
    fn test_aggregate_standard_profile() {
        // [Logic Trace Mapping]
        // 1. Setup Data
        // 2. Execute
        // 3. Assert
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        let profile = create_mock_profile();

        assert!(aggregate(&mut manifest, &profile).is_ok());

        assert_eq!(
            manifest.traits.get(&TraitKey::MorphologyType),
            Some(&TraitValue::MorphType(MorphType::FUSIONAL))
        );
    }
}
