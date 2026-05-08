// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV
// ... (Standard GPL Header) ...

//! # Example: Standalone Capability Engine
//! Ref: [012-LMS-ENG]
//! Location: `crates/bistun-lms/examples/standalone_resolver.rs`
//!
//! **Why**: Demonstrates the absolute minimum integration required to boot the capability engine and resolve a BCP 47 tag.
//! **Impact**: Serves as the "Hello World" executable specification for downstream Rust developers embedding the library.

#[cfg(feature = "simulation")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bistun_lms::LinguisticManager;
    use bistun_lms::data::repository::SimulatedSnapshotProvider;

    println!("🚀 Booting Bistun LMS (Standalone Mode)...");

    // [STEP 1]: Instantiate the manager and a simulated provider.
    // The simulated provider automatically generates a valid Ed25519 keypair for testing.
    let manager = LinguisticManager::new();
    let provider = SimulatedSnapshotProvider::new();

    // [STEP 2]: Initial Hydration (Blocking until Ready)
    manager.initialize(&provider, &provider.public_key).await;

    // [STEP 3]: Resolve a Capability (Wait-Free)
    let manifest = manager.resolve_capabilities("zh-TW")?;

    println!("✅ Resolution Successful!");
    println!("📦 Input Tag: zh-TW");
    println!("📦 Canonical Locale: {}", manifest.resolved_locale);

    // Demonstrate High-Water Mark Typology Extraction
    println!(
        "   - Morphology: {:?}",
        manifest.traits.get(&bistun_core::traits::TraitKey::MorphologyType).unwrap()
    );

    // Demonstrate Telemetry Injection
    println!("   - Resolution Path: {}", manifest.metadata.get("resolution_path").unwrap());
    println!("   - Pipeline Latency: {}ms", manifest.metadata.get("resolution_time_ms").unwrap());

    Ok(())
}

// Fallback for when developers run examples without the required features
#[cfg(not(feature = "simulation"))]
fn main() {
    println!(
        "Please run this example with: cargo run --example standalone_resolver --features 'simulation'"
    );
}
