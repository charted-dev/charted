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

use axum::{
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Cow, fmt::Debug};
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

/// Represents a [`Result`][std::result::Result] type that corresponds to a API response
/// result type.
pub type Result<T = ()> = std::result::Result<ApiResponse<T>, ApiResponse>;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T: Debug + Serialize = ()> {
    /// Status code of this [`ApiResponse`].
    #[serde(skip)]
    pub(crate) status: StatusCode,

    /// whether or not if this request was successful
    pub success: bool,

    /// inner data to send to the user, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// any errors to send to the user to indicate that this request failed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

impl<T: Serialize + Debug> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let mut res = Response::new(serde_json::to_string(&self).expect("this should never happen"));
        *res.status_mut() = self.status;
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        res.into_response()
    }
}

/// Represents an error that could occur.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Error {
    /// A contextual error code that can be looked up from the documentation to see
    /// why the request failed.
    pub code: ErrorCode,

    /// Humane message that is based off the contextual [error code][Error::code] to give
    /// a brief description.
    pub message: Cow<'static, str>,

    /// Other details to send to the user to give even more context about this error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

pub const INTERNAL_SERVER_ERROR: Error = Error {
    code: ErrorCode::InternalServerError,
    message: Cow::Borrowed("Internal Server Error"),
    details: None,
};

/// Represents a error code that can happen.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    // ~ COMMON
    /// Internal Server Error
    InternalServerError,

    /// reached an unexpected 'end-of-file' marker
    ReachedUnexpectedEof,

    /// was unable to process something internally
    UnableToProcess,

    /// given REST handler by your request was not found.
    HandlerNotFound,

    /// given entity to lookup was not found.
    EntityNotFound,

    /// given entity to create already exists.
    EntityAlreadyExists,

    /// unable to validate the input data successfully
    ValidationFailed,

    /// the query given for the CDN was not found.
    UnknownCdnQuery,

    /// received an invalid `Content-Type` header value
    InvalidContentType,

    /// this route requires a `Bearer` session to work.
    SessionOnlyRoute,

    /// received an invalid HTTP header key or value
    InvalidHttpHeader,

    /// was unable to decode expected Base64 data.
    UnableToDecodeBase64,

    /// received invalid UTF-8 data
    InvalidUtf8,

    /// received invalid request body
    InvalidBody,

    /// missing a required header
    MissingHeader,

    /// registrations are disabled
    RegistrationsDisabled,

    /// missing a password to use for authentication
    MissingPassword,

    /// given access was not permitted
    AccessNotPermitted,

    /// something went wrong with the given input/output stream.
    Io,

    // ~ SESSIONS
    /// received JWT claim was not found or was invalid
    InvalidJwtClaim,

    /// was missing an `Authorization` header
    MissingAuthorizationHeader,

    /// password given was invalid
    InvalidPassword,

    /// received invalid authentication type
    InvalidAuthenticationType,

    /// received an invalid part in an Authorization header value
    InvalidAuthorizationParts,

    /// received an invalid JWT token
    InvalidSessionToken,

    /// Session already expired.
    SessionExpired,

    /// unknown session.
    UnknownSession,

    /// a refresh token is required in this request.
    RefreshTokenRequired,

    // ~ PAGINATION
    /// the `?per_page` query parameter is maxed out to 100
    MaxPerPageExceeded,

    // ~ PATH PARAMETERS
    /// unable to parse a path parameter.
    UnableToParsePathParameter,

    /// missing a required path parameter in the request.
    MissingPathParameter,

    // ~ JSON BODY
    /// while parsing through the JSON tree received, something went wrong
    InvalidJsonPayload,

    // ~ MULTIPART
    /// multipart field expected was not found
    UnknownMultipartField,

    /// incomplete field data given
    IncompleteMultipartFieldData,

    /// unable to completely read multipart header received
    ReadMultipartHeaderFailed,

    /// was unable to decode the `Content-Type` header in this request
    DecodeMultipartContentTypeFailed,

    /// missing a multipart boundry to parse
    MissingMultipartBoundary,

    /// expected multipart/form-data; received something else
    NoMultipartReceived,

    /// received incomplete multipart stream
    IncompleteMultipartStream,

    /// was unable to decode a header name in a multipart request
    DecodeMultipartHeaderNameFailed,

    /// exceeded the maximum amount to stream from
    StreamSizeExceeded,

    /// exceeded the maximum amount of fields to use
    MultipartFieldsSizeExceeded,

    /// received unknown error while reading the given stream
    MultipartStreamReadFailed,

    /// missing an expected multipart field in this request.
    MissingMultipartField,

    /// received an invalid multipart boundary
    InvalidMultipartBoundary,
}

impl<'s> ToSchema<'s> for ErrorCode {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "ErrorCode",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .description(Some("Represents a error code that can happen"))
                    .schema_type(SchemaType::String)
                    .build(),
            )),
        )
    }
}

impl From<(ErrorCode, &'static str)> for Error {
    fn from((code, message): (ErrorCode, &'static str)) -> Self {
        Error {
            code,
            message: Cow::Borrowed(message),
            details: None,
        }
    }
}

impl From<(ErrorCode, String)> for Error {
    fn from((code, message): (ErrorCode, String)) -> Self {
        Error {
            code,
            message: Cow::Owned(message),
            details: None,
        }
    }
}

impl From<(ErrorCode, String, Value)> for Error {
    fn from((code, message, details): (ErrorCode, String, Value)) -> Self {
        Error {
            code,
            message: Cow::Owned(message),
            details: Some(details),
        }
    }
}

impl From<(ErrorCode, &'static str, Value)> for Error {
    fn from((code, message, details): (ErrorCode, &'static str, Value)) -> Self {
        Error {
            code,
            message: Cow::Borrowed(message),
            details: Some(details),
        }
    }
}

impl From<(ErrorCode, &'static str, Option<Value>)> for Error {
    fn from((code, message, details): (ErrorCode, &'static str, Option<Value>)) -> Self {
        Error {
            code,
            details,
            message: Cow::Borrowed(message),
        }
    }
}

/// Returns a successful API response.
pub fn ok<T: Serialize + Debug>(status: StatusCode, data: T) -> ApiResponse<T> {
    ApiResponse {
        status,
        success: true,
        data: Some(data),
        errors: vec![],
    }
}

pub fn err<E: Into<Error>>(status: StatusCode, error: E) -> ApiResponse {
    ApiResponse {
        status,
        success: false,
        data: None,
        errors: vec![error.into()],
    }
}

pub fn no_content() -> ApiResponse {
    ApiResponse {
        status: StatusCode::NO_CONTENT,
        success: true,
        data: None,
        errors: vec![],
    }
}
