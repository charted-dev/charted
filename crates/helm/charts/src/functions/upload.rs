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

use crate::{ACCEPTABLE_CONTENT_TYPES, ALLOWED_FILES, EXEMPTED_FILES};
use azalia::remi::{
    StorageService,
    core::{StorageService as _, UploadRequest},
};
use charted_core::ResultExt;
use charted_types::{Ulid, Version};
use eyre::{Context, bail, eyre};
use flate2::bufread::MultiGzDecoder;
use multer::Multipart;
use tar::Archive;
use tracing::{info, instrument, trace};

#[instrument(name = "charted.helm.charts.upload", skip_all, fields(%owner, %repo, version = version.as_ref()))]
pub async fn upload_helm_chart<'m, V: AsRef<str>>(
    mut multipart: Multipart<'m>,
    storage: &StorageService,
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
        bail!("missing content type from field")
    };

    if !ACCEPTABLE_CONTENT_TYPES.contains(&ct.as_ref()) {
        bail!(
            "invalid content type received [{}]: wanted: {}",
            ct,
            ACCEPTABLE_CONTENT_TYPES.join(", ")
        )
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

            bail!(
                "expected either 'charts/' or 'templates/' to be avaliable, received {:?} instead",
                path
            )
        }

        if !hdr.entry_type().is_file() {
            bail!("expected file in path {:?}", path)
        }

        let name = path
            .file_name()
            .ok_or_else(|| eyre!("path was relative -- wanted a valid absolute path"))?;

        if !EXEMPTED_FILES.iter().any(|x| name == *x) || !ALLOWED_FILES.iter().any(|x| name == *x) {
            bail!("unknown file: {:?}", name)
        }
    }

    info!("we are hoping that this is an actual Helm chart and not something else, uploading!");

    let request = UploadRequest::default()
        .with_content_type(Some(ct.as_ref()))
        .with_data(bytes);

    storage
        .upload(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"), request)
        .await
        .map(|_| ())
        .into_report()
}

#[cfg(test)]
super::tests::testcases! {
    test_successful_upload(storage) {};
}
