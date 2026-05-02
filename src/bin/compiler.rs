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

//! # WORM Registry Compiler
//! Ref: [005-LMS-INGEST], [002-LMS-DATA]
//! Location: `src/bin/compiler.rs`
//!
//! **Why**: This standalone binary compiles the 7,000+ language WORM snapshot. It serves as the authoritative build step for the LMS System of Record, ensuring that raw data from ISO and CLDR is normalized and validated.
//!
//! **Impact**: Acts as the firewall against "Linguistic Poisoning". It prevents corrupted upstream data or curation errors from reaching the production SDK by enforcing strict typological integrity.
//!
//! ### Glossary
//! * **WORM**: Write-Once, Read-Many. The compiled, immutable payload format used for the registry.
//! * **Poison Pill**: An intentionally corrupted profile injected during the build process to verify the DNA Linter's ability to reject "Linguistic Chimeras".

use bistun::data::compiler::ingest::RegistryCompiler;
use bistun::data::compiler::linter::validate_profile;
use bistun::data::store::LocaleProfile;
use bistun::models::traits::{Direction, MorphType, SegType};
use std::fs;

/// The primary entry point for WORM compilation.
///
/// Time: O(N) where N is the number of ingested linguistic records | Space: O(N)
///
/// # Logic Trace (Internal)
/// 1. Read raw datasets (`.tab`, `.txt`, `.json`) from the local `data/raw/` directory.
/// 2. Construct the `RegistryCompiler` builder and ingest the raw string buffers.
/// 3. Execute `compile()` to trigger the Truth Hierarchy resolution and merge ISO/CLDR sources.
/// 4. Inject a "Poison Pill" (a known invalid profile) into the proposed list.
/// 5. Filter every resulting profile through the `validate_profile` DNA Linter.
/// 6. Serialize the survivors into the pretty-printed `snapshot.json` WORM artifact.
///
/// # Arguments
/// * None (Reads from filesystem).
///
/// # Returns
/// * `()` (Writes `snapshot.json` to the current working directory).
///
/// # Golden I/O
/// * **Input**: ISO-639-3 TSV, CLDR Script JSON, LikelySubtags JSON.
/// * **Output**: A valid, minimized `snapshot.json` containing `LocaleProfile` objects.
///
/// # Errors, Panics, & Safety
/// * **Errors**: Non-fatal validation rejections are logged to STDOUT.
/// * **Panics**: Panics if mandatory raw data files are missing or if output disk permissions are denied.
/// * **Safety**: Built for offline execution; not intended for high-concurrency environments.
fn main() {
    println!("🚀 Starting Bistun LMS Global WORM Compiler...");

    // [STEP 1]: I/O Extraction [Ref: 005-LMS-INGEST]
    println!("📦 [1/3] Reading raw data sources from data/raw/...");

    let iso_639_raw = fs::read_to_string("data/raw/iso-639-3.tab").expect("Missing iso-639-3.tab");
    let cldr_script_raw =
        fs::read_to_string("data/raw/scriptMetadata.json").expect("Missing scriptMetadata.json");
    let cldr_subtags_raw =
        fs::read_to_string("data/raw/likelySubtags.json").expect("Missing likelySubtags.json");
    let cldr_plurals_raw =
        fs::read_to_string("data/raw/plurals.json").unwrap_or_else(|_| "{}".to_string());

    // [STEP 2] & [STEP 3]: Registry Ingestion & Aggregation
    let raw_profiles = RegistryCompiler::new()
        .with_iso_639_3(&iso_639_raw)
        .with_cldr_scripts(&cldr_script_raw)
        .with_cldr_subtags(&cldr_subtags_raw)
        .with_cldr_plurals(&cldr_plurals_raw)
        .compile()
        .expect("Aggregation/Parsing failed");

    println!("  - Aggregated {} proposed profiles from all sources", raw_profiles.len());

    // [STEP 4]: Poison Pill Injection (Verification Logic)
    let mut draft_pool = raw_profiles;
    draft_pool.push(LocaleProfile {
        id: "xx-POISON".to_string(),
        morph: MorphType::TEMPLATIC,
        base_seg: SegType::CHARACTER,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: false,
        plurals: vec![],
    });

    // [STEP 5]: DNA Linter (Validation) [Ref: 003-LMS-VAL]
    println!("\n🧬 [2/3] Running DNA Linter on proposed profiles...");
    let mut valid_profiles = Vec::new();
    let mut rejected = 0;

    for profile in draft_pool {
        match validate_profile(&profile) {
            Ok(_) => valid_profiles.push(profile),
            Err(e) => {
                rejected += 1;
                // Only log the first few rejections to keep STDOUT clean
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

    // [STEP 6]: Serialization [Ref: 006-LMS-SEC]
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
    println!("... (and {} more)", valid_profiles.len().saturating_sub(5));
    println!("==========================================");
}
