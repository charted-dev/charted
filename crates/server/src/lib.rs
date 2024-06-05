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

//! The `charted_server` crate implements the types, implementation, and Axum extractors of charted's
//! REST API Specification for transmitting Helm charts safely and securely.

pub use charted_proc_macros::controller;

pub mod extract;
pub mod middleware;
pub mod multipart;
pub mod pagination;

mod models;
pub use models::*;

mod state;
pub use state::*;

mod version;
pub use version::*;
