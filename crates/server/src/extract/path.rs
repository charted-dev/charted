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
    extract::{path::ErrorKind, rejection::PathRejection, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use charted_core::api;
use serde::de::DeserializeOwned;
use serde_json::json;

charted_core::create_newtype_wrapper! {
    /// `Path` is a newtype wrapper for [`axum::extract::Path`], in which it uses
    /// [`api::Response`] to transmit errors properly.
    pub Path<T> for pub T;
}

impl<S: Send + Sync, T: DeserializeOwned + Send + Sync> FromRequestParts<S> for Path<T> {
    type Rejection = api::Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        ::axum::extract::Path::<T>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(x)| Self(x))
            .map_err(|e| {
                tracing::error!(error = %e, "failed to parse path parameter");
                sentry::capture_error(&e);

                match e {
                    PathRejection::FailedToDeserializePathParams(e) => {
                        let mut status = StatusCode::BAD_REQUEST;
                        let error = match e.into_kind() {
                            ErrorKind::WrongNumberOfParameters { .. } => {
                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                (
                                    api::ErrorCode::WrongParameters,
                                    "received wrong list of path parameters -- this is our fault; please report this on GitHub",
                                    None
                                )
                            }

                            ErrorKind::ParseErrorAtKey { key, expected_type, .. } => (
                                api::ErrorCode::ParsingFailedInPathParam,
                                "failed to parse path parameter into expected type",
                                Some(json!({
                                    "key": key,
                                    "expected_type": expected_type,
                                }))
                            ),

                            ErrorKind::ParseErrorAtIndex { index, expected_type, .. } => (
                                api::ErrorCode::ParsingFailedInPathParam,
                                "failed to parse path parameter at index into expected type",
                                Some(
                                    json!({
                                        "index": index,
                                        "expected_type": expected_type,
                                    })
                                )
                            ),

                            ErrorKind::ParseError { expected_type, .. } => (
                                api::ErrorCode::ParsingFailedInPathParam,
                                "failed to parse path parameter",
                                Some(json!({
                                    "expected_type": expected_type,
                                }))
                            ),

                            ErrorKind::InvalidUtf8InPathParam { key } => (
                                api::ErrorCode::InvalidUtf8,
                                "invalid utf-8 given for path parameter",
                                Some(json!({
                                    "key": key,
                                }))
                            ),

                            ErrorKind::UnsupportedType { name } => {
                                tracing::warn!(%name, "failed to serialize into supported type; mainly caused by nested maps");
                                return api::internal_server_error();
                            }

                            ErrorKind::Message(msg) => {
                                tracing::warn!(msg);
                                return api::internal_server_error();
                            },

                            _ => unreachable!()
                        };

                        api::err(status, error)
                    }

                    _ => api::internal_server_error(),
                }
            })
    }
}
