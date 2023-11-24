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

use crate::models::res::{err, ApiResponse, ErrorCode};
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{BodyStream, FromRequest},
    http::{header, HeaderMap, Request, StatusCode},
    response::IntoResponse,
    BoxError, RequestExt,
};
use charted_storage::Bytes;
use serde_json::{json, Value};
use std::{fmt::Display, sync::Arc};

/// Wrapper for [`axum::extract::Multipart`] but replaces the MultipartRejection
/// with one that uses JSON instead of text to represent errors.
///
/// This also uses the `Deref` trait to get access to the inner multipart
/// instead of implementing a wrapper `Field`.
#[derive(Debug)]
pub struct Multipart {
    inner: multer::Multipart<'static>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for Multipart
where
    B: HttpBody + Send + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = MultipartRejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let boundary = boundary(req.headers())?;
        let stream = (match req.with_limited_body() {
            Ok(limited) => BodyStream::from_request(limited, state).await,
            Err(unlimited) => BodyStream::from_request(unlimited, state).await,
        })
        .unwrap_or_else(|err| match err {});

        Ok(Self {
            inner: multer::Multipart::new(stream, boundary),
        })
    }
}

impl std::ops::Deref for Multipart {
    type Target = multer::Multipart<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Multipart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn boundary(headers: &HeaderMap) -> Result<String, MultipartRejection> {
    let Some(val) = headers.get(header::CONTENT_TYPE) else {
        return Err(multer::Error::NoBoundary.into());
    };

    let Ok(val) = val.to_str() else {
        return Err(MultipartRejection::InvalidBoundary);
    };

    multer::parse_boundary(val).map_err(|e| e.into())
}

#[derive(Debug, Clone)]
pub enum MultipartRejection {
    Multer(Arc<multer::Error>),
    InvalidBoundary,
}

impl Display for MultipartRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBoundary => f.write_str("invalid multipart boundary specified"),
            Self::Multer(err) => Display::fmt(err, f),
        }
    }
}

impl std::error::Error for MultipartRejection {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::Multer(err) => Some(err),
            _ => None,
        }
    }
}

impl From<multer::Error> for MultipartRejection {
    fn from(value: multer::Error) -> Self {
        Self::Multer(Arc::new(value))
    }
}

impl MultipartRejection {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidBoundary => StatusCode::BAD_REQUEST,
            Self::Multer(err) => multer_err_to_status_code(err),
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Self::InvalidBoundary => "received invalid boundary details",
            Self::Multer(err) => multer_err_to_message(err),
        }
    }

    fn details(&self) -> Option<Value> {
        match self {
            Self::InvalidBoundary => None,
            Self::Multer(err) => multer_err_to_details(err),
        }
    }

    fn code(&self) -> ErrorCode {
        match self {
            Self::InvalidBoundary => ErrorCode::InvalidMultipartBoundary,
            Self::Multer(err) => multer_err_to_err_code(err),
        }
    }

    pub fn response(self) -> ApiResponse {
        err(self.status_code(), (self.code(), self.message(), self.details()))
    }
}

impl IntoResponse for MultipartRejection {
    fn into_response(self) -> axum::response::Response {
        self.response().into_response()
    }
}

