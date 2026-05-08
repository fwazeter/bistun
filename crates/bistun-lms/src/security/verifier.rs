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
//! * **Ed25519**: A high-speed, highly secure public-key signature system.

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bistun_core::error::LmsError;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

/// Verifies the cryptographic signature of an incoming WORM snapshot using Ed25519.
///
/// Time: O(N) where N is payload size | Space: O(1) beyond key parsing
///
/// # Logic Trace (Internal)
/// 1. **Decode Public Key**: Parse the Base64 pinned public key into an Ed25519 `VerifyingKey`.
/// 2. **Decode Signature**: Parse the Base64 signature string into a cryptographic `Signature`.
/// 3. **Verify Cryptographic Integrity**: Execute the Ed25519 verification against the raw payload bytes.
/// 4. **Return Success**: Yield `Ok(())` if the signature is authentic.
///
/// # Examples
/// ```text
/// // See internal `tests` module for hermetic execution using dynamic Ed25519 keypairs.
/// ```
///
/// # Arguments
/// * `payload` (&str): The raw JSON string of the WORM snapshot.
/// * `signature_b64` (&str): The Base64 encoded Ed25519 signature of the payload.
/// * `public_key_b64` (&str): The Base64 encoded Ed25519 public key of the authoritative Curator.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful cryptographic verification.
///
/// # Golden I/O
/// * **Input**: `("{\"profiles\":[]}", "Base64_Signature", "Base64_Public_Key")`
/// * **Output**: `Ok(())`
///
/// # Errors, Panics, & Safety
/// * **Errors**: Returns `LmsError::SecurityFault` if key/signature parsing fails, or if the mathematical verification fails.
/// * **Panics**: None.
/// * **Safety**: Safe synchronous execution.
pub fn verify_snapshot(
    payload: &str,
    signature_b64: &str,
    public_key_b64: &str,
) -> Result<(), LmsError> {
    // [STEP 1]: Decode Public Key
    let pub_key_bytes = BASE64.decode(public_key_b64).map_err(|e| LmsError::SecurityFault {
        pipeline_step: "Phase 0: Security Gate".to_string(),
        context: "Key Parsing".to_string(),
        reason: format!("Failed to decode Base64 public key: {}", e),
    })?;

    let verifying_key =
        VerifyingKey::try_from(pub_key_bytes.as_slice()).map_err(|e| LmsError::SecurityFault {
            pipeline_step: "Phase 0: Security Gate".to_string(),
            context: "Key Parsing".to_string(),
            reason: format!("Invalid Ed25519 public key format: {}", e),
        })?;

    // [STEP 2]: Decode Signature
    let sig_bytes = BASE64.decode(signature_b64).map_err(|e| LmsError::SecurityFault {
        pipeline_step: "Phase 0: Security Gate".to_string(),
        context: "Signature Parsing".to_string(),
        reason: format!("Failed to decode Base64 signature: {}", e),
    })?;

    let signature = Signature::from_slice(&sig_bytes).map_err(|e| LmsError::SecurityFault {
        pipeline_step: "Phase 0: Security Gate".to_string(),
        context: "Signature Parsing".to_string(),
        reason: format!("Invalid Ed25519 signature format: {}", e),
    })?;

    // [STEP 3]: Verify Cryptographic Integrity
    verifying_key.verify(payload.as_bytes(), &signature).map_err(|_| LmsError::SecurityFault {
        pipeline_step: "Phase 0: Security Gate".to_string(),
        context: "Cryptographic Validation".to_string(),
        reason: "CRITICAL: Snapshot signature does not match the authoritative Public Key!"
            .to_string(),
    })?;

    // [STEP 4]: Return Success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;

    /// Helper to dynamically generate a valid Ed25519 keypair and signature for testing.
    fn generate_test_credentials(payload: &str) -> (String, String) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        let public_key = BASE64.encode(signing_key.verifying_key().as_bytes());
        let signature = BASE64.encode(signing_key.sign(payload.as_bytes()).to_bytes());

        (signature, public_key)
    }

    #[test]
    fn test_verify_passes_valid_signature() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Provide valid payload and generate matching Ed25519 signature.
        let payload = "{\"registry\": \"data\"}";
        let (valid_sig, pub_key) = generate_test_credentials(payload);

        // [STEP 2] & [STEP 3]: Execute & Assert: Verify it passes cryptographic validation.
        assert!(verify_snapshot(payload, &valid_sig, &pub_key).is_ok());
    }

    #[test]
    fn test_verify_catches_invalid_signature() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Generate valid keys, but forge a signature of the correct length.
        let payload = "{\"registry\": \"data\"}";
        let (_, pub_key) = generate_test_credentials(payload);

        // 64-byte forged signature
        let forged_bytes = [0u8; 64];
        let forged_sig = BASE64.encode(forged_bytes);

        // [STEP 2]: Execute: Run verifier against forged data.
        let err = verify_snapshot(payload, &forged_sig, &pub_key).unwrap_err();

        // [STEP 3]: Assert: Verify it throws a SecurityFault due to math mismatch.
        assert!(matches!(err, LmsError::SecurityFault { .. }));
    }

    #[test]
    fn test_verify_catches_malformed_base64() {
        // [Logic Trace Mapping]
        // [STEP 1]: Setup: Provide invalid Base64 strings.
        let payload = "{\"registry\": \"data\"}";

        // [STEP 2]: Execute: Run verifier.
        let err = verify_snapshot(payload, "invalid_b64!@#", "invalid_b64!@#").unwrap_err();

        // [STEP 3]: Assert: Verify it throws a SecurityFault during parsing.
        assert!(matches!(err, LmsError::SecurityFault { .. }));
    }
}
