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

use crate::{models::res::err, Server};
use argon2::{PasswordHash, PasswordVerifier};
use axum::{
    body::BoxBody,
    extract::{rejection::TypedHeaderRejectionReason, State},
    headers::Header,
    http::{header::AUTHORIZATION, HeaderName, HeaderValue, Request, Response, StatusCode},
    response::IntoResponse,
    RequestExt, TypedHeader,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use charted_common::{
    models::{entities::User, NameOrSnowflake},
    server::{hash_password, ARGON2},
};
use charted_config::ConfigExt;
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use tower_http::auth::AsyncAuthorizeRequest;

#[derive(Clone)]
pub enum SessionError {
    JsonWebToken(jsonwebtoken::errors::Error),
    Base64(base64::DecodeError),
    MissingAuthorizationHeader,
    UnknownAuthType(String),
    InvalidParts(String),
    InvalidPassword,
    UnknownSession,
    InvalidUtf8,
}

impl Debug for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::MissingAuthorizationHeader => f.write_str("missing `Authorization` header"),
            SessionError::UnknownAuthType(t) => f.write_fmt(format_args!(
                "unknown authentication type received: '{t}'; expected [Bearer, ApiKey, Basic]"
            )),
            SessionError::UnknownSession => f.write_str("unknown session"),
            SessionError::InvalidParts(why) => {
                f.write_fmt(format_args!("received invalid parts in `Authorization` header: {why}"))
            }
            SessionError::JsonWebToken(err) => Debug::fmt(err, f),
            SessionError::Base64(err) => Debug::fmt(err, f),
            SessionError::InvalidPassword => f.write_str("invalid password specified"),
            SessionError::InvalidUtf8 => f.write_str("invalid utf-8 from header"),
        }
    }
}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::MissingAuthorizationHeader => f.write_str("missing `Authorization` header"),
            SessionError::UnknownAuthType(t) => f.write_fmt(format_args!(
                "unknown authentication type received: '{t}'; expected [Bearer, ApiKey, Basic]"
            )),
            SessionError::UnknownSession => f.write_str("unknown session"),
            SessionError::InvalidParts(why) => {
                f.write_fmt(format_args!("received invalid parts in `Authorization` header: {why}"))
            }
            SessionError::Base64(err) => Display::fmt(err, f),
            SessionError::JsonWebToken(err) => Display::fmt(err, f),
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
            | SessionError::UnknownAuthType(_)
            | SessionError::InvalidParts(_)
            | SessionError::Base64(_) => StatusCode::NOT_ACCEPTABLE,
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
            SessionError::Base64(_) => "UNABLE_TO_DECODE_BASE64",
            SessionError::MissingAuthorizationHeader => "MISSING_AUTHORIZATION_HEADER",
            SessionError::InvalidPassword => "INVALID_PASSWORD",
            SessionError::InvalidUtf8 => "INVALID_UTF8_IN_HEADER",
            SessionError::UnknownAuthType(_) => "INVALID_AUTHENTICATION_TYPE",
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
    fn into_response(self) -> axum::response::Response {
        err(self.status_code(), (self.code(), format!("{self}").as_str()).into()).into_response()
    }
}

#[derive(Clone)]
pub struct SessionAuth;

/// Represents an extractor that extracts a user session, if there is one available.
#[derive(Debug, Clone)]
pub struct Session {
    pub session: Option<charted_sessions::Session>,
    pub user: User,
}

/// Wrapper over [`axum::headers::Authorization`] that doesn't know
/// which type of authentication to use.
///
/// This was implemented since charted-server supports more than 3 authentication
/// types, and http's Authorization struct is not helpful to us.
#[derive(Clone)]
pub struct Authorization(HeaderValue);
impl Authorization {
    pub fn to_tuple(&self) -> Result<(String, String), SessionError> {
        let header = self.0.to_owned();
        let value = String::from_utf8(header.as_ref().to_vec()).map_err(|e| {
            error!(%e, "received invalid utf-8 chars when trying to parse header value");
            sentry::capture_error(&e);

            SessionError::InvalidUtf8
        })?;

        let mut iter = value.split(' ');
        let Some(ty) = iter.next() else {
            return Err(SessionError::MissingAuthorizationHeader);
        };

        let Some(token) = iter.next() else {
            return Err(SessionError::InvalidParts("missing the token itself?!".into()));
        };

        match ty {
            "Basic" | "Bearer" | "ApiKey" => Ok((ty.to_string(), token.to_string())),
            _ => Err(SessionError::UnknownAuthType(ty.to_string())),
        }
    }
}

impl Header for Authorization {
    fn name() -> &'static HeaderName {
        &AUTHORIZATION
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, axum::headers::Error> {
        values
            .next()
            .map(|val| Authorization(val.clone()))
            .ok_or_else(axum::headers::Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(self.0.clone()));
    }
}

