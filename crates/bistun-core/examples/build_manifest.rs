// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Example: The Lifecycle of Linguistic Truth
//! Crate: `bistun-core`
//! Ref: [011-LMS-DTO]
//! Location: `crates/bistun-core/examples/build_manifest.rs`
//!
//! This example demonstrates the v2.0.0 transition from a Registry Model
//! (`LocaleProfile`) to a Transmission DTO (`CapabilityManifest`).

use bistun_core::{
    CapabilityManifest, CasingRule, Direction, LmsRule, LocaleProfile, MorphType, NormRule,
    PluralRule, SegType, TraitKey, TraitValue, TransRule,
};
use hashbrown::HashMap;

/// Entry point demonstrating the registry-to-manifest lifecycle.
///
/// # Logic Trace (Internal)
/// 1. Simulate the "Ground Truth" stored in the `WORM` Registry.
/// 2. Construct the transmission `DTO`.
/// 3. Perform high-water mark aggregation and resource bridging.
/// 4. Serialize the final contract to `JSON`.
fn main() {
    println!("🚀 Bistun Core v2.0.0: Registry-to-Manifest Lifecycle");
    println!("---------------------------------------------------------");

    // [STEP 1]: Simulate the "Ground Truth" stored in the WORM Registry
    println!("[1] Fetching LocaleProfile for 'th-TH' from Registry...");

    let mut traits = HashMap::new();
    traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::ISOLATING));
    traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(SegType::DICTIONARY));
    traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(Direction::LTR));
    traits.insert(TraitKey::HasBidiElements, TraitValue::Boolean(true));
    traits.insert(TraitKey::DefaultNumberingSystem, TraitValue::String("thai".to_string()));

    let mut rules = HashMap::new();
    rules.insert("TRANSLITERATION_DEFAULT".to_string(), LmsRule::Trans(TransRule::NONE));
    rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFC));
    rules.insert("PLURAL_LOGIC".to_string(), LmsRule::Plural(PluralRule::CARDINAL_ONLY));
    rules.insert("CASING_STRATEGY".to_string(), LmsRule::Casing(CasingRule::CASE_SENSITIVE));

    let mut resources = HashMap::new();
    resources.insert("icu_thai".to_string(), "required".to_string());

    let profile = LocaleProfile { id: "th-TH".to_string(), traits, rules, resources };
    println!("    -> Identity: {}", profile.id);

    // [STEP 2]: Construct the Transmission DTO
    println!("[2] Mapping Profile to CapabilityManifest...");
    let mut manifest = CapabilityManifest::new(profile.id.clone());

    // [STEP 3]: High-Water Mark Logic (Simulated Aggregation)
    // We map the Registry truth into the serialized Contract format using efficient clone_from
    manifest.traits.clone_from(&profile.traits);
    manifest.rules.clone_from(&profile.rules);

    // [STEP 4]: Resource Bridge
    // Translate logical asset IDs into physical URIs for the client using concise iteration
    for (key, _val) in &profile.resources {
        manifest.resources.insert(key.clone(), format!("https://cdn.bistun.io/v2/{key}.dat"));
    }

    // [STEP 5]: Inject Operational Metadata & User Overrides
    manifest.extensions.insert("nu".to_string(), "latn".to_string());
    manifest.metadata.insert("registry_version".to_string(), "2.0.0".to_string());
    manifest.metadata.insert("resolution_time_ms".to_string(), "0.4501".to_string());

    // [STEP 6]: Final Serialization for the API Layer
    println!("[3] Serializing v2.0.0 Contract to JSON...");
    let json_payload = serde_json::to_string_pretty(&manifest)
        .expect("LMS-OPS: Failed to serialize the CapabilityManifest");

    println!("---------------------------------------------------------");
    println!("📦 FINAL API PAYLOAD:");
    println!("{json_payload}");
    println!("---------------------------------------------------------");

    println!("✅ Walkthrough completed successfully.");
}
