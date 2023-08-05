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

#![allow(dead_code)]

use super::Metadata;
use crate::{models::res::err, Server};
use argon2::{PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use charted_common::models::{entities::User, NameOrSnowflake};
use charted_config::ConfigExt;
use charted_database::{
    controllers::{users::UserDatabaseController, DatabaseController},
    extensions::snowflake::SnowflakeExt,
};
use charted_sessions_local::{LocalSessionProvider, ARGON2};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Clone)]
pub enum SessionError {
    JsonWebToken(jsonwebtoken::errors::Error),
    InvalidParts(&'static str),
    MissingAuthorizationHeader,
    Argon2(argon2::Error),
    UnknownAuthType,
    InvalidPassword,
    UnknownSession,
    InvalidUtf8,
}

impl Debug for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::MissingAuthorizationHeader => f.write_str("missing `Authorization` header"),
            SessionError::UnknownAuthType => f.write_str("unknown authentication type received"),
            SessionError::UnknownSession => f.write_str("unknown session"),
            SessionError::InvalidParts(why) => {
                f.write_fmt(format_args!("received invalid parts in `Authorization` header: {why}"))
            }
            SessionError::JsonWebToken(err) => Debug::fmt(err, f),
            SessionError::Argon2(_) => f.write_str("Internal Server Error"),
            SessionError::InvalidPassword => f.write_str("invalid password specified"),
            SessionError::InvalidUtf8 => f.write_str("invalid utf-8 from header"),
        }
    }
}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::MissingAuthorizationHeader => f.write_str("missing `Authorization` header"),
            SessionError::UnknownAuthType => f.write_str("unknown authentication type received"),
            SessionError::UnknownSession => f.write_str("unknown session"),
            SessionError::InvalidParts(why) => {
                f.write_fmt(format_args!("received invalid parts in `Authorization` header: {why}"))
            }
            SessionError::JsonWebToken(err) => Debug::fmt(err, f),
            SessionError::Argon2(_) => f.write_str("Internal Server Error"),
            SessionError::InvalidPassword => f.write_str("invalid password specified"),
            SessionError::InvalidUtf8 => f.write_str("invalid utf-8 from header"),
        }
    }
}

impl std::error::Error for SessionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SessionError::JsonWebToken(err) => Some(err),
            _ => None,
        }
    }
}

impl SessionError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            SessionError::MissingAuthorizationHeader
            | SessionError::InvalidUtf8
            | SessionError::UnknownAuthType
            | SessionError::InvalidParts(_) => StatusCode::NOT_ACCEPTABLE,
            SessionError::Argon2(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SessionError::InvalidPassword => StatusCode::UNAUTHORIZED,
            SessionError::UnknownSession => StatusCode::NOT_FOUND,
            SessionError::JsonWebToken(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => StatusCode::FORBIDDEN,
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            SessionError::MissingAuthorizationHeader => "MISSING_AUTHORIZATION_HEADER",
            SessionError::InvalidPassword => "INVALID_PASSWORD",
            SessionError::InvalidUtf8 => "INVALID_UTF8_IN_HEADER",
            SessionError::Argon2(_) => "INTERNAL_SERVER_ERROR",
            SessionError::UnknownAuthType => "INVALID_AUTHENTICATION_TYPE",
            SessionError::UnknownSession => "UNKNOWN_SESSION",
            SessionError::InvalidParts(_) => "INVALID_AUTHORIZATION_PARTS",
            SessionError::JsonWebToken(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => "INVALID_SESSION_TOKEN",
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => "SESSION_EXPIRED",
                _ => "INTERNAL_SERVER_ERROR",
            },
        }
    }
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        err(self.status_code(), (self.code(), format!("{self}").as_str()).into()).into_response()
    }
}

/// Represents an extractor that extracts a user session, if there is one available.
#[derive(Debug, Clone)]
pub struct Session(pub charted_sessions::Session);

/// Extension to return the current [`User`] that is representing
/// this request.
#[derive(Debug, Clone)]
pub struct CurrentUser(pub User);

/// Middleware to optionally run the [`auth`] middleware. You will need to use
/// an `Option` when using the [`Session`] or [`CurrentUser`] extractors.
pub async fn optional_auth<B: Send>(
    State(server): State<Server>,
    metadata: Metadata,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    match metadata.headers.get(AUTHORIZATION) {
        Some(_) => auth(State(server), metadata, req, next).await,
        None => Ok(next.run(req).await),
    }
}

