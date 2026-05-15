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

#![cfg(feature = "testing")]

//! # Simulation & Golden Data Set
//! Crate: `bistun-core`
//! Ref: [LMS-TEST], [011-LMS-DTO]
//! Location: `crates/bistun-core/src/simulation.rs`
//!
//! **Why**: Provides authoritative, pre-constructed models and `JSON` payloads for development and testing across the entire monorepo.
//! **Impact**: Eliminates "Mock Drift" across the ecosystem by providing a single source of truth for test data, ensuring UI and Sidecar components test against the exact same Linguistic `DNA`.
//!
//! ### Glossary
//! * **Golden Data**: Authoritative, manually verified datasets used to validate system correctness.
//! * **Mock Drift**: The phenomenon where test data becomes outdated compared to production schemas.

use crate::manifest::{CapabilityManifest, TraitValue};
use crate::registry::LocaleProfile;
use crate::traits::{
    CasingRule, Direction, LmsRule, MorphType, NormRule, PluralRule, SegType, TraitKey, TransRule,
};
use hashbrown::HashMap;

/// The hardcoded `WORM` snapshot fallback for development and testing.
///
/// Time: `O(1)` | Space: `O(1)`
///
/// # Logic Trace (Internal)
/// 1. Provides a static `JSON` string representing a complete `WORM` payload.
/// 2. Utilizes a nested `V2.0.0` architecture (traits, rules, resources) to align with scalability.
pub const SIMULATED_WORM_JSON: &str = r#"{
  "metadata": {
    "version": "v2.0.0-simulated",
    "build_date": "2026-05-01T12:00:00Z",
    "checksum": "a1b2c3d4e5f6g7h8i9j0"
  },
  "aliases": {
    "in": "id",
    "in-ID": "id",
    "iw": "he",
    "no": "nb",
    "zh-TW": "zh-Hant",
    "zh-CN": "zh-Hans"
  },
  "profiles": [
    {
      "ID": "en-US",
      "traits": {
        "MORPHOLOGY_TYPE": "FUSIONAL",
        "SEGMENTATION_STRATEGY": "SPACE",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["one", "other"],
        "DEFAULT_NUMBERING_SYSTEM": "latn",
        "DEFAULT_CALENDAR": "gregory"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "NONE",
        "PLURAL_LOGIC": "CARDINAL_ONLY",
        "CASING_STRATEGY": "CASE_INSENSITIVE"
      },
      "resources": {}
    },
    {
      "ID": "ar-EG",
      "traits": {
        "MORPHOLOGY_TYPE": "TEMPLATIC",
        "SEGMENTATION_STRATEGY": "SPACE",
        "PRIMARY_DIRECTION": "RTL",
        "HAS_BIDI_ELEMENTS": true,
        "REQUIRES_SHAPING": true,
        "PLURAL_CATEGORIES": ["zero", "one", "two", "few", "many", "other"],
        "DEFAULT_NUMBERING_SYSTEM": "arab",
        "DEFAULT_CALENDAR": "islamic"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "ROMANIZATION",
        "PLURAL_LOGIC": "MULTIPLE_CATEGORIES",
        "CASING_STRATEGY": "CASE_SENSITIVE"
      },
      "resources": {
        "icu_arab": "required"
      }
    },
    {
      "ID": "he",
      "traits": {
        "MORPHOLOGY_TYPE": "TEMPLATIC",
        "SEGMENTATION_STRATEGY": "SPACE",
        "PRIMARY_DIRECTION": "RTL",
        "HAS_BIDI_ELEMENTS": true,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["one", "two", "many", "other"],
        "DEFAULT_NUMBERING_SYSTEM": "latn",
        "DEFAULT_CALENDAR": "hebrew"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "NONE",
        "PLURAL_LOGIC": "MULTIPLE_CATEGORIES",
        "CASING_STRATEGY": "CASE_SENSITIVE"
      },
      "resources": {}
    },
    {
      "ID": "id",
      "traits": {
        "MORPHOLOGY_TYPE": "FUSIONAL",
        "SEGMENTATION_STRATEGY": "SPACE",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["other"]
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "NONE"
      },
      "resources": {}
    },
    {
      "ID": "nb",
      "traits": {
        "MORPHOLOGY_TYPE": "FUSIONAL",
        "SEGMENTATION_STRATEGY": "SPACE",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["one", "other"]
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "NONE"
      },
      "resources": {}
    },
    {
      "ID": "zh-Hant",
      "traits": {
        "MORPHOLOGY_TYPE": "ISOLATING",
        "SEGMENTATION_STRATEGY": "CHARACTER",
        "PRIMARY_DIRECTION": "TTB",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["other"],
        "DEFAULT_NUMBERING_SYSTEM": "hanidec"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "PHONETIC",
        "PLURAL_LOGIC": "CARDINAL_ONLY"
      },
      "resources": {}
    },
    {
      "ID": "zh-Hans",
      "traits": {
        "MORPHOLOGY_TYPE": "ISOLATING",
        "SEGMENTATION_STRATEGY": "CHARACTER",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["other"],
        "DEFAULT_NUMBERING_SYSTEM": "hanidec"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "PHONETIC",
        "PLURAL_LOGIC": "CARDINAL_ONLY"
      },
      "resources": {}
    },
    {
      "ID": "th-TH",
      "traits": {
        "MORPHOLOGY_TYPE": "ISOLATING",
        "SEGMENTATION_STRATEGY": "DICTIONARY",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": true,
        "PLURAL_CATEGORIES": ["other"],
        "DEFAULT_NUMBERING_SYSTEM": "thai",
        "DEFAULT_CALENDAR": "buddhist"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "NONE",
        "PLURAL_LOGIC": "CARDINAL_ONLY",
        "CASING_STRATEGY": "CASE_SENSITIVE"
      },
      "resources": {
        "icu_thai": "required"
      }
    },
    {
      "ID": "ja-JP",
      "traits": {
        "MORPHOLOGY_TYPE": "AGGLUTINATIVE",
        "SEGMENTATION_STRATEGY": "CHARACTER",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["other"],
        "DEFAULT_NUMBERING_SYSTEM": "jpan",
        "DEFAULT_CALENDAR": "japanese"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "PHONETIC",
        "PLURAL_LOGIC": "CARDINAL_ONLY",
        "CASING_STRATEGY": "CASE_SENSITIVE"
      },
      "resources": {}
    },
    {
      "ID": "pi",
      "traits": {
        "MORPHOLOGY_TYPE": "FUSIONAL",
        "SEGMENTATION_STRATEGY": "SPACE",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": false,
        "PLURAL_CATEGORIES": ["one", "other"]
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "NONE"
      },
      "resources": {}
    },
    {
      "ID": "sa",
      "traits": {
        "MORPHOLOGY_TYPE": "FUSIONAL",
        "SEGMENTATION_STRATEGY": "SPACE",
        "PRIMARY_DIRECTION": "LTR",
        "HAS_BIDI_ELEMENTS": false,
        "REQUIRES_SHAPING": true,
        "PLURAL_CATEGORIES": ["one", "two", "other"],
        "DEFAULT_NUMBERING_SYSTEM": "deva"
      },
      "rules": {
        "NORMALIZATION_DEFAULT": "NFC",
        "TRANSLITERATION_DEFAULT": "ICU_TRANSFORM",
        "PLURAL_LOGIC": "MULTIPLE_CATEGORIES",
        "CASING_STRATEGY": "CASE_SENSITIVE"
      },
      "resources": {
        "icu_indic": "required"
      }
    }
  ]
}"#;

