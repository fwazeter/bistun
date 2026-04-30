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

//! # BCP 47 Taxonomic Resolver
//! Ref: [001-LMS-CORE]
//!
//! **Why**: This module serves as Phase 1 (Resolve) of the pipeline. It maps messy, user-provided BCP 47 tags into a canonical `LocaleEntry` via exact matching and truncation.
//! **Impact**: If this module fails, the system cannot identify the requested language, defaulting every request to the fallback locale (e.g., 'en-US') and breaking localized experiences.
//!
//! ### Glossary
//! * **Truncation**: The process of stripping BCP 47 subtags from right-to-left (e.g., `en-AU-u-nu-latn` -> `en-AU`).
//! * **Canonical ID**: The authoritative tag mapping to a known Typological and Orthographic profile.

use std::fmt;

/// Represents the canonical linguistic profile resolved from the Taxonomy engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleEntry {
    pub id: String,
}

/// Standardized error variants for the LMS capability engine.
#[derive(Debug, PartialEq, Eq)]
pub enum LmsError {
    InvalidTag,
    ResolutionFailed(String),
}

impl fmt::Display for LmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LmsError::InvalidTag => write!(f, "The provided BCP 47 tag is invalid or empty"),
            LmsError::ResolutionFailed(tag) => {
                write!(f, "Failed to resolve locale for tag: {}", tag)
            }
        }
    }
}

/// Resolves a BCP 47 string to a [`LocaleEntry`].
///
/// Time: O(N) where N is the number of subtags | Space: O(1) during truncation.
///
/// # Logic Trace (Internal)
/// 1. **Ingestion**: Sanitize input. Reject empty or trivially invalid strings.
/// 2. **Exact Match**: Attempt to match the raw tag directly against the registry.
/// 3. **Truncation Loop**: Iteratively strip the right-most subtag via `-` and attempt to match.
/// 4. **Return/Fallback**: Return the canonical `LocaleEntry` if matched, otherwise bubble up an `LmsError`.
///
/// # Examples
/// ```rust
/// use bistun::core::resolver::bcp47::resolve;
///
/// let entry = resolve("zh-Hant-TW").unwrap();
/// assert_eq!(entry.id, "zh-Hant");
/// ```
///
/// # Golden I/O
/// * **Input**: "en-AU-u-ca-gregory"
/// * **Output**: `LocaleEntry { id: "en-AU" }`
///
/// # Errors
/// * Returns [`LmsError::InvalidTag`] if the input is empty.
/// * Returns [`LmsError::ResolutionFailed`] if truncation bottoms out without a match.
pub fn resolve(tag: &str) -> Result<LocaleEntry, LmsError> {
    // 1. Ingestion / Sanitization
    let trimmed = tag.trim();
    if trimmed.is_empty() {
        return Err(LmsError::InvalidTag);
    }

    let mut current_tag = trimmed;

    // 2 & 3. Exact Match and Truncation Loop
    loop {
        // [STUB]: In Phase 4, this will be replaced with a `hashbrown::HashMap` lookup
        // against our memory-mapped `Registry`. For now, we stub known values.
        if is_known_locale(current_tag) {
            return Ok(LocaleEntry { id: current_tag.to_string() });
        }

        // Truncate the right-most subtag using high-performance slice manipulation
        match current_tag.rsplit_once('-') {
            Some((prefix, _suffix)) => {
                current_tag = prefix;
            }
            None => {
                // We have truncated down to the base language and found nothing.
                break;
            }
        }
    }

    // 4. Return/Fallback -- Original not compliant to plan
    // Err(LmsError::ResolutionFailed(trimmed.to_string()))
    // [STEP 4]: Return/Fallback
    // Rationale: Following [012-LMS-ENG], we return "en-US" as the
    // ultimate safety net if truncation exhausts.
    Ok(LocaleEntry { id: "en-US".to_string() })
}

/// [\STUB\] Internal helper to represent the `Registry` Exact Match check.
/// Time: O(1)
fn is_known_locale(tag: &str) -> bool {
    matches!(tag, "en-US" | "en-AU" | "ar-EG" | "zh-Hant" | "th-TH")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_exact_match() {
        // [Logic Trace Mapping]
        // 1. Setup/Execute: Pass a perfectly formed tag known to the registry.
        // 2. Assert: Verify O(1) exact match bypasses truncation.
        let result = resolve("ar-EG").expect("Should resolve exactly");
        assert_eq!(result.id, "ar-EG");
    }

    #[test]
    fn test_resolve_truncation_rfc4647() {
        // [Logic Trace Mapping]
        // 1. Setup: Provide a tag overloaded with Unicode extensions (-u-).
        // 2. Execute: Pass through resolver.
        // 3. Assert: Verify the truncation loop cleanly strips down to the base ID.
        let result = resolve("en-AU-u-nu-latn-ca-gregory").expect("Should resolve via truncation");
        assert_eq!(result.id, "en-AU");
    }

    /*#[test]
    fn test_resolve_fails_gracefully_on_unknown() {
        // [Logic Trace Mapping]
        // 1. Setup: Provide a tag that does not exist in our mocked registry.
        // 2. Execute: Pass through resolver to exhaust the truncation loop.
        // 3. Assert: Verify LmsError::ResolutionFailed is returned.
        let err = resolve("xx-YY-u-ext").expect_err("Should fail resolution");
        assert_eq!(err, LmsError::ResolutionFailed("xx-YY-u-ext".to_string()));
    }*/

    #[test]
    fn test_resolve_falls_back_to_default_on_unknown() {
        // [Logic Trace Mapping]
        // 1. Setup: Provide a tag that does not exist in our mocked registry.
        // 2. Execute: Pass through resolver to exhaust truncation.
        // 3. Assert: Verify it returns "en-US" (DefaultFallbackResolver) per [012-LMS-ENG].
        let result = resolve("xx-YY-u-ext").expect("Should now resolve via fallback");
        assert_eq!(result.id, "en-US");
    }

    #[test]
    fn test_resolve_rejects_empty_input() {
        // [Logic Trace Mapping]
        // 1. Setup: Provide whitespace strings.
        // 2. Assert: Verify LmsError::InvalidTag handles the boundary condition.
        assert_eq!(resolve("").unwrap_err(), LmsError::InvalidTag);
        assert_eq!(resolve("   ").unwrap_err(), LmsError::InvalidTag);
    }
}
