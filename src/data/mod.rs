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

//! # Memory Management & Persistence
//! Ref: [010-LMS-MEM]
//!
//! **Why**: This module manages the in-memory cache and state for all linguistic data.
//! **Impact**: If this module fails, the application will panic due to memory corruption or exhaust host memory via duplicate allocations.

pub mod compiler;
pub mod repository;
pub mod store;
pub mod swap;
