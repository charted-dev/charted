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
    models::res::{err, ApiResponse, ErrorCode},
    Server,
};
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use charted_metrics::SingleRegistry;
use serde_json::json;

pub async fn metrics(State(server): State<Server>) -> Result<impl IntoResponse, ApiResponse> {
    match server.registry {
        SingleRegistry::Prometheus(registry) => {
            let mut buf = String::new();
            registry.write_metrics(&mut buf).map_err(|e| {
                sentry::capture_error(&e);
                error!("unable to collect prometheus metrics: {e}");

                err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        ErrorCode::UnableToProcess,
                        "Unable to process Prometheus registry metrics",
                    ),
                )
            })?;

            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
                buf,
            )
                .into_response())
        }
        _ => Ok((
            StatusCode::NOT_FOUND,
            err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::HandlerNotFound,
                    "Route was not found",
                    json!({
                        "method": "get",
                        "url": "/metrics"
                    }),
                ),
            ),
        )
            .into_response()),
    }
}
