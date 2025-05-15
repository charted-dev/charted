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
pub(in crate::middleware::authn::tests) mod util;

use crate::middleware::authn::{Options, error::Error};
use axum::{body::Body, http::Request};
use charted_core::{api, assert_response_is_client_error, assert_response_ok, assert_status_code};
use tower::Service;
use util::{consume_body, testcase};

testcase! {
    disallow_if_no_header(service) {
        let res = service
            .call(Request::post("/echo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_response_is_client_error!(res, "expected server to return 406");
        assert_status_code!(res, NOT_ACCEPTABLE);

        let body = res.into_body();
        let body = consume_body!(body as api::Response);

        assert_eq!(body, Error::MissingAuthorizationHeader.into());
    };
}

testcase! {
    [options(Options {
        allow_unauthorized: true,
        ..Default::default()
    })]
    allow_if_no_header_and_can_be_unauthorized(service) {
        let res = service
            .call(Request::post("/echo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_response_ok!(res);
        assert!(
            axum::body::to_bytes(res.into_body(), usize::MAX)
                .await
                .unwrap()
                .is_empty()
        );
    };
}
