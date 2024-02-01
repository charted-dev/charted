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

use crate::server::{models::yaml::Yaml, openapi::Document};
use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa::{openapi::OpenApi, OpenApi as _};

pub async fn json() -> impl IntoResponse {
    let document = Document::openapi();
    (StatusCode::OK, Json(document))
}

pub async fn yaml() -> Yaml<OpenApi> {
    let document = Document::openapi();
    Yaml(StatusCode::OK, document)
}
