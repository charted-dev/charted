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

mod elasticsearch;
mod meilisearch;
mod postgres;
mod redis;

// macro_rules! gen_metricset_priv {
//     ($(#[$toplevel_meta:meta])* $name:ident - $env:literal {
//         wildcard: $w:literal;
//         $($(#[$meta:meta])* $value:ident [key: $key:literal];)*
//     }) => {
//         use crate::var;

//         #[derive(Debug, Clone, Copy, PartialEq, Eq, ::clap::Parser, ::serde::Serialize, ::serde::Deserialize)]
//         $(#[$toplevel_meta])*
//         pub enum $name {
//             $(
//                 $(#[$meta])*
//                 $value,
//             )*

//             /// Represents the wildcard in this enum set.
//             Wildcard,
//         }

//         impl $crate::FromEnv<$name> for $name {
//             fn from_env() -> $name {
//                 let values = var!($env, {
//                     or_else: vec![];
//                     mapper: |val| val.split(',').collect::<Vec<_>>();
//                 });

//                 for val in values.iter() {
//                     match val {
//                         $($key => $name::$value,)*
//                         _ => panic!("Unable to find entry [{val}] in set [{}]", stringify!($name))
//                     }
//                 }

//                 panic!("")
//             }
//         }

//         impl $name {
//             /// Wildcard value for this metricset.
//             const WILDCARD: $name = $name::Wildcard;

//             /// Returns all of the keys that can be enabled
//             /// for the metrics pipeline for this metricset.
//             pub fn values() -> &'static [&'static str] {
//                 &[$w, $($key,)*]
//             }

//             /// Checks if any of the variants provided is a wildcard
//             /// type or not.
//             pub fn is_wildcard(&self, variants: &[$name]) -> bool {
//                 variants.iter().any(|val| *val == $name::WILDCARD)
//             }

//             /// Checks if a slice of variants has the selected metric set.
//             pub fn enabled(&self, set: &[$name], selected: $name) -> bool {
//                 self.is_wildcard(set) || set.iter().any(|val| *val == selected)
//             }
//         }

//         impl ToString for $name {
//             fn to_string(&self) -> String {
//                 match self {
//                     $(
//                         $name::$value => $key.to_string(),
//                     )*
//                 }
//             }
//         }

//         mod private {
//             use super::*;

//             pub trait Sealed {}
//             impl Sealed for $name {}
//         }
//     };
// }

// pub(crate) use gen_metricset_priv as gen_metricset;
