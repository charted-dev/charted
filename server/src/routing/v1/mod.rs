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

use crate::{models::res::err, Server};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::*,
    Router,
};
use cdn::*;
use charted_config::Config;
use features::*;
use heartbeat::*;
use info::*;
use main::*;
use metrics::*;
use openapi::*;
use serde_json::json;

pub mod cdn;
pub mod features;
pub mod heartbeat;
pub mod info;
pub mod main;
pub mod metrics;
pub mod openapi;

pub fn create_router(_server: Server) -> Router<Server> {
    let mut router = Router::new()
        .route("/openapi.json", get(openapi))
        .route("/heartbeat", get(HeartbeatRestController::run))
        .route("/features", get(FeaturesRestController::run))
        .route("/info", get(InfoRestController::run))
        .route("/", get(MainRestController::run))
        .fallback(fallback);

    let config = Config::get();
    if config.metrics.prometheus {
        router = router.clone().route("/metrics", get(metrics));
    }

    if config.cdn.enabled {
        let prefix = match config.cdn.prefix {
            Some(prefix) => {
                if !prefix.starts_with('/') {
                    error!(%prefix, "invalid cdn prefix, must be a valid path! opting to /cdn instead");
                    "/cdn".into()
                } else {
                    prefix
                }
            }

            None => "/cdn".into(),
        };

        let final_path = format!("{}/*file", prefix.trim_end_matches('/'));
        router = router.clone().route(&final_path, get(cdn));
    }

    router
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
