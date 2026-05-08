// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Example: The Lifecycle of Linguistic Truth
//!
//! This example demonstrates the v1.0.0 transition from a Registry Model
//! (LocaleProfile) to a Transmission DTO (CapabilityManifest).

use bistun_core::{
    CapabilityManifest, Direction, LmsError, LocaleProfile, MorphType, NormType, SegType, TraitKey,
    TraitValue, TransType,
};

fn main() -> Result<(), LmsError> {
    println!("🚀 Bistun Core v1.0.0: Registry-to-Manifest Lifecycle");
    println!("---------------------------------------------------------");

    // [STEP 1]: Simulate the "Ground Truth" stored in the WORM Registry
    println!("[1] Fetching LocaleProfile for 'th-TH' from Registry...");
    let profile = LocaleProfile {
        id: "th-TH".to_string(),
        morph: MorphType::ISOLATING,
        base_seg: SegType::DICTIONARY, // High-complexity Thai requirement
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: true,
        plurals: vec!["other".to_string()],
        unicode_blocks: vec!["Thai".to_string()],
        normalization: NormType::NFC,
        transliteration: TransType::ICU_TRANSFORM,
        required_resource: Some("icu_thai".to_string()),
    };
    println!("    -> Identity: {}", profile.id);

    // [STEP 2]: Construct the Transmission DTO
    println!("[2] Mapping Profile to CapabilityManifest...");
    let mut manifest = CapabilityManifest::new(profile.id.clone());

    // [STEP 3]: High-Water Mark Logic (Simulated Aggregation)
    // We map the Registry truth into the serialized Contract format
    manifest.traits.insert(TraitKey::PrimaryDirection, TraitValue::Direction(profile.direction));
    manifest.traits.insert(TraitKey::SegmentationStrategy, TraitValue::SegType(profile.base_seg));
    manifest
        .traits
        .insert(TraitKey::RequiresShaping, TraitValue::Boolean(profile.requires_shaping));
    manifest
        .traits
        .insert(TraitKey::TransliterationType, TraitValue::TransType(profile.transliteration));

    // [STEP 4]: Inject Operational Metadata
    manifest.metadata.insert("registry_version".to_string(), "1.0.0".to_string());
    manifest
        .metadata
        .insert("resource_uri".to_string(), "https://cdn.bistun.io/icu_thai.dat".to_string());

    // [STEP 5]: Final Serialization for the API Layer
    println!("[3] Serializing v1.0.0 Contract to JSON...");
    let json_payload = serde_json::to_string_pretty(&manifest)
        .expect("CRITICAL: Failed to serialize the CapabilityManifest");

    println!("---------------------------------------------------------");
    println!("📦 FINAL API PAYLOAD:");
    println!("{}", json_payload);
    println!("---------------------------------------------------------");

    println!("✅ Walkthrough completed successfully.");
    Ok(())
}
