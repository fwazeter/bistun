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

//! # JWS Registry Verifier
//! Ref: [006-LMS-SEC]
//!
//! **Why**: This module ensures that any WORM snapshot attempting to load into the Flyweight pool was cryptographically signed by the authoritative LMS compiler.
//! **Impact**: Prevents corrupted or malicious locale instructions from entering the system.
//!
//! ### Glossary
//! * **JWS (JSON Web Signature)**: A cryptographic standard for ensuring the payload has not been tampered with.

use crate::core::resolver::bcp47::LmsError;

/// Verifies the cryptographic signature of an incoming WORM snapshot.
///
/// Time: O(N) where N is payload size | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. **Ingestion**: Accept the raw payload and its detached JWS signature.
/// 2. **Presence Check**: Reject immediately if the signature is missing.
/// 3. **Verification**: (Stubbed) Perform cryptographic signature verification against the pinned public key.
/// 4. **Return**: Yield success if valid, or bubble up a `SecurityFault`.
pub fn verify_snapshot(_payload: &str, signature: &str) -> Result<(), LmsError> {
    // 1 & 2. Presence Check
    if signature.trim().is_empty() {
        return Err(LmsError::SecurityFault("Missing signature".to_string()));
    }

    // 3. Verification [STUB for 006-LMS-SEC]
    // In future phases, this will use standard ECDSA/RSA validation against a pinned public key.
    if signature != "valid-lms-signature" {
        return Err(LmsError::SecurityFault("Invalid cryptographic signature".to_string()));
    }

    // 4. Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_passes_valid_signature() {
        let payload = "{\"registry\": \"data\"}";
        assert!(verify_snapshot(payload, "valid-lms-signature").is_ok());
    }

    #[test]
    fn test_verify_catches_invalid_signature() {
        let payload = "{\"registry\": \"malicious-data\"}";
        let err = verify_snapshot(payload, "forged-signature").unwrap_err();
        assert!(matches!(err, LmsError::SecurityFault(_)));
    }

    #[test]
    fn test_verify_catches_empty_signature() {
        let payload = "{\"registry\": \"data\"}";
        let err = verify_snapshot(payload, "   ").unwrap_err();
        assert!(matches!(err, LmsError::SecurityFault(_)));
    }
}
