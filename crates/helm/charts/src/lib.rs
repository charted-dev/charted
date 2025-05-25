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

#[cfg(test)]
pub mod testutil;

#[cfg(test)]
mod tests;

mod ext;

use axum::http::StatusCode;
use charted_core::{BoxedFuture, ResultExt, api};
use charted_datastore::{
    DataStore, Namespace,
    remi::{Blob, File, StorageService, UploadRequest},
};
use charted_helm_types::ChartIndex;
use charted_types::{QueryableVersion, Ulid, Version};
pub use ext::*;
use eyre::{Context, bail};
use flate2::bufread::MultiGzDecoder;
use futures_util::future::FutureExt;
use itertools::Itertools;
use multer::Multipart;
use tar::Archive;
use tracing::{error, info, instrument, trace, warn};

/// Accepted content types that are allowed to be sent as a tarball
pub(crate) const ACCEPTABLE_CONTENT_TYPES: &[&str] = &["application/gzip", "application/tar+gzip"];

/// Exempted files that aren't usually in a Helm chart, but they are allowed to be in one.
pub(crate) const EXEMPTED_FILES: &[&str] = &["values.schema.json", "README.md", "LICENSE"];
pub(crate) const ALLOWED_FILES: &[&str] = &["README.md", "LICENSE", "values.yaml", "Chart.yaml", "Chart.lock"];

/// Newtype wrapper for the `metadata` namespace.
#[derive(Clone, derive_more::Display, derive_more::Deref)]
#[display("namespace '{}'", self.namespace)]
pub struct MetadataNamespace<'storage> {
    ds: &'storage DataStore,

    #[deref]
    namespace: Namespace<'storage>,
}

impl<'storage> MetadataNamespace<'storage> {
    /// Creates a new, specialized [`Namespace`] that holds all persistent metadata
    /// about users.
    pub fn new(ds: &'storage DataStore) -> Self {
        Self {
            namespace: ds.namespace("metadata"),
            ds,
        }
    }

    /// Retrieve a chart index from the datastore. Returns [`None`] if it wasn't found.
    pub async fn get_chart_index(&self, owner: Ulid) -> eyre::Result<Option<ChartIndex>> {
        match self.open(format!("{owner}/index.yaml")).await.into_report() {
            Ok(Some(bytes)) => serde_yaml_ng::from_slice(&bytes).into_report(),
            Ok(None) => Ok(None),
            Err(e) => {
                error!(error = %e, "unable to read file contents from object storage");
                sentry::capture_error(&*e);

                Err(e)
            }
        }
    }

    /// Creates a [`ChartIndex`] for the specified user. If the folder doesn't exist
    /// if we are on the filesystem, then that will take care of it.
    pub async fn create_chart_index(&self, owner: Ulid) -> eyre::Result<ChartIndex> {
        info!(owner.id = %owner, "creating `index.yaml`...");

        if let Some(fs) = self.ds.as_filesystem() {
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

        self.upload(format!("{owner}/index.yaml"), request)
            .await
            .into_report()?;

        Ok(index)
    }
}

/// Newtype wrapper for the `repositories/{owner}/{repo}` namespace.
#[derive(Clone, derive_more::Display, derive_more::Deref)]
#[display("{}", self.namespace)]
pub struct OwnerRepoNamespace<'storage> {
    ds: &'storage DataStore,
    owner: Ulid,
    repo: Ulid,

    #[deref]
    namespace: Namespace<'storage>,
}

impl<'storage> OwnerRepoNamespace<'storage> {
    pub fn new(ds: &'storage DataStore, owner: Ulid, repo: Ulid) -> Self {
        OwnerRepoNamespace {
            owner,
            repo,
            namespace: ds.namespace(format!("repositories/{owner}/{repo}")),
            ds,
        }
    }

    //// fetchers \\\\

