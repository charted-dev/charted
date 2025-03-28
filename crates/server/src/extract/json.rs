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

use axum::{
    body::Bytes,
    extract::{self, FromRequest},
    http::{StatusCode, header},
};
use charted_core::api::{self, ErrorCode};
use serde::de::DeserializeOwned;
use serde_json::{error::Category, json};
use std::borrow::Cow;
use tracing::error;

/// `Json` is a Axum extractor that implements [`FromRequest`] to transform
/// `T` into a deserialized struct from a JSON payload via the [`serde_json`]
/// crate.
#[derive(Debug, Clone)]
pub struct Json<T>(pub T);
impl<T: DeserializeOwned, S: Send + Sync> FromRequest<S> for Json<T> {
    type Rejection = api::Response;

    async fn from_request(req: extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let Some(ct) = headers.get(header::CONTENT_TYPE) else {
            return Err(api::err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::MissingContentType,
                    "missing `content-type` header in request",
                ),
            ));
        };

        let value = ct.to_str().map_err(|e| {
            tracing::error!(error = %e, "received invalid utf-8 in `content-type` header");

            api::err(
                StatusCode::BAD_REQUEST,
                (ErrorCode::InvalidUtf8, "received invalid utf-8 in content type header"),
            )
        })?;

        let mime = value.parse::<mime::Mime>().map_err(|e| {
            tracing::error!(error = %e, "failed to parse into mime type");

            api::err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::InvalidContentType,
                    "received invalid mime type in `content-type` header",
                ),
            )
        })?;

        let has_ct =
            mime.type_() == "application" && (mime.subtype() == "json" || mime.suffix().is_some_and(|n| n == "json"));

        if !has_ct {
            return Err(api::err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::InvalidContentType,
                    "wanted `application/json` for `content-type` header",
                    json!({"received":mime.to_string()}),
                ),
            ));
        }

        let bytes = Bytes::from_request(req, state).await.map_err(|e| {
            tracing::error!(error = %e, "failed to deserialize body into a byte array");

            api::err(e.status(), (ErrorCode::InvalidBody, e.body_text()))
        })?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        serde_path_to_error::deserialize(deserializer).map(Json).map_err(|e| {
            let path = match e.path().to_string().as_str() {
                "." => Cow::Borrowed("body"),
                p => Cow::Owned(format!("body.{p}")),
            };

            let error = e.inner();
            error!(%error, %path, "unable to deserialize body from JSON");

            match error.classify() {
                Category::Syntax | Category::Data => api::err(
                    StatusCode::BAD_REQUEST,
                    (
                        ErrorCode::InvalidJsonPayload,
                        error.to_string(),
                        json!({
                            "line": error.line(),
                            "path": path,
                            "col": error.column(),
                        }),
                    ),
                ),

                Category::Eof => api::err(
                    StatusCode::BAD_REQUEST,
                    (
                        ErrorCode::ReachedUnexpectedEof,
                        "reached an unexpected 'end of file' marker; this is a bug, please report this!",
                        json!({
                            "report_url": "https://github.com/charted-dev/charted/issues/new",
                            "path": path
                        }),
                    ),
                ),

                Category::Io => api::err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    (
                        ErrorCode::Io,
                        "input/output error reached; this is a bug, please report this!",
                        json!({
                            "report_url": "https://github.com/charted-dev/charted/issues/new",
                            "path": path
                        }),
                    ),
                ),
            }
        })
    }
}
