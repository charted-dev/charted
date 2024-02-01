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

use crate::{
    common::models::Name,
    server::models::res::{err, ApiResponse, ErrorCode},
};
use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::Request,
    extract::{rejection::PathRejection, FromRequest, FromRequestParts, Path},
    http::{header, request::Parts, HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{error::Category, json, Value};
use std::{borrow::Cow, fmt::Display, ops::Deref};
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

/// Wrapper for [`semver::Version`] that will extract a valid SemVer from a path
/// and used for OpenAPI code generation.
pub struct Version(pub semver::Version);
impl Deref for Version {
    type Target = semver::Version;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Version {
    type Rejection = ApiResponse;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Path::<semver::Version>::from_request_parts(parts, _state)
            .await
            .map(|ver| Version(ver.0))
            .map_err(|e| {
                tracing::error!(error = %e, "unable to parse semver version");
                sentry::capture_error(&e);

                let (code, message) = match e {
                    PathRejection::FailedToDeserializePathParams(_) => (
                        ErrorCode::UnableToParsePathParameter,
                        "was unable to parse valid semver version from path",
                    ),

                    PathRejection::MissingPathParams(_) => {
                        (ErrorCode::MissingPathParameter, "missing required version parameter")
                    }

                    _ => unreachable!(),
                };

                err(StatusCode::BAD_REQUEST, (code, message))
            })
    }
}

impl<'s> ToSchema<'s> for Version {
    fn schema() -> (&'s str, RefOr<Schema>) {
        let obj = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .description(Some("Represents a semantic version (https://semver.org) that Helm and charted-server will only accept"))
            .pattern(Some(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"))
            .build();

        ("Version", RefOr::T(Schema::Object(obj)))
    }
}

pub(crate) struct VersionReq;
impl<'s> ToSchema<'s> for VersionReq {
    fn schema() -> (&'s str, RefOr<Schema>) {
        let obj = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .description(Some("Represents a semantic version (https://semver.org) requirement (i.e, `>=1.2.0`) that Helm and charted-server will only accept"))
            .build();

        ("VersionReq", RefOr::T(Schema::Object(obj)))
    }
}

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
impl<T, S> FromRequest<S> for Json<T>
where
    T: for<'de> Deserialize<'de>,
    S: Send + Sync,
{
    type Rejection = ApiResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !has_content_type(req.headers()) {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::MissingHeader,
                    "expected request to have a Content-Type with [application/json]",
                ),
            ));
        }

        let bytes = Bytes::from_request(req, state).await.map_err(|e| {
            error!(%e, "received invalid bytes");
            err(e.status(), (ErrorCode::InvalidBody, e.body_text()))
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

#[derive(Debug)]
pub enum NameOrSnowflakeError {
    InvalidName { name: Name, why: Cow<'static, str> },
    PathRejection(PathRejection),
}

impl Display for NameOrSnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathRejection(params) => Display::fmt(params, f),
            Self::InvalidName { .. } => f.write_str("invalid name"),
        }
    }
}

impl std::error::Error for NameOrSnowflakeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PathRejection(params) => params.source(),
            _ => None,
        }
    }
}

impl NameOrSnowflakeError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::PathRejection(_) => StatusCode::BAD_REQUEST,
            Self::InvalidName { .. } => StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn code(&self) -> ErrorCode {
        match self {
            Self::PathRejection(_) => ErrorCode::BadRequest,
            Self::InvalidName { .. } => ErrorCode::InvalidType,
        }
    }

    fn details(&self) -> Option<Value> {
        match self {
            Self::PathRejection(_) => None,
            Self::InvalidName { name, .. } => Some(json!({"name":name})),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::PathRejection(_) => self.to_string(),
            Self::InvalidName { why, .. } => format!("{self}: {why}"),
        }
    }
}

impl IntoResponse for NameOrSnowflakeError {
    fn into_response(self) -> axum::response::Response {
        err(self.status_code(), (self.code(), self.message(), self.details())).into_response()
    }
}

/// Extractor to [`Path`][axum::extract::Path] to extract it as a [`NameOrSnowflake`][crate::common::models::NameOrSnowflake].
#[derive(Debug, Clone)]
pub struct NameOrSnowflake(pub crate::common::models::NameOrSnowflake);

#[async_trait]
impl<S> FromRequestParts<S> for NameOrSnowflake
where
    S: Send + Sync,
{
    type Rejection = NameOrSnowflakeError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = Path::<crate::common::models::NameOrSnowflake>::from_request_parts(parts, state)
            .await
            .map_err(NameOrSnowflakeError::PathRejection)?;

        match path.0 {
            crate::common::models::NameOrSnowflake::Name(name) => {
                // this is here since `Path`'s `deserialize_any` uses `deserialize_str`,
                // which defeats on what we need to do, so if it can be parsed as a `u64`,
                // then it's a valid Snowflake
                if let Ok(id) = name.parse::<u64>() {
                    return Ok(Self(crate::common::models::NameOrSnowflake::Snowflake(id)));
                }

                match name.is_valid() {
                    Ok(()) => Ok(Self(crate::common::models::NameOrSnowflake::Name(name))),
                    Err(e) => Err(NameOrSnowflakeError::InvalidName {
                        name,
                        why: Cow::Owned(e.to_string()),
                    }),
                }
            }
            ref_ => Ok(Self(ref_)),
        }
    }
}
