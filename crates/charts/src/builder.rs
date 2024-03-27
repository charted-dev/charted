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

use axum::http::StatusCode;
use charted_server::{
    err,
    multipart::{err_to_msg, expand_details_from_err, to_err_code, to_status_code},
    ApiResponse, ErrorCode,
};
use serde_json::{json, Value};
use std::{borrow::Cow, fmt::Display, io};

/// Represents a request payload to upload a release tarball.
#[derive(Default)]
pub struct UploadReleaseTarballRequest {
    /// version of the release
    pub version: String,

    /// ID of the owner that owns the repository
    pub owner: u64,

    /// ID of the repository to upload in
    pub repo: u64,
}

/// Possible error outcomes when uploading a tarball to the API server
#[derive(Debug)]
pub enum Error {
    /// received an invalid content type
    InvalidContentType(Cow<'static, str>),

    /// received more than one or two files
    MaxFileLimit(usize),

    /// missing the required `Content-Type`
    MissingContentType,

    /// Received an multipart error
    Multipart(multer::Error),

    /// received an invalid semver version
    SemVer(semver::Error),

    /// the API server was unable to validate that the tarball received
    /// was a valid tarball.
    InvalidTarball,

    /// Missing a required file
    MissingFile,

    /// Some I/O error occurred
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error as E;

        match self {
            E::InvalidContentType(ct) => write!(f, "received invalid content type [{ct}]"),
            E::MaxFileLimit(files) => write!(
                f,
                "reached maximum files to receive by the request; wanted 1-2, received {files} file(s) instead"
            ),

            E::MissingContentType => f.write_str("missing required `Content-Type`"),
            E::InvalidTarball => f.write_str("received an invalid tarball"),
            E::MissingFile => write!(f, "wanted a tarball release file or provenance file, but was missing"),

            E::Multipart(err) => Display::fmt(err, f),
            E::SemVer(err) => Display::fmt(err, f),
            E::Io(err) => Display::fmt(err, f),
        }
    }
}

impl From<multer::Error> for Error {
    fn from(value: multer::Error) -> Self {
        Self::Multipart(value)
    }
}

impl From<semver::Error> for Error {
    fn from(value: semver::Error) -> Self {
        Self::SemVer(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Error {
    fn status(&self) -> StatusCode {
        use Error as E;

        match self {
            E::InvalidContentType(_)
            | E::MaxFileLimit(_)
            | E::SemVer(_)
            | E::MissingFile
            | E::MissingContentType
            | E::InvalidTarball => StatusCode::NOT_ACCEPTABLE,

            E::Multipart(err) => to_status_code(err),
            E::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> ErrorCode {
        use Error as E;

        match self {
            E::InvalidContentType(_) => ErrorCode::InvalidContentType,
            E::MissingContentType => ErrorCode::MissingHeader,
            E::MaxFileLimit(_) => ErrorCode::MultipartFieldsSizeExceeded,
            E::InvalidTarball => ErrorCode::InvalidBody,
            E::Multipart(err) => to_err_code(err),
            E::MissingFile => ErrorCode::MissingMultipartField,
            E::SemVer(_) => ErrorCode::InvalidType,
            E::Io(_) => ErrorCode::Io,
        }
    }

    fn message(&self) -> String {
        use Error as E;

        match self {
            E::InvalidContentType(_) => String::from("received an invalid content type"),
            E::MaxFileLimit(_) => String::from("received maximum amount of files"),
            E::Multipart(err) => err_to_msg(err).to_string(),
            E::SemVer(_) => String::from("received an invalid semver value"),
            E::Io(_) => String::from("unexpected error that occurred, try again later"),
            e => format!("{e}"),
        }
    }

    fn details(&self) -> Option<Value> {
        use Error as E;

        match self {
            E::InvalidContentType(ct) => Some(json!({"contentType":ct})),
            E::MaxFileLimit(received) => Some(json!({"expected":2,"received":received})),
            E::Multipart(err) => expand_details_from_err(err),

            _ => None,
        }
    }
}

impl From<Error> for ApiResponse {
    fn from(value: Error) -> Self {
        if let Error::Multipart(err) = value {
            return err.into();
        }

        err(value.status(), (value.error_code(), value.message(), value.details()))
    }
}
