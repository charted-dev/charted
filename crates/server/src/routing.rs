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
//
//! Routing module.

use crate::Env;
use axum::{
    Extension, Router,
    body::Body,
    extract::DefaultBodyLimit,
    http::{Method, Request, Response, StatusCode},
    response::IntoResponse,
};
use charted_core::api;
use metrics_exporter_prometheus::PrometheusHandle;
use serde_json::json;
use std::{any::Any, borrow::Cow};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

pub mod v1;

macro_rules! mk_router(
    ($cx:ident, $default:ident $(,)? $($version:ident),*) => {{
        #[allow(unused_mut)]
        let mut router = ::axum::Router::new()
            .merge($crate::routing::$default::create_router($cx))
            .nest(
                concat!("/", stringify!($default)),
                $crate::routing::$default::create_router($cx)
            );

        $(
            router = router.nest(
                concat!("/", stringify!($version)),
                $crate::routing::$version::create_router($cx)
            );
        )*

        router
    }};
);

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub(crate) async fn prometheus_scrape(
    Extension(handle): Extension<Option<PrometheusHandle>>,
) -> impl IntoResponse {
    let Some(handle) = handle else {
        unreachable!()
    };

    handle.render()
}

fn panic_handler(message: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = azalia::message_from_panic(message);
    error!(%details, "HTTP service has panicked");

    api::err(StatusCode::INTERNAL_SERVER_ERROR, api::Error {
        code: api::ErrorCode::InternalServerError,
        message: Cow::Borrowed("unable to process this request at this time. if this keeps occurring, report this to Noelware via GitHub issues!"),
        details: Some(json!({
            "report_url": concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new?labels=rust"),
        }))
    }).into_response()
}

async fn four_oh_four_not_found(req: Request<Body>) -> api::Response {
    api::err(
        StatusCode::NOT_FOUND,
        (
            api::ErrorCode::RestEndpointNotFound,
            "rest endpoint was not found",
            json!({
                "method": req.method().as_str(),
                "uri": req.uri().path(),
            }),
        ),
    )
}

async fn four_oh_five_method_not_allowed(req: Request<Body>) -> api::Response {
    api::err(
        StatusCode::METHOD_NOT_ALLOWED,
        (
            api::ErrorCode::InvalidHttpMethod,
            "HTTP method allowed for this rest endpoint don't correlate with the one you sent",
            json!({
                "method": req.method().as_str(),
                "uri": req.uri().path(),
            }),
        ),
    )
}

// TODO(@auguwu): customise this with `server.max_body_size`?
const MAX_BODY_LIMIT: usize = 100 * 1024 * 1024;

pub fn create_router(env: &Env) -> Router<Env> {
    mk_router!(env, v1)
        .layer(
            ServiceBuilder::new()
                .layer(sentry_tower::NewSentryLayer::new_from_top())
                .layer(sentry_tower::SentryHttpLayer::new().enable_transaction())
                .layer(tower_http::catch_panic::CatchPanicLayer::custom(panic_handler))
                .layer(CompressionLayer::new().gzip(true))
                .layer(DefaultBodyLimit::max(MAX_BODY_LIMIT))
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
                .layer(axum::middleware::from_fn(crate::middleware::request_id))
                .layer(axum::middleware::from_fn_with_state(
                    env.clone(),
                    crate::middleware::log,
                )),
        )
        .fallback(four_oh_four_not_found)
        .method_not_allowed_fallback(four_oh_five_method_not_allowed)
}
