// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026 Francis Xavier Wazeter IV

//! # Performance Verification Benchmark
//! Crate: bistun-lms
//! Ref: [007-LMS-OPS]
//! Location: `crates/bistun-lms/benches/performance_verification.rs`
//!
//! **Why**: This benchmark provides the scientific proof that the resolution
//! pipeline meets the < 1ms performance budget required for production.
//! **Impact**: If this budget is breached, the entire capability engine is considered
//! non-viable for synchronous UI layout and search indexing tasks.

use bistun_lms::LinguisticManager;
use bistun_lms::data::repository::SimulatedSnapshotProvider;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use tokio::runtime::Runtime;

fn bench_resolution_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("Capability Engine");

    // [STEP 1]: Setup Async Runtime for Initialization
    let rt = Runtime::new().expect("LMS-TEST: Failed to create async runtime");
    let manager = LinguisticManager::new();
    let provider = SimulatedSnapshotProvider::new();

    // [STEP 2]: Hydrate the Engine with Golden Data
    rt.block_on(async {
        manager.initialize(&provider, &provider.public_key).await;
    });

    // Test a highly complex multi-domain resolution (Arabic + Overrides)
    let target_locale = "ar-EG-u-nu-latn";

    // [STEP 3]: Execute the Hot-Path Benchmark
    group.bench_function("resolve_capabilities (warm cache)", |b| {
        b.iter(|| {
            // This is the critical < 1ms path
            // ArcSwap allows this to execute with zero locks!
            manager
                .resolve_capabilities(black_box(target_locale))
                .expect("LMS-TEST: Benchmark resolution failed");
        });
    });

    group.finish();
}

criterion_group!(benches, bench_resolution_pipeline);
criterion_main!(benches);
