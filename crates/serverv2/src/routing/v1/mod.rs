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
//! Implementation of version **1** of the [charted REST Specification].
//!
//! [charted REST Specification]: https://charts.noelware.org/docs/server/latest/api/v1

pub mod features;
pub mod healthz;
pub mod indexes;
pub mod main;
pub mod openapi;
pub mod organization;
pub mod repository;
pub mod user;

use crate::Env;
use axum::{Router, routing};

pub fn create_router(_: &Env) -> Router<Env> {
    Router::new().route("/", routing::get(main::main))
}