    /// Sorts all chart indexes in this namespace and returns, in sorted order, the list
    /// of versions avaliable.
    #[instrument(
        name = "charted.helm.indexes.sort",
        skip_all,
        fields(
            owner.id = %self.owner,
            repository.id = %self.repo,
            %prereleases
        )
    )]
    pub async fn sort_versions(&self, prereleases: bool) -> eyre::Result<Vec<Version>> {
        // Only use '#exists(Path) -> bool' on the filesystem datastore
        if self.ds.is_filesystem() {
            // If the namespace doesn't exist in the datastore, return an empty vector.
            if !self.namespace.exists("").await? {
                return Ok(vec![]);
            }
        }

        // All tarballs are stored in `repositories/{owner}/{repo}/tarballs`.
        let versions = self.namespace.blobs(Some("tarballs"), None)
            .await?
            .iter()
            .filter_map(|blob| match blob {
                Blob::File(file) if self.ds.is_filesystem() => {
                    // All charts will have `.tgz` as the extension, if not, then we'll
                    // just skip it as its either malformed or old.
                    let Some(name) = file.name.strip_suffix(".tgz") else {
                        warn!(file.name, "file was not named correctly; possibly tampered with or malformed");
                        return None;
                    };

                    Version::parse(name).inspect_err(|e| {
                        warn!(file.name, error = %e, "file name was not in semver format: possibly tampered with or malformed");
                    }).ok()
                }

                // in the S3 driver, file names are returned as 'repositories/.../.../tarballs/name.tgz',
                // so we have to do some parsing!
                Blob::File(file) => {
                    let Some(name) = file.name.split('/').next_back().and_then(|s| s.strip_suffix(".tgz")) else {
                        warn!(file.name, "file was either not named properly or not in its respective namespace; possibly tampered with or malformed");
                        return None;
                    };

                    Version::parse(name).inspect_err(|e| {
                        warn!(file.name, error = %e, "file name was not in semver format: possibly tampered with or malformed");
                    }).ok()
                }

                Blob::Directory(_) => None,
            })
            .filter(|v| match prereleases {
                false => v.pre.is_empty(),
                true => true
            }).sorted_by(|a, b| b.cmp(a)).collect();

        Ok(versions)
    }

    /// Retrieve a chart index for a specified version or the latest version.
    ///
    /// ## Rationale
    /// If the latest version is being queried, unfortunately, all the versions of the
    /// repository have to be sorted by ascending order to find the latest version. For now,
    /// it works but we would probably add a `latest` field in the repository's metadata
    /// or cache the results.
    ///
    /// If `prereleases` is true, this will also allow querying pre-releases as well.
    #[instrument(
        name = "charted.helm.getChart",
        skip_all,
        fields(
            %prereleases,
            %version,
            owner.id = %self.owner,
            repository.id = %self.repo,
        )
    )]
    pub fn get_chart<'asyncfn>(
        &'asyncfn self,
        version: QueryableVersion,
        prereleases: bool,
    ) -> BoxedFuture<'asyncfn, eyre::Result<Option<File>>> {
        Box::pin(async move {
            if version.is_latest() {
                let sorted = self.sort_versions(prereleases).await?;
                if sorted.is_empty() {
                    return Ok(None);
                }

                return self
                    .get_chart(QueryableVersion::Version(sorted[0].clone()), prereleases)
                    .await;
            }

            info!("querying chart tarball from datastore");

            // Safety: We are under `QueryableVersion::Version` now and if we are
            // still `QueryableVersion::Latest`, it's a bug.
            let version = unsafe { version.as_version_unchecked() };
            if !version.pre.is_empty() && !prereleases {
                bail!("version being queried is a prerelease version but prereleases are not allowed?")
            }

            match self.blob(format!("tarballs/{version}.tgz")).await.into_report()? {
                Some(Blob::File(file)) => Ok(Some(file)),
                _ => Ok(None),
            }
        })
    }

    /// Retrieves a Helm chart's [provenance](https://helm.sh/docs/topics/provenance/) for
    /// a specific version or the latest version of the chart.
    ///
    /// Read the **Rationale** header in [`OwnerRepoNamespace::get_chart`] for more
    /// information on how we query tarballs from the datastore.
    #[instrument(
        name = "charted.helm.getChartProvenance",
        skip_all,
        fields(
            owner.id = %self.owner,
            repository.id = %self.repo,
            prereleases,
            %version
        )
    )]
    pub fn get_chart_provenance<'asyncfn>(
        &'asyncfn self,
        version: QueryableVersion,
        prereleases: bool,
    ) -> BoxedFuture<'asyncfn, eyre::Result<Option<File>>> {
        Box::pin(async {
            if version.is_latest() {
                let sorted = self.sort_versions(prereleases).await?;
                if sorted.is_empty() {
                    return Ok(None);
                }

                return self
                    .get_chart_provenance(QueryableVersion::Version(sorted[0].clone()), prereleases)
                    .await;
            }

            info!("querying chart tarball from datastore");

            // Safety: We are under `QueryableVersion::Version` now and if we are
            // still `QueryableVersion::Latest`, it's a bug.
            let version = unsafe { version.as_version_unchecked() };
            if !version.pre.is_empty() && !prereleases {
                bail!("version being queried is a prerelease version but prereleases are not allowed?")
            }

            match self.blob(format!("tarballs/{version}.prov.tgz")).await.into_report()? {
                Some(Blob::File(file)) => Ok(Some(file)),
                _ => Ok(None),
            }
        })
    }

    /// Deletes a Helm chart from the datastore.
    #[instrument(
        name = "charted.helm.deleteChart",
        skip_all,
        fields(
            owner.id = %self.owner,
            repository.id = %self.repo,
            %version,
        )
    )]
    pub fn delete_chart(&self, version: Version) -> impl Future<Output = eyre::Result<()>> + Send + use<'_> {
        self.delete(format!("tarballs/{version}.tgz")).map(|x| x.into_report())
    }

    /// Deletes a Helm chart's [provenance](https://helm.sh/docs/topics/provenance/) from the datastore.
    #[instrument(
        name = "charted.helm.deleteChart",
        skip_all,
        fields(
            owner.id = %self.owner,
            repository.id = %self.repo,
            %version,
        )
    )]
    pub fn delete_chart_provenance(
        &self,
        version: Version,
    ) -> impl Future<Output = eyre::Result<()>> + Send + use<'_> {
        self.delete(format!("tarballs/{version}.tgz")).map(|x| x.into_report())
    }

    //// upload \\\\
    #[instrument(
        name = "charted.helm.uploadChart",
        skip_all,
        fields(
            owner.id = %self.owner,
            repository.id = %self.repo,
            %version,
        )
    )]
    pub async fn upload_chart<'m>(&self, mut multipart: Multipart<'m>, version: Version) -> api::Result<()> {
        // Find the first field in the multipart stream. We don't really care about
        // the file name since we're going to be doing a bit of paranoia validation
        // on the stream itself once we find the field.
        let field = multipart
            .next_field()
            .await
            .map_err(api::system_failure)?
            .ok_or_else(|| {
                api::err(
                    StatusCode::PRECONDITION_FAILED,
                    (
                        api::ErrorCode::MissingMultipartField,
                        "multipart stream was missing a field",
                    ),
                )
            })?;

        // Get the content type of the field. If we are doing this correctly
        // and not trying to break the system, the HTTP client should already
        // handle it but we still need to be paranoid about the multipart
        // stream anyway.
        let ct = field.content_type().ok_or_else(|| {
            api::err(
                StatusCode::PRECONDITION_FAILED,
                (
                    api::ErrorCode::MissingContentType,
                    "missing `Content-Type` header in field",
                ),
            )
        })?;

        if !ACCEPTABLE_CONTENT_TYPES.contains(&ct.as_ref()) {
            return Err(api::err(
                StatusCode::PRECONDITION_FAILED,
                (
                    api::ErrorCode::InvalidHttpHeaderValue,
                    format!(
                        "invalid `Content-Type` header: {}; wanted either: {}",
                        ct,
                        ACCEPTABLE_CONTENT_TYPES.join(", ")
                    ),
                ),
            ));
        }

        info!("validating multipart stream to check if it's a valid Helm chart");

        // temporary variable for rustc to shut up as well
        //     error[E0505]: cannot move out of `field` because it is borrowed
        //     --> crates/helm/charts/src/lib.rs:396:21
        //      |
        //  340 |         let field = multipart
        //      |             ----- binding `field` declared here
        //  ...
        //  358 |         let ct = field.content_type().ok_or_else(|| {
        //      |                  ----- borrow of `field` occurs here
        //  ...
        //  396 |         let bytes = field.bytes().await.map_err(api::system_failure)?;
        //      |                     ^^^^^ move out of `field` occurs here
        //  ...
        //  491 |             .with_content_type(Some(ct.as_ref()))
        //      |                                     -- borrow later used here
        let ct = ct.to_owned();

        // Now, this is the paranoia stage on where things can fail but tests
        // should cover this. The structure that we want inside of the tarball
        // is:
        //
        //       >> charted-0.1.0.tgz
        //       --> templates/**/*.[yaml|yml]
        //       --> charts/**/*.tgz
        //       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        //       --> Chart.lock
        //       --> Chart.yaml
        //       --> README.md, LICENSE
        //       --> values.yaml, values.schema.json
        let bytes = field.bytes().await.map_err(api::system_failure)?;

        // temporary variable so that rustc can shut up
        //
        //     error[E0716]: temporary value dropped while borrowed
        //     --> crates/helm/charts/src/lib.rs:397:65
        //      |
        //  397 |         let mut archive = Archive::new(MultiGzDecoder::new(&mut bytes.as_ref()));
        //      |                                                                 ^^^^^^^^^^^^^^  - temporary value is freed at the end of this statement
        //      |                                                                 |
        //      |                                                                 creates a temporary value which is freed while still in use
        //  398 |
        //  399 |         for entry in archive.entries().map_err(api::system_failure)? {}
        //      |                      ------- borrow later used here
        //      |
        //      = note: consider using a `let` binding to create a longer lived value
        let mut bytes_as_slice = bytes.as_ref();
        let mut archive = Archive::new(MultiGzDecoder::new(&mut bytes_as_slice));

        for entry in archive.entries().map_err(api::system_failure)? {
            let entry = entry
                .context("failed to compute tar entry")
                .map_err(api::system_failure_from_report)?;

            // Retrieve the entry's metadata.
            let header = entry.header();

            // On Unix, `tar` will never call `Err(...)` on the path, so it's sound
            // to use `unwrap_unchecked()` here.
            //
            // https://github.com/alexcrichton/tar-rs/blob/d5c546e2e72271746fb0ab19db152f5b4ab4b36a/src/header.rs#L1709-L1718
            #[cfg(unix)]
            let path = unsafe { entry.path().unwrap_unchecked() };

            // On non-Unix (Windows), this can fail if the path is not
            // valid unicode.
            #[cfg(not(unix))]
            let path = entry
                .path()
                .context("path was not in valid unicode")
                .map_err(api::system_failure_from_report)?;

            trace!(path = %path.display(), "validating entry in archive");

            // If the entry in the archive is a directory, we only want the
            // "templates" and "charts" directories to be avaliable but it's
            // also ok if they're not.
            if header.entry_type().is_dir() {
                if path.ends_with("charts") || path.ends_with("templates") {
                    continue;
                }

                return Err(api::err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        api::ErrorCode::InvalidInput,
                        format!(
                            "expected either a `charts/` or `templates/` directory to be avaliable, received {} instead",
                            path.display()
                        ),
                    ),
                ));
            }

            if !header.entry_type().is_file() {
                return Err(api::err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        api::ErrorCode::InvalidInput,
                        format!("excepted a file in path {}", path.display()),
                    ),
                ));
            }

            let name = path.file_name().ok_or_else(|| {
                api::err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (api::ErrorCode::InvalidInput, "path was not valid utf-8"),
                )
            })?;

            if !EXEMPTED_FILES.iter().any(|x| name == *x) || !ALLOWED_FILES.iter().any(|x| name == *x) {
                return Err(api::err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        api::ErrorCode::AccessNotPermitted,
                        format!("path '{}' is not allowed", path.display()),
                    ),
                ));
            }
        }

        info!("paranoia checks: done; assuming this is a valid archive. uploading to datastore");

        let request = UploadRequest::default()
            .with_content_type(Some(ct.as_ref()))
            .with_data(bytes);

        self.upload(format!("tarballs/{version}.tgz"), request)
            .await
            .map(|_| api::no_content())
            .map_err(api::system_failure)
    }
}
