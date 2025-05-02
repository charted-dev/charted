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

#![cfg(feature = "yaml")]

use serde::Serialize;

/// Response datatype for HTTP that transmits [YAML](https://yaml.org).
#[derive(Debug, Clone, Default)]
pub struct Yaml<T> {
    #[cfg(feature = "axum")]
    status: axum::http::StatusCode,
    data: T,
}

impl<T> Yaml<T> {
    /// Creates a new [`Yaml`] object.
    #[cfg(feature = "axum")]
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
    pub const fn new(status: axum::http::StatusCode, data: T) -> Self {
        Yaml { status, data }
    }

    /// Creates a new [`Yaml`] object.
    #[cfg(not(feature = "axum"))]
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(not(feature = "axum"))))]
    pub const fn new(data: T) -> Self {
        Yaml { data }
    }

    /// Returns the [`StatusCode`].
    #[cfg(feature = "axum")]
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
    pub const fn status_code(&self) -> axum::http::StatusCode {
        self.status
    }

    /// Returns the `data`.
    pub const fn data(&self) -> &T {
        &self.data
    }

    /// Replace the status code
    #[cfg(feature = "axum")]
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
    pub fn with_status<S: Into<axum::http::StatusCode>>(mut self, status: S) -> Self {
        self.status = status.into();
        self
    }

    /// Replace the data that will be transmitted.
    pub fn with_data(mut self, data: T) -> Self {
        self.data = data.into();
        self
    }
}

#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
impl<T: Serialize> axum::response::IntoResponse for Yaml<T> {
    fn into_response(self) -> axum::response::Response {
        let serialized = serde_yaml_ng::to_string(&self.data).unwrap();
        axum::http::Response::builder()
            .status(self.status)
            .header(axum::http::header::CONTENT_TYPE, "text/yaml; charset=utf-8")
            .body(axum::body::Body::from(serialized))
            .unwrap()
    }
}
