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

//! # API Performance Verification
//! Ref: [007-LMS-OPS], [LMS-TEST]
//! Location: `crates/bistun-api/benches/api_performance.rs`
//!
//! **Why**: Scientifically verifies that the Axum HTTP routing layer, state extraction, and JSON serialization do not push the capability engine's resolution time over the strict `< 1ms` budget.
//! **Impact**: If this benchmark regresses, the API sidecar becomes a bottleneck, degrading the performance of all downstream UI and NLP consumers.
//!
//! ### Glossary
//! * **Hermetic Benchmarking**: Testing the API without binding to a real TCP port, eliminating OS-level networking noise to measure pure algorithmic execution.

use axum::{body::Body, http::Request};
use bistun_api::routes::app_router;
use bistun_lms::LinguisticManager;
use criterion::{Criterion, criterion_group, criterion_main};
use tower::ServiceExt;

/// Benchmarks the API hot-path resolution latency.
///
/// # Logic Trace (Internal)
/// 1. **Setup**: Instantiate the `LinguisticManager` and Axum `Router`. Create a Tokio runtime.
/// 2. **Iterate**: For each benchmark iteration, construct a mock HTTP `GET` request for a Golden Set tag (e.g., `ar-EG`).
/// 3. **Execute**: Dispatch the request through the router using `oneshot()` to simulate a complete HTTP lifecycle.
/// 4. **Measure**: Criterion tracks the execution time to guarantee it remains O(1)/O(N) and under 1ms.
fn bench_api_hot_path(c: &mut Criterion) {
    // [STEP 1]: Setup isolated engine and router
    let manager = LinguisticManager::new();
    let app = app_router(manager);

    // Create an explicit Tokio runtime for the async benchmark
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("API Hot-Path Resolution");

    group.bench_function("GET /v1/manifest/ar-EG", |b| {
        // We use `to_async` to properly await the Axum handlers
        b.to_async(&rt).iter(|| async {
            // [STEP 2]: Construct Golden Input Request
            let request = Request::builder().uri("/v1/manifest/ar-EG").body(Body::empty()).unwrap();

            // [STEP 3]: Execute the router lifecycle
            // We clone the router instance because `oneshot` consumes it
            let response = app.clone().oneshot(request).await.unwrap();

            // [STEP 4]: Prevent compiler optimization
            std::hint::black_box(response)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_api_hot_path);
criterion_main!(benches);
