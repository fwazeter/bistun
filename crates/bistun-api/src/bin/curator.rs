// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! # The LMS Curator (Compiler CLI)
//! Crate: `bistun-api`
//! Ref: [006-LMS-SEC], [002-LMS-DATA]
//! Location: `crates/bistun-api/src/bin/curator.rs`
//!
//! **Why**: This standalone utility empowers administrators to generate authoritative Ed25519 keypairs, format/canonicalize WORM payloads, and sign distributions.
//! **Impact**: This is the only tool capable of bypassing the Pre-Crypto Header Gate and "unlocking" the security gate in the production microservice.
//!
//! ### Glossary
//! * **Canonicalization**: The process of formatting JSON into a strictly deterministic string (minified, sorted) so cryptographic bytes remain identical across systems.
//! * **WORM Payload**: The final `.json` data bundle distributed to the capability engine.

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bistun_lms::security::verifier::verify_snapshot;
use chrono::Utc;
use clap::{Parser, Subcommand};
use ed25519_dalek::{Signature, Signer, SigningKey};
use rand::rngs::OsRng;
use serde_json::Value;

// =========================================================================
// CRITICAL: Global Cryptography Traits
// We explicitly import KeyInit and Mac from the digest re-export to
// guarantee the compiler resolves the associated methods.
// =========================================================================
use hmac::Hmac;
use hmac::digest::{KeyInit, Mac};
use sha2::{Digest, Sha256};

use std::fs;
use std::path::PathBuf;

