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

// use crate::{metrics::REQUEST_LATENCY_HISTOGRAM, Instance};
use crate::middleware::XRequestId;
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{header::USER_AGENT, Extensions, HeaderMap, Method, Request, Uri, Version},
    middleware::Next,
    response::IntoResponse,
};
use std::time::Instant;
use tracing::{info, info_span};

#[derive(FromRequestParts)]
pub struct Metadata {
    extensions: Extensions,
    version: Version,
    headers: HeaderMap,
    method: Method,
    uri: Uri,
}

pub async fn log(metadata: Metadata, req: Request<Body>, next: Next) -> impl IntoResponse {
    // let histogram = &*REQUEST_LATENCY_HISTOGRAM;
    // instance.requests.fetch_add(1, Ordering::SeqCst);

    let uri = metadata.uri.path();
    if uri.contains("/heartbeat") {
        return next.run(req).await;
    }

    let start = Instant::now();
    let method = metadata.method.as_str();
    let version = match metadata.version {
        Version::HTTP_09 => "http/0.9",
        Version::HTTP_10 => "http/1.0",
        Version::HTTP_11 => "http/1.1",
        Version::HTTP_2 => "http/2.0",
        Version::HTTP_3 => "http/3.0",
        _ => unimplemented!(),
    };

    let id = metadata.extensions.get::<XRequestId>().unwrap();
    let user_agent = metadata
        .headers
        .get(USER_AGENT)
        .map(|f| String::from_utf8_lossy(f.as_bytes()).to_string());

    let http_span = info_span!(
        "charted.http.request",
        req.ua = user_agent,
        req.id = %id,
        http.uri = uri,
        http.method = method,
        http.version = version
    );

    let _guard = http_span.enter();
    info!(
        http.uri = uri,
        http.method = method,
        http.version = version,
        req.id = %id,
        req.ua = user_agent,
        "processing request"
    );

    let res = next.run(req).await;
    let now = start.elapsed();
    // histogram.observe(now.as_secs_f64());

    info!(
        http.uri = uri,
        http.method = method,
        http.version = version,
        req.ua = user_agent,
        response.status = res.status().as_u16(),
        latency = ?now,
        req.id = %id,
        "processed request successfully"
    );

    res
}
