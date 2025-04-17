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
    core::{Blob, StorageService as _},
};
use charted_types::{Ulid, Version};
use itertools::Itertools;
use tracing::{instrument, warn};

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

#[cfg(test)]
crate::functions::tests::testcases! {
    test_sort_versions(storage) {
        use azalia::remi::core::UploadRequest;

        crate::init(&storage).await.unwrap();

        let fixture = crate::functions::tests::fixture!("youtrack.tgz");
        let contents = tokio::fs::read(&fixture).await.unwrap();

        let owner = Ulid::new("01J5SG1FXT019M8Q2TB84QVV8V").unwrap();
        let repo = Ulid::new("01J5SG1JAEG4RJCGYC5KJ6QYS2").unwrap();

        for version in ["0.1.0-beta", "0.2.1", "1.0.0-beta.1", "2024.3.24", "1.0.0+d1cebae"] {
            let request = UploadRequest::default()
                .with_content_type(Some("application/tar+gzip"))
                .with_data(contents.clone());

            storage.upload(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"), request).await.unwrap();
        }

        // shows non-prereleases
        let versions = crate::sort_versions(&storage, owner, repo, false).await.unwrap();
        assert_eq!(versions, &[
            semver::Version::parse("2024.3.24").unwrap().into(),
            semver::Version::parse("1.0.0+d1cebae").unwrap().into(),
            semver::Version::parse("0.2.1").unwrap().into(),
        ]);

        // shows all pre-releases
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
