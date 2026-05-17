// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Capability Engine Integration Tests
//! Crate: bistun-lms
//! Ref: [LMS-TEST], [011-LMS-DTO]
//! Location: `crates/bistun-lms/tests/engine_integration.rs`
//!
//! Hermetically verifies the 5-Phase pipeline against the `bistun-core` golden data under the V2.0.0 domain architecture.

#![cfg(feature = "simulation")]

use bistun_core::{Direction, SdkState, SegType, TraitKey, TraitValue};
use bistun_lms::LinguisticManager;
use bistun_lms::data::repository::SimulatedSnapshotProvider;

#[tokio::test]
async fn test_full_pipeline_resolution() {
    // [STEP 1]: Boot the engine with Golden Data
    let manager = LinguisticManager::new();
    let provider = SimulatedSnapshotProvider::new();
    manager.initialize(&provider, &provider.public_key).await;
    assert_eq!(manager.status(), SdkState::Ready);

    // [STEP 2]: Test Exact Match & Typology (Thai dictionary segmentation)
    let thai = manager.resolve_capabilities("th-TH").expect("LMS-TEST: Failed to resolve th-TH");
    assert_eq!(thai.resolved_locale, "th-TH");

    // Verify Domain 1 (Traits)
    assert_eq!(
        thai.traits.get(&TraitKey::SegmentationStrategy),
        Some(&TraitValue::SegType(SegType::DICTIONARY))
    );
    // Verify Domain 2 (Rules Synthesis)
    assert!(thai.rules.contains_key("TRANSLITERATION_DEFAULT"));
    // Verify Domain 3 (Resource Bridge)
    assert!(thai.resources.contains_key("icu_thai"));

    // [STEP 3]: Test Advanced Script Archetype (Japanese Font Preloading)
    let japan = manager.resolve_capabilities("ja-JP").expect("LMS-TEST: Failed to resolve ja-JP");
    assert_eq!(japan.resolved_locale, "ja-JP");

    // Mathematically verify that Phase 2 Aggregation preserves font subset matrices
    if let Some(TraitValue::StringArray(blocks)) = japan.traits.get(&TraitKey::UnicodePreloadBlocks)
    {
        assert!(blocks.contains(&"Hiragana".to_string()));
        assert!(blocks.contains(&"CJK Unified Ideographs".to_string()));
    } else {
        panic!("LMS-TEST: ja-JP missing mandatory UNICODE_PRELOAD_BLOCKS trait array mapping");
    }

    // [STEP 4]: Test Complex Shaping Scripts (Sanskrit Typology)
    let sanskrit = manager.resolve_capabilities("sa").expect("LMS-TEST: Failed to resolve sa");
    assert_eq!(sanskrit.resolved_locale, "sa");
    // Verify the Resource Bridge successfully mapped the abstract requirement to a physical URI
    assert_eq!(sanskrit.resources.get("icu_indic"), Some(&"/icu_indic.postcard".to_string()));

    // [STEP 5]: Test Truncation & Phase 3 Overrides (Arabic with Latin Numerals)
    let arabic = manager
        .resolve_capabilities("ar-EG-u-nu-latn")
        .expect("LMS-TEST: Failed to resolve ar-EG-u-nu-latn");
    assert_eq!(arabic.resolved_locale, "ar-EG");

    // Domain 1: Verify Base DNA remains culturally pure
    assert_eq!(
        arabic.traits.get(&TraitKey::PrimaryDirection),
        Some(&TraitValue::Direction(Direction::RTL))
    );
    // Domain 4: Verify Extensions securely hold user overrides
    assert_eq!(arabic.extensions.get("nu"), Some(&"latn".to_string()));

    // [STEP 6]: Test Aliasing (Indonesian 'in' -> 'id')
    let indonesian = manager.resolve_capabilities("in").expect("LMS-TEST: Failed to resolve in");
    assert_eq!(indonesian.resolved_locale, "id");

    // [STEP 7]: Test System Fallback (Alien Tag)
    let unknown = manager
        .resolve_capabilities("xx-YY-u-nu-foo")
        .expect("LMS-TEST: Failed to resolve fallback");
    assert_eq!(unknown.resolved_locale, "en-US");

    // [STEP 8]: Verify Telemetry Injection (Phase 5)
    assert!(unknown.metadata.contains_key("resolution_time_ms"));
    assert!(unknown.metadata.contains_key("resolution_path"));
    assert!(unknown.metadata.contains_key("registry_version"));
}
