// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # Registry Data Shim
//! Ref: [011-LMS-DTO], [010-LMS-MEM]
//! Location: `crates/bistun-lms/src/data/store.rs`
//!
//! **Why**: This module acts as a bridge between the engine and the foundation.
//! It re-exports the authoritative models from `bistun-core` to ensure type consistency
//! without requiring a massive refactor of internal engine imports.

// [FIX]: Redirect all requests for these types to the centralized DNA foundation
pub use bistun_core::{LocaleProfile, RegistryMetadata, RegistryStore};