fn multer_err_to_status_code(err: &multer::Error) -> StatusCode {
    match err {
        multer::Error::UnknownField { .. }
        | multer::Error::IncompleteFieldData { .. }
        | multer::Error::IncompleteHeaders
        | multer::Error::ReadHeaderFailed(..)
        | multer::Error::DecodeHeaderName { .. }
        | multer::Error::DecodeContentType(..)
        | multer::Error::NoBoundary
        | multer::Error::DecodeHeaderValue { .. }
        | multer::Error::NoMultipart
        | multer::Error::IncompleteStream => StatusCode::BAD_REQUEST,
        multer::Error::FieldSizeExceeded { .. } | multer::Error::StreamSizeExceeded { .. } => {
            StatusCode::PAYLOAD_TOO_LARGE
        }
        multer::Error::StreamReadFailed(err) => {
            if let Some(err) = err.downcast_ref::<multer::Error>() {
                return multer_err_to_status_code(err);
            }

            StatusCode::INTERNAL_SERVER_ERROR
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn multer_err_to_message(err: &multer::Error) -> &'static str {
    match err {
        multer::Error::UnknownField { .. } => "received unknown field",
        multer::Error::IncompleteFieldData { .. } => "received incomplete field data in request",
        multer::Error::ReadHeaderFailed(_) => "was unable to read multipart header",
        multer::Error::NoBoundary => "was missing a multipart boundary",
        multer::Error::NoMultipart => "missing `multipart/form-data` contents",
        multer::Error::IncompleteStream => "received incomplete stream, did it corrupt?",
        multer::Error::DecodeContentType(_) => "was unable to decode `Content-Type` header for field",
        multer::Error::DecodeHeaderName { .. } => "decoding header name failed",
        multer::Error::DecodeHeaderValue { .. } => "decoding header value failed",
        multer::Error::FieldSizeExceeded { .. } => "exceeded field size capacity",
        multer::Error::StreamReadFailed(err) => {
            if let Some(err) = err.downcast_ref::<multer::Error>() {
                return multer_err_to_message(err);
            }

            "reading stream had failed"
        }

        _ => unreachable!(),
    }
}

fn multer_err_to_err_code(err: &multer::Error) -> ErrorCode {
    match err {
        multer::Error::UnknownField { .. } => ErrorCode::UnknownMultipartField,
        multer::Error::IncompleteFieldData { .. } => ErrorCode::IncompleteMultipartFieldData,
        multer::Error::ReadHeaderFailed(_) => ErrorCode::ReadMultipartHeaderFailed,
        multer::Error::DecodeContentType(_) => ErrorCode::DecodeMultipartContentTypeFailed,
        multer::Error::NoBoundary => ErrorCode::MissingMultipartBoundary,
        multer::Error::NoMultipart => ErrorCode::NoMultipartReceived,
        multer::Error::IncompleteStream => ErrorCode::IncompleteMultipartStream,
        multer::Error::DecodeHeaderName { .. } => ErrorCode::DecodeMultipartHeaderNameFailed,
        multer::Error::StreamSizeExceeded { .. } => ErrorCode::StreamSizeExceeded,
        multer::Error::FieldSizeExceeded { .. } => ErrorCode::MultipartFieldsSizeExceeded,
        multer::Error::StreamReadFailed(err) => {
            if let Some(err) = err.downcast_ref::<multer::Error>() {
                return multer_err_to_err_code(err);
            }

            ErrorCode::MultipartStreamReadFailed
        }

        _ => unreachable!(),
    }
}

fn multer_err_to_details(err: &multer::Error) -> Option<Value> {
    match err {
        multer::Error::UnknownField { field_name } => field_name.as_ref().map(|field| json!({ "field": field })),
        multer::Error::IncompleteFieldData { field_name } => field_name.as_ref().map(|field| json!({ "field": field })),
        multer::Error::ReadHeaderFailed(_) => None,
        multer::Error::DecodeContentType(_) => None,
        multer::Error::NoBoundary => None,
        multer::Error::NoMultipart => None,
        multer::Error::IncompleteStream => None,
        multer::Error::DecodeHeaderName { name, .. } => Some(json!({ "header": name })),
        multer::Error::StreamSizeExceeded { limit } => Some(json!({ "limit": limit })),
        multer::Error::FieldSizeExceeded { limit, field_name } => field_name.as_ref().map(|field| {
            json!({
                "field": field,
                "limit": limit,
            })
        }),

        multer::Error::StreamReadFailed(err) => {
            if let Some(err) = err.downcast_ref::<multer::Error>() {
                return multer_err_to_details(err);
            }

            None
        }

        _ => None,
    }
}
