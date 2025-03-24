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

use crate::middleware::authn::{
    Error,
    tests::{consume_body, create_router},
};
use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use charted_core::api::{self, ErrorCode};
use charted_types::name;
use std::borrow::Cow;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn disallow_if_not_enabled_by_config() {
    let mut router = create_router(Default::default(), false, |_| {}).await;
    let mut service = router.as_service::<Body>();

    let service = service.ready().await.unwrap();
    let res = service
        .call(
            Request::post("/echo")
                .header(AUTHORIZATION, "Basic bm9lbDpub2VsaXNjdXRpZXV3dQo=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body = consume_body::<api::Response>(res.into_body()).await;
    assert!(!body.errors.is_empty());
    assert_eq!(body.errors[0].code, ErrorCode::BadRequest);
    assert_eq!(
        body.errors[0].message,
        "instance has disabled the use of `Basic` authentication"
    );
}

#[tokio::test]
async fn decoding_error_missing_colon() {
    let mut router = create_router(Default::default(), true, |_| {}).await;
    let mut service = router.as_service::<Body>();

    let service = service.ready().await.unwrap();

    let mut auth = String::new();
    STANDARD.encode_string(b"wwwww", &mut auth);

    let res = service
        .call(
            Request::post("/echo")
                .header(AUTHORIZATION, format!("Basic {auth}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_ACCEPTABLE);

    let body = consume_body::<api::Response>(res.into_body()).await;
    assert_eq!(
        body,
        Error::Message {
            message: Cow::Borrowed("input must be in the form of 'username:password'"),
            code: None
        }
        .into()
    );
}

#[tokio::test]
async fn decoding_error_more_than_one_colon() {
    let mut router = create_router(Default::default(), true, |_| {}).await;
    let mut service = router.as_service::<Body>();

    let service = service.ready().await.unwrap();

    let mut auth = String::new();
    STANDARD.encode_string(b"wwwww:wwwww:::::::.;?!", &mut auth);

    let res = service
        .call(
            Request::post("/echo")
                .header(AUTHORIZATION, format!("Basic {auth}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_ACCEPTABLE);

    let body = consume_body::<api::Response>(res.into_body()).await;
    assert_eq!(
        body,
        Error::Message {
            message: Cow::Borrowed("received more than one `:` in basic auth input"),
            code: None
        }
        .into()
    );
}

#[tokio::test]
async fn invalid_name_in_username() {
    let mut router = create_router(Default::default(), true, |_| {}).await;
    let mut service = router.as_service::<Body>();

    let service = service.ready().await.unwrap();

    let mut auth = String::new();
    STANDARD.encode_string(b"noelisTHEbEST!!!!~:owouwudaowo", &mut auth);

    let res = service
        .call(
            Request::post("/echo")
                .header(AUTHORIZATION, format!("Basic {auth}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_ACCEPTABLE);

    let body = consume_body::<api::Response>(res.into_body()).await;
    assert_eq!(
        body,
        Error::InvalidName {
            input: Cow::Borrowed("noelisTHEbEST!!!!~"),
            error: name::Error::InvalidCharacter {
                input: Cow::Borrowed("noelisthebest!!!!~"),
                at: 13,
                ch: '!'
            }
        }
        .into()
    );
}

#[tokio::test]
async fn empty_username() {
    let mut router = create_router(Default::default(), true, |_| {}).await;
    let mut service = router.as_service::<Body>();

    let service = service.ready().await.unwrap();

    let mut auth = String::new();
    STANDARD.encode_string(b":owouwudaowo", &mut auth);

    let res = service
        .call(
            Request::post("/echo")
                .header(AUTHORIZATION, format!("Basic {auth}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_ACCEPTABLE);

    let body = consume_body::<api::Response>(res.into_body()).await;
    assert_eq!(
        body,
        Error::InvalidName {
            input: Cow::Borrowed(""),
            error: name::Error::Empty,
        }
        .into()
    );
}
