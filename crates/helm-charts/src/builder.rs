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

use crate::{HelmCharts, ACCEPTABLE_CONTENT_TYPES};
use flate2::read::MultiGzDecoder;
use multer::{Error, Multipart};
use semver::Version;
use std::{fmt::Display, io, sync::Arc};
use tar::Archive;

/// Possible error outcomes when uploading a tarball to charted-server.
#[derive(Debug, Clone)]
pub enum ReleaseTarballError {
    /// Received an invalid content type
    InvalidContentType(String),

    /// Received more than 1-2 files.
    MaxExceeded(usize),

    /// Missing the Content-Type of the tarball.
    MissingContentType,

    /// Multipart error occurred.
    Multipart(Arc<Error>),

    /// The version given was not a valid SemVer v2 version.
    Semver(Arc<semver::Error>),

    /// The server was unable to check if the tarball is valid or not.
    NotValidTarball,

    /// Missing a required tarball file.
    MissingFiles,

    /// I/O error occurred
    Io(Arc<io::Error>),
}

impl Display for ReleaseTarballError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidContentType(ct) => f.write_fmt(format_args!(
                "received invalid content type, expected one of [{}], but received [{ct}]",
                ACCEPTABLE_CONTENT_TYPES.join(", ")
            )),

            Self::MaxExceeded(files) => {
                f.write_fmt(format_args!("expected 1-2 files, received [{files}] amount of files"))
            }

            Self::MissingContentType => f.write_str("missing `Content-Type` of the tarball itself"),
            Self::NotValidTarball => f.write_str("the server was unable to check if the tarball was valid"),
            Self::Multipart(err) => f.write_fmt(format_args!("multipart error: {err}")),
            Self::MissingFiles => f.write_str("expected a tarball release file, but was missing"),
            Self::Semver(err) => f.write_fmt(format_args!("semver error: {err}")),
            Self::Io(err) => f.write_fmt(format_args!("i/o error: {err}")),
        }
    }
}

impl std::error::Error for ReleaseTarballError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::Multipart(err) => Some(err),
            Self::Semver(err) => Some(err),
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<Error> for ReleaseTarballError {
    fn from(value: Error) -> Self {
        Self::Multipart(Arc::new(value))
    }
}

impl From<semver::Error> for ReleaseTarballError {
    fn from(value: semver::Error) -> Self {
        Self::Semver(Arc::new(value))
    }
}

impl From<io::Error> for ReleaseTarballError {
    fn from(value: io::Error) -> Self {
        Self::Io(Arc::new(value))
    }
}

/// Abstraction over how we should do the uploading step.
#[derive(Debug, Clone, Default)]
pub struct UploadReleaseTarball {
    /// version of the release
    pub version: String,

    /// owner id that owns the repository
    pub owner: u64,

    /// repository id
    pub repo: u64,
}

impl UploadReleaseTarball {
    /// Sets the release version for this upload.
    pub fn with_version<I: Into<String>>(mut self, version: I) -> Self {
        self.version = version.into();
        self
    }

    /// Sets the `owner` field to whatever the `owner` variable is.
    pub fn with_owner(mut self, owner: u64) -> Self {
        self.owner = owner;
        self
    }

    /// Sets the `owner` field to whatever the `owner` variable is.
    pub fn with_repo(mut self, repo: u64) -> Self {
        self.repo = repo;
        self
    }

    pub async fn upload(self, _charts: HelmCharts, mut multipart: Multipart<'_>) -> Result<(), ReleaseTarballError> {
        // first, we parse the version and check if it is a valid SemVer string
        let _version = Version::parse(&self.version)?;
        let Some(field) = multipart.next_field().await? else {
            return Err(ReleaseTarballError::MissingFiles);
        };

        let Some(content_type) = field.content_type() else {
            return Err(ReleaseTarballError::MissingContentType);
        };

        let received = content_type.to_string();
        if !ACCEPTABLE_CONTENT_TYPES.contains(&received.as_str()) {
            return Err(ReleaseTarballError::InvalidContentType(received));
        }

        // next is validation over the tarball itself, to see if it has the available
        // structure we need:
        //
        //    >> charted-0.1.0-beta.tgz
        //    --> templates/
        //    --> Chart.lock
        //    --> Chart.yaml
        //    --> README.md or LICENSE
        //    --> values.yaml
        //    --> values.schema.json
        let bytes = field.bytes().await?;
        let mut bytes_ref = bytes.as_ref();
        let decoder = MultiGzDecoder::new(&mut bytes_ref);
        let mut archive = Archive::new(decoder);
        let entries = archive.entries()?;

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
