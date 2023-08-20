// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use std::ops::Deref;

use crate::models::res::{err, ApiResponse};
use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::FromRequest,
    http::{header, HeaderMap, Request, StatusCode},
    BoxError,
};
use serde::de::DeserializeOwned;
use serde_json::{error::Category, json};

/// Wrapper for [`axum::Json`] that uses charted-server's [API response][ApiResponse]
/// for the details on why it failed to parse JSON from the request
/// body.
pub struct Json<T>(pub T);

impl<T> Deref for Json<T>
where
    T: DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Json<T>
where
    T: serde::de::DeserializeOwned,
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = ApiResponse;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        if !has_content_type(req.headers()) {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (
                    "MISSING_CONTENT_TYPE",
                    "Expected request to have a Content-Type with [application/json]",
                )
                    .into(),
            ));
        }

        let bytes = Bytes::from_request(req, state).await.map_err(|e| {
            error!(%e, "received invalid bytes");
            err(e.status(), ("INVALID_BODY", e.body_text().as_str()).into())
        })?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        match serde_path_to_error::deserialize(deserializer) {
            Ok(value) => Ok(Json(value)),
            Err(e) => {
                let path = match e.path().to_string().as_str() {
                    "." => "body".to_string(),
                    path => format!("body.{path}"),
                };

                let inner = e.inner();
                match inner.classify() {
                    Category::Syntax => Err(err(
                        StatusCode::BAD_REQUEST,
                        (
                            "INVALID_JSON",
                            format!("received invalid JSON: {inner}").as_str(),
                            json!({
                                "col": inner.column(),
                                "line": inner.line(),
                                "path": path,
                            }),
                        )
                            .into(),
                    )),

                    Category::Data => Err(err(
                        StatusCode::NOT_ACCEPTABLE,
                        (
                            "SEMANTIC_ERRORS",
                            format!("data in path was semantically incorrect: {inner}").as_str(),
                            json!({
                                "col": inner.column(),
                                "line": inner.line(),
                                "path": path,
                            }),
                        )
                            .into(),
                    )),

                    Category::Eof => Err(err(
                        StatusCode::BAD_REQUEST,
                        (
                            "REACHED_UNEXPECTED_EOF",
                            "reached unexpected eof",
                            json!({
                                "path": path,
                            }),
                        )
                            .into(),
                    )),

                    Category::Io => Err(err(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        (
                            "IO",
                            "received invalid I/O when parsing body",
                            json!({
                                "path": path,
                            }),
                        )
                            .into(),
                    )),
                }
            }
        }
    }
}

fn has_content_type(headers: &HeaderMap) -> bool {
    let Some(value) = headers.get(header::CONTENT_TYPE) else {
        return false;
    };

    let Ok(value) = value.to_str() else {
        return false;
    };

    let Ok(mime) = value.parse::<mime::Mime>() else {
        return false;
    };

    mime.type_() == "application" && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"))
}
