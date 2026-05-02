// Bistun Linguistic Metadata Service (LMS)
// Copyright (C) 2026  Francis Xavier Wazeter IV
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

//! # Memory Management & Persistence Domain
//! Ref: [010-LMS-MEM]
//! Location: `src/data/mod.rs`
//!
//! **Why**: This module manages the in-memory cache, state transitions, and WORM hydration for all linguistic data.
//! **Impact**: If this module fails, the application will either serve stale data, panic due to lock poisoning, or exhaust host memory via redundant allocations.
//!
//! ### Glossary
//! * **Atomic Hot-Swap**: The process of replacing the entire active registry in memory without blocking active requests.
//! * **WORM (Write-Once, Read-Many)**: A persistence strategy where snapshots are treated as immutable once compiled.

/// Offline build tools and ingestion logic.
pub mod compiler;

/// Retrieval and transport of WORM snapshot payloads.
pub mod providers;

/// Orchestration of registry hydration and compilation.
pub mod repository;

/// In-memory Flyweight pools for locale definitions.
pub mod store;

/// Thread-safe concurrency management and hot-swap logic.
pub mod swap;
