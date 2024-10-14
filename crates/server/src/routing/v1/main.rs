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
use charted_core::{api, VERSION};
use charted_proc_macros::generate_api_response;
use serde::Serialize;
use utoipa::ToSchema;

/// Response object for the `GET /` REST controller.
#[derive(Serialize, ToSchema)]
pub struct MainResponse {
    /// The message, which will always be "Hello, world!"
    message: String,

    /// You know, for Helm charts?
    tagline: String,

    /// Documentation URL for this generic entrypoint response.
    docs: String,
}

impl Default for MainResponse {
    fn default() -> Self {
        MainResponse {
            message: "Hello, world! 👋".into(),
            tagline: "You know, for Helm charts?".into(),
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}"),
        }
    }
}

generate_api_response!(MainResponse);

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn main() -> api::Response<MainResponse> {
    api::from_default(StatusCode::OK)
}
