// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # Security & Verification Domain
//! Ref: [006-LMS-SEC]
//!
//! **Why**: This module validates the cryptographic integrity of linguistic registries before they are loaded into memory.
//! **Impact**: If this module fails or is bypassed, the system is vulnerable to supply-chain attacks and corrupted data hydration.

pub mod verifier;
