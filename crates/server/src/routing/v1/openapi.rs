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

use crate::openapi::Document;
use std::sync::LazyLock;
use utoipa::OpenApi;

// The only reason this is in a `LazyLock` and not used as `Document::openapi` is because
// it can prevent unnecessary allocations and computations in the `Modify` implementation
// for it (since it needs to pre-generate routes for the default API revision) and I would prefer
// it is done once rather than every request.
static CACHED: LazyLock<utoipa::openapi::OpenApi> = LazyLock::new(Document::openapi);

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn openapi() -> String {
    serde_json::to_string(&*CACHED).expect("it should be serialized to a JSON value")
}
