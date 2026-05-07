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

//! # QA Verification: Linguistic Edge Cases
//! Ref: [001-LMS-CORE], [012-LMS-ENG], [005-LMS-INGEST]
//! Location: `tests/qa_edge_cases.rs`
//!
//! **Why**: Proves that the `LinguisticManager` gracefully handles deprecated tags, macrolanguages, and regional script overrides.
//!
//! **Impact**: This is the final verification layer for the "System of Record". It ensures that the Truth Hierarchy successfully merges disparate ISO and CLDR data into valid rendering instructions.
//!
//! ### Glossary
//! * **Linguistic DNA**: The core set of typological and orthographic traits that define a language's mechanical behavior.
//! * **Macrolanguage**: A BCP 47 tag that represents a cluster of closely related individual languages (e.g., 'no' for Norwegian).

use bistun::data::repository::SimulatedSnapshotProvider;
use bistun::manager::{LinguisticManager, SdkState};
use bistun::models::manifest::TraitValue;
use bistun::models::traits::{Direction, TraitKey};

/// Initializes the SDK manager using the default simulation provider.
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `LinguisticManager`: A fully hydrated and ready manager instance.
///
/// # Errors, Panics, & Safety
/// * **Panics**: Panics if the manager fails to reach the `Ready` state during bootstrap.
async fn setup_manager() -> LinguisticManager {
    // [STEP 1]: Instantiate the manager.
    let manager = LinguisticManager::new();

    // [STEP 2]: Trigger async WORM hydration and await completion via Dependency Injection.
    let provider = SimulatedSnapshotProvider::new();

    // [FIX]: Pass the dynamic public key to the security gate
    manager.initialize(&provider, &provider.public_key).await;

    // [STEP 3]: Assert operational readiness.
    assert_eq!(manager.status(), SdkState::Ready, "Manager must boot successfully for QA tests.");

    manager
}

#[tokio::test]
async fn test_qa_01_hebrew_anomaly_bidi_without_shaping() {
    let manager = setup_manager().await;
    let manifest = manager.resolve_capabilities("he").expect("Failed to resolve Hebrew");

    assert_eq!(
        manifest.traits.get(&TraitKey::PrimaryDirection),
        Some(&TraitValue::Direction(Direction::RTL))
    );
    assert_eq!(manifest.traits.get(&TraitKey::HasBidiElements), Some(&TraitValue::Boolean(true)));
    assert_eq!(manifest.traits.get(&TraitKey::RequiresShaping), Some(&TraitValue::Boolean(false)));
}

#[tokio::test]
async fn test_qa_02_legacy_indonesian_tag() {
    let manager = setup_manager().await;
    let manifest = manager.resolve_capabilities("in").expect("Failed to resolve legacy tag");
    assert_eq!(manifest.resolved_locale, "id");
}

#[tokio::test]
async fn test_qa_03_macrolanguage_norwegian() {
    let manager = setup_manager().await;
    let manifest = manager.resolve_capabilities("no").expect("Failed to resolve macrolanguage");
    assert_eq!(manifest.resolved_locale, "nb");
}

#[tokio::test]
async fn test_qa_04_chinese_regional_script_variants() {
    let manager = setup_manager().await;
    let taiwan = manager.resolve_capabilities("zh-TW").expect("Failed zh-TW");

    assert_eq!(
        taiwan.traits.get(&TraitKey::PrimaryDirection),
        Some(&TraitValue::Direction(Direction::TTB))
    );

    let mainland = manager.resolve_capabilities("zh-CN").expect("Failed zh-CN");

    assert_eq!(
        mainland.traits.get(&TraitKey::PrimaryDirection),
        Some(&TraitValue::Direction(Direction::LTR))
    );
}
