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

use azalia::remi::{StorageService, core::StorageService as _};
use charted_core::ResultExt;
use charted_helm_types::ChartIndex;
use charted_types::{Ulid, Version};
use eyre::bail;
use std::pin::Pin;
use tracing::{error, info, instrument};

/// Returns a [`ChartIndex`] from the `owner`
#[instrument(name = "charted.helm.indexes.fetch", skip_all, fields(%owner))]
pub async fn get_chart_index(storage: &StorageService, owner: Ulid) -> eyre::Result<Option<ChartIndex>> {
    match storage.open(format!("./metadata/{owner}/index.yaml")).await {
        Ok(Some(bytes)) => serde_yaml_ng::from_slice(&bytes).into_report(),
        Ok(None) => Ok(None),
        Err(e) => {
            error!(error = %e, "unable to read file contents from object storage");
            sentry::capture_error(&e);

            Err(e.into())
        }
    }
}

#[instrument(
    name = "charted.helm.chart.fetch",
    skip_all,
    fields(
        %owner,
        %repo,
        allow_prereleases,
        version = version.as_ref()
    )
)]
pub fn get_chart<'asyncfn, V: AsRef<str> + Send + 'asyncfn>(
    storage: &'asyncfn StorageService,
    owner: Ulid,
    repo: Ulid,
    version: V,
    allow_prereleases: bool,
) -> Pin<Box<dyn Future<Output = eyre::Result<Option<azalia::remi::core::Blob>>> + Send + 'asyncfn>> {
    Box::pin(async move {
        let version = version.as_ref();
        if version == "latest" || version == "current" {
            let sorted = super::sort_versions(storage, owner, repo, allow_prereleases).await?;
            if sorted.is_empty() {
                return Ok(None);
            }

            let first = sorted.first().unwrap();
            return get_chart(storage, owner, repo, first.to_string(), allow_prereleases).await;
        }

        info!(owner.id = %owner, repository.id = %repo, version, "fetching chart from object storage");
        let ver = Version::parse(version)?;
        if !ver.pre.is_empty() && !allow_prereleases {
            bail!(
                "`?preleases=false` was specified but given version [{}] is a pre-release version",
                version
            )
        }

        storage
            .blob(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"))
            .await
            .into_report()
    })
}

#[instrument(
    name = "charted.helm.chart.fetch[provenance]",
    skip_all,
    fields(
        %owner,
        %repo,
        allow_prereleases,
        version = version.as_ref()
    )
)]
pub fn get_chart_provenance<'asyncfn, V: AsRef<str> + Send + 'asyncfn>(
    storage: &'asyncfn StorageService,
    owner: Ulid,
    repo: Ulid,
    version: V,
    allow_prereleases: bool,
) -> Pin<Box<dyn Future<Output = eyre::Result<Option<azalia::remi::core::Blob>>> + Send + 'asyncfn>> {
    Box::pin(async move {
        let version = version.as_ref();
        if version == "latest" || version == "current" {
            let sorted = super::sort_versions(storage, owner, repo, allow_prereleases).await?;
            if sorted.is_empty() {
                return Ok(None);
            }

            let first = sorted.first().unwrap();
            return get_chart_provenance(storage, owner, repo, first.to_string(), allow_prereleases).await;
        }

        info!(owner.id = %owner, repository.id = %repo, version, "fetching chart from object storage");
        let ver = Version::parse(version)?;
        if !ver.pre.is_empty() && !allow_prereleases {
            bail!(
                "`?preleases=false` was specified but given version [{}] is a pre-release version",
                version
            )
        }

        storage
            .blob(format!("./repositories/{owner}/{repo}/tarballs/{version}.prov.tgz"))
            .await
            .into_report()
    })
}
