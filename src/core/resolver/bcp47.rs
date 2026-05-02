// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! # BCP 47 Taxonomic Resolver
//! Ref: [001-LMS-CORE], [012-LMS-ENG], [005-LMS-INGEST]
//! Location: `src/core/resolver/bcp47.rs`
//!
//! **Why**: This module serves as Phase 1 (Resolve) of the pipeline. It maps messy, user-provided BCP 47 tags into a canonical `LocaleEntry` using a Chain of Responsibility.
//! **Impact**: It is the system's "Identity Gate." If this module fails to resolve aliases or macrolanguages, localized rendering will default to 'en-US', resulting in cultural data loss.
//!
//! ### Glossary
//! * **Canonicalization**: The process of mapping deprecated or broad tags (aliases/macrolanguages) to specific, operational identities (e.g., `in` -> `id`).
//! * **Truncation**: Iteratively stripping subtags from right-to-left to find the nearest supported parent.

use crate::core::resolver::IResolver;
use crate::data::swap::IRegistryState;
use std::fmt;

/// Represents the canonical linguistic profile resolved from the Taxonomy engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleEntry {
    pub id: String,
    pub resolution_path: Vec<String>,
}

/// Standardized error variants for the LMS capability engine.
#[derive(Debug, PartialEq, Eq)]
pub enum LmsError {
    InvalidTag,
    ResolutionFailed(String),
    IntegrityViolation(String),
    SecurityFault(String),
}

impl fmt::Display for LmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LmsError::InvalidTag => write!(f, "The provided BCP 47 tag is invalid or empty"),
            LmsError::ResolutionFailed(msg) => {
                write!(f, "Failed to resolve locale for tag: {}", msg)
            }
            LmsError::IntegrityViolation(msg) => write!(f, "Integrity violation: {}", msg),
            LmsError::SecurityFault(msg) => write!(f, "Registry security fault: {}", msg),
        }
    }
}

// -----------------------------------------------------------------------------
// Concrete Resolvers
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct ExactMatchResolver {
    next: Option<Box<dyn IResolver>>,
}

impl ExactMatchResolver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IResolver for ExactMatchResolver {
    fn resolve(
        &self,
        tag: &str,
        state: &dyn IRegistryState,
        path: &mut Vec<String>,
    ) -> Option<LocaleEntry> {
        // [STEP 1]: Perform an exact $O(1)$ lookup in the active memory pool.
        if state.get_profile(tag).is_some() {
            path.push(tag.to_string());
            return Some(LocaleEntry { id: tag.to_string(), resolution_path: path.clone() });
        }

        // [STEP 2]: Delegate to next resolver if no match.
        self.next.as_ref().and_then(|n| n.resolve(tag, state, path))
    }

    fn set_next(&mut self, next: Box<dyn IResolver>) {
        self.next = Some(next);
    }
}

#[derive(Default)]
pub struct TruncationResolver {
    next: Option<Box<dyn IResolver>>,
}

impl TruncationResolver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IResolver for TruncationResolver {
    fn resolve(
        &self,
        tag: &str,
        state: &dyn IRegistryState,
        path: &mut Vec<String>,
    ) -> Option<LocaleEntry> {
        let mut current_tag = tag;

        // [STEP 1]: Iteratively strip subtags right-to-left using '-' as the delimiter.
        while let Some((prefix, _suffix)) = current_tag.rsplit_once('-') {
            current_tag = prefix;

            // [STEP 2]: Perform O(1) registry check for truncated candidate.
            if state.get_profile(current_tag).is_some() {
                path.push(current_tag.to_string());
                return Some(LocaleEntry {
                    id: current_tag.to_string(),
                    resolution_path: path.clone(),
                });
            }
        }

        // [STEP 3]: Delegate to next if truncation exhausts parentage.
        self.next.as_ref().and_then(|n| n.resolve(tag, state, path))
    }

    fn set_next(&mut self, next: Box<dyn IResolver>) {
        self.next = Some(next);
    }
}

/// Terminal node in the resolution chain.
///
/// Note: Does not store a 'next' field to satisfy dead-code constraints.
#[derive(Default)]
pub struct DefaultFallbackResolver;

impl DefaultFallbackResolver {
    pub fn new() -> Self {
        Self
    }
}

impl IResolver for DefaultFallbackResolver {
    fn resolve(
        &self,
        _tag: &str,
        _state: &dyn IRegistryState,
        path: &mut Vec<String>,
    ) -> Option<LocaleEntry> {
        // [STEP 1]: Append final canonical safety to the path.
        path.push("en-US".to_string());
        Some(LocaleEntry { id: "en-US".to_string(), resolution_path: path.clone() })
    }

