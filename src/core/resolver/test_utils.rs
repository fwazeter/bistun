// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV

#![cfg(test)] // Strictly limits this file to the testing compiler pass

use super::{IResolver, orchestrator::LocaleEntry};
use crate::data::store::LocaleProfile;
use crate::data::swap::IRegistryState;
use crate::models::traits::{Direction, MorphType, NormType, SegType, TransType};
use mockall::mock;
use std::sync::Arc;

// 1. Unified Mock for the Registry Memory Pool
mock! {
    pub RegistryState {}
    impl IRegistryState for RegistryState {
        fn get_profile(&self, id: &str) -> Option<Arc<LocaleProfile>>;
        fn resolve_alias(&self, tag: &str) -> Option<String>;
        fn get_version(&self) -> String;
        fn get_base_resource_uri(&self) -> String; // [NEW]
    }
}

// 2. Unified Mock for the Chain of Responsibility Delegation
mock! {
    pub NextResolver {}
    impl IResolver for NextResolver {
        fn resolve(&self, tag: &str, state: &dyn IRegistryState, path: &mut Vec<String>) -> Option<LocaleEntry>;
        fn set_next(&mut self, next: Box<dyn IResolver>);
    }
}

// 3. Unified Flyweight Stub Generator
pub fn create_stub(id: &str) -> Arc<LocaleProfile> {
    Arc::new(LocaleProfile {
        id: id.to_string(),
        morph: MorphType::FUSIONAL,
        base_seg: SegType::SPACE,
        alt_seg: None,
        direction: Direction::LTR,
        has_bidi: false,
        requires_shaping: false,
        plurals: vec![],
        unicode_blocks: vec![],
        normalization: NormType::NFC,
        transliteration: TransType::NONE,
        required_resource: None,
    })
}
