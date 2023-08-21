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

use axum::extract::{multipart::MultipartError, Multipart};
use bytes::Bytes;
use charted_common::models::helm::{ChartIndex, ChartIndexSpec};
use charted_storage::MultiStorageService;
use eyre::{Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use remi_core::{Blob, StorageService, UploadRequest};
use semver::Version;
use std::{
    fmt::{Debug, Display},
    fs::create_dir_all,
    sync::Arc,
};
use tracing::{error, info, instrument, warn};

/// Acceptable content types to use to send in a tarball.
pub static ACCEPTABLE_CONTENT_TYPES: Lazy<Vec<&str>> =
    Lazy::new(|| vec!["application/gzip", "application/tar+gzip", "application/tar"]);

#[allow(dead_code)]
static ALLOWED_FILES_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("(Chart.lock|Chart.ya?ml|values.ya?ml|[.]helmignore|templates/\\w+.*[.](txt|tpl|ya?ml)|charts/\\w+.*.(tgz|tar.gz))").unwrap()
});

#[allow(dead_code)]
static EXEMPTED_FILES: Lazy<Vec<&str>> = Lazy::new(|| vec!["values.schema.json"]);

#[derive(Debug, Clone, Default)]
pub struct UploadReleaseTarball {
    pub provenance_file: Option<Bytes>,
    pub tarball: Option<Bytes>,
    pub version: String,
    pub owner: u64,
    pub repo: u64,
}

#[derive(Clone)]
pub enum ModifyReleaseTarballError {
    InvalidContentType(&'static str),
    Multipart(Arc<MultipartError>),
    MaxExceeded(usize),
    MissingContentType,
    NotValidTarball,
    MissingFiles,
}

impl Debug for ModifyReleaseTarballError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifyReleaseTarballError::NotValidTarball => {
                f.write_str("given tarball was not in a valid .tar.gz format")
            }
            ModifyReleaseTarballError::InvalidContentType(received) => f.write_fmt(format_args!(
                "invalid content type, expected one of [{}] but received {received}",
                ACCEPTABLE_CONTENT_TYPES.join(", ")
            )),
            ModifyReleaseTarballError::MissingContentType => f.write_str("tarball didn't add a `Content-Type` to it"),
            ModifyReleaseTarballError::MissingFiles => f.write_str("missing a required tarball"),
            ModifyReleaseTarballError::Multipart(err) => Debug::fmt(err, f),
            ModifyReleaseTarballError::MaxExceeded(size) => f.write_fmt(format_args!(
                "expected 1 required file, and an optional tarball. received {size} files."
            )),
        }
    }
}

impl Display for ModifyReleaseTarballError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifyReleaseTarballError::NotValidTarball => {
                f.write_str("given tarball was not in a valid .tar.gz format")
            }
            ModifyReleaseTarballError::InvalidContentType(received) => f.write_fmt(format_args!(
                "invalid content type, expected one of [{}] but received {received}",
                ACCEPTABLE_CONTENT_TYPES.join(", ")
            )),
            ModifyReleaseTarballError::MissingContentType => f.write_str("tarball didn't add a `Content-Type` to it"),
            ModifyReleaseTarballError::MissingFiles => f.write_str("missing a required tarball"),
            ModifyReleaseTarballError::Multipart(err) => Debug::fmt(err, f),
            ModifyReleaseTarballError::MaxExceeded(size) => f.write_fmt(format_args!(
                "expected 1 required file, and an optional tarball. received {size} files."
            )),
        }
    }
}

impl std::error::Error for ModifyReleaseTarballError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Multipart(err) => Some(err),
            _ => None,
        }
    }
}

impl From<MultipartError> for ModifyReleaseTarballError {
    fn from(value: MultipartError) -> Self {
        ModifyReleaseTarballError::Multipart(Arc::new(value))
    }
}

