// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # Operations & Telemetry Domain
//! Ref: [007-LMS-OPS]
//!
//! **Why**: This module handles system observability, performance metrics, and manifest metadata injection.
//! **Impact**: If this module fails, we lose visibility into system latency, breaching our Service Level Indicators (SLIs).

pub mod telemetry;
