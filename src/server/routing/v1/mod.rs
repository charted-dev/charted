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

pub mod admin;
pub mod apikey;
pub mod cdn;
pub mod features;
pub mod heartbeat;
pub mod indexes;
pub mod info;
pub mod main;
pub mod metrics;
pub mod openapi;
pub mod organization;
pub mod repository;
pub mod user;

use crate::{openapi::generate_response_schema, Instance};
use axum::{extract::Request, http::StatusCode, response::IntoResponse, routing, Router};
use charted_common::VERSION;
use charted_server::{err, ErrorCode};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

/// Generic entrypoint message for any API routes like `/users`.
#[derive(Serialize, ToSchema)]
pub struct EntrypointResponse {
    /// A cute message to greet you with
    pub message: String,

    /// URL to the documentation to where you can explore more routes for
    /// this specific API.
    pub docs: String,
}

impl EntrypointResponse {
    pub fn new(entity: impl Into<String>) -> EntrypointResponse {
        let entity = entity.into();
        EntrypointResponse {
            message: format!("Welcome to the {entity} API!"),
            docs: format!(
                "https://charts.noelware.org/docs/server/{VERSION}/api/reference/{}",
                entity.to_lowercase().replace(' ', "")
            ),
        }
    }
}

generate_response_schema!(EntrypointResponse);

pub fn create_router(instance: &Instance) -> Router<Instance> {
    let mut router = Router::new()
        .nest("/organizations", organization::create_router())
        .nest("/repositories", repository::create_router())
        .nest("/apikeys", apikey::create_router())
        .nest("/admin", admin::create_router())
        .route("/openapi.json", routing::get(openapi::json))
        .route("/openapi.yaml", routing::get(openapi::yaml))
        .route("/heartbeat", routing::get(heartbeat::HeartbeatRestController::run))
        .route("/features", routing::get(features::FeaturesRestController::run))
        .route(
            "/index/:idOrName",
            routing::get(indexes::GetChartIndexRestController::run),
        )
        .nest("/users", user::create_router())
        .route("/info", routing::get(info::InfoRestController::run))
        .route("/", routing::get(main::MainRestController::run))
        .fallback(fallback);

    if instance.config.metrics.prometheus {
        router = router.clone().route("/metrics", routing::get(metrics::metrics));
    }

    if instance.config.cdn.enabled {
        let prefix = match instance.config.cdn.prefix {
            Some(ref prefix) => {
                if !prefix.starts_with('/') {
                    error!(%prefix, "invalid cdn prefix, must be a valid path! opting to /cdn instead");
                    "/cdn".into()
                } else {
                    prefix.clone()
                }
            }

            None => "/cdn".into(),
        };

        let final_path = format!("{}/*file", prefix.trim_end_matches('/'));
        router = router.clone().route(&final_path, routing::get(cdn::cdn));
    }

    router
}

async fn fallback(req: Request) -> impl IntoResponse {
    err(
        StatusCode::NOT_FOUND,
        (
            ErrorCode::HandlerNotFound,
            "route was not found",
            json!({"method":req.method().as_str(),"url":req.uri().path()}),
        ),
    )
}
