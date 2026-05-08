//! # Performance Verification Benchmark
//! Ref: [007-LMS-OPS]
//!
//! **Why**: This benchmark provides the scientific proof that the resolution
//! pipeline meets the <1ms performance budget required for production.

use bistun_lms::LinguisticManager;
use bistun_lms::data::repository::SimulatedSnapshotProvider;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use tokio::runtime::Runtime;

fn bench_resolution_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("Capability Engine");

    // [STEP 1]: Setup Async Runtime for Initialization
    let rt = Runtime::new().unwrap();
    let manager = LinguisticManager::new();
    let provider = SimulatedSnapshotProvider::new();

    // [STEP 2]: Hydrate the Engine with Golden Data
    rt.block_on(async {
        manager.initialize(&provider, &provider.public_key).await;
    });

    let target_locale = "ar-EG-u-nu-latn";

    // [STEP 3]: Execute the Hot-Path Benchmark
    group.bench_function("resolve_capabilities (warm cache)", |b| {
        b.iter(|| {
            // This is the critical < 1ms path
            manager.resolve_capabilities(black_box(target_locale)).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, bench_resolution_pipeline);
criterion_main!(benches);
