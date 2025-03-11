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

use crate::{Context, openapi::Document};
use axum::extract::State;
use std::sync::OnceLock;
use utoipa::{OpenApi as _, openapi::OpenApi};

// We stash it in a OnceLock since we iterate through the server features
// and to compute all features for each request is kinda dirty.
static CACHED: OnceLock<OpenApi> = OnceLock::new();

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get(State(cx): State<Context>) -> String {
    let document = CACHED.get_or_init(|| {
        let mut doc = Document::openapi();
        for feat in cx.features.values() {
            feat.extends_openapi(&mut doc);
        }

        doc
    });

    document.to_pretty_json().expect("to be serialized correctly")
}