/// Provides a "Golden" [`LocaleProfile`] for English (`en-US`).
///
/// Time: `O(1)` | Space: `O(1)`
#[must_use]
pub fn en_us_profile() -> LocaleProfile {
    let mut traits = HashMap::new();
    let mut rules = HashMap::new();

    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
    traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));
    traits.insert(
        TraitKey::PluralCategories,
        TraitValue::StringArray(vec!["one".into(), "other".into()]),
    );
    traits.insert(TraitKey::DefaultNumberingSystem, TraitValue::String("latn".into()));
    traits.insert(TraitKey::DefaultCalendar, TraitValue::String("gregory".into()));

    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::NONE));
    rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::CARDINAL_ONLY));
    rules.insert("CASING_STRATEGY".to_string(), LmsRule::Casing(CasingRule::CASE_INSENSITIVE));

    LocaleProfile { id: "en-US".to_string(), traits, rules, resources: HashMap::new() }
}

/// Provides a "Golden" [`LocaleProfile`] for Arabic (`ar-EG`).
///
/// Time: `O(1)` | Space: `O(1)`
#[must_use]
pub fn ar_eg_profile() -> LocaleProfile {
    let mut traits = HashMap::new();
    let mut rules = HashMap::new();
    let mut resources = HashMap::new();

    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::TEMPLATIC));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::RTL));
    traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
    traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
    traits.insert(TraitKey::DefaultNumberingSystem, TraitValue::String("arab".into()));
    traits.insert(TraitKey::DefaultCalendar, TraitValue::String("islamic".into()));
    traits.insert(
        TraitKey::PluralCategories,
        TraitValue::StringArray(vec![
            "zero".into(),
            "one".into(),
            "two".into(),
            "few".into(),
            "many".into(),
            "other".into(),
        ]),
    );

    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::ROMANIZATION));
    rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::MULTIPLE_CATEGORIES));
    rules.insert("CASING_STRATEGY".to_string(), LmsRule::Casing(CasingRule::CASE_SENSITIVE));

    resources.insert("icu_arab".to_string(), "required".to_string());

    LocaleProfile { id: "ar-EG".to_string(), traits, rules, resources }
}

