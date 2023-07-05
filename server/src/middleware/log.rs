// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use std::time::Instant;

use axum::{
    http::{Method, Request, Uri, Version},
    middleware::Next,
    response::IntoResponse,
};

#[derive(axum::extract::FromRequestParts)]
pub struct Metadata {
    pub(crate) uri: Uri,
    pub(crate) method: Method,
    pub(crate) version: Version,
}

pub async fn log<B>(metadata: Metadata, req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let uri = metadata.uri.path();
    let method = metadata.method.as_str();
    let version = match metadata.version {
        Version::HTTP_09 => "http/0.9",
        Version::HTTP_10 => "http/1.0",
        Version::HTTP_11 => "http/1.1",
        Version::HTTP_2 => "http/2.0",
        Version::HTTP_3 => "http/3.0",
        _ => "http/???",
    };

    if !uri.contains("/heartbeat") {
        info!(
            %uri,
            %method,
            %version,
            "processing request"
        );
    }

    let res = next.run(req).await;
    let now = start.elapsed();

    if !uri.contains("/heartbeat") {
        let status = res.status();
        info!(
            %uri,
            %method,
            %version,
            "processed request -> {} [{:?}]",
            status,
            now
        );
    }

    res
}
