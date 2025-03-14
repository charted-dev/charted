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

use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::IntoResponse,
};
use charted_core::rand_string;
use std::{fmt::Display, ops::Deref};

/// Represents the generated `x-request-id` header that the server creates on each
/// request invocation.
#[derive(Debug, Clone)]
pub struct XRequestId(String);

impl XRequestId {
    /// Generates a new [`XRequestId`].
    pub(self) fn generate() -> XRequestId {
        XRequestId(rand_string(12))
    }
}

impl Display for XRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for XRequestId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<XRequestId> for HeaderValue {
    fn from(value: XRequestId) -> HeaderValue {
        // we know that it'll always be valid UTF-8
        HeaderValue::from_str(&value).unwrap()
    }
}

#[cfg_attr(debug_assertions, axum::debug_middleware)]
pub async fn request_id(mut req: Request<Body>, next: Next) -> impl IntoResponse {
    let id = XRequestId::generate();
    req.extensions_mut().insert(id.clone());

    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", id.into());
    headers.insert(
        "server",
        HeaderValue::from_str(
            format!(
                "Noelware/charted-server (+https://github.com/charted-dev/charted; {})",
                charted_core::version()
            )
            .as_str(),
        )
        .unwrap(),
    );

    (headers, next.run(req).await)
}
