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

//! the `charted-server` crate implements types and Axum extractors that the `charted` package
//! uses to implement the rest of the API and `charted-helm-plugin` package to use the types
//! instead of including the `charted` package.

pub use charted_proc_macros::controller;

pub mod extract;
pub mod middleware;
pub mod multipart;
pub mod pagination;

mod models;
pub use models::*;

/// Represents the REST version that an API controller is supported on.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr,
)]
#[repr(u8)]
pub enum APIVersion {
    /// v1
    #[default]
    V1 = 1,
}

impl std::fmt::Display for APIVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            APIVersion::V1 => "v1",
        })
    }
}
