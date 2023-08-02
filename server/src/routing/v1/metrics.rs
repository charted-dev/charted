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

use crate::{
    models::res::{err, ApiResponse, Empty},
    Server,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use charted_common::rust::Cast;
use charted_metrics::prometheus::PrometheusRegistry;
use serde_json::json;

pub async fn metrics(State(server): State<Server>) -> Result<impl IntoResponse, ApiResponse<Empty>> {
    let registry = server.registry.clone();
    match registry.cast::<PrometheusRegistry>() {
        Some(registry) => {
            let prom_registry = registry.registry();
            let response = match prom_registry.lock() {
                Ok(_guard) => {
                    let mut buf = String::new();
                    registry.write_metrics(&mut buf).map_err(|e| {
                        sentry::capture_error(&e);
                        error!("unable to get prometheus metrics: {e}");

                        err(
                            StatusCode::UNPROCESSABLE_ENTITY,
                            ("UNABLE_TO_PROCESS", "Unable to process Prometheus registry metrics.").into(),
                        )
                    })?;

                    Ok((StatusCode::OK, buf).into_response())
                }
                Err(_) => Err(err(
                    StatusCode::SERVICE_UNAVAILABLE,
                    (
                        "UNABLE_TO_PROCESS",
                        "Unable to process this request. Try again later! :(",
                    )
                        .into(),
                )),
            };

            response
        }
        None => Ok((
            StatusCode::NOT_FOUND,
            err(
                StatusCode::NOT_FOUND,
                (
                    "HANDLER_NOT_FOUND",
                    "Route was not found",
                    json!({
                        "method": "get",
                        "url": "/metrics"
                    }),
                )
                    .into(),
            ),
        )
            .into_response()),
    }
}
