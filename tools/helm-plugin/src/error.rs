// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

use reqwest::StatusCode;

use crate::api::response::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to determine instance URL host")]
    UnableToDetermineURLHost,

    #[error("This feature is currently not supported in the Helm Plugin ({issue_url:?})")]
    FeatureNotSupported { issue_url: Option<String> },

    #[error("Received status code [{status}]:\n{body}")]
    HttpRequest { status: StatusCode, body: String },

    #[error("Unknown error: {0:?}")]
    Unknown(#[from] Box<dyn std::error::Error>),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON deserialization error [{0}]")]
    JsonDeserialization(#[from] serde_json::Error),

    #[error("API Server sent errors: {errors:?}")]
    ApiError { errors: Vec<ApiError> },
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}
