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

use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Yaml<T>(StatusCode, T);

impl<T> Yaml<T> {
    pub fn ok(data: T) -> Yaml<T> {
        Yaml(StatusCode::OK, data)
    }
}

impl<T: Serialize> IntoResponse for Yaml<T> {
    fn into_response(self) -> axum::response::Response {
        let mut res = Response::new(serde_yaml::to_string(&self.1).unwrap());
        *res.status_mut() = self.0;
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/yaml; charset=utf-8"),
        );

        res.into_response()
    }
}

/// Creates a new [`Yaml`] object that can be used to send YAML-based data.
pub fn yaml<T>(status: StatusCode, data: T) -> Yaml<T> {
    Yaml(status, data)
}
