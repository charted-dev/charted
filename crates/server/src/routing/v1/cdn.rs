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

use crate::{extract::Path, ServerContext};
use axum::{
    body::Body,
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use charted_core::response::{err, ApiResponse, ErrorCode};
use remi::{Blob, StorageService as _};
use serde_json::json;
use tracing::{error, info};

pub async fn cdn(State(ctx): State<ServerContext>, Path(path): Path<String>) -> Result<impl IntoResponse, ApiResponse> {
    // Reject any path that contains double dots since they're trying to access data that
    // they probably shouldn't seen. Naughty naughty...
    if path.contains("..") {
        return Err::<Response<Body>, ApiResponse>(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::HandlerNotFound,
                "route was not found",
                json!({"method":"get","query":path}),
            ),
        ));
    }

    let paths = path.trim_start_matches('/').split('/').collect::<Vec<_>>();
    let query = match ctx.storage {
        noelware_remi::StorageService::Filesystem(_) => format!("./{}", paths.join("/")),
        _ => paths.join("/"),
    };

    info!(query, "perofmring CDN query");
    let Some(Blob::File(file)) = ctx
        .storage
        .blob(&query)
        .await
        .inspect_err(|e| {
            error!(%query, error = %e, "unable to perform cdn query");
            sentry::capture_error(&e);
        })
        .map_err(|_| {
            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                (
                    ErrorCode::InternalServerError,
                    "unable to perform cdn query at this moment, try again later",
                    json!({"query":query}),
                ),
            )
        })?
    else {
        return Err::<Response<_>, ApiResponse>(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "query passed in was not found",
                json!({"query":query}),
            ),
        ));
    };

    Ok((
        [(
            header::CONTENT_TYPE,
            file.content_type.unwrap_or(String::from("application/octet-stream")),
        )],
        file.data,
    )
        .into_response())
}
