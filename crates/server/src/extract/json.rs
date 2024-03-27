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

use crate::{err, ApiResponse, ErrorCode};
use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::{header, StatusCode},
};
use serde::de::DeserializeOwned;
use serde_json::{error::Category, json};
use std::ops::Deref;
use tracing::error;

/// Wrapper extractor for the [`axum::extract::Json`] extractor that uses charted's API schemas
/// for reporting errors.
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let Some(value) = headers.get(header::CONTENT_TYPE) else {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (ErrorCode::MissingContentType, "missing `Content-Type` header"),
            ));
        };

        let Ok(value) = value.to_str() else {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (ErrorCode::InvalidUtf8, "wanted valid utf-8 in `Content-Type` header"),
            ));
        };

        let Ok(mime) = value.parse::<mime::Mime>() else {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::InvalidContentType,
                    "wanted a valid media type in `Content-Type` header",
                ),
            ));
        };

        let has_content_type =
            mime.type_() == "application" && (mime.subtype() == "json" || mime.suffix().map_or(false, |n| n == "json"));

        if !has_content_type {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::InvalidContentType,
                    "wanted `application/json`, received invalid content type",
                ),
            ));
        }

        let bytes = Bytes::from_request(req, state)
            .await
            .inspect_err(|e| {
                error!(%e, "received invalid bytes");
            })
            .map_err(|e| err(e.status(), (ErrorCode::InvalidBody, e.body_text())))?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        match serde_path_to_error::deserialize(deserializer) {
            Ok(value) => Ok(Json(value)),
            Err(e) => {
                let path = match e.path().to_string().as_str() {
                    "." => "body".to_string(),
                    path => format!("body.{path}"),
                };

                let inner = e.inner();
                error!(error = %inner, "unable to deserialize body from JSON");
                sentry::capture_error(&inner);

                match inner.classify() {
                    Category::Syntax => Err(err(
                        StatusCode::BAD_REQUEST,
                        (
                            ErrorCode::InvalidJsonPayload,
                            format!("received invalid JSON: {inner}"),
                            json!({
                                "col": inner.column(),
                                "line": inner.line(),
                                "path": path,
                            }),
                        ),
                    )),

                    Category::Data => Err(err(
                        StatusCode::NOT_ACCEPTABLE,
                        (
                            ErrorCode::InvalidJsonPayload,
                            inner.to_string(),
                            json!({
                                "col": inner.column(),
                                "line": inner.line(),
                                "path": path,
                            }),
                        ),
                    )),

                    Category::Eof => Err(err(
                        StatusCode::BAD_REQUEST,
                        (
                            ErrorCode::ReachedUnexpectedEof,
                            "reached unexpected eof",
                            json!({
                                "path": path,
                            }),
                        ),
                    )),

                    Category::Io => Err(err(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        (
                            ErrorCode::Io,
                            "received invalid I/O when parsing body",
                            json!({
                                "path": path,
                            }),
                        ),
                    )),
                }
            }
        }
    }
}
