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

use crate::ServerContext;
use axum::{http::StatusCode, response::IntoResponse};
use charted_core::api;
use std::{borrow::Cow, fmt::Display};

#[derive(Debug)]
pub enum Error {
    /// An unknown authentication type was received.
    UnknownAuthenticationType(Cow<'static, str>),

    /// An error occurred while decoding base64 data from user input.
    DecodeBase64(base64::DecodeError),

    /// Generic message that will be put into the `message` field of an API error.
    Message(Cow<'static, str>),

    /// Something in the database failed.
    Database(diesel::result::Error),

    /// Failed to decode from a ULID.
    DecodeUlid(charted_types::ulid::DecodeError),

    /// An error occured during JWT validation.
    Jwt(jsonwebtoken::errors::Error),

    /// An unknown error that hasn't been handled yet. It is most likely
    /// wrapped in a [`eyre::Report`].
    Unknown(eyre::Report),

    /// The request missed the `Authorization` HTTP header.
    MissingAuthorizationHeader,

    /// The token received was not the correct refresh token.
    RefreshTokenRequired,

    /// The password given from the basic authentication scheme was invalid.
    InvalidPassword,

    /// Session queried was unknown to the server.
    UnknownSession,
}

impl Error {
    pub(crate) fn invalid_utf8() -> Self {
        Error::msg("received invalid utf-8 content")
    }

    pub(crate) fn msg<I: Into<Cow<'static, str>>>(msg: I) -> Self {
        Error::Message(msg.into())
    }

    pub fn status_code(&self) -> StatusCode {
        use Error as E;

        match self {
            E::MissingAuthorizationHeader
            | E::UnknownAuthenticationType(_)
            | E::Message(_)
            | E::DecodeBase64(_)
            | E::RefreshTokenRequired
            | E::DecodeUlid(_) => StatusCode::NOT_ACCEPTABLE,

            E::InvalidPassword => StatusCode::UNAUTHORIZED,
            E::UnknownSession => StatusCode::NOT_FOUND,
            E::Database(_) | E::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            E::Jwt(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => StatusCode::FORBIDDEN,
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn api_error_code(&self) -> api::ErrorCode {
        use api::ErrorCode::*;
        use Error as E;

        match self {
            E::RefreshTokenRequired => RefreshTokenRequired,
            E::InvalidPassword => InvalidPassword,
            E::UnknownSession => UnknownSession,
            E::MissingAuthorizationHeader => MissingAuthorizationHeader,
            E::Message(_) => InvalidAuthorizationParts,
            E::UnknownAuthenticationType(_) => InvalidAuthenticationType,
            E::DecodeBase64(_) => UnableToDecodeBase64,
            E::DecodeUlid(_) => UnableToDecodeUlid,
            E::Database(_) | E::Unknown(_) => InternalServerError,
            E::Jwt(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => SessionExpired,
                jsonwebtoken::errors::ErrorKind::InvalidToken => InvalidSessionToken,
                _ => InternalServerError,
            },
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        <api::Response as From<Error>>::from(self).into_response()
    }
}

impl From<Error> for api::Response {
    fn from(value: Error) -> Self {
        api::err(value.status_code(), (value.api_error_code(), value.to_string()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error as E;

        match self {
            E::UnknownAuthenticationType(ty) => {
                let cx = ServerContext::get();

                #[allow(clippy::obfuscated_if_else)]
                let schemes = cx
                    .config
                    .sessions
                    .enable_basic_auth
                    .then_some("[Bearer, Basic, ApiKey]")
                    .unwrap_or("[Bearer, ApiKey]");

                write!(
                    f,
                    "received invalid authorization type [{ty}]: expected oneof {schemes}"
                )
            }

            E::MissingAuthorizationHeader => f.write_str("missing `Authorization` header from request"),
            E::RefreshTokenRequired => f.write_str("endpoint expected a valid refresh token"),
            E::InvalidPassword => f.write_str("received invalid password"),
            E::UnknownSession => f.write_str("unknown session"),
            E::Message(msg) => f.write_str(msg),
            E::DecodeBase64(err) => Display::fmt(err, f),
            E::DecodeUlid(err) => Display::fmt(err, f),
            E::Database(_) => f.write_str("database error: please report this if this is a common occurrence"),
            E::Unknown(e) => write!(
                f,
                "unknown error occurred, report this if this is a common occurrence: {e}"
            ),

            E::Jwt(err) => Display::fmt(err, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DecodeBase64(err) => Some(err),
            Error::Jwt(err) => Some(err),

            _ => None,
        }
    }
}

impl From<base64::DecodeError> for Error {
    fn from(value: base64::DecodeError) -> Self {
        Self::DecodeBase64(value)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(value)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Database(value)
    }
}

impl From<charted_types::ulid::DecodeError> for Error {
    fn from(value: charted_types::ulid::DecodeError) -> Self {
        Self::DecodeUlid(value)
    }
}

impl From<eyre::Report> for Error {
    fn from(value: eyre::Report) -> Self {
        Self::Unknown(value)
    }
}