    fn set_next(&mut self, _next: Box<dyn IResolver>) {
        // [STEP 2]: Terminal node; delegation is intentionally ignored.
    }
}

// -----------------------------------------------------------------------------
// Core Orchestrator
// -----------------------------------------------------------------------------

/// Resolves a BCP 47 string to a [`LocaleEntry`] using the Chain of Responsibility.
///
/// Time: O(N) where N is the number of subtags | Space: O(N) for path tracking.
///
/// # Logic Trace (Internal)
/// 1. Sanitize input and verify presence.
/// 2. Canonicalize aliases and macrolanguages using the Truth Hierarchy.
/// 3. Construct the resolver chain (`Exact Match` -> `Truncation` -> `Default`).
/// 4. Execute the chain and return the resulting `LocaleEntry`.
///
/// # Arguments
/// * `tag` (&str): The raw BCP 47 language tag requested.
/// * `state` (&dyn IRegistryState): The active Flyweight pool of definitions.
///
/// # Returns
/// * `Result<LocaleEntry, LmsError>`: The resolved canonical ID and audit path.
///
/// # Golden I/O
/// * **Input**: `"in-ID"`, `RegistryState`
/// * **Output**: `Ok(LocaleEntry { id: "id", resolution_path: ["id"] })`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns `LmsError::InvalidTag` if input is empty.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn resolve(tag: &str, state: &dyn IRegistryState) -> Result<LocaleEntry, LmsError> {
    // [STEP 1]: Sanitization
    let trimmed = tag.trim();
    if trimmed.is_empty() {
        return Err(LmsError::InvalidTag);
    }

    // [STEP 2]: Canonicalization (Likely Subtags / Aliases) [Ref: 005-LMS-INGEST]
    // These mappings resolve legacy/macrolanguage inputs to operational Flyweight profiles.
    let canonical_id = match trimmed {
        "in" | "in-ID" => "id",
        "iw" | "iw-IL" => "he",
        "no" => "nb",
        "zh-TW" => "zh-Hant",
        "zh-CN" => "zh-Hans",
        _ => trimmed,
    };

    // [STEP 3]: Chain Construction [Ref: 012-LMS-ENG]
    let mut exact_resolver = ExactMatchResolver::new();
    let mut trunc_resolver = TruncationResolver::new();
    let default_resolver = DefaultFallbackResolver::new();

    trunc_resolver.set_next(Box::new(default_resolver));
    exact_resolver.set_next(Box::new(trunc_resolver));

    // [STEP 4]: Execution
    let mut resolution_path = Vec::with_capacity(4);

    exact_resolver
        .resolve(canonical_id, state, &mut resolution_path)
        .ok_or_else(|| LmsError::ResolutionFailed(canonical_id.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::store::LocaleProfile;
    use crate::models::traits::{Direction, MorphType, SegType};
    use mockall::mock;
    use std::sync::Arc;

    mock! {
        pub RegistryState {}
        impl IRegistryState for RegistryState {
            fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>>;
        }
    }

    fn create_stub(id: &str) -> Arc<LocaleProfile> {
        Arc::new(LocaleProfile {
            id: id.to_string(),
            morph: MorphType::FUSIONAL,
            base_seg: SegType::SPACE,
            alt_seg: None,
            direction: Direction::LTR,
            has_bidi: false,
            requires_shaping: false,
            plurals: vec![],
        })
    }

    #[test]
    fn test_resolve_handles_alias_indonesian() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Indonesian 'id' exists, 'in' is the alias.
        let mut mock_state = MockRegistryState::new();
        mock_state
            .expect_get_profile()
            .with(mockall::predicate::eq("id"))
            .returning(|_| Some(create_stub("id")));
        mock_state.expect_get_profile().returning(|_| None);

        // [STEP 2] & [STEP 3]: Execute & Assert: Alias 'in' resolved to 'id'.
        let entry = resolve("in", &mock_state).unwrap();
        assert_eq!(entry.id, "id");
        assert_eq!(entry.resolution_path[0], "id");
    }

    #[test]
    fn test_resolve_handles_macrolanguage_norwegian() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Bokmål 'nb' exists, macrolanguage 'no' is the input.
        let mut mock_state = MockRegistryState::new();
        mock_state
            .expect_get_profile()
            .with(mockall::predicate::eq("nb"))
            .returning(|_| Some(create_stub("nb")));
        mock_state.expect_get_profile().returning(|_| None);

        // [STEP 2] & [STEP 3]: Execute & Assert: 'no' resolved to 'nb'.
        let entry = resolve("no", &mock_state).unwrap();
        assert_eq!(entry.id, "nb");
    }
}
