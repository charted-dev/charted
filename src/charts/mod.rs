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

mod builder;
pub use builder::*;

use crate::{common::models::helm::ChartIndex, lazy, regex};
use async_recursion::async_recursion;
use eyre::Context;
use flate2::bufread::MultiGzDecoder;
use itertools::Itertools;
use multer::Multipart;
use noelware_remi::StorageService;
use once_cell::sync::Lazy;
use regex::Regex;
use remi::{Blob, Bytes, StorageService as _, UploadRequest};
use semver::Version;
use std::borrow::Cow;
use tar::Archive;
use tokio::fs::create_dir_all;

/// Accepted content types that are allowed to be sent as a tarball
const ACCEPTABLE_CONTENT_TYPES: &[&str] = &["application/gzip", "application/tar+gzip"];

/// Exempted files that aren't usually in a Helm chart, but they are allowed to be in one.
#[allow(dead_code)]
const EXEMPTED_FILES: &[&str] = &["values.schema.json", "README.md"];

/// Regular expression on all allowed files in a Helm chart
#[allow(dead_code)]
static ALLOWED_FILES: Lazy<Regex> = lazy!(regex!(
    "(Chart.lock|Chart.ya?ml|values.ya?ml|[.]helmignore|templates/\\w+.*[.](txt|tpl|ya?ml)|charts/\\w+.*.(tgz|tar.gz))"
));

#[derive(Clone)]
pub struct HelmCharts {
    storage: StorageService,
}

impl HelmCharts {
    pub fn new(storage: StorageService) -> HelmCharts {
        HelmCharts { storage }
    }

    /// Does pre-initialization work. This will create the `metadata` and `tarballs` directories
    /// (if they don't exist), if the storage persistence layer is the local filesystem.
    pub async fn init(&self) -> eyre::Result<()> {
        if let StorageService::Filesystem(ref fs) = self.storage {
            for path in [
                fs.normalize("./metadata")?.unwrap(),
                fs.normalize("./repositories")?.unwrap(),
            ] {
                if !fs.exists(&path).await? {
                    warn!(path = %path.display(), "directory doesn't exist, creating");
                    create_dir_all(path).await?;
                }
            }
        }

        Ok(())
    }

    /// Sorts all of the versions that were contained in a user or organization's
    /// repository. The first item *should* be the latest version, or the latest
    /// pre-release if `prereleases` is true.
    ///
    /// ## Arguments
    /// - `owner`:       Repository owner's ID
    /// - `repo`:        Repository ID
    /// - `prereleases`: Whether if prereleases should be in the final result.
    #[instrument(name = "charted.helm.indexes.sort", skip(self))]
    pub async fn sort_versions(&self, owner: u64, repo: u64, prereleases: bool) -> eyre::Result<Vec<Version>> {
        if !self.storage.exists(format!("./repositories/{owner}/{repo}")).await? {
            return Ok(vec![]);
        }

        let blobs = self
            .storage
            .blobs(Some(format!("./repositories/{owner}/{repo}/tarballs")), None)
            .await?;

        Ok(blobs
            .iter()
            .filter_map(|blob| match blob {
                Blob::File(file) => {
                    let name = file.name.strip_suffix(".tgz").unwrap();
                    match Version::parse(name) {
                        Ok(ver) => Some(ver),
                        Err(e) => {
                            #[cfg(test)]
                            eprintln!("when trying to parse {name}, received error: {e}");

                            #[cfg(not(test))]
                            warn!(name, owner, repo, error = %e, "when trying to sort versions from repo, received an error with tarball name");

                            None
                        }
                    }
                }

                Blob::Directory(_) => None,
            })
            .filter(|v| match prereleases {
                false => v.pre.is_empty(),
                true => true,
            })
            .sorted_by(|a, b| b.cmp(a))
            .collect())
    }

