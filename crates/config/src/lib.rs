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

mod config;
mod database;
mod from_env;
mod logging;
mod merge;
mod metrics;
mod redis;
mod search;
mod secure_setting;
mod server;
mod storage;

pub use config::*;
pub use database::*;
pub use from_env::*;
pub use logging::*;
pub use merge::*;
pub use metrics::*;
pub use redis::*;
pub use search::*;
pub use secure_setting::*;
pub use server::*;
pub use storage::*;

/// Simple macro to implement a configuration struct
/// that is consistent towards the whole project.
///
/// This will:
/// * Add the `Debug`, `Clone`, `serde::Serialize`, `serde::Deserialize`, and `clap::Parser` traits
/// * Implement `Default` and `charted_config::FromEnv`
///
/// ## Example
/// ```no_run
/// # use charted_config::{make_config, var};
/// #
/// make_config! {
///     /// Simple doc comment.
///     MyConfig {
///         /// A doc comment for this property.
///         pub name: String {
///             default: "".into();
///             env: var!("CHARTED_SOME_CONFIG_KEY", "".into())
///         };
///     }
/// }
///
/// /*
/// expanded:
///
/// /// Simple doc comment.
/// #[derive(Debug, Clone, clap::Parser, ::serde::Serialize, ::serde::Deserialize)]
/// pub struct MyConfig {
///     /// A doc comment for this property.
///     pub name: String,
/// }
///
/// impl Default for MyConfig {
///     fn default() -> MyConfig {
///         MyConfig {
///             name: "".into(),
///         }
///     }
/// }
///
/// impl charted_config::FromEnv<MyConfig> for MyConfig {
///     fn from_env() -> MyConfig {
///         MyConfig {
///             name: ::std::env::var("CHARTED_SOME_CONFIG_KEY").unwrap_or("".into())
///         }
///     }
/// }
/// */
/// ```
#[macro_export]
macro_rules! make_config {
    ($(#[$top_level_meta:meta])* $name:ident {
        $(
            $(#[$meta:meta])*
            $vis:vis $key:ident: $ty:ty {
                default: $default:expr;
                env_value: $env:expr;
            };
        )* $(;)?
    }) => {
        $(#[$top_level_meta])*
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::clap::Parser)]
        pub struct $name {
            $(
                $(#[$meta])*
                $vis $key: $ty,
            )*
        }

        impl Default for $name {
            fn default() -> $name {
                $name {
                    $(
                        $key: $default,
                    )*
                }
            }
        }

        impl $crate::FromEnv<$name> for $name {
            fn from_env() -> $name {
                $name {
                    $(
                        $key: $env,
                    )*
                }
            }
        }

        impl $crate::Merge for $name {
            fn merge(&mut self, other: Self) {
                $(
                    $crate::Merge::merge(&mut self.$key, other.$key);
                )*
            }
        }
    };
}

/// Simple macro to export an environment variable for easy use. This is useful
/// to not repeat yourself when fetching and possibly validating an
/// environment variable.
///
/// ## Example
/// ```no_run
/// let p = var!("CHARTED_BARK", to: bool, or_else: false);
/// // ::std::env::var("CHARTED_BARK").map(|p| p.parse::<bool>().expect("Unable to resolve env var [CHARTED_BARK] to a [bool] value")).unwrap_or(false)
///
/// let p2 = var!("CHARTED_BARK", or_else: "thing".into());
/// // ::std::env::var("CHARTED_BARK").unwrap_or("thing".into());
/// ```
#[macro_export]
macro_rules! var {
    ($key:literal, to: $ty:ty, or_else: $else_:expr) => {
        var!($key, mapper: |p| {
            p.parse::<$ty>().expect(concat!(
                "Unable to resolve env var [",
                $key,
                "] to a [",
                stringify!($ty),
                "] value"
            ))
        })
        .unwrap_or($else_)
    };

    ($key:literal, to: $ty:ty, is_optional: true) => {
        var!($key, mapper: |p| p.parse::<$ty>().ok()).unwrap_or(None)
    };

    ($key:literal, to: $ty:ty) => {
        var!($key, mapper: |p| {
            p.parse::<$ty>().expect(concat!(
                "Unable to resolve env var [",
                $key,
                "] to a [",
                stringify!($ty),
                "] value"
            ))
        })
        .unwrap()
    };

    ($key:literal, {
        or_else: $else_:expr;
        mapper: $mapper:expr;
    }) => {
        var!($key, mapper: $mapper).unwrap_or($else_)
    };

    ($key:literal, mapper: $expr:expr) => {
        var!($key).map($expr)
    };

    ($key:literal, use_default: true) => {
        var!($key, or_else_do: |_| Default::default())
    };

    ($key:literal, or_else_do: $expr:expr) => {
        var!($key).unwrap_or_else($expr)
    };

    ($key:literal, or_else: $else_:expr) => {
        var!($key).unwrap_or($else_)
    };

    ($key:literal, is_optional: true) => {
        var!($key).ok()
    };

    ($key:literal) => {
        ::std::env::var($key)
    };
}
