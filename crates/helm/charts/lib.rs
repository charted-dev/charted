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
    core::{Blob, StorageService as _, UploadRequest},
};
use charted_helm_types::ChartIndex;
use charted_types::{Ulid, Version};
use eyre::{Context, Report, eyre};
use flate2::bufread::MultiGzDecoder;
use itertools::Itertools;
use multer::Multipart;
use std::{future::Future, pin::Pin};
use tar::Archive;
use tracing::{error, info, instrument, trace, warn};

/// Accepted content types that are allowed to be sent as a tarball
const ACCEPTABLE_CONTENT_TYPES: &[&str] = &["application/gzip", "application/tar+gzip"];

/// Exempted files that aren't usually in a Helm chart, but they are allowed to be in one.
const EXEMPTED_FILES: &[&str] = &["values.schema.json", "README.md", "LICENSE"];
const ALLOWED_FILES: &[&str] = &["README.md", "LICENSE", "values.yaml", "Chart.yaml", "Chart.lock"];

/// Initializes the storage service to contain the following directories:
///
/// * `$DATA_DIR/metadata` - used for holding user/organization indexes
/// * `$DATA_DIR/repositories` - used for holding repository metadata (charts, readmes,
///   etc)
pub async fn init(storage: &StorageService) -> eyre::Result<()> {
    if let StorageService::Filesystem(fs) = storage {
        let paths = [fs.normalize("./metadata")?.unwrap(), fs.normalize("./repositories")?.unwrap()];

        for path in paths {
            if !tokio::fs::try_exists(&path).await? {
                warn!(path = %path.display(), "creating directory as it doesn't exist");
                tokio::fs::create_dir_all(path).await?;
            }
        }
    }

    Ok(())
}

/// Sorts all of the versions that were contained in a user or organization's
/// repository. The first item *should* be the latest version, or the latest
/// pre-release if `prereleases` is true.
#[instrument(name = "charted.helm.indexes.sort-versions", skip_all, fields(%owner, %repo, allow_prereleases = prereleases))]
pub async fn sort_versions(
    storage: &StorageService,
    owner: Ulid,
    repo: Ulid,
    prereleases: bool,
) -> eyre::Result<Vec<Version>> {
    // If the object storage tells us that it doesn't exist, then it don't exist.
    if !storage.exists(format!("./repositories/{owner}/{repo}")).await? {
        return Ok(vec![]);
    }

    // Collect all the blobs that we can.
    let blobs = storage
        .blobs(Some(format!("./repositories/{owner}/{repo}/tarballs")), None)
        .await?;

    let filtered = blobs.iter().filter_map(|blob| match blob {
        Blob::File(file) => {
            let Some(name) = file.name.strip_suffix(".tgz") else {
                warn!(file.name, owner.id = %owner, repository.id = %repo, "name didn't end in .tgz");
                return None;
            };

            match Version::parse(name) {
                Ok(ver) => Some(ver),
                Err(e) => {
                    warn!(name, owner.id = %owner, repository.id = %repo, error = %e, "file name was not valid semver, skipping");
                    None
                }
            }
        }

        // We should never get directories in this case -- all files are named
        // '{version}.tgz' in object storage.
        Blob::Directory(_) => None,
    }).filter(|v| match prereleases {
        false => v.pre.is_empty(),
        true => true
    }).sorted_by(|a, b| b.cmp(a)).collect();

    Ok(filtered)
}

/// Returns a [`ChartIndex`] from the `owner`
#[instrument(name = "charted.helm.indexes.fetch", skip_all, fields(%owner))]
pub async fn get_chart_index(storage: &StorageService, owner: Ulid) -> eyre::Result<Option<ChartIndex>> {
    match storage.open(format!("./metadata/{owner}/index.yaml")).await {
        Ok(Some(bytes)) => serde_yaml_ng::from_slice(&bytes).context("failed to parse `index.yaml`"),
        Ok(None) => Ok(None),
        Err(e) => {
            error!(error = %e, "unable to read file contents from object storage");
            sentry::capture_error(&e);

            Err(e.into())
        }
    }
}

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

