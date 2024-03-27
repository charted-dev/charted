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

mod stats;

use super::EntrypointResponse;
use crate::Instance;
use axum::{http::StatusCode, routing, Router};
use charted_server::{ok, ApiResponse};

pub fn create_router() -> Router<Instance> {
    Router::new()
        .route("/", routing::get(main))
        .route("/stats", routing::get(stats::stats))
}

async fn main() -> ApiResponse<EntrypointResponse> {
    ok(
        StatusCode::OK,
        EntrypointResponse {
            message: String::from("Hello to the Admin API!"),
            docs: String::from("/admin is not documented and probably never will be"),
        },
    )
}
