// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! # Data Ingestion & Truth Hierarchy
//! Ref: [005-LMS-INGEST]
//!
//! **Why**: Merges disparate raw data from ISO 639-3, ISO 15924, and Unicode CLDR into a unified `LocaleProfile`.
//! **Impact**: If this module fails, conflicting standards will cause corrupted linguistic registries, leading to catastrophic UI/NLP failures downstream.
//!
//! ### Glossary
//! * **Truth Hierarchy**: The deterministic conflict resolution engine (Tier 1: Manual > Tier 2: CLDR > Tier 3: ISO).
//! * **Golden Set**: A subset of highly complex languages (ar, zh, th) used to verify mechanical overrides.

use crate::data::store::LocaleProfile;
use crate::models::traits::{Direction, MorphType, SegType};
use hashbrown::HashMap;
use serde_json::Value;

/// Represents a raw parsed row from the ISO 639-3 TSV file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iso639Record {
    pub id: String,
    pub fallback_direction: Direction,
    pub implied_morph: MorphType,
    pub implied_seg: SegType,
}

/// Represents a raw parsed row from the ISO 15924 TXT file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iso15924Record {
    pub code: String,
    pub english_name: String,
}

/// Represents parsed layout rules from a Unicode CLDR JSON file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CldrScriptRecord {
    pub script_code: String,
    pub character_order: Option<String>,
    pub requires_bidi: Option<bool>,
    pub requires_shaping: Option<bool>,
}

/// Parses the official ISO 639-3 Tab-Separated Values file.
///
/// Time: O(N) | Space: O(N) where N is the number of rows.
///
/// # Logic Trace (Internal)
/// 1. Split the incoming TSV content by newlines and skip the header row.
/// 2. Split each valid line by tabs (`\t`).
/// 3. Extract `Id` (Column 0) and `Part1` (Column 3).
/// 4. If `Part1` (the 2-letter BCP 47 code) exists, use it; otherwise fallback to `Id`.
/// 5. Assign Tier 3 baseline traits (LTR, Fusional, Space) and yield the record.
///
/// # Arguments & Returns
/// * `tsv_content`: The raw text of the ISO 639-3 file.
/// * **Returns**: A vector of `Iso639Record` objects.
///
/// # Golden I/O
/// * **Input**: `"eng\teng\teng\ten\tI\tL\tEnglish"`
/// * **Output**: `Iso639Record { id: "en", ... }`
pub fn parse_iso_639_3(tsv_content: &str) -> Vec<Iso639Record> {
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

/// Parses the official ISO 15924 Semicolon-Separated file.
///
/// Time: O(N) | Space: O(N) where N is the number of rows.
///
/// # Logic Trace (Internal)
/// 1. Split the incoming TXT content by newlines.
/// 2. Skip any empty lines or comments starting with `#`.
/// 3. Split valid lines by semicolons (`;`).
/// 4. Extract Script `Code` (Column 0) and `English Name` (Column 2).
///
/// # Arguments & Returns
/// * `txt_content`: The raw text of the ISO 15924 file.
/// * **Returns**: A vector of `Iso15924Record` objects.
pub fn parse_iso_15924(txt_content: &str) -> Vec<Iso15924Record> {
    let mut records = Vec::new();
    for line in txt_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split(';').collect();
        if cols.len() < 3 {
            continue;
        }

        records.push(Iso15924Record {
            code: cols[0].trim().to_string(),
            english_name: cols[2].trim().to_string(),
        });
    }
    records
}

