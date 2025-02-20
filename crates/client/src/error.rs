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

use charted_core::api;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

/// Representation of any error that could've occurred in this crate.
#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
#[non_exhaustive]
pub enum Error {
    #[display("api server had failed with status code {}: {:#?}", _0.status, _0.errors)]
    Api(#[error(not(source))] api::Response),

    /// URL given failed to parse.
    ParseUrl(url::ParseError),

    /// Something related to [`reqwest`] failed.
    Reqwest(reqwest::Error),

    /// json encoded body failed to deserialize.
    Json(serde_json::Error),

    /// I/o error that occurred.
    Io(io::Error),
}
