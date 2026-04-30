// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # Validation Domain
//! Ref: [003-LMS-VAL]
//!
//! **Why**: This module acts as the QA gatekeeper for the capability engine.
//! **Impact**: Prevents corrupted or mechanically impossible trait combinations from reaching downstream rendering systems.

pub mod integrity;