/// Parses the Unicode CLDR `scriptMetadata.json` file.
///
/// Time: O(N) | Space: O(N) where N is the number of scripts.
///
/// # Logic Trace (Internal)
/// 1. Deserialize the JSON string. Return empty if malformed.
/// 2. Access the root `scriptMetadata` object.
/// 3. Iterate over the keys (Script Codes) and read `rtl` and `shapingReq` fields.
/// 4. Transform `YES`/`NO` string flags into booleans and dynamic `character_order` strings.
///
/// # Arguments & Returns
/// * `json_content`: The raw text of the CLDR metadata file.
/// * **Returns**: A vector of `CldrScriptRecord` objects representing mechanical truth.
pub fn parse_cldr_script_metadata(json_content: &str) -> Vec<CldrScriptRecord> {
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

/// Parses `likelySubtags.json` to map base languages to canonical tags.
///
/// Time: O(N) | Space: O(N)
///
/// # Logic Trace (Internal)
/// 1. Deserialize the JSON string and traverse down to `/supplemental/likelySubtags`.
/// 2. Iterate through key-value pairs mapping partial tags to resolved tags (e.g., `en -> en-Latn-US`).
/// 3. Yield a fast-lookup HashMap.
pub fn parse_cldr_likely_subtags(json_content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Flattened using Rust 1.65+ let chains!
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

/// Parses `numberingSystems.json` to extract valid numeric system overrides.
///
/// Time: O(N) | Space: O(N)
pub fn parse_cldr_numbering_systems(json_content: &str) -> Vec<String> {
    let mut systems = Vec::new();

    // Flattened using Rust 1.65+ let chains!
    if let Ok(parsed) = serde_json::from_str::<Value>(json_content)
        && let Some(nums) =
            parsed.pointer("/supplemental/numberingSystems").and_then(|v| v.as_object())
    {
        for (k, _) in nums {
            systems.push(k.clone());
        }
    }
    systems
}

/// Parses `plurals.json` to extract cardinal plural categories per language.
///
/// Time: O(N) | Space: O(N)
pub fn parse_cldr_plurals(json_content: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    // Flattened using Rust 1.65+ let chains!
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

/// Merges ISO and CLDR data using the Truth Hierarchy.
///
/// Time: O(I + M) | Space: O(I) where I is ISO records and M is CLDR scripts.
///
/// # Logic Trace (Internal)
/// 1. Create an O(1) lookup map of CLDR scripts.
/// 2. Iterate over every ISO 639-3 baseline language.
/// 3. Cross-reference `likelySubtags` to determine the language's native script.
/// 4. Apply **Tier 2** overrides using CLDR Script data (modifying RTL and Shaping).
/// 5. Apply **Tier 1** manual overrides for `MorphType` and `SegType` based on Golden Sets.
/// 6. Yield the finalized, fully-hydrated `LocaleProfile`.
///
/// # Arguments & Returns
/// * `iso_records`: The Tier 3 baseline languages.
/// * `_iso_15924_records`: Raw scripts (Currently reserved for Phase 9 expansions).
/// * `cldr_records`: The Tier 2 script mechanics.
/// * `cldr_subtags`: The taxonomic resolution mapping.
/// * **Returns**: A unified `LocaleProfile` ready for the WORM database.
pub fn merge_sources(
    iso_records: &[Iso639Record],
    _iso_15924_records: &[Iso15924Record],
    cldr_records: &[CldrScriptRecord],
    cldr_subtags: &HashMap<String, String>,
    cldr_plurals: &HashMap<String, Vec<String>>,
) -> Vec<LocaleProfile> {
    let mut profiles = Vec::new();
    let mut cldr_map: HashMap<&str, &CldrScriptRecord> = HashMap::new();
    for cldr in cldr_records {
        cldr_map.insert(&cldr.script_code, cldr);
    }

    for iso in iso_records {
        let base_lang = &iso.id;
        let script_code = match cldr_subtags.get(base_lang) {
            Some(likely_tag) => {
                let parts: Vec<&str> = likely_tag.split('-').collect();
                if parts.len() >= 2 && parts[1].len() == 4 { parts[1] } else { "Latn" }
            }
            None => "Latn",
        };

        let mut final_direction = iso.fallback_direction;
        let mut final_bidi = false;
        let mut final_shaping = false;

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

        let (final_dir, final_morph, final_seg) = match base_lang.as_str() {
            "ar" | "he" | "syr" | "am" => (final_direction, MorphType::TEMPLATIC, SegType::SPACE),
            "zh" | "ja" | "ko" => (Direction::TTB, MorphType::ISOLATING, SegType::CHARACTER),
            "th" | "lo" | "km" | "my" => {
                (final_direction, MorphType::ISOLATING, SegType::DICTIONARY)
            }
            _ => (final_direction, iso.implied_morph, iso.implied_seg),
        };

        // Plural Extraction: Default to ["other"] if CLDR has no data for the language
        let final_plurals =
            cldr_plurals.get(base_lang).cloned().unwrap_or_else(|| vec!["other".to_string()]);

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
    profiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iso_639_3_extracts_bcp47() {
        // [Logic Trace Mapping]: Step 4 - Verify fallback to Id if Part1 (2-letter) is empty.
        let tsv = "Id\tPart2b\tPart2t\tPart1\tScope\tLanguage_Type\tRef_Name\tComment\n\
                   eng\teng\teng\ten\tI\tL\tEnglish\n\
                   aaa\t\t\t\tI\tL\tGhotuo";
        let records = parse_iso_639_3(tsv);
        assert_eq!(records[0].id, "en");
        assert_eq!(records[1].id, "aaa");
    }

    #[test]
    fn test_parse_iso_15924_ignores_comments() {
        // [Logic Trace Mapping]: Step 2 - Verify `#` lines are ignored.
        let txt = "# Comment\nArab;160;Arabic\n# Another\nLatn;215;Latin";
        let records = parse_iso_15924(txt);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].code, "Arab");
    }

    #[test]
    fn test_parse_cldr_script_metadata() {
        // [Logic Trace Mapping]: Step 4 - Verify YES translates to RTL/Bidi
        let json = r#"{"scriptMetadata": {"Arab": { "rtl": "YES", "shapingReq": "YES" }}}"#;
        let records = parse_cldr_script_metadata(json);
        assert_eq!(records[0].character_order.as_deref(), Some("right-to-left"));
        assert_eq!(records[0].requires_bidi, Some(true));
    }

    #[test]
    fn test_merge_sources_truth_hierarchy() {
        // [Logic Trace Mapping]: Step 5 - Verify Tier 1 (Manual) overrides Tier 2 (CLDR)
        let iso = vec![Iso639Record {
            id: "zh".to_string(),
            fallback_direction: Direction::LTR,
            implied_morph: MorphType::FUSIONAL,
            implied_seg: SegType::SPACE,
        }];
        let cldr = vec![CldrScriptRecord {
            script_code: "Hant".to_string(),
            character_order: Some("left-to-right".to_string()),
            requires_bidi: Some(false),
            requires_shaping: Some(false),
        }];
        let mut subtags = HashMap::new();
        subtags.insert("zh".to_string(), "zh-Hant-TW".to_string());

        // empty plurals for test
        let plurals = HashMap::new();

        let merged = merge_sources(&iso, &[], &cldr, &subtags, &plurals);

        assert_eq!(merged[0].id, "zh");
        // Tier 1 dictates Chinese is TTB/Isolating/Character despite ISO/CLDR
        assert_eq!(merged[0].direction, Direction::TTB);
        assert_eq!(merged[0].morph, MorphType::ISOLATING);
        assert_eq!(merged[0].base_seg, SegType::CHARACTER);
    }
}
