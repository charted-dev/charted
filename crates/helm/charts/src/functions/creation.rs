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

use azalia::remi::{
    StorageService,
    core::{StorageService as _, UploadRequest},
};
use charted_helm_types::ChartIndex;
use charted_types::Ulid;
use eyre::Context;
use tracing::{info, instrument, warn};

/// Creates a [`ChartIndex`] for the owner.
#[instrument(name = "charted.helm.indexes.create", skip_all, fields(%owner))]
pub async fn create_chart_index(storage: &StorageService, owner: Ulid) -> eyre::Result<ChartIndex> {
    info!(owner.id = %owner, "creating `index.yaml`...");
    if let StorageService::Filesystem(fs) = storage {
        let path = fs.normalize(format!("./metadata/{owner}"))?.unwrap();
        if !tokio::fs::try_exists(&path).await? {
            warn!(path = %path.display(), "path doesn't exist, creating");
            tokio::fs::create_dir_all(path).await?;
        }
    }

    let index = ChartIndex::default();
    let request = UploadRequest::default()
        .with_data(serde_yaml_ng::to_string(&index)?)
        .with_content_type(Some("application/yaml; charset=utf-8"));

    storage
        .upload(format!("./metadata/{owner}/index.yaml"), request)
        .await
        .map(|_| index)
        .with_context(|| format!("failed to create `index.yaml` for owner [{owner}]"))
}
