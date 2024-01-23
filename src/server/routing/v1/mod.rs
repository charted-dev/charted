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

pub mod cdn;
pub mod features;
pub mod heartbeat;
pub mod indexes;
pub mod info;
pub mod main;
pub mod metrics;
pub mod openapi;

use crate::Instance;
use axum::{routing, Router};

#[allow(unused_variables, clippy::let_and_return)]
pub fn create_router(instance: &Instance) -> Router<Instance> {
    let mut router = Router::new()
        .route("/openapi.json", routing::get(openapi::json))
        .route("/openapi.yaml", routing::get(openapi::yaml))
        .route("/heartbeat", routing::get(heartbeat::HeartbeatRestController::run))
        .route("/features", routing::get(features::FeaturesRestController::run))
        .route(
            "/index/:idOrName",
            routing::get(indexes::GetChartIndexRestController::run),
        )
        .route("/info", routing::get(info::InfoRestController::run))
        .route("/", routing::get(main::MainRestController::run));

    /*
    if instance.config.metrics.enabled {}
    */

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
