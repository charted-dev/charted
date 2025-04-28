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
use axum::http::StatusCode;
use azalia::remi::{
    StorageService,
    core::{StorageService as _, UploadRequest},
};
use charted_core::api;
use charted_types::{Ulid, Version};
use flate2::bufread::MultiGzDecoder;
use multer::Multipart;
use tar::Archive;
use tracing::{info, instrument, trace};

#[instrument(name = "charted.helm.charts.upload", skip_all, fields(%owner, %repo, %version))]
pub async fn upload_helm_chart<'m>(
    mut multipart: Multipart<'m>,
    storage: &StorageService,
    owner: Ulid,
    repo: Ulid,
    version: Version,
) -> api::Result<()> {
    let field = match multipart.next_field().await.map_err(api::system_failure)? {
        Some(field) => field,
        None => {
            return Err(api::err(
                StatusCode::PRECONDITION_FAILED,
                (
                    api::ErrorCode::MissingMultipartField,
                    "no fields were ever sent or parsed",
                ),
            ));
        }
    };

    let Some(ct) = field.content_type() else {
        return Err(api::err(
            StatusCode::PRECONDITION_FAILED,
            (
                api::ErrorCode::MissingContentType,
                "missing `Content-Type` header in field",
            ),
        ));
    };

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

    let bytes = field.bytes().await.map_err(api::system_failure)?;
    let mut r = bytes.as_ref();

    let mut archive = Archive::new(MultiGzDecoder::new(&mut r));
    let entries = archive.entries().map_err(api::system_failure)?;

    for entry in entries.into_iter() {
        let entry = entry.map_err(api::system_failure)?;
        let hdr = entry.header();
        let path = entry.path().map_err(api::system_failure)?;

        trace!(path = %path.display(), "validating tar archive path");
        if hdr.entry_type().is_dir() {
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

        if !hdr.entry_type().is_file() {
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

    info!("we are hoping that this is an actual Helm chart and not something else, uploading!");

    let request = UploadRequest::default()
        .with_content_type(Some(ct.as_ref()))
        .with_data(bytes);

    storage
        .upload(format!("./repositories/{owner}/{repo}/tarballs/{version}.tgz"), request)
        .await
        .map(|_| api::no_content())
        .map_err(api::system_failure)
}

#[cfg(test)]
super::tests::testcases! {
    test_successful_upload(storage) {};
}
