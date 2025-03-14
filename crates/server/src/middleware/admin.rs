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

use crate::Context;
use axum::{
    Extension,
    body::Body,
    extract::State,
    http::{Request, Response},
    middleware::Next,
};
use charted_core::api;
use charted_types::Session;

#[cfg_attr(debug_assertions, axum::debug_middleware)]
#[allow(unused)]
pub async fn admin_guard(
    State(cx): State<Context>,
    Extension(session): Extension<Option<Session>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, api::Response> {
    todo!()
}
