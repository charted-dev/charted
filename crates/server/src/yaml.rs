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

//! The `charted_server::responses` module contains custom response types that don't
//! conform to the usual [api responses][charted_core::api::Response].

use axum::{
    body::Body,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use serde::Serialize;

/// Axum response datatype that transmits [YAML](https://yaml.org).
#[derive(Debug, Clone)]
pub struct Yaml<T> {
    status: StatusCode,
    inner: T,
}

impl<T> From<(StatusCode, T)> for Yaml<T> {
    fn from((status, inner): (StatusCode, T)) -> Self {
        Yaml { status, inner }
    }
}

impl<T: Serialize> IntoResponse for Yaml<T> {
    fn into_response(self) -> axum::response::Response {
        let serialized = serde_yaml_ng::to_string(&self.inner).unwrap();
        Response::builder()
            .status(self.status)
            .header(header::CONTENT_TYPE, "application/yaml; charset=utf-8")
            .body(Body::from(serialized))
            .unwrap()
    }
}
