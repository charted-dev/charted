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

//! The `ops` module contains common operations that operate on entities and regular
//! endpoints.

pub mod avatars;
pub mod user;

use axum::http::StatusCode;
use charted_core::response::{err, internal_server_error, ApiResponse, ErrorCode};
use charted_entities::helm::ChartIndex;
use noelware_remi::{remi::StorageService as _, StorageService};
use serde_json::json;
use tracing::{error, instrument};

/// Pulls and returns a [`ChartIndex`] from a entity class (user/organization).
#[instrument(name = "charted.server.ops.pull_index", skip(storage))]
pub async fn pull_index(storage: &StorageService, entity: &str, id: i64) -> Result<ChartIndex, ApiResponse> {
    let Some(contents) = storage
        .open(format!("./metadata/{id}/index.yaml"))
        .await
        .inspect_err(|e| {
            error!(error = %e, id, "failed to pull the `index.yaml` file");
            sentry::capture_error(e);
        })
        .map_err(|_| internal_server_error())?
    else {
        return Err(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "index for entity doesn't exist! this is a bug that should be fixed.",
                json!({"class":entity,"id":id}),
            ),
        ));
    };

    serde_yaml::from_slice(&contents)
        .inspect_err(|e| {
            error!(error = %e, id, "failed to deserialize contents into a chart index, was it tampered?");
            sentry::capture_error(e);
        })
        .map_err(|_| internal_server_error())
}
