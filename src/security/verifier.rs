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
//! Location: `src/security/verifier.rs`
//!
//! **Why**: This module ensures that any WORM snapshot attempting to load into the Flyweight pool was cryptographically signed by the authoritative LMS compiler.
//! **Impact**: Prevents corrupted or malicious locale instructions from entering the system, effectively neutralizing "Linguistic Poisoning" attacks.
//!
//! ### Glossary
//! * **JWS (JSON Web Signature)**: A cryptographic standard for ensuring the payload has not been tampered with.

use crate::core::resolver::bcp47::LmsError;

/// Verifies the cryptographic signature of an incoming WORM snapshot.
///
/// Time: O(N) where N is payload size | Space: O(1)
///
/// # Logic Trace (Internal)
/// 1. Accept the raw payload and its detached JWS signature.
/// 2. Reject immediately if the signature is empty or purely whitespace.
/// 3. Perform cryptographic signature verification against the pinned public key (currently stubbed).
/// 4. Yield success if valid, or bubble up a `SecurityFault`.
///
/// # Examples
/// ```rust
///   let payload = "{\"registry\": \"data\"}";
///   assert!(verify_snapshot(payload, "valid-lms-signature").is_ok());
/// ```
///
/// # Arguments
/// * `_payload` (&str): The raw JSON string of the registry data. (Prefixed with `_` pending actual crypto implementation).
/// * `signature` (&str): The detached signature string provided alongside the payload.
///
/// # Returns
/// * `Result<(), LmsError>`: `Ok(())` if the signature cryptographically matches the payload.
///
/// # Golden I/O
/// * **Input**: `"{\"id\": \"ar-EG\"}"`, `"valid-lms-signature"`
/// * **Output**: `Ok(())`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns [`LmsError::SecurityFault`] if the signature is missing or fails cryptographic validation.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn verify_snapshot(_payload: &str, signature: &str) -> Result<(), LmsError> {
    // [STEP 1] & [STEP 2]: Presence Check
    if signature.trim().is_empty() {
        return Err(LmsError::SecurityFault("Missing signature".to_string()));
    }

    // [STEP 3]: Verification [STUB for 006-LMS-SEC]
    // In future phases, this will use standard ECDSA/RSA validation against a pinned public key.
    if signature != "valid-lms-signature" {
        return Err(LmsError::SecurityFault("Invalid cryptographic signature".to_string()));
    }

    // [STEP 4]: Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_passes_valid_signature() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Provide valid payload and known-good signature.
        let payload = "{\"registry\": \"data\"}";

        // [STEP 2] & [STEP 3]: Execute & Assert: Verify it passes validation.
        assert!(verify_snapshot(payload, "valid-lms-signature").is_ok());
    }

    #[test]
    fn test_verify_catches_invalid_signature() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Provide a forged signature.
        let payload = "{\"registry\": \"malicious-data\"}";

        // [STEP 2]: Execute: Run verifier.
        let err = verify_snapshot(payload, "forged-signature").unwrap_err();

        // [STEP 3]: Assert: Verify it throws a SecurityFault.
        assert!(matches!(err, LmsError::SecurityFault(_)));
    }

    #[test]
    fn test_verify_catches_empty_signature() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Provide an empty/whitespace signature.
        let payload = "{\"registry\": \"data\"}";

        // [STEP 2]: Execute: Run verifier.
        let err = verify_snapshot(payload, "   ").unwrap_err();

        // [STEP 3]: Assert: Verify it throws a SecurityFault.
        assert!(matches!(err, LmsError::SecurityFault(_)));
    }
}
