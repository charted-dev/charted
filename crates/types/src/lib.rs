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

#![allow(clippy::too_long_first_doc_paragraph)]
#![feature(decl_macro)]

//! The `charted-types` crate defines types that can be used within the lifecycle
//! of the API server.

mod db;
pub use db::*;

mod newtypes;
pub use newtypes::*;

pub(crate) mod util;

pub mod helm;
pub mod name;
pub mod payloads;