/// The root CLI argument parser.
#[derive(Parser, Debug)]
#[command(name = "curator")]
#[command(about = "Bistun LMS Cryptographic Compiler & WORM Signer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available subcommands for the Curator engine.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates a fresh Ed25519 Cryptographic Keypair.
    Keygen,
    /// Ingests, canonicalizes, hashes, and signs a WORM snapshot payload.
    Sign {
        /// Path to the un-signed snapshot JSON file.
        #[arg(short, long, default_value = "data/snapshot.json")]
        payload: PathBuf,
        /// The Base64 Encoded Private Key used for signing.
        #[arg(short, long)]
        key: String,
        /// Optional path to write the detached signature (Defaults to data/snapshot.sig).
        #[arg(short = 'o', long, default_value = "data/snapshot.sig")]
        sig_out: PathBuf,
        /// Optional URL to send a POST webhook to after a successful signature.
        #[arg(short = 'n', long)]
        notify: Option<String>,
        /// Optional HMAC secret for the webhook payload. Required if --notify is used.
        #[arg(short = 'w', long)]
        webhook_secret: Option<String>,
    },
    /// Verifies a signed snapshot locally to ensure production readiness.
    Verify {
        /// Path to the signed snapshot JSON file.
        #[arg(short, long, default_value = "data/snapshot.json")]
        payload: PathBuf,
        /// Path to the detached signature file.
        #[arg(short, long, default_value = "data/snapshot.sig")]
        signature: PathBuf,
        /// The Base64 Encoded Public Key of the authority.
        #[arg(short, long)]
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        // =====================================================================
        // COMMAND: KEYGEN
        // =====================================================================
        Commands::Keygen => {
            println!("🚀 Bistun Curator: Initiating Cryptographic Ceremony...");
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);

            let pub_key_b64 = BASE64.encode(signing_key.verifying_key().as_bytes());
            let priv_key_b64 = BASE64.encode(signing_key.to_bytes());

            println!("--------------------------------------------------");
            println!("🔑 PUBLIC KEY (Pin this to your .env / Kubernetes Secrets):");
            println!("CURATOR_PUBLIC_KEY=\"{}\"", pub_key_b64);
            println!("--------------------------------------------------");
            println!("🤫 PRIVATE KEY (Use this with the `sign` command. KEEP SECRET):");
            println!("{}", priv_key_b64);
            println!("--------------------------------------------------");
        }

        // =====================================================================
        // COMMAND: SIGN
        // =====================================================================
        Commands::Sign { payload, key, sig_out, notify, webhook_secret } => {
            println!("📝 Reading WORM snapshot from {:?}...", payload);

            // [STEP 1]: File I/O
            let raw_json = fs::read_to_string(&payload)
                .expect("LMS-OPS: Target snapshot.json not found or unreadable");
            let mut parsed: Value =
                serde_json::from_str(&raw_json).expect("LMS-OPS: Payload is not valid JSON");

            // [STEP 2]: Canonicalize & Inject Pre-Crypto Headers
            if let Some(obj) = parsed.as_object_mut() {
                let mut hasher = Sha256::new();
                if let Some(profiles) = obj.get("profiles") {
                    hasher.update(profiles.to_string().as_bytes());
                }
                let hash_result = hasher.finalize();
                let hash_hex = hash_result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

                let metadata =
                    obj.entry("metadata").or_insert_with(|| Value::Object(serde_json::Map::new()));

                if let Some(meta_obj) = metadata.as_object_mut() {
                    meta_obj.insert("version".to_string(), Value::String("2.0.0".to_string()));
                    meta_obj
                        .insert("build_date".to_string(), Value::String(Utc::now().to_rfc3339()));
                    meta_obj.insert("checksum".to_string(), Value::String(hash_hex));
                }
            }

            // [STEP 3]: Minify JSON (Zero Whitespace)
            let minified_payload = serde_json::to_string(&parsed)
                .expect("LMS-OPS: Failed to canonicalize JSON structure");

            // [STEP 4]: Hydrate the Signing Key
            let priv_key_bytes =
                BASE64.decode(&key).expect("LMS-OPS: Private key is not valid Base64");
            let signing_key = SigningKey::try_from(priv_key_bytes.as_slice())
                .expect("LMS-OPS: Private key bytes are not a valid Ed25519 signature key");

            // [STEP 5]: Cryptographically Sign
            let signature: Signature = signing_key.sign(minified_payload.as_bytes());
            let sig_b64 = BASE64.encode(signature.to_bytes());

            // [STEP 6]: Persist to Disk
            fs::write(&payload, &minified_payload)
                .expect("LMS-OPS: Failed to overwrite payload with canonicalized JSON");
            fs::write(&sig_out, sig_b64).expect("LMS-OPS: Failed to write signature to disk");

            println!("✅ SUCCESS: Snapshot formatted, hashed, and signed.");
            println!("💾 Minified Payload: {:?}", payload);
            println!("💾 Detached Signature: {:?}", sig_out);

            // [STEP 7]: Transmit Real-Time Webhook Notification (Push Model)
            if let Some(url) = notify {
                println!("🌐 Transmitting real-time webhook notification to {}...", url);
                let secret = webhook_secret
                    .expect("LMS-OPS: --webhook-secret is required when --notify is used.");

                type HmacSha256 = Hmac<Sha256>;

                // Fully qualified trait execution. This eliminates E0425/E0599 permanently.
                let mut mac = <HmacSha256 as KeyInit>::new_from_slice(secret.as_bytes())
                    .expect("LMS-OPS: HMAC can take key of any size");

                mac.update(minified_payload.as_bytes());

                let hmac_result = mac.finalize().into_bytes();
                let hmac_hex: String = hmac_result.iter().map(|b| format!("{:02x}", b)).collect();

                let client = reqwest::blocking::Client::new();
                match client
                    .post(&url)
                    .header("X-LMS-Signature", hmac_hex)
                    .header("Content-Type", "application/json")
                    .body(minified_payload)
                    .send()
                {
                    Ok(res) if res.status().is_success() => {
                        println!("✅ SUCCESS: Webhook acknowledged. Sidecar cache hot-swapped!");
                    }
                    Ok(res) => {
                        println!(
                            "⚠️ WARNING: Webhook delivered but sidecar returned status: {}",
                            res.status()
                        );
                    }
                    Err(e) => {
                        println!("❌ ERROR: Failed to deliver webhook: {}", e);
                    }
                }
            }
        }

        // =====================================================================
        // COMMAND: VERIFY
        // =====================================================================
        Commands::Verify { payload, signature, key } => {
            println!("🔍 Executing LMS Pre-Flight Verification Trace...");

            let raw_payload =
                fs::read_to_string(&payload).expect("LMS-OPS: Snapshot payload missing");
            let raw_signature =
                fs::read_to_string(&signature).expect("LMS-OPS: Snapshot signature missing");

            match verify_snapshot(&raw_payload, &raw_signature, &key) {
                Ok(_) => {
                    println!("✅ INTEGRITY VERIFIED: Payload matches the Public Key authority.")
                }
                Err(e) => {
                    eprintln!("❌ SECURITY FAULT: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
