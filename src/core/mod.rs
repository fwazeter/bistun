// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # The Core Engine Domain
//! Ref: [001-LMS-CORE]
//!
//! **Why**: This module routes the sub-engines of the capability pipeline.
//! **Impact**: If misconfigured, the service boundaries cannot communicate.

pub mod aggregator;
pub mod extension;
pub mod pipeline; // The new engine home!
pub mod resolver;
