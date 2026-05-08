// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Public API Integration Tests
//! Ref: [LMS-TEST]
//!
//! Verifies that external crates can interact with `bistun-core` types
//! and that the resulting serialization honors the API contract.

use bistun_core::{CapabilityManifest, Direction, MorphType, SegType, TraitKey, TraitValue};

#[test]
fn test_public_api_manifest_construction_and_serialization() {
    // [Logic Trace Mapping]
    // [STEP 1]: Setup: External consumer creates a manifest using the public API.
    let mut manifest = CapabilityManifest::new("zh-Hant".to_string());

    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::TTB));
    manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::CHARACTER));
    manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::ISOLATING));

    // [STEP 2]: Execute: Serialize using serde_json.
    let json = serde_json::to_string(&manifest).expect("Serialization failed");

    // [STEP 3]: Assert: Verify the Untagged enum representation matches the JSON contract.
    // Notice that Direction::TTB becomes the string "TTB", not {"Direction": "TTB"}.
    assert!(json.contains(r#""resolved_locale":"zh-Hant""#));
    assert!(json.contains(r#""PRIMARY_DIRECTION":"TTB""#));
    assert!(json.contains(r#""SEGMENTATION_STRATEGY":"CHARACTER""#));
    assert!(json.contains(r#""MORPHOLOGY_TYPE":"ISOLATING""#));
}
