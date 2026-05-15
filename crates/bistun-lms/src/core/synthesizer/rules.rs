// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! # Rule Synthesizer Engine
//! Crate: `bistun-lms`
//! Ref: [013-LMS-RULE]
//! Location: `crates/bistun-lms/src/core/synthesizer/rules.rs`
//!
//! **Why**: This module acts as the "Logic Bridge" (Phase 2.5), translating static linguistic traits into actionable software execution rules and ensuring systemic defaults.
//! **Impact**: If this fails, downstream SDKs will receive raw data without knowing *how* to execute algorithms like stemming, or they will crash due to missing normalization baselines.
//!
//! ### Glossary
//! * **Rule Synthesis**: The $O(1)$ process of deriving a functional command (Rule) from a static classification (Trait).
//! * **Default Mechanics**: Fallback rules (e.g., NFC Normalization) injected to guarantee computational safety.

use bistun_core::error::LmsError;
use bistun_core::manifest::{CapabilityManifest, TraitValue};
use bistun_core::traits::{LmsRule, MorphType, NormRule, TraitKey, TransRule};

/// Derives functional execution rules from the populated traits of a manifest.
///
/// Time: $O(1)$ | Space: $O(1)$ (Mutates in-place)
///
/// # Logic Trace (Internal)
/// 1. \[Ingestion\]: Inspect the `traits` map of the provided manifest.
/// 2. \[Synthesis\]: Extract the `MorphologyType` trait.
/// 3. \[Mapping\]: Match the morphological typology to a specific algorithmic rule.
/// 4. \[Injection\]: Insert the derived `STEMMING_STRATEGY` into the `rules` map of the manifest.
/// 5. \[Default Mechanics\]: Use the entry API to inject `NORMALIZATION_DEFAULT` and `TRANSLITERATION_DEFAULT` if they were not explicitly defined by the registry.
///
/// # Examples
/// ```rust
/// # use bistun_core::manifest::{CapabilityManifest, TraitValue};
/// # use bistun_core::traits::{MorphType, TraitKey, LmsRule, NormRule};
/// # use bistun_lms::core::synthesizer::rules::synthesize_rules;
/// let mut manifest = CapabilityManifest::new("ar-EG".to_string());
/// manifest.traits.insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::TEMPLATIC));
///
/// synthesize_rules(&mut manifest).expect("LMS-TEST: Synthesis failed");
/// assert!(manifest.rules.contains_key("STEMMING_STRATEGY"));
/// assert_eq!(manifest.rules.get("NORMALIZATION_DEFAULT"), Some(&LmsRule::Norm(NormRule::NFC)));
/// ```
///
/// # Arguments
/// * `manifest` (&mut [`CapabilityManifest`]): The mutable DTO currently flowing through the resolution pipeline.
///
/// # Returns
/// * `Result<(), LmsError>`: Returns `Ok(())` upon successful in-place mutation.
///
/// # Golden I/O
/// * **Input**: `manifest.traits` containing `MorphType::TEMPLATIC`
/// * **Output**: `manifest.rules` containing `"STEMMING_STRATEGY"`, `"NORMALIZATION_DEFAULT"`, and `"TRANSLITERATION_DEFAULT"`.
///
/// # Errors
/// * Returns [`LmsError`] if the synthesis logic detects a "Linguistic Chimera" that escaped the Phase 2 aggregator.
///
/// # Panics
/// * None.
///
/// # Safety
/// * Fully safe synchronous execution.
///
/// # Side Effects
/// * Mutates the `rules` dictionary of the provided manifest in-place to avoid heap allocations.
#[must_use = "The Result of synthesize_rules must be handled to ensure manifest integrity before Phase 4"]
pub fn synthesize_rules(manifest: &mut CapabilityManifest) -> Result<(), LmsError> {
    // [STEP 1 & 2]: Extract Morphology Type
    if let Some(TraitValue::MorphType(morph)) = manifest.traits.get(&TraitKey::MorphologyType) {
        // [STEP 3]: Map to Strategy
        let rule = match morph {
            MorphType::TEMPLATIC | MorphType::AGGLUTINATIVE | MorphType::POLYSYNTHETIC => {
                LmsRule::Trans(TransRule::ICU_TRANSFORM)
            }
            _ => LmsRule::Trans(TransRule::ICU_TRANSFORM),
        };

        // [STEP 4]: Inject STEMMING_STRATEGY
        manifest.rules.insert("STEMMING_STRATEGY".to_string(), rule);
    }

    // =========================================================================
    // PHASE 2.B.3: DEFAULT MECHANICS
    // =========================================================================
    // [STEP 5]: Guarantee critical fallbacks for Phase 4 Integrity without
    // overwriting explicit overrides that might exist in the WORM snapshot.
    manifest
        .rules
        .entry("NORMALIZATION_DEFAULT".to_string())
        .or_insert(LmsRule::Norm(NormRule::NFC));

    manifest
        .rules
        .entry("TRANSLITERATION_DEFAULT".to_string())
        .or_insert(LmsRule::Trans(TransRule::NONE));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bistun_core::manifest::TraitValue;
    use bistun_core::traits::{MorphType, TraitKey};

    #[test]
    fn test_synthesizer_injects_templatic_rule_and_defaults() {
        let mut manifest = CapabilityManifest::new("ar-SA".to_string());
        manifest
            .traits
            .insert(TraitKey::MorphologyType, TraitValue::MorphType(MorphType::TEMPLATIC));

        synthesize_rules(&mut manifest).expect("LMS-TEST: Synthesis failed");

        assert!(manifest.rules.contains_key("STEMMING_STRATEGY"));
        // Assert B.3 Default Mechanics
        assert_eq!(
            manifest.rules.get("NORMALIZATION_DEFAULT"),
            Some(&LmsRule::Norm(NormRule::NFC))
        );
        assert_eq!(
            manifest.rules.get("TRANSLITERATION_DEFAULT"),
            Some(&LmsRule::Trans(TransRule::NONE))
        );
    }

    #[test]
    fn test_synthesizer_skips_morphology_but_guarantees_defaults() {
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        // Deliberately empty traits

        synthesize_rules(&mut manifest).expect("LMS-TEST: Synthesis failed");

        // Should not have stemming strategy, but MUST have default mechanics
        assert!(!manifest.rules.contains_key("STEMMING_STRATEGY"));
        assert!(manifest.rules.contains_key("NORMALIZATION_DEFAULT"));
    }

    #[test]
    fn test_synthesizer_does_not_overwrite_existing_rules() {
        let mut manifest = CapabilityManifest::new("en-US".to_string());
        // WORM snapshot explicitly requires NFD
        manifest.rules.insert("NORMALIZATION_DEFAULT".to_string(), LmsRule::Norm(NormRule::NFD));

        synthesize_rules(&mut manifest).expect("LMS-TEST: Synthesis failed");

        // Assert B.3 logic respects the existing NFD rule
        assert_eq!(
            manifest.rules.get("NORMALIZATION_DEFAULT"),
            Some(&LmsRule::Norm(NormRule::NFD))
        );
    }
}
