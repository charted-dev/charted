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

use crate::{models::res::err, Server};
use axum::{
    body::Body,
    extract::DefaultBodyLimit,
    http::{header, Method, Response, StatusCode},
    Router,
};
use once_cell::sync::Lazy;
use serde_json::json;
use std::any::Any;
use tower::ServiceBuilder;

pub mod v1;

#[allow(non_upper_case_globals)]
static default_router: Lazy<Box<dyn Fn() -> Router<Server> + Send + Sync>> = Lazy::new(|| Box::new(v1::create_router));

macro_rules! create_router_internal {
    ($($cr:ident),*) => {
        fn create_router_internal() -> ::axum::Router<$crate::Server> {
            let mut router = ::axum::Router::new().merge(default_router());
            $(
                router = router.clone().nest(concat!("/", stringify!($cr)), $crate::routing::$cr::create_router());
            )*

            router
        }
    };
}

create_router_internal!(v1);

pub fn create_router() -> Router<Server> {
    let stack = ServiceBuilder::new()
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::new())
        .layer(tower_http::catch_panic::CatchPanicLayer::custom(catch_panic))
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::PUT,
                    Method::HEAD,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(tower_http::cors::Any),
        )
        .layer(axum::middleware::from_fn(crate::middleware::log))
        .layer(axum::middleware::from_fn(crate::middleware::request_id));

    Router::new().merge(create_router_internal()).layer(stack)
}

fn catch_panic(error: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = charted_common::panic_message(error);

    error!(%details, "received panic when executing rest handler");
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(
            serde_json::to_string(&err(
                StatusCode::INTERNAL_SERVER_ERROR,
                (
                    "INTERNAL_SERVER_ERROR",
                    "Unable to process your request. Please try again later or report this to Noelware via GitHub",
                    json!({
                        "new_issue_uri": "https://github.com/charted-dev/charted/issues/new"
                    }),
                )
                    .into(),
            ))
            .unwrap(),
        ))
        .unwrap()
}