impl<B> AsyncAuthorizeRequest<B> for SessionAuth
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = BoxBody;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut req: Request<B>) -> Self::Future {
        Box::pin(async move {
            let header = req
                .extract_parts::<TypedHeader<Authorization>>()
                .await
                .map_err(|e| match e.reason() {
                    TypedHeaderRejectionReason::Missing => SessionError::MissingAuthorizationHeader.into_response(),
                    TypedHeaderRejectionReason::Error(e) => {
                        error!(%e, "unable to decode `Authorization` header");
                        sentry::capture_error(&e);

                        err(
                            StatusCode::NOT_ACCEPTABLE,
                            ("INVALID_HTTP_HEADER", "Received invalid `Authorization` header.").into(),
                        )
                        .into_response()
                    }

                    _ => unreachable!(),
                })?;

            let (ty, token) = header.0.to_tuple().map_err(|e| e.into_response())?;
            let State(server) = req
                .extract_parts::<State<Server>>()
                .await
                .expect("unable to grab server state");

            let mut sessions = server.sessions.write().await;
            let config = server.config.clone();
            let pool = server.pool.clone();
            let jwt_secret_key = config.jwt_secret_key().map_err(|e| {
                error!(setting = "config.jwt_secret_key", %e, "unable to parse secure setting");

                err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                )
                .into_response()
            })?;

            let span = info_span!(
                "charted.http.authentication",
                req.uri = req.uri().path(),
                req.method = req.method().as_str(),
                auth.ty = ty,
            );

            let _guard = span.enter();
            match ty.as_str() {
                "Basic" => {
                    let span = info_span!(parent: &span, "charted.http.auth.basic");
                    let _guard = span.enter();

                    let decoded = STANDARD.decode(&token).map_err(|e| {
                        error!(%e, "unable to decode base64 from Authorization header");
                        sentry::capture_error(&e);

                        SessionError::Base64(e).into_response()
                    })?;

                    let decoded = String::from_utf8(decoded).map_err(|e| {
                        error!(%e, "received invalid utf-8 chars when trying to parse header value");
                        sentry::capture_error(&e);

                        SessionError::InvalidUtf8.into_response()
                    })?;

                    let (username, password) = match decoded.split_once(':') {
                        Some((_, password)) if password.contains(':') => {
                            let idx = password.chars().position(|c| c == ':').unwrap();
                            return Err(SessionError::InvalidParts(format!(
                                "received more than once ':' [index {idx}]"
                            ))
                            .into_response());
                        }

                        Some(tuple) => tuple,
                        None => {
                            return Err(SessionError::InvalidParts(
                                "missing ':' in header value while decoding b64 header value".into(),
                            )
                            .into_response())
                        }
                    };

                    let name = NameOrSnowflake::Name(username.into());
                    match name.is_valid() {
                        Ok(()) => {}
                        Err(why) => {
                            error!(reason = why, "received invalid Name parameter");
                            return Err(SessionError::InvalidParts(why.to_string()).into_response());
                        }
                    }

                    let Some(user) = sqlx::query_as::<_, User>("select users.* from users where username = $1;")
                        .bind(username.to_string())
                        .fetch_optional(&pool)
                        .await
                        .map_err(|e| {
                            error!(user = username, %e, "failed to fetch user");
                            sentry::capture_error(&e);

                            err(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                            )
                            .into_response()
                        })?
                    else {
                        return Err(err(
                            StatusCode::NOT_FOUND,
                            ("UNKNOWN_USER", format!("unknown user with name '{username}'").as_str()).into(),
                        )
                        .into_response());
                    };

                    let hashed = hash_password(password.into()).map_err(|e| {
                        error!(%e, "unable to hash password");
                        err(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                        )
                        .into_response()
                    })?;

                    let hash = PasswordHash::new(&hashed).map_err(|e| {
                        error!(%e, "unable to create password hasher");
                        err(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                        )
                        .into_response()
                    })?;

                    match ARGON2.verify_password(password.as_bytes(), &hash) {
                        Ok(()) => {
                            req.extensions_mut().insert(Session { session: None, user });
                        }

                        Err(_) => return Err(SessionError::InvalidPassword.into_response()),
                    }
                }

                "Bearer" => {
                    let span = info_span!(parent: &span, "charted.http.auth.bearer");
                    let _guard = span.enter();

                    let decoded = decode::<HashMap<String, String>>(
                        &token,
                        &DecodingKey::from_secret(jwt_secret_key.as_ref()),
                        &Validation::new(Algorithm::HS512),
                    )
                    .map_err(|e| {
                        error!(%e, "unable to decode jwt token");
                        sentry::capture_error(&e);

                        SessionError::JsonWebToken(e).into_response()
                    })?;

                    let Some(uid) = decoded.claims.get("user_id") else {
                        debug!("missing `user_id` in jwt token, not doing anything...");
                        return Err(SessionError::UnknownSession.into_response());
                    };

                    let id = uid.parse::<u64>().map_err(|_| {
                        err(
                            StatusCode::UNPROCESSABLE_ENTITY,
                            ("INVALID_JWT_CLAIM", "Expected JWT claim [user_id] to be a u64").into(),
                        )
                        .into_response()
                    })?;

                    let Some(user) = sqlx::query_as::<_, User>("select users.* from users where id = $1;")
                        .bind(id as i64)
                        .fetch_optional(&pool)
                        .await
                        .map_err(|e| {
                            error!(id, %e, "failed to fetch user with");
                            sentry::capture_error(&e);

                            err(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                            )
                            .into_response()
                        })?
                    else {
                        return Err(err(
                            StatusCode::NOT_FOUND,
                            ("UNKNOWN_USER", format!("unknown user with ID [{id}]").as_str()).into(),
                        )
                        .into_response());
                    };

                    let session = sessions
                        .from_user(id)
                        .await
                        .map_err(|_| SessionError::UnknownSession.into_response())?
                        .ok_or_else(|| SessionError::UnknownSession.into_response())?;

                    req.extensions_mut().insert(Session {
                        session: Some(session),
                        user,
                    });
                }

                _ => unreachable!(),
            }

            Ok(req)
        })
    }
}
