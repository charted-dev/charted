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

//pub mod features;
pub mod healthz;
pub mod indexes;
pub mod main;
pub mod openapi;
pub mod organization;
pub mod repository;
pub mod user;

use crate::{Env, mk_api_response_types, mk_into_responses};
use axum::{Router, routing};
use charted_core::VERSION;
use serde::Serialize;
use utoipa::ToSchema;

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

mk_api_response_types!(Entrypoint);
mk_into_responses!(for Entrypoint {
    "200" => [ref(EntrypointResponse)];
});

pub fn create_router(env: &Env) -> Router<Env> {
    let mut router = Router::new()
        .nest("/users", user::create_router(env))
        .route("/indexes/{idOrName}", routing::get(indexes::fetch))
        .route("/openapi.json", routing::get(openapi::openapi))
        .route("/healthz", routing::get(healthz::healthz))
        .route("/", routing::get(main::main));

    if let Some(metrics) = env.config.metrics.as_prometheus() &&
        metrics.standalone.is_none()
    {
        router = router.route(&metrics.endpoint, routing::get(super::prometheus_scrape));
    }

    router
}