/// Deletes a user or organization's `index.yaml` file.
#[instrument(name = "charted.helm.indexes.delete", skip_all, fields(%owner))]
pub async fn delete_chart_index(storage: &StorageService, owner: Ulid) -> eyre::Result<()> {
    warn!(owner.id = %owner, "deleting `index.yaml`");
    storage
        .delete(format!("./metadata/{owner}/index.yaml"))
        .await
        .with_context(|| format!("unable to delete `index.yaml` for owner [{owner}]"))
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
) -> Pin<Box<dyn Future<Output = eyre::Result<Option<azalia::remi::core::Bytes>>> + Send + 'asyncfn>> {
    Box::pin(async move {
        let version = version.as_ref();
        if version == "latest" || version == "current" {
            let sorted = sort_versions(storage, owner, repo, allow_prereleases).await?;
            if sorted.is_empty() {
                return Ok(None);
            }

            let first = sorted.first().unwrap();
            return get_chart(storage, owner, repo, first.to_string(), allow_prereleases).await;
        }

        info!(owner.id = %owner, repository.id = %repo, version, "fetching chart from object storage");
        let ver = Version::parse(version)?;
        if !ver.pre.is_empty() && !allow_prereleases {
            return Err(eyre!(
                "`?preleases=false` was specified but given version [{version}] is a pre-release version"
            ));
        }

        storage
            .open(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"))
            .await
            .context("unable to open tarball")
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
) -> Pin<Box<dyn Future<Output = eyre::Result<Option<azalia::remi::core::Bytes>>> + Send + 'asyncfn>> {
    Box::pin(async move {
        let version = version.as_ref();
        if version == "latest" || version == "current" {
            let sorted = sort_versions(storage, owner, repo, allow_prereleases).await?;
            if sorted.is_empty() {
                return Ok(None);
            }

            let first = sorted.first().unwrap();
            return get_chart(storage, owner, repo, first.to_string(), allow_prereleases).await;
        }

        info!(owner.id = %owner, repository.id = %repo, version, "fetching chart from object storage");
        let ver = Version::parse(version)?;
        if !ver.pre.is_empty() && !allow_prereleases {
            return Err(eyre!(
                "`?preleases=false` was specified but given version [{version}] is a pre-release version"
            ));
        }

        storage
            .open(format!(
                "./repositories/{owner}/{repo}/tarballs/{version}.provenance.tgz"
            ))
            .await
            .context("unable to open tarball")
    })
}

#[instrument(name = "charted.helm.charts.upload", skip_all, fields(%owner, %repo, version = version.as_ref()))]
pub async fn upload<'m, V: AsRef<str>>(
    storage: &StorageService,
    mut multipart: Multipart<'m>,
    owner: Ulid,
    repo: Ulid,
    version: V,
) -> eyre::Result<()> {
    let version = Version::parse(version.as_ref())?;
    let field = match multipart.next_field().await? {
        Some(field) => field,
        None => {
            return Err(multer::Error::IncompleteFieldData {
                field_name: Some("{first field}".into()),
            }
            .into());
        }
    };

    let Some(ct) = field.content_type() else {
        return Err(eyre!("missing content type data"));
    };

    if !ACCEPTABLE_CONTENT_TYPES.contains(&ct.as_ref()) {
        return Err(eyre!(
            "invalid content type received: {ct}; wanted either {}",
            ACCEPTABLE_CONTENT_TYPES.join(", ")
        ));
    }

    let ct = ct.clone();
    info!(owner.id = %owner, repository.id = %repo, %version, "now validating chart");

    // Now, we need to *actually* validate that it is a chart. The structure we want is:
    //
    //    >> charted-0.1.0-beta.tgz
    //    --> templates/
    //    --> charts/
    //    ~~~~~~~~~~~~~~~~~~~~~~~~
    //    --> Chart.lock
    //    --> Chart.yaml
    //    --> README.md, LICENSE
    //    --> values.yaml
    //    --> values.schema.json

    let bytes = field.bytes().await?;
    let mut r = bytes.as_ref();

    let mut archive = Archive::new(MultiGzDecoder::new(&mut r));
    let entries = archive.entries()?;

    for entry in entries.into_iter() {
        let entry = entry.context("failed to validate tar entry")?;
        let hdr = entry.header();
        let path = entry.path().context("expected to get a valid path")?;

        trace!(path = %path.display(), "validating tar archive path");
        if hdr.entry_type().is_dir() {
            if path.ends_with("charts") || path.ends_with("templates") {
                continue;
            }

            return Err(eyre!(
                "expected either 'charts/' or 'templates/' to be avaliable, but received instead: {path:?}"
            ));
        }

        if !hdr.entry_type().is_file() {
            return Err(eyre!("expected a file in path {path:?}"));
        }

        let name = path
            .file_name()
            .ok_or_else(|| eyre!("path was relative -- wanted a valid absolute path"))?;

        if !EXEMPTED_FILES.iter().any(|x| name == *x) || !ALLOWED_FILES.iter().any(|x| name == *x) {
            return Err(eyre!("invalid file: {name:?}"));
        }
    }

    info!("we are hoping that this is an actual Helm chart and not something else, uploading!");

    let request = UploadRequest::default()
        .with_content_type(Some(ct.as_ref()))
        .with_data(bytes);

    storage
        .upload(
            format!("./repositories/{owner}/{repo}/tarballs/{version}.tar.gz"),
            request,
        )
        .await
        .map(|_| ())
        .map_err(Into::into)
}

