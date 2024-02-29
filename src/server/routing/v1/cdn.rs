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
    server::{
        extract::Path,
        models::res::{err, ApiResponse, ErrorCode},
    },
    Instance,
};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use remi::{Blob, File, StorageService};
use serde_json::json;

pub async fn cdn(
    Path(path): Path<String>,
    State(Instance { storage, .. }): State<Instance>,
) -> Result<impl IntoResponse, ApiResponse> {
    let paths = path.trim_start_matches('/').split('/').collect::<Vec<_>>();
    let query = match storage {
        noelware_remi::StorageService::Filesystem(_) => format!("./{}", paths.join("/")),
        _ => format!("/{}", paths.join("/")),
    };

    info!(%query, "performing cdn query");
    let Some(blob) = storage
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

    match blob {
        Blob::Directory(_) => Err::<Response<_>, ApiResponse>(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "query passed in was not found",
                json!({"query":query}),
            ),
        )),

        Blob::File(File { content_type, data, .. }) => Ok((
            [(
                header::CONTENT_TYPE,
                content_type.unwrap_or(String::from("application/octet-stream")),
            )],
            data,
        )
            .into_response()),
    }
}