impl UploadReleaseTarball {
    pub fn version<I: Into<String>>(&mut self, version: I) -> &mut Self {
        self.version = version.into();
        self
    }

    pub fn owner(&mut self, owner: u64) -> &mut Self {
        self.owner = owner;
        self
    }

    pub fn repo(&mut self, repo: u64) -> &mut Self {
        self.repo = repo;
        self
    }

    /// Modifies the
    pub async fn from_multipart(
        &mut self,
        mut multipart: Multipart,
    ) -> Result<UploadReleaseTarball, ModifyReleaseTarballError> {
        // step 1: release tarball, step 2: (optional) provenance, step 3: any other file
        let mut provenance = false;
        let mut processed = 0;
        let mut step = 0;

        while let Some(_field) = multipart.next_field().await? {
            processed += 1;

            // break immediately if we are processing >16 files
            if processed >= 16 {
                break;
            }

            // break out if we are over 2 steps, set `proveance` to true if
            // we are 2 steps in
            step += 1;
            if step == 2 {
                provenance = true;
            }

            if step > 2 {
                break;
            }

            // get content type
            // let Some(content_type) = field.content_type() else {
            //     return Err(ModifyReleaseTarballError::MissingContentType);
            // };

            // if !ACCEPTABLE_CONTENT_TYPES.contains(&content_type) {
            //     return Err(ModifyReleaseTarballError::InvalidContentType(content_type));
            // }

            // let bytes = field.bytes().await?;
        }

        if step <= 0 {
            return Err(ModifyReleaseTarballError::MissingFiles);
        }

        if provenance && step > 2 {
            return Err(ModifyReleaseTarballError::MaxExceeded(processed - 2));
        }

        if !provenance && step > 1 {
            return Err(ModifyReleaseTarballError::MaxExceeded(processed - 1));
        }

        unreachable!()
    }
}

/*
match multipart.next_field().await? {
            Some(field) => {
                let Some(content_type) = field.content_type() else {
                    return Err(ModifyReleaseTarballError::MissingContentType);
                };

                if !ACCEPTABLE_CONTENT_TYPES.contains(&content_type) {
                    return Err(ModifyReleaseTarballError::InvalidContentType(content_type));
                }

                self.tarball = Some(field.bytes().await?);

                match multipart.next_field().await? {
                    Some(prov) => {
                        let Some(content_type) = prov.content_type() else {
                            return Err(ModifyReleaseTarballError::MissingContentType);
                        };

                        if !ACCEPTABLE_CONTENT_TYPES.contains(&content_type) {
                            return Err(ModifyReleaseTarballError::InvalidContentType(content_type));
                        }

                        self.provenance_file = Some(prov.bytes().await?);
                        Ok(*self)
                    }
                    None => Ok(*self),
                }
            }
            None => Err(ModifyReleaseTarballError::MissingFiles),
        }
 */

#[derive(Debug, Clone)]
pub struct HelmCharts {
    storage: MultiStorageService,
}

impl HelmCharts {
    /// Creates a new [`HelmCharts`] module.
    pub fn new(storage: MultiStorageService) -> HelmCharts {
        HelmCharts { storage }
    }

