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
//! Ref: [LMS-TEST]
//! Location: `crates/bistun-core/src/simulation.rs`
//!
//! **Why**: Provides authoritative, pre-constructed models and JSON payloads for development and testing across the entire monorepo.
//! **Impact**: Eliminates "Mock Drift" across the ecosystem by providing a single source of truth for test data, ensuring UI and Sidecar components test against the exact same Linguistic DNA.
//!
//! ### Glossary
//! * **Golden Data**: Authoritative, manually verified datasets used to validate system correctness.
//! * **Mock Drift**: The phenomenon where test data becomes outdated compared to production schemas.

use crate::manifest::{CapabilityManifest, TraitValue};
use crate::registry::LocaleProfile;
use crate::traits::{Direction, MorphType, NormType, SegType, TraitKey, TransType};

/// The hardcoded WORM snapshot fallback for development and testing.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Provides a static JSON string representing a complete WORM payload.
/// 2. Includes metadata, aliases, and a diverse set of locale profiles to test edge cases.
pub const SIMULATED_WORM_JSON: &str = r#"{
  "metadata": {
    "version": "v1.0.0-simulated",
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
      "MORPH": "FUSIONAL",
      "BASE_SEG": "SPACE",
      "ALT_SEG": null,
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["one", "other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "ar-EG",
      "MORPH": "TEMPLATIC",
      "BASE_SEG": "SPACE",
      "ALT_SEG": null,
      "DIRECTION": "RTL",
      "HAS_BIDI": true,
      "REQUIRES_SHAPING": true,
      "PLURALS": ["zero", "one", "two", "few", "many", "other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "he",
      "MORPH": "TEMPLATIC",
      "BASE_SEG": "SPACE",
      "ALT_SEG": null,
      "DIRECTION": "RTL",
      "HAS_BIDI": true,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["one", "two", "many", "other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "id",
      "MORPH": "FUSIONAL",
      "BASE_SEG": "SPACE",
      "ALT_SEG": null,
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "nb",
      "MORPH": "FUSIONAL",
      "BASE_SEG": "SPACE",
      "ALT_SEG": null,
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["one", "other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "zh-Hant",
      "MORPH": "ISOLATING",
      "BASE_SEG": "CHARACTER",
      "ALT_SEG": null,
      "DIRECTION": "TTB",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "zh-Hans",
      "MORPH": "ISOLATING",
      "BASE_SEG": "CHARACTER",
      "ALT_SEG": null,
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "th-TH",
      "MORPH": "ISOLATING",
      "BASE_SEG": "DICTIONARY",
      "ALT_SEG": null,
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": true,
      "PLURALS": ["other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "ja-JP",
      "MORPH": "AGGLUTINATIVE",
      "BASE_SEG": "CHARACTER",
      "ALT_SEG": "DICTIONARY",
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "pi",
      "MORPH": "FUSIONAL",
      "BASE_SEG": "SPACE",
      "ALT_SEG": null,
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": false,
      "PLURALS": ["one", "other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "NONE"
    },
    {
      "ID": "sa",
      "MORPH": "FUSIONAL",
      "BASE_SEG": "SPACE",
      "ALT_SEG": null,
      "DIRECTION": "LTR",
      "HAS_BIDI": false,
      "REQUIRES_SHAPING": true,
      "PLURALS": ["one", "two", "other"],
      "UNICODE_BLOCKS": [],
      "NORMALIZATION": "NFC",
      "TRANSLITERATION": "ICU_TRANSFORM",
      "REQUIRED_RESOURCE": "icu_indic"
    }
  ]
}"#;

/// Provides a "Golden" [`LocaleProfile`] for English (en-US).
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Instantiates a hardcoded `LocaleProfile` representing a standard baseline Latin-script language.
/// 2. Returns the profile for use in test hydration.
///
/// # Examples
/// ```rust
/// use bistun_core::simulation::en_us_profile;
/// let profile = en_us_profile();
/// assert_eq!(profile.id, "en-US");
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `LocaleProfile`: The authoritative Registry definition for English.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous instantiation.
pub fn en_us_profile() -> LocaleProfile {
    LocaleProfile {
        id: "en-US".to_string(),
        morph: MorphType::FUSIONAL,
        base_seg: SegType::SPACE,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: false,
        plurals: vec!["one".into(), "other".into()],
        unicode_blocks: vec!["Basic Latin".into()],
        normalization: NormType::NFC,
        transliteration: TransType::NONE,
        required_resource: None,
    }
}

/// Provides a "Golden" [`LocaleProfile`] for Arabic (ar-EG).
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Instantiates a hardcoded `LocaleProfile` representing a complex RTL, templatic, and shaping-heavy language.
/// 2. Returns the profile for use in test hydration.
///
/// # Examples
/// ```rust
/// use bistun_core::simulation::ar_eg_profile;
/// let profile = ar_eg_profile();
/// assert_eq!(profile.id, "ar-EG");
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `LocaleProfile`: The authoritative Registry definition for Arabic.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous instantiation.
pub fn ar_eg_profile() -> LocaleProfile {
    LocaleProfile {
        id: "ar-EG".to_string(),
        morph: MorphType::TEMPLATIC,
        base_seg: SegType::SPACE,
        alt_seg: None,
        direction: Direction::RTL,
        has_bidi: true,
        requires_shaping: true,
        plurals: vec![
            "zero".into(),
            "one".into(),
            "two".into(),
            "few".into(),
            "many".into(),
            "other".into(),
        ],
        unicode_blocks: vec!["Arabic".into()],
        normalization: NormType::NFC,
        transliteration: TransType::NONE,
        required_resource: None,
    }
}

/// Provides a "Golden" [`LocaleProfile`] for Thai (th-TH).
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Instantiates a hardcoded `LocaleProfile` representing a complex dictionary-based segmentation language.
/// 2. Returns the profile for use in test hydration.
///
/// # Examples
/// ```rust
/// use bistun_core::simulation::th_th_profile;
/// let profile = th_th_profile();
/// assert_eq!(profile.id, "th-TH");
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `LocaleProfile`: The authoritative Registry definition for Thai.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous instantiation.
pub fn th_th_profile() -> LocaleProfile {
    LocaleProfile {
        id: "th-TH".to_string(),
        morph: MorphType::ISOLATING,
        base_seg: SegType::DICTIONARY,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: true,
        plurals: vec!["other".into()],
        unicode_blocks: vec!["Thai".into()],
        normalization: NormType::NFC,
        transliteration: TransType::ICU_TRANSFORM,
        required_resource: Some("icu_thai".into()),
    }
}

/// Provides a "Golden" [`LocaleProfile`] for Japanese (ja-JP).
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Instantiates a hardcoded `LocaleProfile` representing an agglutinative language with mixed segmentation strategies.
/// 2. Returns the profile for use in test hydration.
///
/// # Examples
/// ```rust
/// use bistun_core::simulation::ja_jp_profile;
/// let profile = ja_jp_profile();
/// assert_eq!(profile.id, "ja-JP");
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `LocaleProfile`: The authoritative Registry definition for Japanese.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous instantiation.
pub fn ja_jp_profile() -> LocaleProfile {
    LocaleProfile {
        id: "ja-JP".to_string(),
        morph: MorphType::AGGLUTINATIVE,
        base_seg: SegType::CHARACTER,
        alt_seg: Some(SegType::DICTIONARY), // Represents the High-Water Mark override potential
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: false,
        plurals: vec!["other".into()],
        unicode_blocks: vec!["Hiragana".into(), "Katakana".into(), "CJK Unified Ideographs".into()],
        normalization: NormType::NFC,
        transliteration: TransType::NONE,
        required_resource: None,
    }
}

/// Provides a "Golden" [`LocaleProfile`] for Pali (pi).
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Instantiates a hardcoded `LocaleProfile` representing an ancient Middle Indo-Aryan language.
/// 2. Returns the profile for use in test hydration.
///
/// # Examples
/// ```rust
/// use bistun_core::simulation::pi_profile;
/// let profile = pi_profile();
/// assert_eq!(profile.id, "pi");
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `LocaleProfile`: The authoritative Registry definition for Pali.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous instantiation.
pub fn pi_profile() -> LocaleProfile {
    LocaleProfile {
        id: "pi".to_string(),
        morph: MorphType::FUSIONAL,
        base_seg: SegType::SPACE,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: false,
        plurals: vec!["one".into(), "other".into()],
        unicode_blocks: vec!["Basic Latin".into(), "Latin Extended Additional".into()],
        normalization: NormType::NFC,
        transliteration: TransType::NONE,
        required_resource: None,
    }
}

/// Provides a "Golden" [`LocaleProfile`] for Sanskrit (sa).
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Instantiates a hardcoded `LocaleProfile` representing a highly fusional language requiring complex Devanagari script shaping.
/// 2. Returns the profile for use in test hydration.
///
/// # Examples
/// ```rust
/// use bistun_core::simulation::sa_profile;
/// let profile = sa_profile();
/// assert_eq!(profile.id, "sa");
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `LocaleProfile`: The authoritative Registry definition for Sanskrit.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous instantiation.
pub fn sa_profile() -> LocaleProfile {
    LocaleProfile {
        id: "sa".to_string(),
        morph: MorphType::FUSIONAL,
        base_seg: SegType::SPACE,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: true, // Devanagari natively demands complex shaping
        plurals: vec!["one".into(), "two".into(), "other".into()], // Features a dual plural category
        unicode_blocks: vec!["Devanagari".into()],
        normalization: NormType::NFC,
        transliteration: TransType::ICU_TRANSFORM, // Commonly mapped via IAST
        required_resource: Some("icu_indic".to_string()),
    }
}

/// Returns a pre-resolved [`CapabilityManifest`] for Thai.
///
/// Time: O(1) | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Instantiates an empty `CapabilityManifest` for "th-TH".
/// 2. Injects manual traits mimicking the output of the 5-Phase pipeline.
/// 3. Returns the populated manifest.
///
/// # Examples
/// ```rust
/// use bistun_core::simulation::th_th_manifest;
/// let manifest = th_th_manifest();
/// assert_eq!(manifest.resolved_locale, "th-TH");
/// ```
///
/// # Arguments
/// * None.
///
/// # Returns
/// * `CapabilityManifest`: A fully hydrated instructional payload for UI testing.
///
/// # Errors, Panics, & Safety
/// * **Errors**: None.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous instantiation.
pub fn th_th_manifest() -> CapabilityManifest {
    let mut manifest = CapabilityManifest::new("th-TH".to_string());
    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    manifest
        .traits
        .insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::DICTIONARY));
    manifest.traits.insert(TraitKey::RequiresShaping, TraitValue::Boolean(true));
    manifest.metadata.insert("registry_version".into(), "SIMULATED".into());
    manifest
}
