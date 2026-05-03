// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
// Location: `src/strategy/stemming/mod.rs`

//! # Stemming Strategies
//! Ref: [009-LMS-STRAT]
//!
//! **Why**: This module serves as the registry boundary for all Typological stemming algorithms (e.g., Isolating, Agglutinative, Fusional).
//! **Impact**: Without this module, the `StemmingProvider` cannot route linguistic metadata to concrete execution logic, breaking NLP search and indexing pipelines.
//!
//! ### Glossary
//! * **Stemming**: The process of reducing inflected (or sometimes derived) words to their word stem, base, or root form.

pub mod isolating;
