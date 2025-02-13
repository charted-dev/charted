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

use super::XRequestId;
use crate::Context;
use axum::{
    body::Body,
    extract::{FromRequestParts, MatchedPath, Request, State},
    http::{header::USER_AGENT, Extensions, HeaderMap, Method, Uri, Version},
    middleware::Next,
    response::IntoResponse,
};
use std::{fmt::Display, sync::atomic::Ordering, time::Instant};
use tracing::{info, instrument};

#[derive(FromRequestParts)]
pub struct Metadata {
    extensions: Extensions,
    version: Version,
    headers: HeaderMap,
    matched: Option<MatchedPath>,
    method: Method,
    uri: Uri,
}

#[instrument(name = "charted.http.request", skip_all, fields(
    req.matched_path = %display_opt(metadata.matched.as_ref().map(MatchedPath::as_str)),
    req.ua = %display_opt(get_user_agent(&metadata)),
    req.id = %metadata.extensions.get::<XRequestId>().unwrap(),
    http.version = http_version(&metadata),
    http.method = metadata.method.as_str(),
    http.uri = metadata.uri.path(),
))]
pub async fn log(metadata: Metadata, State(ctx): State<Context>, req: Request<Body>, next: Next) -> impl IntoResponse {
    let uri = metadata.uri.path();
    if uri.contains("/heartbeat") {
        return next.run(req).await;
    }

    ctx.requests.fetch_add(1, Ordering::SeqCst);

    let start = Instant::now();
    info!("processing request");

    let res = next.run(req).await;
    let elapsed: charted_core::serde::Duration = start.elapsed().into();

    info!(latency = %elapsed, "processed request");
    res
}

fn http_version(Metadata { version, .. }: &Metadata) -> &'static str {
    match *version {
        Version::HTTP_09 => "http/0.9",
        Version::HTTP_10 => "http/1.0",
        Version::HTTP_11 => "http/1.1",
        Version::HTTP_2 => "http/2.0",
        Version::HTTP_3 => "http/3.0",
        _ => unimplemented!(),
    }
}

fn get_user_agent(Metadata { headers, .. }: &Metadata) -> Option<String> {
    headers
        .get(USER_AGENT)
        .map(|f| String::from_utf8_lossy(f.as_bytes()).to_string())
}

fn display_opt<T: Display>(opt: Option<T>) -> impl Display {
    struct Helper<T: Display>(Option<T>);
    impl<T: Display> Display for Helper<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0 {
                Some(ref display) => Display::fmt(display, f),
                None => f.write_str("<unknown>"),
            }
        }
    }

    Helper(opt)
}
