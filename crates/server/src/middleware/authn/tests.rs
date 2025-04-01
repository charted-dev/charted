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

mod apikey;
mod basic;
mod bearer;

use super::{Error, Options};
use crate::Context;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing,
};
use charted_config::Config;
use charted_core::api;
use serde::de::DeserializeOwned;
use tower::{Service, ServiceExt};

pub(in crate::middleware::authn::tests) async fn echo(req: axum::extract::Request) -> impl IntoResponse {
    (StatusCode::OK, Response::new(req.into_body()))
}

pub(in crate::middleware::authn::tests) async fn consume_body<T: DeserializeOwned>(body: Body) -> T {
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

pub(in crate::middleware::authn::tests) async fn create_context(
    config_override: impl FnOnce(&mut Config),
) -> Context {
    Context::for_testing(config_override).await.unwrap()
}

pub(in crate::middleware::authn::tests) async fn create_router(
    options: Options,
    basic_auth: bool,
    config_override: impl FnOnce(&mut Config),
) -> Router {
    let context = create_context(|cfg| {
        cfg.sessions.enable_basic_auth = basic_auth;
        config_override(cfg)
    })
    .await;

    Router::new()
        .route("/echo", routing::post(echo).layer(super::new(context.clone(), options)))
        .with_state(context)
}

#[tokio::test]
async fn disallow_if_no_header() {
    let mut router = create_router(Default::default(), false, |_| {}).await;
    let mut service = router.as_service::<Body>();

    let service = service.ready().await.unwrap();
    let res = service
        .call(Request::post("/echo").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_ACCEPTABLE);

    let body = consume_body::<api::Response>(res.into_body()).await;
    assert_eq!(body, Error::MissingAuthorizationHeader.into());
}

#[tokio::test]
async fn allow_if_no_header_and_can_be_allowed() {
    let mut router = create_router(
        Options {
            allow_unauthorized: true,
            ..Default::default()
        },
        false,
        |_| {},
    )
    .await;

    let mut service = router.as_service::<Body>();
    let service = service.ready().await.unwrap();

    let res = service
        .call(Request::post("/echo").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(
        axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap()
            .is_empty()
    );
}
