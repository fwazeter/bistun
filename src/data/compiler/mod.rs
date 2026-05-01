// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # WORM Compiler & Ingestion Engine
//! Ref: [005-LMS-INGEST], [002-LMS-DATA]
//!
//! **Why**: This module processes raw data (ISO/CLDR), applies the Truth Hierarchy, and compiles the finalized, cryptographically signed WORM snapshot.
//! **Impact**: This is a standalone build tool logic domain. It prevents malformed data from ever reaching the runtime memory pools.

pub mod ingest;
pub mod linter;
