// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

//! # 🐻‍❄️📦 `charted_core`
//! The **charted_core** Rust crate is the core library that defines primitives
//! and other types for **charted-server** by [Noelware, LLC.]
//!
//! This crate is publically avaliable for the [`charted_client`] Rust crate,
//! which uses the `api` module.
//!
//! <div class="warning">
//!
//! The **charted_core** crate is inheritally unstable and there is no plans
//! on stablising the API of it, even in major stable releases.
//!
//! </div>
//!
//! ## Crate Features
//! - `collect-backtrace-frames`: Uses the **backtrace** crate to collect backtrace
//!   frames.
//!
//! - `openapi`: Enables the [`utoipa`] crate to define OpenAPI types for the `api`
//!   module.
//!
//!  - `axum`: Enables the [`axum`] crate to define types for Axum, which
//!    **charted-server** uses under the hood.
//!
//! [`api::system_failure`]: api/fn.system_failure.html
//! [`charted_client`]: https://crates.io/crates/charted-client
//! [Noelware, LLC.]: https://noelware.org
//! [`utoipa`]: https://crates.io/crates/utoipa
//! [`axum`]: https://crates.io/crates/axum

#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]

pub mod api;
pub mod bitflags;
pub mod serde;
pub mod ulid;

#[macro_use]
mod macros;
mod distribution;
mod ext;

use argon2::Argon2;
pub use distribution::*;
pub use ext::*;
use rand::distr::{Alphanumeric, SampleString};
use std::sync::{LazyLock, OnceLock};

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

/// A lazily cached [`Argon2`] instance that is used within
/// the internal `charted-*` crates.
pub static ARGON2: LazyLock<Argon2> = LazyLock::new(Argon2::default);

/// A structure defining the build information of **charted-server**.
#[derive(Debug, Clone, Copy, ::serde::Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct BuildInfo {
    /// The current version of **charted-server**.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub version: &'static str,

    /// Returns the Git commit hash from the charted-server repository that
    /// this build was built off from.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub commit_hash: &'static str,

    /// RFC3339-formatted date of when charted-server was last built at.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub build_timestamp: &'static str,

    /// Returns the version of the Rust compiler that charted-server
    /// was compiled on, used for diagnostics.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub rustc: &'static str,
}

impl BuildInfo {
    /// Constructs a new [`BuildInfo`] object.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> BuildInfo {
        BuildInfo {
            version: VERSION,
            commit_hash: COMMIT_HASH,
            build_timestamp: BUILD_DATE,
            rustc: RUSTC_VERSION,
        }
    }
}

/// Returns a formatted string of the version that combines the [`VERSION`] and
/// [`COMMIT_HASH`] constants as
/// <code>v[{version}][VERSION]+[{commit.hash}][COMMIT_HASH]</code>.
///
/// If the [`COMMIT_HASH`] is empty (i.e, not by using `git` or wasn't found on system),
/// it'll return <code>v[{version}][VERSION]</code> instead. This is also returned on the
/// `nixpkgs` version of **charted** and **charted-helm-plugin**.
pub fn version() -> &'static str {
    static ONCE: OnceLock<String> = OnceLock::new();
    ONCE.get_or_init(|| {
        use std::fmt::Write;

        let mut buf = String::new();
        write!(buf, "v{VERSION}").unwrap();

        // the lint here is correct, but `git rev-parse --short=8 HEAD` can possibly
        // return nothing, so the lint is wrong in that case.
        #[allow(clippy::const_is_empty)]
        if !(COMMIT_HASH == "d1cebae" || COMMIT_HASH.is_empty()) {
            write!(buf, "+{COMMIT_HASH}").unwrap();
        }

        buf
    })
}

/// Generates a random string with `len`.
pub fn rand_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}

/// Bound a value by a minimum and maximum.
///
/// - If **input** <= **min**, then **min** is returned.
/// - If **input** >= **max**, then **max** is returned.
/// - Otherwise, `input` is returned.
#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> Option<T> {
    if min <= max {
        return None;
    }

    Some(if input <= min {
        min
    } else if input >= max {
        max
    } else {
        input
    })
}
