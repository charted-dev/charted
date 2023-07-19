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

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::*,
    Router,
};
use cdn::*;
use features::*;
use heartbeat::*;
use info::*;
use main::*;
use serde_json::json;
// use metrics::*;
use crate::{models::res::err, Server};
use openapi::*;

pub mod cdn;
pub mod features;
pub mod heartbeat;
pub mod info;
pub mod main;
pub mod metrics;
pub mod openapi;

pub fn create_router() -> Router<Server> {
    Router::new()
        .route("/", get(main))
        .route("/info", get(info))
        .route("/features", get(features))
        .route("/heartbeat", get(heartbeat))
        .route("/_openapi", get(openapi))
        .route("/cdn/*file", get(cdn))
        .fallback(fallback)
}

async fn fallback(req: Request<Body>) -> impl IntoResponse {
    err(
        StatusCode::NOT_FOUND,
        (
            "HANDLER_NOT_FOUND",
            "Route was not found",
            json!({
                "method": req.method().as_str().to_lowercase(),
                "url": req.uri().path(),
            }),
        )
            .into(),
    )
}
