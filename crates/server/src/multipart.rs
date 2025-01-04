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
    extract::{FromRequest, Request},
    http::{header, StatusCode},
    response::IntoResponse,
    RequestExt,
};
use charted_core::api;
use serde_json::json;
use std::{borrow::Cow, fmt::Display, ops::DerefMut};

charted_core::create_newtype_wrapper! {
    /// `Multipart` is an Axum extractor that implements [`FromRequest`], so it has to be
    /// the last parameter in a REST controller.
    #[derive(Debug)]
    pub Multipart for ::multer::Multipart<'static>;
}

impl DerefMut for Multipart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S: Send + Sync> FromRequest<S> for Multipart {
    type Rejection = Rejection;

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        let Some(ct) = req.headers().get(header::CONTENT_TYPE) else {
            return Err(Rejection::NoContentTypeAvaliable);
        };

        let value = ct.to_str().map_err(|e| {
            tracing::error!(error = %e, "received invalid utf-8 in multipart body");
            Rejection::InvalidUtf8ForBoundary
        })?;

        let boundary = multer::parse_boundary(value).map_err(|e| {
            tracing::error!(error = %e, "received invalid multipart body content");
            match e {
                multer::Error::NoBoundary => Rejection::NoBoundary,
                e => Rejection::Multer(e),
            }
        })?;

        let stream = req.with_limited_body().into_body();
        Ok(Self(multer::Multipart::new(stream.into_data_stream(), boundary)))
    }
}

///////////////////////// ERRORS //////////////////////////////////

#[derive(Debug)]
pub enum Rejection {
    /// Error that occurred from the [`multer::Multipart`] instance.
    Multer(multer::Error),

    /// The boundary given was an invalid UTF-8 encoded piece of data.
    InvalidUtf8ForBoundary,

    /// No `Content-Type` header was given in the request.
    NoContentTypeAvaliable,

    /// No multipart boundary was specified.
    NoBoundary,
}

impl Rejection {
    fn error_code(&self) -> api::ErrorCode {
        match self {
            Rejection::NoContentTypeAvaliable => api::ErrorCode::InvalidContentType,
            Rejection::NoBoundary => api::ErrorCode::InvalidMultipartBoundary,
            Rejection::InvalidUtf8ForBoundary => api::ErrorCode::InvalidUtf8,
            Rejection::Multer(err) => match err {
                multer::Error::UnknownField { .. } => api::ErrorCode::UnknownMultipartField,
                multer::Error::IncompleteFieldData { .. } => api::ErrorCode::IncompleteMultipartFieldData,
                multer::Error::ReadHeaderFailed(_) => api::ErrorCode::ReadMultipartHeaderFailed,
                multer::Error::DecodeContentType(_) => api::ErrorCode::DecodeMultipartContentTypeFailed,
                multer::Error::NoBoundary => api::ErrorCode::MissingMultipartBoundary,
                multer::Error::NoMultipart => api::ErrorCode::NoMultipartReceived,
                multer::Error::IncompleteStream => api::ErrorCode::IncompleteMultipartStream,
                multer::Error::DecodeHeaderName { .. } => api::ErrorCode::DecodeMultipartHeaderNameFailed,
                multer::Error::StreamSizeExceeded { .. } => api::ErrorCode::StreamSizeExceeded,
                multer::Error::FieldSizeExceeded { .. } => api::ErrorCode::MultipartFieldsSizeExceeded,
                multer::Error::StreamReadFailed(_) => api::ErrorCode::MultipartStreamReadFailed,

                _ => unreachable!(),
            },
        }
    }

    fn err_message(&self) -> Cow<'static, str> {
        match self {
            Rejection::NoContentTypeAvaliable => Cow::Borrowed("missing `content-type` header"),
            Rejection::InvalidUtf8ForBoundary => Cow::Borrowed("received invalid utf-8 in multipart boundary decoding"),
            Rejection::NoBoundary => Cow::Borrowed("missing multipart boundary"),
            Rejection::Multer(err) => match err {
                multer::Error::UnknownField { .. } => Cow::Borrowed("received unknown field"),
                multer::Error::IncompleteFieldData { .. } => Cow::Borrowed("received incomplete field data in request"),
                multer::Error::ReadHeaderFailed(_) => Cow::Borrowed("was unable to read multipart header"),
                multer::Error::NoBoundary => Cow::Borrowed("was missing a multipart boundary"),
                multer::Error::NoMultipart => Cow::Borrowed("missing `multipart/form-data` contents"),
                multer::Error::IncompleteStream => Cow::Borrowed("received incomplete stream, did it corrupt?"),
                multer::Error::DecodeContentType(_) => {
                    Cow::Borrowed("was unable to decode `Content-Type` header for field")
                }
                multer::Error::DecodeHeaderName { .. } => Cow::Borrowed("decoding header name failed"),
                multer::Error::DecodeHeaderValue { .. } => Cow::Borrowed("decoding header value failed"),
                multer::Error::FieldSizeExceeded { .. } => Cow::Borrowed("exceeded field size capacity"),
                multer::Error::StreamReadFailed(_) => Cow::Borrowed("reading stream had failed"),

                _ => unreachable!(),
            },
        }
    }

    fn expand_details(&self) -> Option<serde_json::Value> {
        match self {
            Rejection::Multer(err) => match err {
                multer::Error::UnknownField { field_name } => {
                    field_name.as_ref().map(|field| json!({ "field": field }))
                }

                multer::Error::IncompleteFieldData { field_name } => {
                    field_name.as_ref().map(|field| json!({ "field": field }))
                }

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

                multer::Error::StreamReadFailed(_) => None,

                _ => None,
            },
            _ => None,
        }
    }
}

impl Display for Rejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rejection::Multer(err) => Display::fmt(err, f),
            Rejection::NoContentTypeAvaliable => f.write_str("no `content-type` header was specified"),
            Rejection::NoBoundary => f.write_str("received no multipart boundary"),
            Rejection::InvalidUtf8ForBoundary => f.write_str("received invalid utf-8 in multipart boundary decoding"),
        }
    }
}

impl std::error::Error for Rejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Rejection::Multer(err) => Some(err),
            _ => None,
        }
    }
}

impl IntoResponse for Rejection {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Rejection::Multer(ref err) => multer_into_status_code(err),
            _ => StatusCode::BAD_REQUEST,
        };

        let error = api::Error {
            code: self.error_code(),
            message: self.err_message(),
            details: self.expand_details(),
        };

        api::err(status, error).into_response()
    }
}

pub fn multer_into_status_code(error: &multer::Error) -> StatusCode {
    match error {
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
                return multer_into_status_code(err);
            }

            StatusCode::INTERNAL_SERVER_ERROR
        }

        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
