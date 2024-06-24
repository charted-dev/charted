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
use std::{borrow::Cow, collections::HashMap, env::VarError, future::Future, pin::Pin, str::FromStr, sync::OnceLock};

pub mod serde;

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
///
/// **Deprecated** (since 0.1.0-beta): use [`azalia::TRUTHY_REGEX`] instead.
///
/// [`azalia::TRUTHY_REGEX`]: https://crates.noelware.cloud/~/azalia/*/docs#static-TRUTHY_REGEX
#[deprecated(
    since = "0.1.0-beta",
    note = "use `azalia::TRUTHY_REGEX` instead, removal in 0.2.0-beta"
)]
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
            Err(e) => Err(eyre!("unable to represent `${env}` as valid: {}", onfail(e))),
        },

        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!(
            "unable to represent environment variable `${env}` as valid unicode",
        )),
    }
}

pub fn env_string(key: impl Into<String>, default: impl Into<String>) -> eyre::Result<String> {
    let env = key.into();
    match noelware_config::env!(&env) {
        Ok(val) => Ok(val),
        Err(VarError::NotPresent) => Ok(default.into()),
        Err(VarError::NotUnicode(_)) => Err(eyre!(
            "unable to represent environment variable `${env}` as valid unicode"
        )),
    }
}

pub fn env_map<S: Into<String>>(key: S) -> eyre::Result<HashMap<String, String>> {
    let env = key.into();
    match noelware_config::env!(&env) {
        Ok(val) => {
            let mut iter = val.split(';');
            let mut next = iter.next();
            let mut map = azalia::hashmap!(String, String);

            while let Some(item) = next {
                let Some((key, value)) = item.split_once('=') else {
                    next = iter.next();
                    continue;
                };
                map.insert(key.to_owned(), value.to_owned());
                next = iter.next();
            }

            Ok(map)
        }

        Err(VarError::NotPresent) => Ok(azalia::hashmap!()),
        Err(_) => Err(eyre!(
            "unable to represent environment variable `${env}` as valid unicode"
        )),
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

#[cfg(test)]
mod tests {
    use super::*;
    use noelware_config::expand_with;

    #[test]
    fn test_env() {
        expand_with("HELLO", "1", || {
            let res = env("HELLO", i32::MAX, |e| Cow::Owned(format!("{e}")));
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), 1);
        });

        expand_with("WORLD", "invalid", || {
            assert!(env("WORLD", i32::MAX, |_| Cow::Borrowed("should happen")).is_err());
        });
    }

    #[test]
    fn test_env_string() {
        expand_with("HELLO", "blah", || {
            assert!(env_string("HELLO", "...").is_ok());
        });

        assert_eq!(env_string("THIS_SHOULD_NEVER_EXIST_WAAAAAAAA", "...").unwrap(), "...");
    }

    #[test]
    fn test_env_map() {
        expand_with(
            "CHARTED_DATABASE_OPTIONS",
            "hello=world;wah=true;weee=1=2;wahhh=1=2=3=4",
            || {
                let res = env_map("CHARTED_DATABASE_OPTIONS");
                assert!(res.is_ok());
                assert_eq!(
                    res.unwrap(),
                    azalia::hashmap! {
                        "hello" => "world",
                        "wah" => "true",
                        "weee" => "1=2",
                        "wahhh" => "1=2=3=4"
                    }
                );
            },
        );

        expand_with("HELLO", "", || {
            let res = env_map("HELLO");
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), azalia::hashmap!());
        });
    }
}
