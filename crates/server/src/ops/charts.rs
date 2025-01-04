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

use crate::ServerContext;
use azalia::remi::core::{Bytes, StorageService, UploadRequest};
use charted_types::{
    helm::{self, ChartIndex},
    Ulid, User,
};
use eyre::Context;
use tracing::{error, instrument};

#[instrument(name = "charted.server.ops.indexes.get", skip(ctx))]
pub async fn get_index(ctx: &ServerContext, id: Ulid) -> eyre::Result<Option<helm::ChartIndex>> {
    let Some(content) = ctx
        .storage
        .open(format!("./metadata/{id}/index.yaml"))
        .await
        .inspect_err(|e| {
            error!(error = %e, %id, "failed to lookup chart index from data storage");
            sentry::capture_error(e);
        })?
    else {
        return Ok(None);
    };

    serde_yaml_ng::from_slice(&content)
        .map(Some)
        .inspect_err(|e| {
            error!(error = %e, %id, "failed to deserialize chart into `helm::ChartIndex`");
            sentry::capture_error(e);
        })
        .context("failed to deserialize chart into `helm::ChartIndex`")
}

#[instrument(name = "charted.server.ops.indexes.get", skip_all, fields(user.id))]
pub async fn create_index(cx: &ServerContext, user: &User) -> eyre::Result<()> {
    let index = ChartIndex::default();
    let serialized = serde_yaml_ng::to_string(&index)?;

    cx.storage
        .upload(
            format!("./metadata/{}/index.yaml", user.id),
            UploadRequest::default()
                .with_content_type(Some("application/yaml"))
                .with_data(Bytes::from(serialized)),
        )
        .await
        .inspect_err(|e| {
            error!(error = %e, %user.id, "unable to upload chart index for user");
            sentry::capture_error(e);
        })
        .context("failed to upload chart index")
}
