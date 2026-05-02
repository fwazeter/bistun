// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! # Data Ingestion & Truth Hierarchy
//! Ref: [005-LMS-INGEST]
//! Location: `src/data/compiler/ingest.rs`
//!
//! **Why**: Merges disparate raw data from ISO 639-3, ISO 15924, and Unicode CLDR into a unified `LocaleProfile`.
//! **Impact**: If this module fails, conflicting standards will cause corrupted linguistic registries, leading to catastrophic UI/NLP failures downstream.
//!
//! ### Glossary
//! * **Truth Hierarchy**: The deterministic conflict resolution engine (Tier 1: Manual > Tier 2: CLDR > Tier 3: ISO).
//! * **Facade Pattern**: A structural design pattern that provides a simplified interface to a complex body of code.

use crate::core::resolver::bcp47::LmsError;
use crate::data::store::LocaleProfile;
use crate::models::traits::{Direction, MorphType, SegType};
use hashbrown::HashMap;
use serde_json::Value;

// -----------------------------------------------------------------------------
// Private Intermediate Structures
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
struct Iso639Record {
    id: String,
    fallback_direction: Direction,
    implied_morph: MorphType,
    implied_seg: SegType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CldrScriptRecord {
    script_code: String,
    character_order: Option<String>,
    requires_bidi: Option<bool>,
    requires_shaping: Option<bool>,
}

// -----------------------------------------------------------------------------
// Public Facade / Builder
// -----------------------------------------------------------------------------

/// Orchestrates the ingestion, parsing, and merging of raw linguistic data sets.
///
/// Time: O(I + M) overall compilation | Space: O(I + M) intermediate allocations
#[derive(Debug, Default)]
pub struct RegistryCompiler<'a> {
    iso_639_3_tsv: Option<&'a str>,
    cldr_script_metadata: Option<&'a str>,
    cldr_likely_subtags: Option<&'a str>,
    cldr_plurals: Option<&'a str>,
}

