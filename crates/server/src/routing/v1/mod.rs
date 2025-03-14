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

mod admin;
pub mod features;
pub mod healthz;
pub mod index;
pub mod main;
pub mod openapi;
pub mod organization;
pub mod repository;
pub mod user;

use crate::{Context, openapi::ApiResponse};
use axum::{Extension, Router, response::IntoResponse, routing};
use charted_core::VERSION;
use metrics_exporter_prometheus::PrometheusHandle;
use serde::Serialize;
use std::collections::BTreeMap;
use utoipa::{
    IntoResponses, ToSchema,
    openapi::{Ref, RefOr, Response},
};

/// Generic entrypoint message for any API route like `/users`.
#[derive(Serialize, ToSchema)]
pub struct Entrypoint {
    /// Humane message to greet you.
    pub message: String,

    /// URI to the documentation for this entrypoint.
    pub docs: String,
}

impl Entrypoint {
    pub fn new(entity: impl AsRef<str>) -> Self {
        let entity = entity.as_ref();
        Self {
            message: format!("welcome to the {entity} API"),
            docs: format!(
                "https://charts.noelware.org/docs/server/{VERSION}/api/reference/{}",
                entity.to_lowercase().replace(' ', "")
            ),
        }
    }
}

pub type EntrypointResponse = ApiResponse<Entrypoint>;
impl IntoResponses for EntrypointResponse {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap!(
            "200" => RefOr::Ref(Ref::from_response_name("EntrypointResponse"))
        )
    }
}

pub fn create_router(ctx: &Context) -> Router<Context> {
    let mut router = Router::new()
        .nest("/users", user::create_router(ctx))
        .route("/indexes/{idOrName}", routing::get(index::fetch))
        .route("/openapi.json", routing::get(openapi::get))
        .route("/features", routing::get(features::features))
        .route("/_healthz", routing::get(healthz::healthz))
        .route("/", routing::get(main::main));

    if let Some(config) = ctx.config.metrics.as_prometheus() &&
        config.standalone.is_none()
    {
        router = router.route(&config.endpoint, routing::get(prometheus_scrape));
    }

    for feat in ctx.features.values() {
        let (path, extended) = feat.extend_router();
        router = router.nest(path, extended);
    }

    router
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub(crate) async fn prometheus_scrape(Extension(handle): Extension<Option<PrometheusHandle>>) -> impl IntoResponse {
    let Some(handle) = handle else {
        unreachable!()
    };

    handle.render()
}