#[allow(clippy::await_holding_refcell_ref)]
pub async fn auth<B>(
    State(server): State<Server>,
    metadata: Metadata,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    // we know that this is dropped before the explicit `.await` call. for stylisic
    // reasons, i won't wrap this in a block.
    let mut sessions = server.sessions.borrow_mut();
    let config = server.config.clone();
    let jwt_secret_key = config.jwt_secret_key().map_err(|e| {
        error!(%e, "unable to parse secure setting");
        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
        .into_response()
    })?;

    let users = server.controller::<UserDatabaseController>();
    let auth = match metadata.headers.get(AUTHORIZATION) {
        Some(value) => value.to_str().map_err(|e| {
            error!(%e, "received invalid utf-8 characters when trying to parse `Authorization` header");
            sentry::capture_error(&e);

            SessionError::InvalidUtf8.into_response()
        })?,

        None => return Err(SessionError::MissingAuthorizationHeader.into_response()),
    };

    let (ty, token) = match auth.split_once(' ') {
        Some((_, token)) if token.contains(' ') => {
            return Err(SessionError::InvalidParts(
                "received more than once space, needs to be one space (i.e: [Bearer|Basic|ApiKey] 'token')",
            )
            .into_response())
        }
        Some((ty, token)) => match ty.to_lowercase().as_str() {
            "bearer" | "basic" | "apikey" => (ty, token),
            _ => {
                return Err(SessionError::InvalidParts(
                    "received invalid header type, expected 'Basic', 'Bearer', or 'ApiKey'",
                )
                .into_response())
            }
        },
        None => return Err(SessionError::InvalidParts("missing authorization type").into_response()),
    };

    match ty {
        "Bearer" => {
            let decoded = decode::<HashMap<String, String>>(
                token,
                &DecodingKey::from_secret(jwt_secret_key.as_ref()),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|e| {
                error!(%e, "unable to decode jwt token");
                sentry::capture_error(&e);

                SessionError::JsonWebToken(e).into_response()
            })?;

            let Some(user_id) = decoded.claims.get("user_id") else {
                return Err(SessionError::UnknownSession.into_response());
            };

            let id = user_id.parse::<u64>().map_err(|_| {
                err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ("UNABLE_TO_PROCESS", "Unable to process session due to invalid data").into(),
                )
                .into_response()
            })?;

            let user = users
                .get(id)
                .await
                .map_err(|_| SessionError::UnknownSession.into_response())?
                .ok_or_else(|| SessionError::UnknownSession.into_response())?;

            let session = sessions
                .from_user(id)
                .await
                .map_err(|_| SessionError::UnknownSession.into_response())?
                .ok_or_else(|| SessionError::UnknownSession.into_response())?;

            req.extensions_mut().insert(Session(session));
            req.extensions_mut().insert(CurrentUser(user));
        }

        "Basic" => {
            let (username, password) = match token.split_once(':') {
                Some((_, password)) if password.contains(':') => {
                    return Err(SessionError::InvalidParts("received more than one ':' in header value").into_response())
                }
                Some(tuple) => tuple,
                None => return Err(SessionError::InvalidParts("missing `:` in header value").into_response()),
            };

            let user = match users
                .get_with_id_or_name(NameOrSnowflake::Name(username.to_string()))
                .await
            {
                Ok(Some(user)) => user,
                Ok(None) => {
                    return Err(err(
                        StatusCode::NOT_FOUND,
                        ("UNKNOWN_USER", format!("Unable to find user {username}").as_str()).into(),
                    )
                    .into_response())
                }
                Err(e) => {
                    error!(%e, "unable to locate user");
                    //sentry::capture_error(&e);

                    return Err(err(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                    )
                    .into_response());
                }
            };

            let hashed = LocalSessionProvider::hash_password(password.into()).map_err(|e| {
                error!(%e, "unable to hash password");
                err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                )
                .into_response()
            })?;

            let hash = PasswordHash::new(&hashed).map_err(|e| {
                error!(%e, "unable to verify password");
                err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                )
                .into_response()
            })?;

            match ARGON2.verify_password(password.as_bytes(), &hash) {
                Ok(()) => {
                    req.extensions_mut().insert(CurrentUser(user));
                }

                Err(_) => return Err(SessionError::InvalidPassword.into_response()),
            }
        }

        "ApiKey" => return Err(SessionError::UnknownSession.into_response()),
        _ => unreachable!(),
    }

    // drop the RefMut, not the manager itself
    drop(sessions);
    Ok(next.run(req).await)
}
