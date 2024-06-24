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

use std::{borrow::Cow, fmt::Display};
use axum::{http::StatusCode, response::IntoResponse};
use charted_core::response::{err, ErrorCode};

#[derive(Debug)]
pub enum Error {
    /// Received an unknown authorization type.
    UnknownAuthType(Cow<'static, str>),

    /// Generic message that is passed through as the `message` field in a API response
    /// error.
    Message(Cow<'static, str>),

    /// Error that occurred while decoding base64 content
    Base64(base64::DecodeError),

    /// Error that occurred when either encoding or decoding JWT tokens
    Jwt(jsonwebtoken::errors::Error),

    /// Bearer token given needs to be a valid refresh token.
    RefreshTokenRequired,

    /// missing the `Authorization` header in the request.
    MissingHeader,

    /// Invalid password given
    InvalidPassword,

    /// Unknown session
    UnknownSession,
}

impl Error {
    pub fn invalid_utf8() -> Error {
        Error::message("received invalid utf-8")
    }

    pub fn message<I: Into<Cow<'static, str>>>(msg: I) -> Error {
        Error::Message(msg.into())
    }

    pub fn status_code(&self) -> StatusCode {
        use Error::*;
        match self {
            MissingHeader | UnknownAuthType(_) | Message(_) | Base64(_) | RefreshTokenRequired => {
                StatusCode::NOT_ACCEPTABLE
            }

            InvalidPassword => StatusCode::UNAUTHORIZED,
            UnknownSession => StatusCode::NOT_FOUND,
            Jwt(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => StatusCode::FORBIDDEN,
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_code(&self) -> ErrorCode {
        use Error::*;
        use ErrorCode::*;

        match self {
            Error::RefreshTokenRequired => ErrorCode::RefreshTokenRequired,
            Error::InvalidPassword => ErrorCode::InvalidPassword,
            Error::UnknownSession => ErrorCode::UnknownSession,
            Error::MissingHeader => ErrorCode::MissingAuthorizationHeader,
            Message(_) => InvalidAuthorizationParts,
            UnknownAuthType(_) => InvalidAuthenticationType,
            Base64(_) => UnableToDecodeBase64,
            Jwt(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => SessionExpired,
                jsonwebtoken::errors::ErrorKind::InvalidToken => InvalidSessionToken,
                _ => InternalServerError,
            },
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        err(self.status_code(), (self.error_code(), self.to_string())).into_response()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RefreshTokenRequired => f.write_str("refresh token is required for this route"),
            Error::UnknownAuthType(ty) => {
                write!(
                    f,
                    "received invalid authorization type: [{ty}]; expected one of [Bearer, Basic, ApiKey]"
                )
            }

            Error::InvalidPassword => f.write_str("received invalid password"),
            Error::UnknownSession => f.write_str("unknown session"),
            Error::MissingHeader => f.write_str("missing `Authorization` header"),
            Error::Message(msg) => f.write_str(msg),
            Error::Base64(err) => Display::fmt(err, f),
            Error::Jwt(err) => Display::fmt(err, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Base64(err) => Some(err),
            Error::Jwt(err) => Some(err),
            _ => None,
        }
    }
}

impl From<base64::DecodeError> for Error {
    fn from(value: base64::DecodeError) -> Self {
        Self::Base64(value)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(value)
    }
}
