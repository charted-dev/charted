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

//! Types that are used with the API server.

use serde::Serialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::borrow::Cow;

/// Specification version for charted's HTTP specification.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
    derive_more::Display,
)]
#[display("{}", self.as_str())]
#[repr(u8)]
pub enum Version {
    /// ## `v1`
    ///
    /// Released since the initial release of **charted-server**.
    #[default]
    V1 = 1,
}

impl Version {
    pub const fn as_str(&self) -> &str {
        match self {
            Version::V1 => "v1",
        }
    }

    pub const fn as_slice<'a>() -> &'a [Version] {
        &[Version::V1]
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

#[cfg(feature = "schemars")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "schemars")))]
impl ::schemars::JsonSchema for Version {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("charted_core::api::Version")
    }

    fn schema_name() -> String {
        String::from("Version")
    }

    fn json_schema(_: &mut ::schemars::gen::SchemaGenerator) -> ::schemars::schema::Schema {
        ::schemars::schema::Schema::Object(::schemars::schema::SchemaObject {
            instance_type: Some(::schemars::schema::SingleOrVec::Single(
                ::schemars::schema::InstanceType::Number.into(),
            )),

            enum_values: Some(vec![::serde_json::Value::Number(1.into())]),

            ..Default::default()
        })
    }
}

pub type Result<T> = std::result::Result<Response<T>, Response>;

/// Representation of a response that the API server sends for each request.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Response<T = ()> {
    #[serde(skip)]
    pub(crate) status: axum::http::StatusCode,

    /// Was the request that was processed a success?
    pub success: bool,

    /// The data that the REST endpoint sends back, if any.
    ///
    /// When this field is empty, it'll always respond with a `204 No Content`
    /// status code if `errors` is also empty.
    ///
    /// The `success` field will always be set to `true` when
    /// the `data` field is avaliable. All errors are handled
    /// by the `errors` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// The error trace for the request that was processed by
    /// the API server.
    ///
    /// The `success` field will always be set to `false` when
    /// the `errors` field is avaliable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
impl<T: Serialize> ::axum::response::IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let data = serde_json::to_string(&self).unwrap();
        axum::http::Response::builder()
            .status(self.status)
            .header(axum::http::header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(axum::body::Body::from(data))
            .unwrap()
    }
}

/// Representation of a error from an error trace.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Error {
    /// Contextualized error code on why this request failed.
    ///
    /// This field can be looked up from the documentation to give
    /// a better representation of the error.
    pub code: ErrorCode,

    /// A humane description based off the contextualised `"code"` field.
    pub message: Cow<'static, str>,

    /// If provided, this gives more information about the error
    /// and why it could've possibly failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

/// Contextualized error code on why this request failed.
///
/// This field can be looked up from the documentation to give
/// a better representation of the error.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// A system failure occurred.
    SystemFailure,

    /// Unexpected EOF when encoding or decoding data.
    UnexpectedEOF,

    /// The endpoint that you're trying to reach is not avaliable.
    RestEndpointNotFound,

    /// The endpoint that you're trying to reach is using an invalid HTTP method.
    InvalidHTTPMethod,

    /// The entity was not found.
    EntityNotFound,

    /// The entity already exists.
    EntityAlreadyExists,

    /// Unexpected internal server error.
    InternalServerError,

    /// Validation for the input data received failed.
    ValidationFailed,

    /// The `Content-Type` header value was invalid.
    InvalidContentType,

    /// Received an invalid HTTP header name.
    InvalidHTTPHeaderName,

    /// Received an invalid HTTP header name.
    InvalidHTTPHeaderValue,

    /// This endpoint only allows Bearer tokens.
    RequiresSessionToken,

    /// Unable to decode base64 content given.
    UnableToDecodeBase64,

    /// Unable to decode ULID given.
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

    /// reached an unexpected EOF marker.
    ReachedUnexpectedEof,

    // ~ PATH PARAMETERS
    /// unable to parse a path parameter.
    UnableToParsePathParameter,

    /// missing a required path parameter in the request.
    MissingPathParameter,

    /// received the wrong list of path parameters, this is usually a bug within charted itself.
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

    /// expected `multipart/form-data`; received something else
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

/// Return a successful API response.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn ok<T>(status: axum::http::StatusCode, data: T) -> Response<T> {
    Response {
        success: true,
        errors: Vec::new(),
        status,
        data: Some(data),
    }
}

/// Returns a empty HTTP API response.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn no_content() -> Response<()> {
    from_default(axum::http::StatusCode::NO_CONTENT)
}

/// Return a success HTTP API response from `T`'s [`Default`] implementation.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn from_default<T: Default>(status: axum::http::StatusCode) -> Response<T> {
    ok(status, T::default())
}

/// Returns a failed HTTP API response.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn err<E: Into<Error>>(status: axum::http::StatusCode, error: E) -> Response {
    let error = error.into();
    Response {
        success: false,
        errors: vec![error],
        status,
        data: None,
    }
}

/// Propagate a HTTP API response as a internal server error.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn internal_server_error() -> Response {
    err(
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        (ErrorCode::InternalServerError, "Internal Server Error"),
    )
}

/// Propagate a system failure response.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn system_failure<E: std::error::Error>(error: E) -> Response {
    if cfg!(debug_assertions) {
        let mut errors = Vec::new();
        for err in error.source().iter().take(5) {
            errors.push(Value::String(err.to_string()));
        }

        return err(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            (
                ErrorCode::SystemFailure,
                format!("system failure occurred: {error}"),
                Some(Value::Array(errors)),
            ),
        );
    }

    err(
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        (ErrorCode::SystemFailure, "system failure occurred", None),
    )
}
