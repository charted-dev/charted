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

//! The `api` module contains types that go along with the API server.

use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
};
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};
use serde::Serialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::borrow::Cow;
use utoipa::ToSchema;

/// Represents the REST version that an API controller is supported on.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Version {
    #[default]
    V1 = 1,
}

impl Version {
    pub fn as_str(&self) -> &str {
        match self {
            Version::V1 => "v1",
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JsonSchema for Version {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("charted_core::api::Version")
    }

    fn schema_name() -> String {
        String::from("Version")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(InstanceType::Number.into())),
            enum_values: Some(vec![Value::Number(1.into())]),

            ..Default::default()
        })
    }
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            1 => Version::V1,
            _ => panic!("reached an unexpected value for From<u8> -> APIVersion"),
        }
    }
}

impl From<Version> for u8 {
    fn from(value: Version) -> Self {
        match value {
            Version::V1 => 1,
        }
    }
}

impl From<Version> for serde_json::Number {
    fn from(value: Version) -> Self {
        match value {
            Version::V1 => serde_json::Number::from(1),
        }
    }
}

pub type Result<T> = std::result::Result<Response<T>, Response>;

/// Represents a response object for all REST endpoints.
#[derive(Debug, Serialize, ToSchema)]
pub struct Response<T = ()> {
    #[serde(skip)]
    pub(crate) status: StatusCode,

    /// Was the request a success or not?
    pub success: bool,

    /// If the request sends a payload, this is where it'll be sent. `success` is always
    /// *true*; if not the case, blame it on Noel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// If the request failed, this is a list of errors as a "stacktrace." `success` is always
    /// *false*; if not the case, blame it on Noel.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        // Safety: we know that the derive macro for serde will always succeed. If this isn't
        // the case, then it is considered undefined behaviour -- please file an issue
        // if this is the case.
        let data = unsafe { serde_json::to_string(&self).unwrap_unchecked() };
        axum::http::Response::builder()
            .status(self.status)
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(Body::from(data))
            .expect("this should succeed")
    }
}

/// Error that happened when going through a request.
#[derive(Debug, Serialize, ToSchema)]
pub struct Error {
    /// A contextual error code that can be looked up from the documentation to see
    /// why the request failed.
    pub code: ErrorCode,

    /// Humane message that is based off the contextual [error code][Error::code] to give
    /// a brief description.
    pub message: Cow<'static, str>,

    /// Other details to send to the user to give even more context about this error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

/// Error object when a internal error had occurred.
pub const INTERNAL_SERVER_ERROR: Error = Error {
    code: ErrorCode::InternalServerError,
    message: Cow::Borrowed("internal server error"),
    details: None,
};

/// Represents what kind this error is.
#[derive(Debug, Serialize, ToSchema)]
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

    /// was unable to decode into a ULID.
    UnableToDecodeUlid,

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

    /// received an invalid type that was expected
    InvalidType,

    /// generic bad request error, the message gives more context on why it is considered
    /// a bad request.
    BadRequest,

    /// missing a `Content-Type` header in your request
    MissingContentType,

    // ~ PATH PARAMETERS
    /// received the wrong list of path parameters, this is usually a bug within the code
    /// and nothing with you.
    WrongParameters,

    /// the server had failed to validate the path parameter's content.
    ParsingFailedInPathParam,

    // ~ QUERY PARAMETERS
    /// failed to parse query parameters specified in the uri of the request
    ParsingQueryParamsFailed,

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

impl From<(ErrorCode, String, Option<Value>)> for Error {
    fn from((code, message, details): (ErrorCode, String, Option<Value>)) -> Self {
        Error {
            code,
            message: Cow::Owned(message),
            details,
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

impl From<(ErrorCode, Cow<'static, str>)> for Error {
    fn from((code, message): (ErrorCode, Cow<'static, str>)) -> Self {
        Error {
            code,
            details: None,
            message,
        }
    }
}

impl From<(ErrorCode, Cow<'static, str>, Value)> for Error {
    fn from((code, message, details): (ErrorCode, Cow<'static, str>, Value)) -> Self {
        Error {
            code,
            message,
            details: Some(details),
        }
    }
}

impl From<(ErrorCode, Cow<'static, str>, Option<Value>)> for Error {
    fn from((code, message, details): (ErrorCode, Cow<'static, str>, Option<Value>)) -> Self {
        Error { code, details, message }
    }
}

/// Returns a successful API response.
pub fn ok<T>(status: StatusCode, data: T) -> Response<T> {
    Response {
        success: true,
        errors: Vec::new(),
        status,
        data: Some(data),
    }
}

pub fn from_default<T: Default>(status: StatusCode) -> Response<T> {
    ok(status, T::default())
}

pub fn err<E: Into<Error>>(status: StatusCode, error: E) -> Response {
    let error = error.into();
    Response {
        success: false,
        errors: vec![error],
        status,
        data: None,
    }
}

/// Propagate a [`Response`] with the `500 Internal Server Error` HTTP status
/// and the [`INTERNAL_SERVER_ERROR`] error details.
pub fn internal_server_error() -> Response {
    err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR)
}
