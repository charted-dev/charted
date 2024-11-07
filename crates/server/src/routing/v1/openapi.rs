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

#![allow(clippy::incompatible_msrv)]

use crate::{openapi::Document, ServerContext};
use axum::extract::State;
use std::sync::OnceLock;
use utoipa::OpenApi;

// This is wrapped in a `OnceLock` and initialized on the first request is due to
// that any feature can extend the OpenAPI document to document routes when the
// feature is enabled.
static CACHED: OnceLock<utoipa::openapi::OpenApi> = OnceLock::new();

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn openapi(State(cx): State<ServerContext>) -> String {
    let document = CACHED.get_or_init(|| {
        let cx = cx.clone();
        let mut doc = Document::openapi();

        for feature in &cx.features {
            feature.extends_openapi(&mut doc);
        }

        doc
    });

    serde_json::to_string_pretty(document).expect("serialize")
}
