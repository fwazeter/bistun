// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Example: Zero-Downtime Hot-Swap & Capability Resolution
//!
//! Demonstrates how to initialize the engine, resolve a BCP 47 tag,
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
    println!("✅ Engine Status: {:?}", manager.status());

    // [STEP 3]: Resolve a Capability (Wait-Free)
    // We request Arabic from Egypt, but explicitly ask for Latin numerals (-u-nu-latn)
    let manifest = manager.resolve_capabilities("ar-EG-u-nu-latn")?;
    println!("📦 Resolved Locale: {}", manifest.resolved_locale);
    println!(
        "   - Direction: {:?}",
        manifest.traits.get(&bistun_core::traits::TraitKey::PrimaryDirection).unwrap()
    );
    println!(
        "   - Numbering: {:?}",
        manifest.traits.get(&bistun_core::traits::TraitKey::NumberingSystem).unwrap()
    );
    println!("   - Telemetry Latency: {}ms", manifest.metadata.get("resolution_time_ms").unwrap());

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

    println!("📊 Final Sync Metrics: {:?}", manager.metrics());
    Ok(())
}

// Fallback for when developers run examples without the required features
#[cfg(not(all(feature = "async-worker", feature = "simulation")))]
fn main() {
    println!(
        "Please run this example with: cargo run --example hot_swap_demo --features 'async-worker,simulation'"
    );
}
