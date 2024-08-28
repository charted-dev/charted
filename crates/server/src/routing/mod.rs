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

use crate::ServerContext;
use axum::{
    body::Body,
    extract::DefaultBodyLimit,
    http::{Method, Response, StatusCode},
    response::IntoResponse,
    Router,
};
use charted_core::api;
use serde_json::json;
use std::{any::Any, borrow::Cow};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{self, CorsLayer},
};

pub mod v1;

charted_core::create_newtype_wrapper! {
    /// Newtype wrapper for [`utoipa::openapi::PathItem`] so it can be used
    /// with the [`inventory`] crate.
    pub PathItem for ::utoipa::openapi::PathItem;
}

inventory::collect!(PathItem);

macro_rules! mk_router {
    ($cx:ident, $($version:ident),*) => {{
        let mut router = ::axum::Router::new()
            .merge(v1::create_router(&$cx));

        $(
            router = router
                .clone()
                .nest(concat!("/", stringify!($version)), $crate::routing::$version::create_router(&$cx));
        )*

        router
    }};
}

fn panic_handler(message: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = azalia::message_from_panic(message);
    tracing::error!(%details, "http server has panicked");

    api::err(StatusCode::INTERNAL_SERVER_ERROR, api::Error {
        code: api::ErrorCode::InternalServerError,
        message: Cow::Borrowed("unable to process this request at this time. if this keeps occurring, report this to Noelware via GitHub issues!"),
        details: Some(json!({
            "report_url": "https://github.com/charted-dev/charted/issues/new"
        }))
    }).into_response()
}

pub fn create_router(cx: &ServerContext) -> Router<ServerContext> {
    let stack = ServiceBuilder::new()
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction())
        .layer(tower_http::catch_panic::CatchPanicLayer::custom(panic_handler))
        .layer(CompressionLayer::new().gzip(true))
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::PUT,
                    Method::HEAD,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(cors::Any),
        )
        .layer(axum::middleware::from_fn(crate::middleware::request_id))
        .layer(axum::middleware::from_fn(crate::middleware::log));

    Router::new().merge(mk_router!(cx, v1)).layer(stack)
}
