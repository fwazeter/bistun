// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Example: Zero-Downtime Hot-Swap & Capability Resolution
//!
//! Demonstrates how to initialize the engine, resolve a `BCP 47` tag,
//! and spawn a wait-free background sync worker.

#[cfg(all(feature = "async-worker", feature = "simulation"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bistun_lms::LinguisticManager;
    use bistun_lms::data::repository::SimulatedSnapshotProvider;
    use std::time::Duration;

    println!("🚀 Booting Bistun LMS Capability Engine...");

    // [STEP 1]: Instantiate the manager and a simulated provider.
    let manager = LinguisticManager::new();
    let provider = SimulatedSnapshotProvider::new();

    // [STEP 2]: Initial Hydration (Blocking until Ready)
    manager.initialize(&provider, &provider.public_key).await;

    let status = manager.status();
    println!("✅ Engine Status: {status:?}");

    // [STEP 3]: Resolve a Capability (Wait-Free)
    // We request Arabic from Egypt, but explicitly ask for Latin numerals (-u-nu-latn)
    let manifest = manager.resolve_capabilities("ar-EG-u-nu-latn")?;
    let resolved_locale = manifest.resolved_locale;
    println!("📦 Resolved Locale: {resolved_locale}");

    let direction = manifest
        .traits
        .get(&bistun_core::traits::TraitKey::PrimaryDirection)
        .expect("LMS-OPS: Missing PrimaryDirection trait");
    println!("   - Direction: {direction:?}");

    let numbering = manifest
        .traits
        .get(&bistun_core::traits::TraitKey::DefaultNumberingSystem)
        .expect("LMS-OPS: Missing DefaultNumberingSystem trait");
    println!("   - Numbering: {numbering:?}");

    let latency = manifest
        .metadata
        .get("resolution_time_ms")
        .expect("LMS-OPS: Missing resolution_time_ms metadata");
    println!("   - Telemetry Latency: {latency}ms");

    // [STEP 4]: Spawn Background Sync (Zero-Downtime Hot-Swap)
    println!("🔄 Spawning background Hot-Swap worker (1-second interval)...");

    // Manually clone the provider so it uses the exact same valid cryptographic signature
    let background_provider = SimulatedSnapshotProvider {
        payload: provider.payload.clone(),
        signature: provider.signature.clone(),
        public_key: provider.public_key.clone(),
    };

    manager.spawn_background_sync(1, background_provider, provider.public_key.clone());

    // Let the background worker fire once to prove stability
    tokio::time::sleep(Duration::from_secs(2)).await;

    let metrics = manager.metrics();
    println!("📊 Final Sync Metrics: {metrics:?}");

    Ok(())
}

// Fallback for when developers run examples without the required features
#[cfg(not(all(feature = "async-worker", feature = "simulation")))]
fn main() {
    println!(
        "Please run this example with: cargo run --example hot_swap_demo --features 'async-worker,simulation'"
    );
}
