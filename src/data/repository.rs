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
use crate::models::traits::{Direction, MorphType, SegType};

/// Hydrates a fresh `RegistryStore` from static sources.
///
/// Time: O(M) where M is the number of locales | Space: O(M) for the new map allocation
///
/// # Logic Trace (Internal)
/// 1. Instantiates a completely isolated, fresh `RegistryStore`.
/// 2. [\STUB\]: Simulates reading from a JSON WORM file on disk by populating known golden sets.
/// 3. Returns the hydrated store to be hot-swapped into the active state.
///
/// # Examples
/// ```rust
/// use bistun::data::repository::hydrate_snapshot;
///
/// let fresh_store = hydrate_snapshot().unwrap();
/// assert!(fresh_store.get_profile("th-TH").is_some());
/// ```
pub fn hydrate_snapshot() -> Result<RegistryStore, LmsError> {
    let mut store = RegistryStore::new();

    // [STUB]: Phase 8 will replace this with `serde_json::from_reader(File::open("snapshot.json"))`

    // 1. English (System Default Fallback per [012-LMS-ENG])
    store.insert_stub(LocaleProfile {
        id: "en-US".to_string(),
        morph: MorphType::FUSIONAL,
        base_seg: SegType::SPACE,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: false,
    });

    // 2. Arabic
    store.insert_stub(LocaleProfile {
        id: "ar-EG".to_string(),
        morph: MorphType::TEMPLATIC,
        base_seg: SegType::SPACE,
        alt_seg: None,
        direction: Direction::RTL,
        has_bidi: true,
        requires_shaping: true,
    });

    // 3. Thai
    store.insert_stub(LocaleProfile {
        id: "th-TH".to_string(),
        morph: MorphType::ISOLATING,
        base_seg: SegType::DICTIONARY,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: true,
    });

    // 4. Traditional Chinese
    store.insert_stub(LocaleProfile {
        id: "zh-Hant".to_string(),
        morph: MorphType::ISOLATING,
        base_seg: SegType::CHARACTER,
        alt_seg: None,
        direction: Direction::TTB,
        has_bidi: false,
        requires_shaping: false,
    });

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