impl<'a> RegistryCompiler<'a> {
    /// Initializes a new, empty RegistryCompiler.
    ///
    /// Time: O(1) | Space: O(1)
    ///
    /// # Logic Trace (Internal)
    /// 1. Return a default initialized builder.
    ///
    /// # Examples
    /// ```rust
    ///   let compiler = RegistryCompiler::new();
    /// ```
    ///
    /// # Arguments
    /// * None.
    ///
    /// # Returns
    /// * `Self`: The empty compiler builder.
    ///
    /// # Golden I/O
    /// * **Input**: `()`
    /// * **Output**: `RegistryCompiler { ..None }`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: None.
    /// * **Panics**: None.
    /// * **Safety**: Safe.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_iso_639_3(mut self, tsv: &'a str) -> Self {
        self.iso_639_3_tsv = Some(tsv);
        self
    }

    pub fn with_cldr_scripts(mut self, json: &'a str) -> Self {
        self.cldr_script_metadata = Some(json);
        self
    }

    pub fn with_cldr_subtags(mut self, json: &'a str) -> Self {
        self.cldr_likely_subtags = Some(json);
        self
    }

    pub fn with_cldr_plurals(mut self, json: &'a str) -> Self {
        self.cldr_plurals = Some(json);
        self
    }

    /// Compiles the provided raw datasets into a unified vector of `LocaleProfile` objects.
    ///
    /// Time: O(I + M) | Space: O(I)
    ///
    /// # Logic Trace (Internal)
    /// 1. Verify required baseline data is present.
    /// 2. Parse raw data into intermediate structures via private helpers.
    /// 3. Cross-reference `likelySubtags` to map languages to native scripts.
    /// 4. Apply Tier 2 overrides (CLDR Script mechanics).
    /// 5. Apply Tier 1 overrides (Manual Golden Sets for Morphology/Segmentation).
    /// 6. Yield the finalized, fully-hydrated profiles.
    ///
    /// # Arguments
    /// * `self`: Consumes the builder.
    ///
    /// # Returns
    /// * `Result<Vec<LocaleProfile>, LmsError>`: The fully compiled linguistic registry data.
    ///
    /// # Golden I/O
    /// * **Input**: Raw TSV and JSON payloads.
    /// * **Output**: `Ok([LocaleProfile { id: "ar", direction: RTL, morph: TEMPLATIC, ... }])`
    ///
    /// # Errors, Panics, & Safety
    /// * **Errors**: Returns `LmsError::IntegrityViolation` if the baseline ISO-639-3 data was not provided.
    /// * **Panics**: None.
    /// * **Safety**: Safe synchronous compilation.
    pub fn compile(self) -> Result<Vec<LocaleProfile>, LmsError> {
        // [STEP 1]: Verify required data
        let iso_tsv = self.iso_639_3_tsv.ok_or_else(|| {
            LmsError::IntegrityViolation(
                "ISO 639-3 baseline data is required for compilation".into(),
            )
        })?;

        // [STEP 2]: Parse into intermediate structures
        let iso_records = Self::parse_iso_639_3(iso_tsv);
        let cldr_scripts =
            self.cldr_script_metadata.map_or(Vec::new(), Self::parse_cldr_script_metadata);
        let cldr_subtags =
            self.cldr_likely_subtags.map_or(HashMap::new(), Self::parse_cldr_likely_subtags);
        let cldr_plurals = self.cldr_plurals.map_or(HashMap::new(), Self::parse_cldr_plurals);

        let mut profiles = Vec::with_capacity(iso_records.len());

        let mut cldr_map: HashMap<&str, &CldrScriptRecord> = HashMap::new();
        for cldr in &cldr_scripts {
            cldr_map.insert(&cldr.script_code, cldr);
        }

        // [STEP 3] through [STEP 5]: Merge and apply hierarchy
        for iso in iso_records {
            let base_lang = &iso.id;

            let script_code = match cldr_subtags.get(base_lang) {
                Some(likely_tag) => {
                    let mut parts = likely_tag.split('-');
                    parts.next();
                    match parts.next() {
                        Some(script) if script.len() == 4 => script,
                        _ => "Latn",
                    }
                }
                None => "Latn",
            };

            let mut final_direction = iso.fallback_direction;
            let mut final_bidi = false;
            let mut final_shaping = false;

            // [STEP 4]: Tier 2 (CLDR)
            if let Some(cldr) = cldr_map.get(script_code) {
                if let Some(order) = &cldr.character_order {
                    final_direction = match order.as_str() {
                        "right-to-left" => Direction::RTL,
                        "top-to-bottom" => Direction::TTB,
                        _ => Direction::LTR,
                    };
                }
                if let Some(bidi) = cldr.requires_bidi {
                    final_bidi = bidi;
                }
                if let Some(shaping) = cldr.requires_shaping {
                    final_shaping = shaping;
                }
            }

            // [STEP 5]: Tier 1 (Manual Overrides)
            let (final_dir, final_morph, final_seg) = match base_lang.as_str() {
                "ar" | "he" | "syr" | "am" => {
                    (final_direction, MorphType::TEMPLATIC, SegType::SPACE)
                }
                "zh" | "ja" | "ko" => (Direction::TTB, MorphType::ISOLATING, SegType::CHARACTER),
                "th" | "lo" | "km" | "my" => {
                    (final_direction, MorphType::ISOLATING, SegType::DICTIONARY)
                }
                _ => (final_direction, iso.implied_morph, iso.implied_seg),
            };

            let final_plurals =
                cldr_plurals.get(base_lang).cloned().unwrap_or_else(|| vec!["other".to_string()]);

            // [STEP 6]: Yield Profile
            profiles.push(LocaleProfile {
                id: base_lang.to_string(),
                morph: final_morph,
                base_seg: final_seg,
                alt_seg: None,
                direction: final_dir,
                has_bidi: final_bidi,
                requires_shaping: final_shaping,
                plurals: final_plurals,
            });
        }

        Ok(profiles)
    }

    // -----------------------------------------------------------------------------
    // Private Parsing Helpers
    // -----------------------------------------------------------------------------

    fn parse_iso_639_3(tsv_content: &str) -> Vec<Iso639Record> {
        let mut records = Vec::new();
        for line in tsv_content.lines().skip(1) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 7 {
                continue;
            }

            let iso3 = cols[0].trim();
            let iso2 = cols[3].trim();
            let final_id = if !iso2.is_empty() { iso2 } else { iso3 };

            records.push(Iso639Record {
                id: final_id.to_string(),
                fallback_direction: Direction::LTR,
                implied_morph: MorphType::FUSIONAL,
                implied_seg: SegType::SPACE,
            });
        }
        records
    }

    fn parse_cldr_script_metadata(json_content: &str) -> Vec<CldrScriptRecord> {
        let mut records = Vec::new();
        let parsed: Value = match serde_json::from_str(json_content) {
            Ok(val) => val,
            Err(_) => return records,
        };

        if let Some(metadata) = parsed.get("scriptMetadata").and_then(|m| m.as_object()) {
            for (script_code, data) in metadata {
                let rtl_val = data.get("rtl").and_then(|v| v.as_str());
                let shape_val = data.get("shapingReq").and_then(|v| v.as_str());

                let character_order = match rtl_val {
                    Some("YES") => Some("right-to-left".to_string()),
                    Some("NO") => Some("left-to-right".to_string()),
                    _ => None,
                };

                records.push(CldrScriptRecord {
                    script_code: script_code.clone(),
                    character_order,
                    requires_bidi: rtl_val.map(|v| v == "YES"),
                    requires_shaping: shape_val.map(|v| v == "YES"),
                });
            }
        }
        records
    }

    fn parse_cldr_likely_subtags(json_content: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        // [STEP 1]: Modernized collapsible-if via let-chains
        if let Ok(parsed) = serde_json::from_str::<Value>(json_content)
            && let Some(subtags) =
                parsed.pointer("/supplemental/likelySubtags").and_then(|v| v.as_object())
        {
            for (k, v) in subtags {
                if let Some(val_str) = v.as_str() {
                    map.insert(k.clone(), val_str.to_string());
                }
            }
        }
        map
    }

    fn parse_cldr_plurals(json_content: &str) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        // [STEP 1]: Modernized collapsible-if via let-chains
        if let Ok(parsed) = serde_json::from_str::<Value>(json_content)
            && let Some(plurals) =
                parsed.pointer("/supplemental/plurals-type-cardinal").and_then(|v| v.as_object())
        {
            for (lang, rules) in plurals {
                if let Some(rules_obj) = rules.as_object() {
                    let mut categories = Vec::new();
                    for key in rules_obj.keys() {
                        if let Some(cat) = key.strip_prefix("pluralRule-count-") {
                            categories.push(cat.to_string());
                        }
                    }
                    map.insert(lang.clone(), categories);
                }
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_requires_iso_baseline() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup & Execute: Attempt to compile an empty builder.
        let compiler = RegistryCompiler::new();
        let result = compiler.compile();

        // [STEP 2]: Assert: Verify it fails gracefully.
        assert!(matches!(result, Err(LmsError::IntegrityViolation(_))));
    }

    #[test]
    fn test_compiler_merges_truth_hierarchy() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Inject minimal payloads representing the Truth Hierarchy.
        let iso_tsv = "Id\tPart2b\tPart2t\tPart1\tScope\tLanguage_Type\tRef_Name\tComment\n\
                       zho\tzho\tzho\tzh\tI\tL\tChinese";

        let cldr_scripts = r#"{"scriptMetadata": {"Hant": { "rtl": "NO", "shapingReq": "NO" }}}"#;
        let cldr_subtags = r#"{"supplemental": {"likelySubtags": {"zh": "zh-Hant-TW"}}}"#;

        // [STEP 2]: Execute: Build and compile.
        let profiles = RegistryCompiler::new()
            .with_iso_639_3(iso_tsv)
            .with_cldr_scripts(cldr_scripts)
            .with_cldr_subtags(cldr_subtags)
            .compile()
            .expect("Compilation failed");

        // [STEP 3]: Assert: Verify Tier 1 manual logic overrides Tier 2 and Tier 3 data.
        assert_eq!(profiles.len(), 1);
        let zh = &profiles[0];

        assert_eq!(zh.id, "zh");
        assert_eq!(zh.direction, Direction::TTB); // Overridden by Tier 1 despite CLDR saying "NO" rtl
        assert_eq!(zh.morph, MorphType::ISOLATING);
        assert_eq!(zh.base_seg, SegType::CHARACTER);
    }
}
