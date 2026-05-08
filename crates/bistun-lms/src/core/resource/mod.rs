// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # Resource Resolution Domain
//! Ref: [001-LMS-CORE], [002-LMS-DATA]
//! Location: `src/core/resource/mod.rs`
//!
//! **Why**: This module coordinates Phase 2.5 of the pipeline, translating abstract resource IDs into physical URIs.
//! **Impact**: Failure in this domain prevents consuming applications from downloading necessary binary data blobs (like ICU4X data).
//!
//! ### Glossary
//! * **Pointer Pattern**: Synthesizing a URL string for the client to resolve externally, rather than streaming heavy files through the microservice.

pub mod resolver;