    /// Returns a [`ChartIndex`] from the `owner`.
    ///
    /// ## Arguments
    /// - `owner`: Owner ID to find the `index.yaml` file from.
    #[instrument(name = "charted.helm.indexes.get", skip(self))]
    pub async fn get_index(&self, owner: u64) -> eyre::Result<Option<ChartIndex>> {
        match self.storage.open(format!("./metadata/{owner}/index.yaml")).await {
            Ok(Some(bytes)) => {
                serde_yaml::from_slice(bytes.as_ref()).context("unable to parse index.yaml file (owner: {owner})")
            }

            Ok(None) => Ok(None),
            Err(e) => {
                error!(error = %e, "received error when trying to read [./metadata/{owner}/index.yaml]");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    /// Creates a [`ChartIndex`] for the owner.
    ///
    /// ## Arguments
    /// - `owner`: owner id to create the index from.
    #[instrument(name = "charted.helm.indexes.create", skip(self))]
    pub async fn create_index(&self, owner: u64) -> eyre::Result<()> {
        info!(owner.id = owner, "creating `index.yaml` for owner");
        if let StorageService::Filesystem(ref fs) = self.storage {
            let path = fs.normalize(format!("./metadata/{owner}"))?.unwrap();
            if !path.try_exists()? {
                warn!(path = %path.display(), "path doesn't exist, creating!");
                create_dir_all(&path).await?;
            }
        }

        self.storage
            .upload(
                format!("./metadata/{owner}/index.yaml"),
                UploadRequest::default()
                    .with_content_type(Some("text/yaml; charset=utf-8"))
                    .with_data(serde_yaml::to_string(&ChartIndex::default())?),
            )
            .await
            .context(format!("unable to create a `index.yaml` file for owner [{owner}]"))
    }

    /// Deletes a user or organization's `index.yaml` file.
    ///
    /// ## Arguments
    /// - `owner`: Owner of the index to delete.
    #[instrument(name = "charted.helm.indexes.delete", skip(self))]
    pub async fn delete_index(&self, owner: u64) -> eyre::Result<()> {
        warn!(owner.id = owner, "deleting index.yaml for");
        self.storage
            .delete(format!("./metadata/{owner}/index.yaml"))
            .await
            .context(format!("unable to delete index for owner [{owner}]"))
    }

    /// Grabs a release tarball for the specified `version`. If the version specified
    /// was `latest` or `current`, it will attempt to get all the releases and sort it
    /// by the latest one. To also show pre-release versions, you can set the `allow_prerelease`
    /// argument to be true.
    ///
    /// ## Arguments
    /// - `owner`: Owner of the repository
    /// - `repo`: Repository ID
    /// - `version`: Release version
    /// - `allow_prerelease`: If prerelease version should be shown when `latest` or `current`
    /// is specified in the `version` argument.
    #[instrument(name = "charted.helm.tarballs.get", skip(self, version), fields(version = version.as_ref()))]
    #[async_recursion]
    pub async fn get_tarball(
        &self,
        owner: u64,
        repo: u64,
        version: impl AsRef<str> + Send + 'async_recursion,
        prereleases: bool,
    ) -> eyre::Result<Option<Bytes>> {
        let version = version.as_ref();
        if version == "latest" || version == "current" {
            let sorted = self.sort_versions(owner, repo, prereleases).await?;
            let Some(ver) = sorted.first() else {
                return Ok(None);
            };

            return self.get_tarball(owner, repo, ver.to_string(), prereleases).await;
        }

        info!(owner.id = owner, repository.id = repo, version, "fetching tarball");
        let ver = Version::parse(version)?;
        if !ver.pre.is_empty() && !prereleases {
            return Err(eyre!(
                "specified a prerelease version but preleases aren't allowed to be queried"
            ));
        }

        self.storage
            .open(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"))
            .await
            .context("unable to open tarball")
    }

    pub async fn upload<'m>(
        &self,
        request: UploadReleaseTarballRequest,
        mut multipart: Multipart<'m>,
    ) -> Result<(), Error> {
        let version = Version::parse(&request.version)?;
        let field = match multipart.next_field().await? {
            Some(field) => field,
            None => return Err(Error::MissingFile),
        };

        let Some(content_type) = field.content_type() else {
            return Err(Error::MissingContentType);
        };

        if !ACCEPTABLE_CONTENT_TYPES.contains(&content_type.as_ref()) {
            return Err(Error::InvalidContentType(Cow::Owned(content_type.to_string())));
        }

        info!(owner.id = request.owner, repository.id = request.repo, %version, "now validating tarball given...");

        // next is validation over the tarball itself, to see if it has the available
        // structure we need:
        //
        //    >> charted-0.1.0-beta.tgz
        //    --> templates/
        //    --> charts/
        //    ~~~~~~~~~~~~~~~~~~~~~~~~
        //    --> Chart.lock
        //    --> Chart.yaml
        //    --> README.md or LICENSE
        //    --> values.yaml
        //    --> values.schema.json
        let bytes = field.bytes().await?;
        let mut ref_ = bytes.as_ref();
        let decoder = MultiGzDecoder::new(&mut ref_);
        let mut archive = Archive::new(decoder);
        let entries = archive.entries()?;
        for entry in entries.into_iter() {
            let _entry = entry?;
        }

        todo!()
    }
}

/*
impl UploadReleaseTarball {
    pub async fn upload(self, _charts: HelmCharts, mut multipart: Multipart<'_>) -> Result<(), ReleaseTarballError> {
        for entry in entries.into_iter() {
            let entry = entry?;

            // skip directories
            if entry.header().entry_type().is_dir() {
                continue;
            }

            // skip non files
            if !entry.header().entry_type().is_file() {
                continue;
            }

            let path = entry.path()?;
            dbg!(path);
        }

        Ok(())
    }
}
*/
