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

#![feature(once_cell_try)]

pub use charted_proc_macros as macros;

mod distribution;
pub use distribution::*;

pub mod api;
pub mod bitflags;
pub mod openapi;
pub mod serde;

#[cfg(feature = "testkit")]
pub mod testkit;

#[macro_use]
#[path = "macros.rs"]
mod macros_;

use argon2::Argon2;
use std::{
    fmt,
    sync::{LazyLock, OnceLock},
};

/// Type-alias that represents a boxed future.
pub type BoxedFuture<'a, Output> =
    ::core::pin::Pin<::std::boxed::Box<dyn ::core::future::Future<Output = Output> + Send + 'a>>;

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

#[allow(clippy::incompatible_msrv)]
pub static ARGON2: LazyLock<Argon2> = LazyLock::new(Argon2::default);

/// Returns a formtted version string of `v0.0.0+{commit hash}` if [`COMMIT_HASH`] is not empty
/// or `d1cebae`. Otherwise, `v0.0.0` is returned instead.
pub fn version() -> &'static str {
    static ONCE: OnceLock<String> = OnceLock::new();
    ONCE.get_or_try_init(|| -> Result<String, fmt::Error> {
        use std::fmt::Write;

        let mut buf = String::new();
        write!(buf, "v{VERSION}")?;

        #[allow(clippy::const_is_empty)] // lint is right but sometimes `git rev-parse --short=8 HEAD` will return empty
        if !(COMMIT_HASH == "d1cebae" || COMMIT_HASH.is_empty()) {
            write!(buf, "+{COMMIT_HASH}")?;
        }

        Ok(buf)
    })
    .unwrap_or_else(|e| panic!("internal error: {e}"))
}
