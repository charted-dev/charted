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

pub mod caching;
pub mod cdn;
pub mod config;
pub mod database;
mod from_env;
pub mod logging;
pub mod metrics;
pub mod redis;
pub mod search;
pub mod secure_setting;
pub mod server;
pub mod sessions;
pub mod storage;

pub use config::*;
pub use database::*;
pub use from_env::*;
pub use logging::*;
pub use metrics::*;
pub use redis::*;
pub use secure_setting::*;
pub use server::*;
pub use storage::*;

/// Generic Rust functional macro to implement a common-style configuration struct
/// that can be passed onto a [`Config`][crate::Config] struct (which also uses this macro)
/// to be serialized from.
///
/// ## Goals
/// * Implement common Rust traits (`Debug`, `Clone`) and serde traits (`Serialize`, `Deserialize`).
/// * Implement `Default` and [`FromEnv`][crate::FromEnv] automatically.
///
/// ## Example
/// ```
/// # use charted_config::{make_config, var};
/// #
/// make_config! {
///     /// Simple doc comment
///     SomeConfig {
///         /// A doc comment for this property.
///         pub name: String {
///             // Used for the `Default` trait implementation
///             default: "".into();
///
///             // Used for the `FromEnv` trait implementation
///             env_value: var!("CHARTED_SOMETHING", or_else: "".into());
///         };
///     }
/// }
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
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
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

        impl $crate::FromEnv for $name {
            type Output = $name;

            fn from_env() -> Self::Output {
                $name {
                    $(
                        $key: $env,
                    )*
                }
            }
        }
    };
}

/// Generic Rust functional macro to help with locating an environment variable
/// in the host machine. This macro is used to help with creating [configuration objects][crate::make_config]
/// with the `make_config!` macro.
///
/// ## Variants
/// ### `var!($key: literal)`
/// This will just expand `$key` into a Result<[`String`][alloc::string::String], [`VarError`][std::env::VarError]> variant.
///
/// ```
/// # use charted_config::var;
/// #
/// let result = var!("SOME_ENV_VARIABLE");
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE");
/// #
/// # assert!(result.is_err());
/// ```
///
/// ### `var!($key: literal, is_optional: true)`
/// Expands the `$key` into a Option type if a [`VarError`][std::env::VarError] occurs.
///
/// ```
/// # use charted_config::var;
/// #
/// let result = var!("SOME_ENV_VARIABLE", is_optional: true);
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").ok();
/// #
/// # assert!(result.is_none());
/// ```
///
/// ### `var!($key: literal, or_else: $else: expr)`
/// Expands `$key` into a String, but if a [`VarError`][std::env::VarError] occurs, then a provided `$else`
/// is used as the default.
///
/// ```
/// # use charted_config::var;
/// #
/// let result = var!("SOME_ENV_VARIABLE", or_else: "".into());
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").unwrap_or("".into());
/// #
/// # assert!(result.is_empty());
/// ```
///
/// ### `var!($key: literal, or_else_do: $else: expr)`
/// Same as [`var!($key: literal, or_else: $else: expr)`][crate::var], but uses `.unwrap_or_else` to
/// accept a [`Fn`][std::ops::Fn].
///
/// ```
/// # use charted_config::var;
/// #
/// let result = var!("SOME_ENV_VARIABLE", or_else_do: |_| Default::default());
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").unwrap_or_else(|_| Default::default());
/// #
/// # assert!(result.is_empty());
/// ```
///
/// ### `var!($key: literal, use_default: true)`
/// Same as [`var!($key: literal, or_else_do: $else: expr)`][crate::var], but will use the
/// [Default][core::default::Default] implementation, if it can be resolved.
///
/// ```
/// # use charted_config::var;
/// #
/// let result = var!("SOME_ENV_VARIABLE", use_default: true);
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").unwrap_or_else(|_| Default::default());
/// #
/// # assert!(result.is_empty());
/// ```
///
/// ### `var!($key: literal, mapper: $mapper: expr)`
/// Uses the [`.map`][result-map] method with an accepted `mapper` to map to a different type.
///
/// ```
/// # use charted_config::var;
/// #
/// let result = var!("SOME_ENV_VARIABLE", mapper: |val| &val == "true");
///
/// /*
/// expanded:
/// ::std::env::var("SOME_ENV_VARIABLE").map(|val| &val == "true");
/// */
/// #
/// # assert!(result.is_err());
/// ```
///
/// [result-map]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.map
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
