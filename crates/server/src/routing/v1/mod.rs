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

pub mod features;
pub mod info;
pub mod main;
pub mod openapi;
pub mod organization;
pub mod repository;
pub mod user;

use crate::Context;
use axum::{Extension, Router, response::IntoResponse, routing};
use charted_config::metrics;
use charted_core::VERSION;
use metrics_exporter_prometheus::PrometheusHandle;
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

pub fn create_router(ctx: &Context) -> Router<Context> {
    let mut router = Router::new().route("/", routing::get(main::main));

    match ctx.config.metrics {
        metrics::Config::Prometheus(ref config) if config.standalone.is_none() => {
            router = router.route(&config.endpoint, routing::get(prometheus_scrape));
        }

        _ => {}
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
