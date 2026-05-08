// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Capability Engine Integration Tests
//! Ref: [LMS-TEST]
//!
//! Hermetically verifies the 5-Phase pipeline against the `bistun-core` golden data.

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
    let thai = manager.resolve_capabilities("th-TH").unwrap();
    assert_eq!(thai.resolved_locale, "th-TH");
    assert_eq!(
        thai.traits.get(&TraitKey::SegmentationStrategy),
        Some(&TraitValue::SegType(SegType::DICTIONARY))
    );

    // [STEP 3]: Test Truncation & Orthographic Override (Arabic with Latin Numerals)
    let arabic = manager.resolve_capabilities("ar-EG-u-nu-latn").unwrap();
    assert_eq!(arabic.resolved_locale, "ar-EG"); // Truncated down to parent locale
    assert_eq!(
        arabic.traits.get(&TraitKey::PrimaryDirection),
        Some(&TraitValue::Direction(Direction::RTL))
    );
    assert_eq!(
        arabic.traits.get(&TraitKey::NumberingSystem),
        Some(&TraitValue::String("latn".to_string()))
    );

    // [STEP 4]: Test Aliasing (Indonesian 'in' -> 'id')
    let indonesian = manager.resolve_capabilities("in").unwrap();
    assert_eq!(indonesian.resolved_locale, "id");

    // [STEP 5]: Test System Fallback (Alien Tag)
    let unknown = manager.resolve_capabilities("xx-YY-u-nu-foo").unwrap();
    assert_eq!(unknown.resolved_locale, "en-US");

    // [STEP 6]: Verify Telemetry Injection (Phase 5)
    assert!(unknown.metadata.contains_key("resolution_time_ms"));
    assert!(unknown.metadata.contains_key("resolution_path"));
}
