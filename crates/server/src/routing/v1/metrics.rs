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

use crate::ServerContext;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use charted_core::response::{err, ApiResponse, ErrorCode};
use charted_metrics::prometheus::Prometheus;
use serde_json::json;
use tracing::error;

pub async fn metrics(
    State(ServerContext { metrics, .. }): State<ServerContext>,
) -> Result<impl IntoResponse, ApiResponse> {
    let registry = metrics.as_arc_any();
    let prometheus = registry.downcast_ref::<Prometheus>().ok_or_else(|| {
        err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::HandlerNotFound,
                "route was not found",
                json!({"route":"/","method":"get"}),
            ),
        )
    })?;

    let mut buf = String::new();
    prometheus
        .write_to(&mut buf)
        .inspect_err(|e| {
            error!(error = %e, "unable to collect Prometheus metrics");
            sentry::capture_error(e);
        })
        .map_err(|_| {
            err(
                StatusCode::UNPROCESSABLE_ENTITY,
                (ErrorCode::UnableToProcess, "was unable to process Prometheus metrics"),
            )
        })?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        buf,
    )
        .into_response())
}
