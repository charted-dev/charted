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

use crate::{
    metrics::registries::prometheus::Prometheus,
    server::models::res::{err, ApiResponse, ErrorCode},
    Instance,
};
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};

pub async fn metrics(State(Instance { metrics, .. }): State<Instance>) -> Result<impl IntoResponse, ApiResponse> {
    // upcast Arc<dyn Registry> ~> Arc<dyn Any> so we can downcast which `Registry` impl is being used.
    let registry = metrics.as_arc_any();
    match registry.downcast_ref::<Prometheus>() {
        Some(metrics) => {
            let mut buf = String::new();
            metrics.write_to(&mut buf).map_err(|e| {
                error!(error = %e, "unable to collect Prometheus metrics");
                sentry::capture_error(&e);

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
        None => Ok((
            StatusCode::NOT_FOUND,
            err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::HandlerNotFound,
                    "Prometheus metrics is not enabled on this instance",
                ),
            ),
        )
            .into_response()),
    }
}
