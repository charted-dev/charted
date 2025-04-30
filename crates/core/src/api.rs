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

use serde::{Deserialize, Serialize};
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

    fn json_schema(_: &mut ::schemars::r#gen::SchemaGenerator) -> ::schemars::schema::Schema {
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
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(deny_unknown_fields)]
pub struct Response<T = ()> {
    /// List of HTTP headers to append to this response.
    #[cfg(feature = "axum")]
    #[serde(skip)]
    pub headers: ::axum::http::HeaderMap<::axum::http::HeaderValue>,

    /// The status of the response.
    #[cfg(feature = "axum")]
    #[serde(skip)]
    pub status: ::axum::http::StatusCode,

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

impl<T> Response<T> {
    #[cfg(feature = "axum")]
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
    pub fn with_header<K: Into<axum::http::HeaderName>, V: Into<axum::http::HeaderValue>>(
        mut self,
        key: K,
        value: V,
    ) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

impl<T: PartialEq> PartialEq for Response<T> {
    fn eq(&self, other: &Self) -> bool {
        self.success == other.success && self.data.eq(&other.data) && self.errors.eq(&other.errors)
    }
}

impl<T: Eq> Eq for Response<T> {}

#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
impl<T: Serialize> ::axum::response::IntoResponse for Response<T> {
    fn into_response(mut self) -> axum::response::Response {
        let data = serde_json::to_string(&self).unwrap();
        let _ = self.headers.insert(
            axum::http::header::CONTENT_TYPE,
            "application/json; charset=utf-8".try_into().unwrap(),
        );

        let mut res = axum::http::Response::builder().status(self.status);
        res.headers_mut().replace(&mut self.headers);
        res.body(axum::body::Body::from(data)).unwrap()
    }
}

/// Representation of a error from an error trace.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ErrorCode {
    /// A system failure occurred.
    SystemFailure,

    /// Unexpected EOF when encoding or decoding data.
    UnexpectedEOF,

    /// The endpoint that you're trying to reach is not avaliable.
    RestEndpointNotFound,

    /// The endpoint that you're trying to reach is using an invalid HTTP method.
    InvalidHttpMethod,

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
    InvalidHttpHeaderName,

    /// Received an invalid HTTP header name.
    InvalidHttpHeaderValue,

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

    /// invalid input was given
    InvalidInput,

    // ~ PATH PARAMETERS
    /// unable to parse a path parameter.
    UnableToParsePathParameter,

    /// missing a required path parameter in the request.
    MissingPathParameter,

    /// received the wrong list of path parameters, this is usually a bug within charted
    /// itself.
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
        headers: ::axum::http::HeaderMap::new(),
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
        headers: ::axum::http::HeaderMap::new(),
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

/// Propagate a system failure response from [`eyre::Report`].
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn system_failure_from_report(report: eyre::Report) -> Response {
    #[derive(Debug)]
    struct AError<'a>(&'a dyn std::error::Error);
    impl std::fmt::Display for AError<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.0, f)
        }
    }

    impl std::error::Error for AError<'_> {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.0.source()
        }
    }

    system_failure(AError(report.as_ref()))
}

/// Propagate a system failure response.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
pub fn system_failure<E: std::error::Error>(error: E) -> Response {
    if cfg!(debug_assertions) {
        use serde_json::json;

        let mut errors = Vec::new();
        for err in error.source().iter().take(5) {
            errors.push(Value::String(err.to_string()));
        }

        let backtrace = collect_backtrace();

        return err(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            (
                ErrorCode::SystemFailure,
                format!("system failure occurred: {error}"),
                json!({
                    "error_type": std::any::type_name::<E>(),
                    "caused_by": errors,
                    "backtrace": backtrace,
                }),
            ),
        );
    }

    #[cfg(not(debug_assertions))]
    const _: serde_json::Value = collect_backtrace();

    err(
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        (ErrorCode::SystemFailure, "system failure occurred"),
    )
}

#[cfg(all(debug_assertions, feature = "collect-backtrace-frames"))]
#[allow(unused)]
#[inline(never)]
#[cold] // system failures should theorically never happen
fn collect_backtrace() -> Value {
    use backtrace::Backtrace;
    use serde_json::json;

    /// A list of functions to skip since they don't really matter.
    const FUNCTIONS_TO_SKIP: &[&str] = &[
        "<core::pin::Pin",
        "<futures_util::future::",
        "<alloc::boxed::Box",
        "core::ops::function::Fn",
        "std::sys::pal",
        "<unknown>",
        "start_thread",
        "__GI",
        "std::thread::Builder::spawn_",
        "__rust_try",
        "std::panicking::try::do_call",
        "<core::panic::unwind_safe::",
        "tokio::runtime",
        "<tokio::runtime",
        "tokio::loom",
        "std::thread::local",
        "<axum::serve",
        "<core::future",
        "<hyper_util",
        "hyper_util",
        "<hyper",
        "hyper",
        "<axum::routing",
        "<tower",
        "<tower_http",
        "<F as futures_core::future::TryFuture>",
        "<sentry_core::futures",
        "<sentry_tower::http",
        "<axum::middleware::from_fn",
        "axum::middleware::from_fn",
        "<tracing::instrument::Instrumented",
        "<axum::handler::future",
        "<F as axum::handler::Handler",
        "std::panic",
    ];

    let bt = Backtrace::new();
    let mut stack = match sentry_backtrace::backtrace_to_stacktrace(&bt) {
        Some(bt) => bt,
        None => return Value::Null,
    };

    stack.frames.reverse();

    let iter = stack.frames.iter().skip(1).filter(|f| {
        let Some(ref f) = f.function else {
            return false;
        };

        !FUNCTIONS_TO_SKIP.iter().any(|p| f.starts_with(p))
    });

    let mut data = Vec::with_capacity(iter.size_hint().0);
    for frame in iter {
        let Some(ref f) = frame.function.as_ref() else {
            continue;
        };

        data.push(json!({
            "function": f,
            "file": frame.abs_path,
            "line": frame.lineno
        }));
    }

    Value::Array(data)
}

// Don't attempt to collect one if we don't have `nightly-backtrace-frames`.
#[cfg(not(all(debug_assertions, feature = "collect-backtrace-frames")))]
#[cfg_attr(debug_assertions, allow(unused))]
#[inline(never)]
#[cold]
const fn collect_backtrace() -> Value {
    Value::Null
}
