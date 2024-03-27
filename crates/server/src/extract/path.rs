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

use crate::{err, internal_server_error, ApiResponse, ErrorCode};
use async_trait::async_trait;
use axum::{
    extract::{path::ErrorKind, rejection::PathRejection, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::ops::Deref;
use tracing::{error, warn};

/// Represents a wrapper type of the [`axum::extract::Path`] extractor but reports rejections as [`ApiResponse`]s.
#[derive(Clone)]
pub struct Path<T>(pub T);

impl<T> Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send + Sync,
    S: Send + Sync,
{
    type Rejection = ApiResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Path(value)) => Ok(Self(value)),
            Err(e) => {
                error!(error = %e, "failed to parse path parameter");
                sentry::capture_error(&e);

                match e {
                    PathRejection::FailedToDeserializePathParams(e) => {
                        let mut status = StatusCode::BAD_REQUEST;
                        let kind = e.into_kind();
                        let error = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => (
                                ErrorCode::WrongParameters,
                                "received a wrong list of path parameters, report this issue immediately to Noelware",
                                None,
                            ),

                            ErrorKind::ParseErrorAtKey { key, .. } => (
                                ErrorCode::ParsingFailedInPathParam,
                                "failed to parse key into expected type wanted",
                                Some(json!({"key":key})),
                            ),

                            ErrorKind::ParseErrorAtIndex { index, .. } => (
                                ErrorCode::ParsingFailedInPathParam,
                                "failed to parse from index into expected type wanted",
                                Some(json!({"index":index})),
                            ),

                            ErrorKind::ParseError { .. } => (
                                ErrorCode::ParsingFailedInPathParam,
                                "failed to parse path parameter",
                                None,
                            ),

                            ErrorKind::InvalidUtf8InPathParam { key } => (
                                ErrorCode::InvalidUtf8,
                                "received invalid utf-8 in path parameter",
                                Some(json!({"key":key})),
                            ),

                            ErrorKind::UnsupportedType { name } => {
                                warn!(%name, "failed to serialize into supported type; this is mainly caused by nested maps");

                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                (ErrorCode::InternalServerError, "Internal Server Error", None)
                            }

                            // a catch-all variant; the message will be logged into the console
                            ErrorKind::Message(msg) => {
                                warn!(msg);

                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                (ErrorCode::InternalServerError, "Internal Server Error", None)
                            }

                            _ => unreachable!(),
                        };

                        Err(err(status, error))
                    }

                    _ => Err(internal_server_error()),
                }
            }
        }
    }
}
