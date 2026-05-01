use bistun::manager::LinguisticManager;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

/// Benchmarks the critical resolution path of the Capability Engine.
///
/// Ref: [007-LMS-OPS], [012-LMS-ENG]
/// Expected Performance: < 1ms per resolution.
fn bench_global_resolution(c: &mut Criterion) {
    // 1. Initial State: Load the full 7,000+ language registry into memory
    let manager = LinguisticManager::default();

    c.bench_function("resolve_global_hebrew", |b| {
        b.iter(|| {
            // 2. Execute: Resolve and Aggregate traits for Hebrew using the correct API method
            // We use black_box to prevent the compiler from optimizing away the call.
            let _ = manager.resolve_capabilities(black_box("he"));
        })
    });

    c.bench_function("resolve_global_traditional_chinese", |b| {
        b.iter(|| {
            let _ = manager.resolve_capabilities(black_box("zh-Hant"));
        })
    });
}

criterion_group!(benches, bench_global_resolution);
criterion_main!(benches);
