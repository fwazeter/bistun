// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

//! # The LMS Curator (Compiler CLI)
//! Ref: [006-LMS-SEC]
//! Location: `src/bin/curator.rs`
//!
//! **Why**: This standalone utility allows administrators to generate authoritative Ed25519 keypairs and sign WORM snapshots.
//! **Impact**: This is the only tool capable of "unlocking" the security gate in the production microservice.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use std::fs;

fn main() {
    println!("🚀 Bistun Curator: Initiating Cryptographic Ceremony...");

    // [STEP 1]: Generate a fresh, cryptographically secure Ed25519 Keypair
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);

    let pub_key_b64 = BASE64.encode(signing_key.verifying_key().as_bytes());
    let priv_key_b64 = BASE64.encode(signing_key.to_bytes());

    println!("--------------------------------------------------");
    println!("🔑 PUBLIC KEY (Paste this into CURATOR_PUBLIC_KEY in sidecar.rs):");
    println!("{}", pub_key_b64);
    println!("--------------------------------------------------");
    println!("🤫 PRIVATE KEY (Keep secret! This is your root of trust):");
    println!("{}", priv_key_b64);
    println!("--------------------------------------------------");

    // [STEP 2]: Load the raw WORM snapshot
    let payload_path = "data/snapshot.json";
    let sig_path = "data/snapshot.sig";

    println!("📝 Reading WORM snapshot from {}...", payload_path);
    let payload = fs::read_to_string(payload_path).expect("CRITICAL: snapshot.json not found in data/ directory");

    // [STEP 3]: Generate Detached Signature
    let signature = signing_key.sign(payload.as_bytes());
    let sig_b64 = BASE64.encode(signature.to_bytes());

    // [STEP 4]: Persist Signature to disk
    fs::write(sig_path, sig_b64).expect("CRITICAL: Failed to write signature to disk");

    println!("✅ SUCCESS: Snapshot signed.");
    println!("💾 Cryptographic signature written to: {}", sig_path);
    println!("⚠️  FINAL STEP: Update your CURATOR_PUBLIC_KEY constant and restart the Sidecar.");
}