    /// Does pre-initialization work. This will create the `metadata` and `tarballs` directories
    /// (if they don't exist), if the storage persistence layer is the local filesystem.
    pub async fn init(&self) -> Result<()> {
        if let MultiStorageService::Filesystem(fs) = self.storage.clone() {
            let paths = vec![
                fs.normalize("./metadata")?.unwrap(),
                fs.normalize("./repositories")?.unwrap(),
            ];

            for path in paths.clone().iter() {
                if !fs.exists(path).await? {
                    warn!("directory [{}] didn't exist, creating...", path.display());
                    create_dir_all(path)?;
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
    #[instrument(name = "helm.index.sort_versions", skip(self))]
    pub async fn sort_versions(&self, owner: u64, repo: u64, prereleases: bool) -> Result<Vec<Version>> {
        if !self.storage.exists(format!("./repositories/{owner}/{repo}")).await? {
            return Ok(vec![]);
        }

        let blobs = self
            .storage
            .blobs(Some(format!("./repositories/{owner}/{repo}/tarballs")), None)
            .await?;

        Ok(blobs
            .into_iter()
            .filter_map(|blob| {
                if let Blob::File(file) = blob {
                    let name = file.name();
                    let without_suffix = name.strip_suffix(".tar.gz").unwrap();
                    Version::parse(without_suffix).ok()
                } else {
                    None
                }
            })
            .filter(|v| match prereleases {
                true => true,
                false => v.pre.is_empty(),
            })
            .sorted_by(|a, b| a.cmp(b))
            .collect::<Vec<_>>())
    }

    /// Returns a [`ChartIndexSpec`] from the `owner`.
    ///
    /// ## Arguments
    /// - `owner`: Owner ID to find the `index.yaml` file from.
    #[instrument(name = "helm.index.get", skip(self))]
    pub async fn get_index(&self, owner: u64) -> Result<Option<ChartIndexSpec>> {
        match self.storage.open(format!("./metadata/{owner}/index.yaml")).await {
            Ok(Some(bytes)) => {
                serde_yaml::from_slice(bytes.as_ref()).context("unable to parse index.yaml file (owner: {owner})")
            }

            Ok(None) => Ok(None),
            Err(e) => {
                error!(%e, "received error when trying to read [./metadata/{owner}/index.yaml]");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    /// Creates a [`ChartIndex`] for the owner.
    ///
    /// ## Arguments
    /// - `owner`: owner id to create the index from.
    #[instrument(name = "helm.index.create", skip(self))]
    pub async fn create_index(&self, owner: u64) -> Result<()> {
        info!(user = owner, "creating index.yaml for");
        if let MultiStorageService::Filesystem(fs) = self.storage.clone() {
            let path = fs.normalize(format!("./metadata/{owner}"))?.unwrap();
            if !path.exists() {
                create_dir_all(path)?;
            }
        }

        let spec = ChartIndex::default();
        let serialized = serde_yaml::to_string(&spec).unwrap();

        self.storage
            .upload(
                format!("./metadata/{owner}/index.yaml"),
                UploadRequest::default()
                    .with_content_type(Some("text/yaml; charset=utf-8".into()))
                    .with_data(Bytes::from(serialized))
                    .seal(),
            )
            .await
            .context(format!("unable to update user {owner}'s chart index"))
    }

    /// Deletes a user or organization's `index.yaml` file.
    ///
    /// ## Arguments
    /// - `owner`: Owner of the index to delete.
    #[instrument(name = "helm.index.delete", skip(self))]
    pub async fn delete_index(&self, owner: u64) -> Result<()> {
        warn!(user = owner, "deleting index.yaml for");
        self.storage
            .delete(format!("./metadata/{owner}/index.yaml"))
            .await
            .context("unable to delete user {owner}'s index")
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
    #[instrument(name = "helm.release.get", skip(self))]
    #[async_recursion::async_recursion]
    pub async fn get_tarball(
        &self,
        owner: u64,
        repo: u64,
        version: &str,
        allow_prerelease: bool,
    ) -> Result<Option<Bytes>> {
        if version == "latest" || version == "current" {
            let sorted = self.sort_versions(owner, repo, allow_prerelease).await?;
            let Some(first) = sorted.first() else {
                return Ok(None);
            };

            return self
                .get_tarball(owner, repo, first.to_string().as_str(), allow_prerelease)
                .await;
        }

        info!(user = owner, repo, version, "grabbing tarball");
        self.storage
            .open(format!("./repositories/{owner}/{repo}/tarballs/{version}.tar.gz"))
            .await
            .context("unable to collect tarball in './repositories/{owner}/{repo}/tarballs/{version}.tar.gz'")
    }
}
