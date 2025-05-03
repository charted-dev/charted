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
    error::Error,
    tests::{consume_body, testcase},
};
use axum::{
    body::Body,
    http::{Request, header::AUTHORIZATION},
};
use base64::engine::{Engine, general_purpose::STANDARD};
use charted_core::{api, assert_response_is_client_error, assert_status_code};
use charted_types::name;
use std::borrow::Cow;
use tower::Service;

fn base64_encode(b: &[u8]) -> String {
    let mut auth = String::new();
    STANDARD.encode_string(b, &mut auth);

    auth
}

testcase! {
    disallow_if_not_enabled_by_config(service) {
        let res = service
            .call(
                Request::post("/echo")
                    .header(AUTHORIZATION, "Basic bm9lbDpub2VsaXNjdXRpZXV3dQo=")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_response_is_client_error!(res);
        assert_status_code!(res, PRECONDITION_FAILED);

        let body = res.into_body();
        let res = consume_body!(body as api::Response);

        assert!(!res.success);
        assert_eq!(res.errors[0].code, api::ErrorCode::UnsupportedAuthorizationKind);
        assert_eq!(res.errors[0].message, "instance has disabled the use of `Basic` authentication");
    };
}

testcase! {
    [env_override(|env| {
        env.config.sessions.enable_basic_auth = true;
    })]
    decoding_error_missing_colon(service) {
        let basic_auth = base64_encode(b"wwwww");
        let res = service
            .call(
                Request::post("/echo")
                    .header(AUTHORIZATION, format!("Basic {basic_auth}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_response_is_client_error!(res);
        assert_status_code!(res, NOT_ACCEPTABLE);

        let body = res.into_body();
        let res = consume_body!(body as api::Response);

        let expected = Error::Message {
            message: Cow::Borrowed("input must be in the form of 'username:password'"),
            code: None,
        };

        assert_eq!(res, expected.into());
    };
}

testcase! {
    [env_override(|env| {
        env.config.sessions.enable_basic_auth = true;
    })]
    decoding_error_more_than_one_colon(service) {
        let basic_auth = base64_encode(b"wwwww:wwwww:::::::.;?!");
        let res = service
            .call(
                Request::post("/echo")
                    .header(AUTHORIZATION, format!("Basic {basic_auth}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_response_is_client_error!(res);
        assert_status_code!(res, NOT_ACCEPTABLE);

        let body = res.into_body();
        let res = consume_body!(body as api::Response);

        let expected = Error::Message {
            message: Cow::Borrowed("received more than one `:` in basic auth input"),
            code: None,
        };

        assert_eq!(res, expected.into());
    };
}

testcase! {
    [env_override(|env| {
        env.config.sessions.enable_basic_auth = true;
    })]
    invalid_name_in_username(service) {
        let basic_auth = base64_encode(b"noelisTHEbEST!!!!~:owouwudaowo");
        let res = service
            .call(
                Request::post("/echo")
                    .header(AUTHORIZATION, format!("Basic {basic_auth}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_response_is_client_error!(res);
        assert_status_code!(res, NOT_ACCEPTABLE);

        let body = res.into_body();
        let res = consume_body!(body as api::Response);

        let expected = Error::InvalidName {
            input: Cow::Borrowed("noelisTHEbEST!!!!~"),
            error: name::Error::InvalidCharacter {
                input: Cow::Borrowed("noelisthebest!!!!~"),
                at: 13,
                ch: '!'
            }
        };

        assert_eq!(res, expected.into());
    };
}

testcase! {
    [env_override(|env| {
        env.config.sessions.enable_basic_auth = true;
    })]
    empty_username(service) {
        let basic_auth = base64_encode(b":owouwudaowo");
        let res = service
            .call(
                Request::post("/echo")
                    .header(AUTHORIZATION, format!("Basic {basic_auth}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_response_is_client_error!(res);
        assert_status_code!(res, NOT_ACCEPTABLE);

        let body = res.into_body();
        let res = consume_body!(body as api::Response);

        let expected = Error::InvalidName {
            input: Cow::Borrowed(""),
            error: name::Error::Empty,
        };

        assert_eq!(res, expected.into());
    };
}
