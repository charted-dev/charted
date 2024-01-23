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
    body::Body,
    extract::DefaultBodyLimit,
    http::{header, Method, Response, StatusCode},
    Router,
};
use serde_json::json;
use std::any::Any;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::{
    server::models::res::{err, ErrorCode},
    Instance,
};

pub mod v1;

macro_rules! create_internal_router {
    ($($cr:ident),*) => {
        fn create_internal_router(instance: &$crate::Instance) -> ::axum::Router<crate::Instance> {
            let mut router = ::axum::Router::new().merge(v1::create_router(instance));
            $(
                router = router.clone().nest(concat!("/", stringify!($cr)), crate::server::routing::$cr::create_router(instance));
            )*

            router
        }
    };
}

create_internal_router!(v1);

fn panic_handler(message: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details: String;
    if let Some(msg) = message.downcast_ref::<String>() {
        details = msg.clone();
    } else if let Some(msg) = message.downcast_ref::<&str>() {
        details = msg.to_string();
    } else {
        details = "unable to downcast message".to_string();
    }

    error!(%details, "http service has panic'd");
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(
            serde_json::to_string(&err(
                StatusCode::INTERNAL_SERVER_ERROR,
                (
                    ErrorCode::InternalServerError,
                    "unable to process request at this time! if this keeps happening, please report it to Noelware",
                    json!({
                        "new_issue": "https://github.com/charted-dev/charted/issues/new"
                    }),
                ),
            ))
            .unwrap(),
        ))
        .unwrap()
}

pub fn create_router(instance: &Instance) -> Router<Instance> {
    let stack = ServiceBuilder::new()
        // .layer(sentry_tower::NewSentryLayer::new_from_top())
        // .layer(sentry_tower::SentryHttpLayer::new())
        .layer(tower_http::catch_panic::CatchPanicLayer::custom(panic_handler))
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::PUT, Method::HEAD, Method::POST, Method::PATCH, Method::DELETE])
                .allow_origin(tower_http::cors::Any)
        )
        .layer(axum::middleware::from_fn(crate::server::middleware::request_id))
        .layer(axum::middleware::from_fn(crate::server::middleware::log));

    Router::new().merge(create_internal_router(instance)).layer(stack)
}
