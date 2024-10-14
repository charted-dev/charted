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

pub mod heartbeat;
pub mod index;
pub mod info;
pub mod main;
pub mod openapi;

use crate::ServerContext;
use axum::{routing, Router};

pub fn create_router(_: &ServerContext) -> Router<ServerContext> {
    Router::new()
        .route("/indexes/:idOrName", routing::get(index::get_chart_index))
        .route("/heartbeat", routing::get(heartbeat::heartbeat))
        .route("/openapi.json", routing::get(openapi::openapi))
        .route("/info", routing::get(info::info))
        .route("/", routing::get(main::main))
}
