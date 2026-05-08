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

//! # Security & Verification Domain
//! Ref: [006-LMS-SEC]
//! Location: `src/security/mod.rs`
//!
//! **Why**: This module validates the cryptographic integrity of linguistic registries before they are loaded into memory. It ensures that the **Linguistic DNA** remains untampered throughout the transit process.
//! **Impact**: If this module is bypassed, the system is vulnerable to supply-chain attacks or "Linguistic Poisoning," which could cause the capability engine to serve malicious or corrupted instructions.
//!
//! ### Glossary
//! * **JWS Verification**: The process of cryptographically proving that a registry snapshot was issued by an authoritative source.
//! * **Linguistic Poisoning**: A threat vector where incorrect trait mappings are injected to degrade search quality or rendering accuracy.

pub mod verifier;
