// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV
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

//! # Public API Integration Tests
//! Crate: bistun-core
//! Ref: [LMS-TEST], [011-LMS-DTO]
//! Location: `crates/bistun-core/tests/dto_integration.rs`
//!
//! **Why**: Verifies that external crates can interact with `bistun-core` types
//! and that the resulting serialization strictly honors the V2.0.0 API contract.
//! **Impact**: Prevents JSON schema drift that would break external parsers in downstream microservices.

use bistun_core::{
    CapabilityManifest, Direction, LmsRule, MorphType, PluralRule, SegType, TraitKey, TraitValue,
    TransRule,
};

#[test]
fn test_public_api_manifest_construction_and_serialization() {
    // [Logic Trace Mapping]
    // [STEP 1]: Setup: External consumer creates a manifest using the public API.
    let mut manifest = CapabilityManifest::new("zh-Hant-u-nu-hanidec".to_string());

    // Domain 1: Traits (Linguistic DNA)
    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::TTB));
    manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::CHARACTER));
    manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::ISOLATING));

    // Domain 2: Rules (Execution Logic)
    manifest
        .rules
        .insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::PHONETIC));
    manifest.rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::CARDINAL_ONLY));

    // Domain 3: Resources (Physical Assets)
    manifest
        .resources
        .insert("font_noto_hant".to_string(), "https://cdn.bistun.io/Noto.woff2".to_string());

    // Domain 4: Extensions (User Overrides)
    manifest.extensions.insert("nu".to_string(), "hanidec".to_string());

    // Domain 5: Metadata (Telemetry)
    manifest.metadata.insert("registry_version".to_string(), "2.0.0".to_string());

    // [STEP 2]: Execute: Serialize using serde_json.
    let json = serde_json::to_string(&manifest).expect("LMS-TEST: Serialization failed");

    // [STEP 3]: Assert: Verify the Untagged enum representation matches the JSON contract.
    // Notice that Direction::TTB becomes the primitive string "TTB", not {"Direction": "TTB"}.
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("LMS-TEST: Parsing failed");

    assert_eq!(parsed["resolved_locale"], "zh-Hant-u-nu-hanidec");

    // Assert Traits Map
    assert_eq!(parsed["traits"]["PRIMARY_DIRECTION"], "TTB");
    assert_eq!(parsed["traits"]["SEGMENTATION_STRATEGY"], "CHARACTER");
    assert_eq!(parsed["traits"]["MORPHOLOGY_TYPE"], "ISOLATING");

    // Assert Rules Map
    assert_eq!(parsed["rules"]["TRANSLITERATION_DEFAULT"], "PHONETIC");
    assert_eq!(parsed["rules"]["PLURAL_LOGIC"], "CARDINAL_ONLY");

    // Assert Resources, Extensions, and Metadata Maps
    assert_eq!(parsed["resources"]["font_noto_hant"], "https://cdn.bistun.io/Noto.woff2");
    assert_eq!(parsed["extensions"]["nu"], "hanidec");
    assert_eq!(parsed["metadata"]["registry_version"], "2.0.0");
}
