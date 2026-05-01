// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! # WORM Registry Compiler
//! Ref: [005-LMS-INGEST], [002-LMS-DATA]
//!
//! **Why**: This standalone binary compiles the 7,000+ language WORM snapshot.
//! **Impact**: Serves as the authoritative build step for the LMS System of Record. It prevents corrupted upstream data from reaching the production SDK.
//!
//! ### Glossary
//! * **WORM**: Write-Once, Read-Many. The compiled payload format.
//! * **Poison Pill**: An intentionally corrupted profile injected to test the compiler's resilience.

use bistun::data::compiler::ingest::{
    merge_sources, parse_cldr_likely_subtags, parse_cldr_numbering_systems, parse_cldr_plurals,
    parse_cldr_script_metadata, parse_iso_639_3, parse_iso_15924,
};
use bistun::data::compiler::linter::validate_profile;
use bistun::data::store::LocaleProfile;
use bistun::models::traits::{Direction, MorphType, SegType};
use std::fs;

/// The primary entry point for WORM compilation.
///
/// Time: O(N) | Space: O(N) where N is the number of ingested linguistic records.
///
/// # Logic Trace (Internal)
/// 1. **I/O Extraction**: Read raw physical datasets (`.tab`, `.txt`, `.json`) from `data/raw/`.
/// 2. **Parsing**: Parse the strings into normalized internal DTOs.
/// 3. **Aggregation**: Execute `merge_sources` to execute the Truth Hierarchy collision mapping.
/// 4. **Validation**: Pass every resulting profile through the strict DNA Linter to guarantee Typological Integrity.
/// 5. **Serialization**: Write the valid surviving profiles to the static `snapshot.json` payload.
///
/// # Panics & Safety
/// * Will `panic!` if the required files do not exist in the `data/raw/` directory. Run `fetch_standards.sh` first.
/// * Will `panic!` if the final WORM payload cannot be written to disk due to OS permissions.
fn main() {
    println!("🚀 Starting Bistun LMS Global WORM Compiler...");

    // [STEP 1 & 2]: I/O Extraction & Parsing [Ref: 005-LMS-INGEST]
    println!("📦 [1/3] Reading raw data sources from data/raw/...");

    let iso_639_raw = fs::read_to_string("data/raw/iso-639-3.tab").expect("Missing iso-639-3.tab");
    let iso_15924_raw = fs::read_to_string("data/raw/iso15924.txt").expect("Missing iso15924.txt");
    let cldr_script_raw =
        fs::read_to_string("data/raw/scriptMetadata.json").expect("Missing scriptMetadata.json");
    let cldr_subtags_raw =
        fs::read_to_string("data/raw/likelySubtags.json").expect("Missing likelySubtags.json");
    let cldr_numbers_raw =
        fs::read_to_string("data/raw/numberingSystems.json").unwrap_or_else(|_| "{}".to_string());
    let cldr_plurals_raw =
        fs::read_to_string("data/raw/plurals.json").unwrap_or_else(|_| "{}".to_string());

    let iso_records = parse_iso_639_3(&iso_639_raw);
    let iso_15924_records = parse_iso_15924(&iso_15924_raw);
    let cldr_script_records = parse_cldr_script_metadata(&cldr_script_raw);
    let cldr_subtags = parse_cldr_likely_subtags(&cldr_subtags_raw);
    let cldr_numbering_systems = parse_cldr_numbering_systems(&cldr_numbers_raw);
    let cldr_plurals = parse_cldr_plurals(&cldr_plurals_raw);

    println!("  - Parsed {} languages from ISO 639-3", iso_records.len());
    println!("  - Parsed {} scripts from ISO 15924", iso_15924_records.len());
    println!("  - Parsed {} script mechanics from CLDR", cldr_script_records.len());
    println!("  - Parsed {} taxonomic relationships from CLDR", cldr_subtags.len());
    if !cldr_numbering_systems.is_empty() {
        println!("  - Parsed {} valid numbering systems", cldr_numbering_systems.len());
    }
    if !cldr_plurals.is_empty() {
        println!("  - Parsed plural rules for {} languages from CLDR", cldr_plurals.len());
    }

    // [STEP 3]: Aggregation
    let mut raw_profiles = merge_sources(
        &iso_records,
        &iso_15924_records,
        &cldr_script_records,
        &cldr_subtags,
        &cldr_plurals,
    );

    // Inject the Poison Pill
    raw_profiles.push(LocaleProfile {
        id: "xx-POISON".to_string(),
        morph: MorphType::TEMPLATIC,
        base_seg: SegType::CHARACTER,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: false,
        plurals: vec![],
    });

    // [STEP 4]: Validation [Ref: 003-LMS-VAL]
    println!("\n🧬 [2/3] Running DNA Linter on {} proposed profiles...", raw_profiles.len());
    let mut valid_profiles = Vec::new();
    let mut rejected = 0;

    for profile in raw_profiles {
        match validate_profile(&profile) {
            Ok(_) => valid_profiles.push(profile),
            Err(e) => {
                rejected += 1;
                if rejected <= 3 {
                    println!("  ❌ {} rejected: {}", profile.id, e);
                }
            }
        }
    }

    println!("  ✅ {} profiles passed strict Typological Integrity.", valid_profiles.len());
    if rejected > 0 {
        println!("  ⚠️  {} total rejections.", rejected);
    }

    // [STEP 5]: Serialization [Ref: 006-LMS-SEC]
    println!("\n🔐 [3/3] Generating WORM snapshot...");
    let worm_payload = serde_json::to_string_pretty(&valid_profiles).expect("Serialization failed");
    fs::write("snapshot.json", &worm_payload).expect("Failed to write snapshot.json");

    println!("✨ Compilation complete! WORM snapshot written to ./snapshot.json.\n");

    println!("=== CLI PREVIEW OF PARSED DATA OBJECTS ===");
    for profile in
        valid_profiles.iter().filter(|p| ["en", "ar", "zh", "th", "he"].contains(&p.id.as_str()))
    {
        println!("{:?}", profile);
    }
    println!("... (and {} more)", valid_profiles.len() - 5);
    println!("==========================================");
}
