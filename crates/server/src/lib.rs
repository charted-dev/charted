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
//
//! # 🐻‍❄️📦 `charted-server`
//! This crate is the official implementation of the [charted REST Specification].
//!
//! [charted REST Specification]: https://charts.noelware.org/docs/server/latest/api

#![feature(let_chains, impl_trait_in_bindings)]

#[macro_use]
extern crate tracing;

mod macros;

mod env;
pub use env::Env;

mod ext;
pub use ext::OwnerExt;

pub mod extract;
pub mod feature;
pub mod middleware;
pub mod openapi;
pub mod ops;
pub mod pagination;
pub mod routing;
pub mod util;

// Private module to aid in macro development
#[doc(hidden)]
pub mod __macro_support {
    pub use axum;
    pub use paste::paste;
    pub use utoipa;
}
