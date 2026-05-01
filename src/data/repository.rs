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

//! # WORM Repository Hydration
//! Ref: [002-LMS-DATA]
//!
//! **Why**: This module compiles raw snapshot data into a highly optimized `RegistryStore` memory pool in the background, isolating heavy I/O from the critical execution path.
//! **Impact**: If this module fails, the service boots with an empty database or cannot process updates, rendering the capability engine inert.
//!
//! ### Glossary
//! * **WORM**: Write-Once, Read-Many. The paradigm where registry snapshots are immutable once compiled.
//! * **Hydration**: The process of reading static data and inflating it into operational memory structures.

use crate::core::resolver::bcp47::LmsError;
use crate::data::store::{LocaleProfile, RegistryStore};
use crate::security::verifier;

/// The hardcoded WORM snapshot (Simulating a disk read for v0.8.0)
const SIMULATED_WORM_JSON: &str = r#"
[
  {
    "id": "en-US",
    "morph": "FUSIONAL",
    "base_seg": "SPACE",
    "alt_seg": null,
    "direction": "LTR",
    "has_bidi": false,
    "requires_shaping": false,
    "plurals": ["one", "other"]
  },
  {
    "id": "ar-EG",
    "morph": "TEMPLATIC",
    "base_seg": "SPACE",
    "alt_seg": null,
    "direction": "RTL",
    "has_bidi": true,
    "requires_shaping": true,
    "plurals": ["zero", "one", "two", "few", "many", "other"]
  },
  {
    "id": "th-TH",
    "morph": "ISOLATING",
    "base_seg": "DICTIONARY",
    "alt_seg": null,
    "direction": "LTR",
    "has_bidi": false,
    "requires_shaping": true,
    "plurals": ["other"]
  },
  {
    "id": "zh-Hant",
    "morph": "ISOLATING",
    "base_seg": "CHARACTER",
    "alt_seg": null,
    "direction": "TTB",
    "has_bidi": false,
    "requires_shaping": false,
    "plurals": ["other"]
  }
]
"#;

/// Hydrates a fresh `RegistryStore` from static sources.
///
/// Time: O(M) where M is the number of locales | Space: O(M) for the new map allocation
///
/// # Logic Trace (Internal)
/// 1. **Security Gate**: Read the raw disk payload and verify its cryptographic signature.
/// 2. **Deserialization**: Parse the JSON WORM payload into a vector of `LocaleProfile` objects.
/// 3. **Instantiation**: Create a completely isolated, fresh `RegistryStore`.
/// 4. **Return**: Yield the hydrated store to be hot-swapped into the active state.
///
/// # Examples
/// ```rust
/// use bistun::data::repository::hydrate_snapshot;
///
/// let fresh_store = hydrate_snapshot().unwrap();
/// assert!(fresh_store.get_profile("th-TH").is_some());
/// ```
pub fn hydrate_snapshot() -> Result<RegistryStore, LmsError> {
    // 1. Security Gate [Ref: 006-LMS-SEC]
    // In Phase 8, this will be `fs::read_to_string` for both the JSON and the `.sig` file.
    let simulated_signature = "valid-lms-signature";
    verifier::verify_snapshot(SIMULATED_WORM_JSON, simulated_signature)?;

    // 2. Deserialization
    let profiles: Vec<LocaleProfile> = serde_json::from_str(SIMULATED_WORM_JSON)
        .map_err(|e| LmsError::SecurityFault(format!("Failed to parse WORM JSON: {}", e)))?;

    // 3. Instantiation
    let mut store = RegistryStore::new();

    // [STUB]: Phase 8 will replace this with `serde_json::from_reader(File::open("snapshot.json"))`
    for profile in profiles {
        store.insert_stub(profile);
    }

    // 4. Return
    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydrate_snapshot_succeeds() {
        // [Logic Trace Mapping]
        // 1. Execute: Call the hydrator.
        // 2. Assert: Verify the returned store is populated with expected golden stubs, including the system default.
        let store = hydrate_snapshot().expect("Hydration failed");

        assert!(store.get_profile("en-US").is_some(), "System Default must exist");
        assert!(store.get_profile("ar-EG").is_some());
        assert!(store.get_profile("th-TH").is_some());
        assert!(store.get_profile("zh-Hant").is_some());
        assert!(store.get_profile("invalid-locale").is_none());
    }
}
