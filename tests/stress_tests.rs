// Bistun Linguistic Metadata Service (LMS)
// Ref: [010-LMS-MEM], [007-LMS-OPS]
// Location: `tests/stress_tests.rs`

use bistun::data::repository::SimulatedSnapshotProvider;
use bistun::manager::LinguisticManager;
use std::time::Instant;
use tokio::task::JoinHandle;

/// Executes a massive burst of concurrent resolution requests.
///
/// Time: O(N) tasks | Space: O(N) result vectors
async fn run_burst(count: usize, manager: LinguisticManager) {
    let mut handles = Vec::with_capacity(count);
    let start = Instant::now();

    println!("🔥 Starting stress test: {} simultaneous requests...", count);

    for i in 0..count {
        let m = manager.clone();
        // Alternate between different tags to stress different resolver branches
        let tag = if i % 2 == 0 { "ar-EG-u-nu-latn" } else { "in" };

        let handle: JoinHandle<()> = tokio::spawn(async move {
            let result = m.resolve_capabilities(tag);
            assert!(result.is_ok(), "Request {} failed during stress test", i);
        });
        handles.push(handle);
    }

    // Await all simultaneous tasks
    for handle in handles {
        handle.await.unwrap();
    }

    let duration = start.elapsed();
    let avg = duration.as_micros() as f64 / count as f64;

    println!("✅ Completed {} requests in {:?}", count, duration);
    println!("📊 Average Resolution (including task overhead): {:.2}µs\n", avg);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_high_concurrency_scaling() {
    // [STEP 1]: Setup: Instantiate and hydrate the manager
    let manager = LinguisticManager::new();
    let provider = SimulatedSnapshotProvider::new();

    // [FIX]: Pass the dynamic public key to the security gate
    manager.initialize(&provider, &provider.public_key).await;

    // [STEP 2]: Execute Burst 1 (1,000)
    run_burst(1_000, manager.clone()).await;

    // [STEP 3]: Execute Burst 2 (10_000)
    run_burst(10_000, manager.clone()).await;

    // [STEP 4]: Execute Burst 3 (100_000)
    run_burst(100_000, manager.clone()).await;
}
