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

use eyre::eyre;
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use std::{borrow::Cow, env::VarError, future::Future, pin::Pin, str::FromStr, sync::OnceLock};

pub mod serde;
pub mod validation;

mod bitfield;
pub use bitfield::*;

mod snowflake;
pub use snowflake::*;

mod macros;

/// Represents a type-alias that wraps [`chrono::DateTime`]<[`chrono::Local`]> for database objects'
/// `created_at` and `updated_at` timestamps.
pub type DateTime = chrono::DateTime<chrono::Local>;

/// Type-alias to represent a boxed future.
pub type BoxedFuture<'a, Output> = Pin<Box<dyn Future<Output = Output> + Send + 'a>>;

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

/// Generic [`Regex`] implementation for possible truthy boolean values.
pub static TRUTHY_REGEX: Lazy<regex::Regex> = lazy!(regex!(r#"^(yes|true|si*|e|enable|1)$"#));

/// Returns a formatted version of `v{version}+{commit}` or `v{version}` if no commit hash
/// was found.
pub fn version() -> &'static str {
    static ONCE: OnceLock<String> = OnceLock::new();
    ONCE.get_or_init(|| {
        use std::fmt::Write;

        let mut buf = String::new();
        write!(buf, "{}", crate::VERSION).unwrap();

        if crate::COMMIT_HASH != "d1cebae" {
            write!(buf, "+{}", crate::COMMIT_HASH).unwrap();
        }

        buf
    })
}

pub fn env<U: FromStr, S: Into<String>, F>(key: S, default: U, onfail: F) -> eyre::Result<U>
where
    F: FnOnce(<U as FromStr>::Err) -> Cow<'static, str>,
{
    let env = key.into();
    match noelware_config::env!(&env) {
        Ok(val) => match val.parse::<U>() {
            Ok(val) => Ok(val),
            Err(e) => Err(eyre!("failed to parse for env `{}`: {}", env, onfail(e))),
        },

        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!("received invalid unicode data for `{}` env", env)),
    }
}

pub fn env_string<S: Into<String>>(key: S, default: String) -> eyre::Result<String> {
    let env = key.into();
    match noelware_config::env!(&env) {
        Ok(val) => Ok(val),
        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!("received invalid unicode for `{env}` env")),
    }
}

/// Returns a randomized alphanumeric string with a specified length.
pub fn rand_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

/// Returns the target architecture that this crate was built off from. charted-server only supports running
/// on x86_64 and ARMv8 chips.
pub fn architecture() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "amd64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "unknown"
    }
}

/// Returns a machine-readable OS name. This will return `unknown` if this crate was built off an operating system
/// that isn't supported by charted-server.
pub fn os() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(windows) {
        "windows"
    } else {
        "unknown"
    }
}
