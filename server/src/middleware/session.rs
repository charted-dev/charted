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
    models::{
        entities::{ApiKeyScope, ApiKeyScopes, User},
        NameOrSnowflake,
    },
    server::{hash_password, ARGON2},
};
use charted_config::{Config, ConfigExt};
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use tower_http::auth::AsyncAuthorizeRequest;

#[derive(Debug, Clone)]
pub enum SessionError {
    JsonWebToken(jsonwebtoken::errors::Error),
    Base64(base64::DecodeError),
    MissingAuthorizationHeader,
    UnknownAuthType(String),
    InvalidParts(String),

    #[allow(unused)]
    RefreshTokenRequired,
    InvalidPassword,
    UnknownSession,
    InvalidUtf8,
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

            SessionError::RefreshTokenRequired => f.write_str("refresh token is required for this route"),
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
            | SessionError::Base64(_)
            | SessionError::RefreshTokenRequired => StatusCode::NOT_ACCEPTABLE,
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
            | SessionError::RefreshTokenRequired => "REFRESH_TOKEN_REQUIRED",
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

/// Represents a [`AsyncAuthorizeRequest`] that does all the session handling
/// with a Tower layer.
#[derive(Clone, Default)]
pub struct SessionAuth {
    #[allow(unused)]
    allow_unauthenticated_requests: bool,
    require_refresh_token: bool,
    scopes: ApiKeyScopes<'static>,
}

impl SessionAuth {
    /// Sets the `allow_unauthenticated_requests` flag to determine if there is no Authorization
    /// header present, requests can still flow in or not.
    #[allow(unused)]
    pub fn allow_unauthenticated_requests(mut self) -> Self {
        self.allow_unauthenticated_requests = true;
        self
    }

    /// Sets the `require_refresh_token` flag to determine if the token used with the `Bearer`
    /// prefix is the refresh token or disallow any requests that are unauthenticated,
    /// or if they have the `ApiKey` or `Basic` token prefixes (if `config.sessions.allow_basic_auth` is
    /// set to `true`).
    pub fn require_refresh_token(mut self) -> Self {
        self.require_refresh_token = true;
        self
    }

    /// Adds a single [`ApiKeyScope`] to this session middleware.
    pub fn scope(mut self, scope: ApiKeyScope) -> Self {
        self.scopes.add([u64::from(scope)].iter()).unwrap();
        self
    }
}

/// Represents an extractor that extracts a user session, if there is one available.
#[derive(Debug, Clone)]
pub struct Session {
    /// The available [`Session`], if the authentication type was `Bearer`
    pub session: Option<charted_sessions::Session>,

    /// [`User`] that is authenticated.
    pub user: User,
}

/// Extension to grab the raw `Authorization` header.
#[derive(Debug, Clone)]
pub struct RawAuthHeader(pub String);
impl std::ops::Deref for RawAuthHeader {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper over [`axum::headers::Authorization`] that doesn't know
/// which type of authentication to use.
///
/// This was implemented since charted-server supports more than 3 authentication
/// types, and http's Authorization struct is not helpful to us.
#[derive(Clone)]
pub struct Authorization(HeaderValue);
impl Authorization {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a tuple of the structure: `(type, token)`.
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

        let config = Config::get();
        match ty {
            "Bearer" | "ApiKey" => Ok((ty.to_string(), token.to_string())),
            "Basic" if config.sessions.enable_basic_auth => Ok((ty.to_string(), token.to_string())),
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
        // using `self` in the async move block is fragile. which i mean by "fragile" is: it breaks
        // the 'static lifetime.
        //
        // do i know why? no
        // do i want to? no
        //
        // * since bools can be copied, we copy them
        // * since ApiKeyScopes cannot be copied BUT we can clone it, so we do that
        let allow_unauth_requests = self.allow_unauthenticated_requests;
        let require_refresh_token = self.require_refresh_token;
        let _scopes = self.scopes.clone();

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

            if header.is_empty() && allow_unauth_requests {
                return Ok(req);
            }

            let (ty, token) = header.to_tuple().map_err(|e| e.into_response())?;
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
            let config = Config::get();
            match ty.as_str() {
                "Basic" if config.sessions.enable_basic_auth => {
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
                            req.extensions_mut().insert(RawAuthHeader(token));
                        }

                        Err(_) => return Err(SessionError::InvalidPassword.into_response()),
                    }
                }

                "Bearer" => {
                    let span = info_span!(parent: &span, "charted.http.auth.bearer");
                    let _guard = span.enter();
                    let decoded = decode::<HashMap<String, Value>>(
                        &token,
                        &DecodingKey::from_secret(jwt_secret_key.as_ref()),
                        &Validation::new(Algorithm::HS512),
                    )
                    .map_err(|e| {
                        error!(%e, "unable to decode jwt token");
                        sentry::capture_error(&e);

                        SessionError::JsonWebToken(e).into_response()
                    })?;

                    let Some(Value::Number(uid)) = decoded.claims.get("user_id") else {
                        debug!("missing `user_id` in jwt token, not doing anything...");
                        return Err(SessionError::UnknownSession.into_response());
                    };

                    let id = uid.as_u64().ok_or_else(|| {
                        err(
                            StatusCode::UNPROCESSABLE_ENTITY,
                            ("INVALID_JWT_CLAIM", "Expected JWT claim [user_id] to be a u64").into(),
                        )
                        .into_response()
                    })?;

                    let Some(user) = sqlx::query_as::<_, User>("select users.* from users where id = $1;")
                        .bind(i64::try_from(id).unwrap())
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

                    let refresh_token = session.refresh_token.as_ref().unwrap();
                    if require_refresh_token && token.as_str() != refresh_token.as_str() {
                        return Err(SessionError::RefreshTokenRequired.into_response());
                    }

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
