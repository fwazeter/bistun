// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV
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

//! # Example: Standalone Capability Engine
//! Crate: bistun-lms
//! Ref: [012-LMS-ENG], [011-LMS-DTO]
//! Location: `crates/bistun-lms/examples/standalone_resolver.rs`
//!
//! **Why**: Demonstrates the absolute minimum integration required to boot the capability engine and resolve a BCP 47 tag using the V2.0.0 Separation of Domains.
//! **Impact**: Serves as the "Hello World" executable specification for downstream Rust developers embedding the library.

#[cfg(feature = "simulation")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bistun_core::traits::TraitKey;
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
    // We pass an alias (zh-TW) with a user override (-u-nu-hanidec) to demonstrate the pipeline.
    let manifest = manager.resolve_capabilities("zh-TW-u-nu-hanidec")?;

    println!("✅ Resolution Successful!");
    println!("📦 Input Tag: zh-TW-u-nu-hanidec");
    println!("📦 Canonical Locale: {}", manifest.resolved_locale);

    // Demonstrate Domain 1: Linguistic DNA (Immutable Truths)
    let morphology = manifest
        .traits
        .get(&TraitKey::MorphologyType)
        .expect("LMS-OPS: Missing MorphologyType trait");
    println!("   - Morphology (Traits): {morphology:?}");

    // Demonstrate Domain 2: Rule Synthesis (Algorithmic Logic)
    let trans_rule = manifest
        .rules
        .get("TRANSLITERATION_DEFAULT")
        .expect("LMS-OPS: Missing TRANSLITERATION_DEFAULT rule");
    println!("   - Transliteration (Rules): {trans_rule:?}");

    // Demonstrate Domain 4: User Overrides (Extensions)
    let nu_ext = manifest.extensions.get("nu").expect("LMS-OPS: Missing 'nu' extension");
    println!("   - Numbering Override (Extensions): {nu_ext:?}");

    // Demonstrate Domain 5: Telemetry Injection
    let res_path =
        manifest.metadata.get("resolution_path").expect("LMS-OPS: Missing resolution_path");
    let res_time =
        manifest.metadata.get("resolution_time_ms").expect("LMS-OPS: Missing resolution_time_ms");
    println!("   - Resolution Path: {res_path}");
    println!("   - Pipeline Latency: {res_time}ms");

    // Demonstrate Atomic Operational Metrics
    println!(
        "📈 Total Manifests Resolved Since Boot: {}",
        manager.resolution_metrics().total_manifests_resolved
    );

    Ok(())
}

// Fallback for when developers run examples without the required features
#[cfg(not(feature = "simulation"))]
fn main() {
    println!(
        "Please run this example with: cargo run --example standalone_resolver --features 'simulation'"
    );
}
