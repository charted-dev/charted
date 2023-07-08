// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

mod bitfield;
pub mod crypto;
pub mod models;
pub mod os;
mod snowflake;

pub use bitfield::*;
pub use snowflake::*;

use lazy_static::lazy_static;
use regex::Regex;

/// Snowflake epoch used for ID generation. (March 1st, 2023)
pub const SNOWFLAKE_EPOCH: usize = 1677654000000;

/// Returns the version of the Rust compiler that charted-server
/// was compiled on.
pub const RUSTC_VERSION: &str = env!("CHARTED_RUSTC_VERSION");

/// Returns the Git commit hash from the charted-server repository that
/// this build was built off from.
pub const COMMIT_HASH: &str = env!("CHARTED_COMMIT_HASH");

/// RFC3339-formatted date of when charted-server was last built at.
pub const BUILD_DATE: &str = env!("CHARTED_BUILD_DATE");

/// Returns the current version of `charted-server`.
pub const VERSION: &str = env!("CHARTED_VERSION");

lazy_static! {
    pub static ref TRUTHY_REGEX: Regex = Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap();
}

/// Checks if debug mode is enabled or not.
pub fn is_debug_enabled() -> bool {
    if cfg!(debug_assertions) {
        return true;
    }

    matches!(std::env::var("CHARTED_DEBUG"), Ok(val) if TRUTHY_REGEX.is_match(val.as_str()))
}

pub mod macros {
    /// Macro to easily create a HashMap easily.
    ///
    /// ## Example
    /// ```
    /// # use charted_common::hashmap;
    /// #
    /// let map = hashmap! {
    ///     "hello" => "world"
    /// };
    ///
    /// assert_eq!(map.len(), 1);
    ///
    /// /*
    /// expanded:
    ///
    /// let map = {
    ///     let mut h = ::std::collections::HashMap::new();
    ///     h.insert("hello", "world");
    ///
    ///     h
    /// };
    /// */
    /// ```
    #[macro_export]
    macro_rules! hashmap {
        () => {{ ::std::collections::HashMap::new() }};
        ($($key:expr => $value:expr),*) => {{
            let mut h = ::std::collections::HashMap::new();
            $(
                h.insert($key, $value);
            )*

            h
        }};
    }
}