pub async fn delete_chart(
    storage: &StorageService,
    owner: u64,
    repo: u64,
    version: impl AsRef<str> + Send,
) -> eyre::Result<()> {
    storage
        .delete(format!("./repositories/{owner}/{repo}/{}.tgz", version.as_ref()))
        .await
        .map(|_| ())
        .map_err(Report::from)
}

#[cfg(test)]
mod tests {
    use charted_types::Ulid;

    macro_rules! testcases {
        ($($name:ident($storage:ident) $code:block;)*) => {
            $(
                #[tokio::test]
                #[cfg_attr(windows, ignore = "fails on windows because it feels like it i guess")]
                async fn $name() {
                    use ::azalia::remi::core::StorageService;

                    let tempdir = ::tempfile::TempDir::new().unwrap();
                    let path = tempdir.into_path();
                    let $storage = ::azalia::remi::StorageService::Filesystem(::azalia::remi::fs::StorageService::with_config(
                        azalia::remi::fs::StorageConfig::new(&path),
                    ));

                    ($storage).init().await.expect("failed to initialize");
                    $code

                    // clean up the storage service so we don't dangle the `path` from being destroyed since it
                    // is a reference to the tempdir
                    ::std::mem::drop($storage);
                    ::std::fs::remove_dir_all(path).expect("tempdir to be removed by now");
                }
            )*
        };
    }

    macro_rules! fixture {
        ($path:literal) => {
            ::std::path::PathBuf::from(::std::env!("CARGO_MANIFEST_DIR"))
                .join("__fixtures__")
                .join($path)
        };
    }

    testcases! {
        get_tarball(storage) {
            let fixture = fixture!("hello-world.tgz");
            let contents = tokio::fs::read(&fixture).await.unwrap();

            let owner = Ulid::new("01J5SG1FXT019M8Q2TB84QVV8V").unwrap();
            let repo = Ulid::new("01J5SG1JAEG4RJCGYC5KJ6QYS2").unwrap();

            for version in ["0.1.0-beta", "0.2.1", "1.0.0-beta.1", "2024.3.24", "1.0.0+d1cebae"] {
                let request = azalia::remi::core::UploadRequest::default()
                    .with_content_type(Some("application/tar+gzip"))
                    .with_data(contents.clone());

                storage.upload(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"), request).await.unwrap();
            }

            crate::init(&storage).await.unwrap();

            let _ = crate::get_chart(&storage, owner, repo, "latest", false).await.unwrap().unwrap();
            let _ = crate::get_chart(&storage, owner, repo, "0.1.0-beta", false).await.unwrap_err();
            let _ = crate::get_chart(&storage, owner, repo, "0.1.0-beta", true).await.unwrap().unwrap();
        };

        sort_versions(storage) {
            let fixture = fixture!("youtrack.tgz");
            let contents = tokio::fs::read(&fixture).await.unwrap();

            let owner = Ulid::new("01J5SG1FXT019M8Q2TB84QVV8V").unwrap();
            let repo = Ulid::new("01J5SG1JAEG4RJCGYC5KJ6QYS2").unwrap();

            for version in ["0.1.0-beta", "0.2.1", "1.0.0-beta.1", "2024.3.24", "1.0.0+d1cebae"] {
                let request = azalia::remi::core::UploadRequest::default()
                    .with_content_type(Some("application/tar+gzip"))
                    .with_data(contents.clone());

                storage.upload(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"), request).await.unwrap();
            }

            crate::init(&storage).await.unwrap();

            let versions = crate::sort_versions(&storage, owner, repo, false).await.unwrap();
            assert_eq!(versions, &[
                semver::Version::parse("2024.3.24").unwrap().into(),
                semver::Version::parse("1.0.0+d1cebae").unwrap().into(),
                semver::Version::parse("0.2.1").unwrap().into(),
            ]);

            let versions = crate::sort_versions(&storage, owner, repo, true).await.unwrap();
            assert_eq!(versions, &[
                semver::Version::parse("2024.3.24").unwrap().into(),
                semver::Version::parse("1.0.0+d1cebae").unwrap().into(),
                semver::Version::parse("1.0.0-beta.1").unwrap().into(),
                semver::Version::parse("0.2.1").unwrap().into(),
                semver::Version::parse("0.1.0-beta").unwrap().into(),
            ]);
        };
    }
}
