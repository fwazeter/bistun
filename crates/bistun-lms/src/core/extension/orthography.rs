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

//! # Orthographic Extension Mapper
//! Crate: `bistun-lms`
//! Ref: [004-LMS-EXT], [011-LMS-DTO]
//! Location: `crates/bistun-lms/src/core/extension/orthography.rs`
//!
//! **Why**: This module serves as Phase 3 (Override/Extension) of the pipeline. It parses user-requested `BCP 47` subtags (like `-u-`) and injects them into the decoupled `extensions` map.
//! **Impact**: If this module fails, user runtime overrides (like requesting a Latin calendar in an Arabic UI) will be ignored, breaking user experience and personalization logic.
//!
//! ### Glossary
//! * **Extension Subtag**: A `BCP 47` singleton (like `-u-` for Unicode or `-t-` for Transform) used to modify the default behavior of a locale.

use bistun_core::error::LmsError;
use bistun_core::manifest::CapabilityManifest;

/// Parses `BCP 47` `-u-` Unicode extensions and injects them into the manifest's `extensions` map.
///
/// Time: `O(N)` string traversal | Space: `O(1)`
///
/// # Logic Trace (Internal)
/// 1. Scan the `raw_tag` for the `-u-` singleton sequence.
/// 2. If the Unicode sequence is found, instantiate a zero-allocation string splitter.
/// 3. Extract recognized keys (`nu` for Numbers, `ca` for Calendar) and map their values into `manifest.extensions`.
/// 4. Return successful completion without mutating the `traits` map.
///
/// # Examples
/// ```rust
/// # use bistun_core::manifest::CapabilityManifest;
/// # use bistun_lms::core::extension::orthography::apply_extensions;
/// let mut manifest = CapabilityManifest::new("ar-EG-u-nu-latn".to_string());
/// apply_extensions(&mut manifest, "ar-EG-u-nu-latn-ca-gregory")
///     .expect("LMS-TEST: Failed to apply extensions");
/// assert_eq!(manifest.extensions.get("nu").expect("LMS-TEST: Missing key"), "latn");
/// ```
///
/// # Arguments
/// * `manifest` (&mut `CapabilityManifest`): The mutable `DTO` being hydrated through the pipeline.
/// * `raw_tag` (&str): The raw `BCP 47` language tag requested, parsed for Unicode extensions.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful parsing and injection.
///
/// # Golden I/O
/// * **Input**: `manifest`, `"ar-EG-u-nu-latn-ca-gregory"`
/// * **Output**: `Ok(())` (Manifest updated with `extensions: {"nu": "latn", "ca": "gregory"}`)
///
/// # Errors
/// * While the function signature allows for an `LmsError`, current parsing logic is internal and essentially infallible; future validation logic for subtag values may return `LmsError::ExtensionMappingFailure`.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Safe synchronous execution with zero heap allocations for tag parsing.
pub fn apply_extensions(manifest: &mut CapabilityManifest, raw_tag: &str) -> Result<(), LmsError> {
    // [STEP 1]: Extension Parsing (BCP 47 `-u-`)
    if let Some(u_ext_start) = raw_tag.find("-u-") {
        let extension_subtag = &raw_tag[u_ext_start + 3..];

        // [STEP 2]: Zero-allocation iterator parsing to protect the < 1ms budget
        let mut iter = extension_subtag.split('-');

        // [STEP 3]: Override Injection (into the decoupled Extensions map)
        while let Some(part) = iter.next() {
            // If we hit another BCP 47 singleton (e.g., -t- or -x-), the -u- block is over.
            if part.len() == 1 {
                break;
            }

            match part {
                "nu" => {
                    if let Some(val) = iter.next() {
                        manifest.extensions.insert("nu".to_string(), val.to_string());
                    }
                }
                "ca" => {
                    if let Some(val) = iter.next() {
                        manifest.extensions.insert("ca".to_string(), val.to_string());
                    }
                }
                _ => {} // Redundant 'continue' removed to satisfy Clippy
            }
        }
    }

    // [STEP 4]: Return
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_unicode_overrides() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Create an empty V2 manifest.
        let mut manifest = CapabilityManifest::new("ar-EG-u-nu-latn-ca-gregory".to_string());

        // [STEP 2]: Execute: Pass the tag with Number (-nu-) and Calendar (-ca-) extensions.
        assert!(apply_extensions(&mut manifest, "ar-EG-u-nu-latn-ca-gregory").is_ok());

        // [STEP 3]: Assert: Verify the extensions map was populated securely.
        assert_eq!(manifest.extensions.get("nu").expect("LMS-TEST: Missing key"), "latn");
        assert_eq!(manifest.extensions.get("ca").expect("LMS-TEST: Missing key"), "gregory");

        // Assert: Verify the traits map was completely untouched!
        assert!(manifest.traits.is_empty());
    }

    #[test]
    fn test_no_extensions_present() {
        let mut manifest = CapabilityManifest::new("en-US".to_string());

        assert!(apply_extensions(&mut manifest, "en-US").is_ok());

        // Ensure no false positives were injected
        assert!(manifest.extensions.is_empty());
    }

    #[test]
    fn test_singleton_boundary_respect() {
        let mut manifest = CapabilityManifest::new("zh-Hant-u-nu-hanidec-t-zh-latn".to_string());

        // We only parse the -u- block. Once we hit -t-, it should break out.
        assert!(apply_extensions(&mut manifest, "zh-Hant-u-nu-hanidec-t-zh-latn").is_ok());

        assert_eq!(manifest.extensions.get("nu").expect("LMS-TEST: Missing key"), "hanidec");
        assert!(manifest.extensions.get("zh").is_none()); // -t- content shouldn't bleed in
    }
}
