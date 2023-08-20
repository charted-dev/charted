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
use axum::{
    extract::{Path, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use charted_storage::MultiStorageService;
use remi_core::{Blob, StorageService};

pub async fn cdn(
    Path(path): Path<String>,
    State(server): State<Server>,
) -> Result<impl IntoResponse, ApiResponse<Empty>> {
    let paths = path.trim_start_matches('/').split('/').collect::<Vec<_>>();
    let query = match server.storage {
        MultiStorageService::Filesystem(_) => format!("./{}", paths.join("/")),
        _ => format!("/{}", paths.join("/")),
    };

    info!("performing cdn query [{}] ~> {query}", paths.join("/"));
    let blob = server.storage.clone().blob(query.clone()).await.map_err(|e| {
        error!(%e, "unable to perform cdn query [{query}]");
        sentry::capture_error(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            (
                "INTERNAL_SERVER_ERROR",
                format!("Unable to perform CDN query [{query}] at the moment, try again later.").as_str(),
            )
                .into(),
        )
    })?;

    if blob.is_none() {
        return Err::<Response<_>, ApiResponse<Empty>>(err(
            StatusCode::NOT_FOUND,
            ("UNKNOWN_QUERY", format!("Route 'GET {query}' was not found.").as_str()).into(),
        ));
    }

    let blob = blob.unwrap();
    match blob {
        Blob::Directory(_) => Err::<Response<_>, ApiResponse<Empty>>(err(
            StatusCode::NOT_FOUND,
            ("UNKNOWN_QUERY", format!("Route 'GET {query}' was not found.").as_str()).into(),
        )),

        Blob::File(file) => {
            let contents = file.data();
            let octet_str = &"application/octet-stream".into();
            let headers = [(header::CONTENT_TYPE, file.content_type().unwrap_or(octet_str).as_str())];

            Ok((headers, contents.clone()).into_response())
        }
    }
}
