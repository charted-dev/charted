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
    http::{HeaderMap, HeaderName, HeaderValue, Request},
    middleware::Next,
    response::IntoResponse,
};
use charted_common::{rand_string, COMMIT_HASH, VERSION};

pub async fn request_id<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(rand_string(24).as_str()).unwrap(),
    );

    headers.insert(
        HeaderName::from_static("server"),
        HeaderValue::from_str(
            format!("Noelware/charted-server (+https://github.com/charted-dev/charted; v{VERSION}+{COMMIT_HASH})")
                .as_str(),
        )
        .unwrap(),
    );

    (headers, next.run(req).await)
}
