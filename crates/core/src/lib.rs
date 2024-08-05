// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use charted_config as config;
pub use charted_database as db;
pub use charted_proc_macros as macros;
pub use charted_types as types;

mod distribution;
pub use distribution::*;

mod bitflags;
pub use bitflags::*;

pub mod openapi;
pub mod serde;
pub mod storage;

#[cfg(feature = "testkit")]
pub mod testkit;

/// Snowflake epoch used for ID generation. (March 1st, 2024)
pub const SNOWFLAKE_EPOCH: usize = 1709280000000;

/// Returns the version of the Rust compiler that charted-server
/// was compiled on.
pub const RUSTC_VERSION: &str = env!("CHARTED_RUSTC_VERSION");

/// Returns the Git commit hash from the charted-server repository that
/// this build was built off from.
pub const COMMIT_HASH: &str = env!("CHARTED_COMMIT_HASH");

/// RFC3339-formatted date of when charted-server was last built at.
pub const BUILD_DATE: &str = env!("CHARTED_BUILD_DATE");

/// Returns the current version of `charted-server`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