/// Provides a "Golden" [`LocaleProfile`] for Thai (`th-TH`).
///
/// Time: `O(1)` | Space: `O(1)`
#[must_use]
pub fn th_th_profile() -> LocaleProfile {
    let mut traits = HashMap::new();
    let mut rules = HashMap::new();
    let mut resources = HashMap::new();

    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::ISOLATING));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::DICTIONARY));
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
    traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
    traits.insert(TraitKey::DefaultNumberingSystem, TraitValue::String("thai".into()));
    traits.insert(TraitKey::DefaultCalendar, TraitValue::String("buddhist".into()));
    traits.insert(TraitKey::PluralCategories, TraitValue::StringArray(vec!["other".into()]));

    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::NONE));
    rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::CARDINAL_ONLY));
    rules.insert("CASING_STRATEGY".to_string(), LmsRule::Casing(CasingRule::CASE_SENSITIVE));

    resources.insert("icu_thai".to_string(), "required".to_string());

    LocaleProfile { id: "th-TH".to_string(), traits, rules, resources }
}

/// Provides a "Golden" [`LocaleProfile`] for Japanese (`ja-JP`).
///
/// Time: `O(1)` | Space: `O(1)`
#[must_use]
pub fn ja_jp_profile() -> LocaleProfile {
    let mut traits = HashMap::new();
    let mut rules = HashMap::new();

    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::AGGLUTINATIVE));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::CHARACTER));
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
    traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));
    traits.insert(TraitKey::DefaultNumberingSystem, TraitValue::String("jpan".into()));
    traits.insert(TraitKey::DefaultCalendar, TraitValue::String("japanese".into()));
    traits.insert(TraitKey::PluralCategories, TraitValue::StringArray(vec!["other".into()]));
    traits.insert(
        TraitKey::UnicodePreloadBlocks,
        TraitValue::StringArray(vec![
            "Hiragana".into(),
            "Katakana".into(),
            "CJK Unified Ideographs".into(),
        ]),
    );

    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::PHONETIC));
    rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::CARDINAL_ONLY));
    rules.insert("CASING_STRATEGY".to_string(), LmsRule::Casing(CasingRule::CASE_SENSITIVE));

    LocaleProfile { id: "ja-JP".to_string(), traits, rules, resources: HashMap::new() }
}

/// Provides a "Golden" [`LocaleProfile`] for Pali (`pi`).
///
/// Time: `O(1)` | Space: `O(1)`
#[must_use]
pub fn pi_profile() -> LocaleProfile {
    let mut traits = HashMap::new();
    let mut rules = HashMap::new();

    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
    traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(false));
    traits.insert(
        TraitKey::PluralCategories,
        TraitValue::StringArray(vec!["one".into(), "other".into()]),
    );
    traits.insert(
        TraitKey::UnicodePreloadBlocks,
        TraitValue::StringArray(vec!["Basic Latin".into(), "Latin Extended Additional".into()]),
    );

    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::NONE));

    LocaleProfile { id: "pi".to_string(), traits, rules, resources: HashMap::new() }
}

/// Provides a "Golden" [`LocaleProfile`] for Sanskrit (`sa`).
///
/// Time: `O(1)` | Space: `O(1)`
#[must_use]
pub fn sa_profile() -> LocaleProfile {
    let mut traits = HashMap::new();
    let mut rules = HashMap::new();
    let mut resources = HashMap::new();

    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::FUSIONAL));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::SPACE));
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(false));
    traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
    traits.insert(TraitKey::DefaultNumberingSystem, TraitValue::String("deva".into()));
    traits.insert(
        TraitKey::PluralCategories,
        TraitValue::StringArray(vec!["one".into(), "two".into(), "other".into()]),
    );
    traits
        .insert(TraitKey::UnicodePreloadBlocks, TraitValue::StringArray(vec!["Devanagari".into()]));

    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::ICU_TRANSFORM));
    rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::MULTIPLE_CATEGORIES));
    rules.insert("CASING_STRATEGY".to_string(), LmsRule::Casing(CasingRule::CASE_SENSITIVE));

    resources.insert("icu_indic".to_string(), "required".to_string());

    LocaleProfile { id: "sa".to_string(), traits, rules, resources }
}

/// Returns a pre-resolved [`CapabilityManifest`] for Thai.
///
/// Time: `O(1)` | Space: `O(1)`
#[must_use]
pub fn th_th_manifest() -> CapabilityManifest {
    let mut manifest = CapabilityManifest::new("th-TH".to_string());

    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    manifest
        .traits
        .insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::DICTIONARY));
    manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
    manifest.traits.insert(TraitKey::DefaultNumberingSystem, TraitValue::String("thai".into()));
    manifest.traits.insert(TraitKey::DefaultCalendar, TraitValue::String("buddhist".into()));

    manifest.rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    manifest.rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::NONE));
    manifest.rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::CARDINAL_ONLY));

    manifest.resources.insert("icu_thai".to_string(), "required".to_string());

    manifest.metadata.insert("registry_version".into(), "SIMULATED".into());
    manifest
}
