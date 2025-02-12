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

use axum::{http::StatusCode, response::IntoResponse};
use charted_core::api;
use charted_types::name;
use std::borrow::Cow;

/// Error type when the session middleware is running and it failed.
#[derive(Debug, derive_more::Display, derive_more::From, derive_more::Error)]
pub enum Error {
    #[display("unknown authentication type: {}", _0)]
    #[from(ignore)]
    UnknownAuthType(#[error(not(source))] Cow<'static, str>),
    DecodeBase64(base64::DecodeError),

    #[display("{}", _0)]
    Message(#[error(not(source))] Cow<'static, str>),

    DecodeUlid(charted_types::ulid::DecodeError),

    #[display("invalid input ({}) for Name: {}", input, error)]
    InvalidName {
        input: Cow<'static, str>,
        #[error(source)]
        error: name::Error,
    },

    Database(sea_orm::DbErr),
    Unknown(eyre::Report),
    Jwt(jsonwebtoken::errors::Error),

    #[display("request is missing the `Authorization` http header")]
    MissingAuthorizationHeader,

    #[display("refresh token is required for this route")]
    RefreshTokenRequired,

    #[display("invalid password given")]
    InvalidPassword,

    #[display("unknown session with id")]
    UnknownSession,
}

impl Error {
    pub(crate) fn invalid_utf8() -> Self {
        Error::msg("invalid utf-8 content")
    }

    pub(crate) fn msg<M: Into<Cow<'static, str>>>(msg: M) -> Self {
        Error::Message(msg.into())
    }

    pub fn status_code(&self) -> StatusCode {
        use Error as E;

        match self {
            E::MissingAuthorizationHeader
            | E::UnknownAuthType(_)
            | E::Message(_)
            | E::DecodeBase64(_)
            | E::RefreshTokenRequired
            | E::DecodeUlid(_)
            | E::InvalidName { .. } => StatusCode::NOT_ACCEPTABLE,

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
            E::UnknownAuthType(_) => InvalidAuthenticationType,
            E::DecodeBase64(_) => UnableToDecodeBase64,
            E::DecodeUlid(_) => UnableToDecodeUlid,
            E::Database(_) | E::Unknown(_) => InternalServerError,
            E::InvalidName { .. } => InvalidInput,
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
