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
//! Ref: [001-LMS-CORE], [012-LMS-ENG]
//!
//! **Why**: This module serves as Phase 1 (Resolve) of the pipeline. It maps messy, user-provided BCP 47 tags into a canonical `LocaleEntry` via exact matching and truncation.
//! **Impact**: If this module fails, the system cannot identify the requested language, defaulting every request to the fallback locale (e.g., 'en-US') and breaking localized experiences.
//!
//! ### Glossary
//! * **Truncation**: The process of stripping BCP 47 subtags from right-to-left (e.g., `en-AU-u-nu-latn` -> `en-AU`).
//! * **Canonical ID**: The authoritative tag mapping to a known Typological and Orthographic profile.

use crate::data::swap::RegistryState;
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
    IntegrityViolation(String),
}

impl fmt::Display for LmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LmsError::InvalidTag => write!(f, "The provided BCP 47 tag is invalid or empty"),
            LmsError::ResolutionFailed(tag) => {
                write!(f, "Failed to resolve locale for tag: {}", tag)
            }
            LmsError::IntegrityViolation(msg) => {
                write!(f, "Integrity violation: {}", msg)
            }
        }
    }
}

/// Resolves a BCP 47 string to a [`LocaleEntry`] using the dynamic RegistryState.
///
/// Time: O(N) where N is the number of subtags | Space: O(1) during truncation.
///
/// # Logic Trace (Internal)
/// 1. **Ingestion**: Sanitize input. Reject empty or trivially invalid strings.
/// 2. **Exact Match**: Attempt to match the raw tag directly against the dynamic registry state.
/// 3. **Truncation Loop**: Iteratively strip the right-most subtag via `-` and attempt to match.
/// 4. **Return/Fallback**: Return the canonical `LocaleEntry` if matched, otherwise fallback to `en-US` per `012-LMS-ENG`.
///
/// # Errors
/// * Returns [`LmsError::InvalidTag`] if the input is empty.
pub fn resolve(tag: &str, state: &RegistryState) -> Result<LocaleEntry, LmsError> {
    // 1. Ingestion / Sanitization
    let trimmed = tag.trim();
    if trimmed.is_empty() {
        return Err(LmsError::InvalidTag);
    }

    let mut current_tag = trimmed;

    // 2 & 3. Exact Match and Truncation Loop
    loop {
        // [DYNAMIC LOOKUP]: Check the memory pool
        if state.get_profile(current_tag).is_some() {
            return Ok(LocaleEntry { id: current_tag.to_string() });
        }

        // Truncate the right-most subtag using high-performance slice manipulation
        match current_tag.rsplit_once('-') {
            Some((prefix, _suffix)) => {
                current_tag = prefix;
            }
            None => {
                break;
            }
        }
    }

    // 4. Default Fallback Resolver [Ref: 012-LMS-ENG]
    // If truncation exhausts and we still have nothing, we ensure the system does not crash
    // by returning the system default.
    Ok(LocaleEntry { id: "en-US".to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::repository;

    fn setup_state() -> RegistryState {
        let state = RegistryState::new();
        if let Ok(store) = repository::hydrate_snapshot() {
            state.swap_registry(store);
        }
        state
    }

    #[test]
    fn test_resolve_exact_match() {
        let state = setup_state();
        let result = resolve("ar-EG", &state).expect("Should resolve exactly");
        assert_eq!(result.id, "ar-EG");
    }

    #[test]
    fn test_resolve_truncation_rfc4647() {
        let state = setup_state();
        // Since our mock repository doesn't have "en-AU", let's use "ar-EG" for the truncation test
        let result =
            resolve("ar-EG-u-nu-latn-ca-gregory", &state).expect("Should resolve via truncation");
        assert_eq!(result.id, "ar-EG");
    }

    #[test]
    fn test_resolve_falls_back_to_default_on_unknown() {
        let state = setup_state();
        let result = resolve("xx-YY-u-ext", &state).expect("Should now resolve via fallback");
        assert_eq!(result.id, "en-US");
    }

    #[test]
    fn test_resolve_rejects_empty_input() {
        let state = setup_state();
        assert_eq!(resolve("", &state).unwrap_err(), LmsError::InvalidTag);
        assert_eq!(resolve("   ", &state).unwrap_err(), LmsError::InvalidTag);
    }
}
