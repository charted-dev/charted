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
    http::{header::CONTENT_TYPE, Method, Response, StatusCode},
    middleware, Router,
};
use charted_core::response::{err, ErrorCode};
use charted_proc_macros::add_paths;
use serde_json::json;
use std::any::Any;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic,
    cors::{self, CorsLayer},
};
use tracing::error;
use utoipa::openapi::Paths;

pub mod v1;

macro_rules! create_router {
    ($ctx:ident; $($version:ident),*) => {{
        let mut router = ::axum::Router::new();
        router = router.clone().merge(v1::create_router($ctx));

        $(
            router = router
                .clone()
                .nest(concat!("/", stringify!($version)), $crate::routing::$version::create_router($ctx));
        )*

        router
    }};
}

fn panic_handler(message: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = azalia::message_from_panic(message);
    error!(error = %details, "received panic when going through a request");

    let body = serde_json::to_string(&err(
        StatusCode::INTERNAL_SERVER_ERROR,
        (
            ErrorCode::InternalServerError,
            "unable to process request at this time! if this keeps happening, please report it to Noelware",
            json!({
                "new_issue": "https://github.com/charted-dev/charted/issues/new"
            }),
        ),
    ))
    .expect("failed to serialize into JSON");

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(body))
        .expect("failed to construct a `Response`")
}

pub fn create_router(ctx: &ServerContext) -> Router<ServerContext> {
    let router = create_router!(ctx; v1);
    let stack = ServiceBuilder::new()
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction())
        .layer(catch_panic::CatchPanicLayer::custom(panic_handler))
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
        .layer(middleware::from_fn(crate::middleware::request_id));

    router.layer(stack)
}

/// Returns a [`Paths`] object of the current API version's paths that are avaliable.
pub fn paths() -> Paths {
    add_paths!(
        // ~~~~~~~~~~~      MAIN     ~~~~~~~~~~~~~~~~~~~
        "/index/{idOrName}" => v1::index::GetChartIndexRestController::paths();
        "/heartbeat" => v1::heartbeat::HeartbeatRestController::paths();
        "/features" => v1::features::FeaturesRestController::paths();
        "/info" => v1::info::InfoRestController::paths();
        "/" => v1::main::MainRestController::paths();
    )
}
