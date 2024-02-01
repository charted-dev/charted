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

use crate::{
    auth,
    common::models::{
        entities::{ApiKeyScopes, User},
        Name,
    },
    db::controllers::DbController,
    server::models::res::{err, internal_server_error, ErrorCode, INTERNAL_SERVER_ERROR},
    Instance,
};
use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Request, Response, StatusCode},
    response::IntoResponse,
    RequestExt,
};
use axum_extra::{headers::Header, typed_header::TypedHeaderRejectionReason, TypedHeader};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::{json, Value};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{Debug, Display},
};
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::Instrument;
use uuid::Uuid;

static AUTHORIZATION: HeaderName = HeaderName::from_static("authorization");

/// Error type when a session is being conducted.
#[derive(Debug, Clone)]
pub enum Error {
    /// received an unknown authorization type,
    UnknownType { received: String },

    /// Received invalid parts to an authorization request. The `why` field tells
    /// a humane mesage on why it failed.
    InvalidParts { why: Cow<'static, str> },

    /// missing the `Authorization` header
    MissingHeader,

    /// Error that could occur when decoding base64 values.
    Base64(base64::DecodeError),

    /// Error that could occur when encoding/decoding JWT tokens.
    Jwt(jsonwebtoken::errors::Error),

    /// the bearer token needs to be a valid refresh token for this route.
    RefreshTokenRequired,

    /// received an invalid password
    InvalidPassword,

    /// received an unknown session
    UnknownSession,

    /// received invalid UTF-8 when decoding the authorization header value
    InvalidUtf8,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            UnknownType { received } => write!(f, "received an unknown Authorization type [{received}]"),
            InvalidParts { why } => write!(f, "received invalid parts to a given `Authorization` header: {why}"),
            RefreshTokenRequired => f.write_str("given route requests a valid refresh token to be passed in"),
            InvalidPassword => f.write_str("invalid password given"),
            UnknownSession => f.write_str("received invalid/unknown session"),
            MissingHeader => f.write_str("missing required `Authorization` header"),
            InvalidUtf8 => f.write_str("received invalid utf-8"),
            Base64(err) => Display::fmt(&err, f),
            Jwt(err) => Display::fmt(&err, f),
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

impl Error {
    pub fn status_code(&self) -> StatusCode {
        use Error::*;
        match self {
            MissingHeader
            | UnknownType { .. }
            | InvalidParts { .. }
            | Base64(_)
            | RefreshTokenRequired
            | InvalidUtf8 => StatusCode::NOT_ACCEPTABLE,

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
            InvalidParts { .. } => InvalidAuthorizationParts,
            UnknownType { .. } => InvalidAuthenticationType,
            Error::InvalidUtf8 => ErrorCode::InvalidUtf8,
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

/// Represents an extractor that extracts a user session, if there is one available.
#[derive(Debug, Clone)]
pub struct Session {
    /// The available [`Session`], if the authentication type was `Bearer`
    pub session: Option<crate::sessions::Session>,

    /// [`User`] that is authenticated.
    pub user: User,
}

/// Represents a [`AsyncAuthorizeRequest`] that does all the session handling with a simple tower layer.
#[derive(Clone, Default)]
pub struct Middleware {
    /// whether or not if the middleware should skip unauthenticated requests
    pub allow_unauthenticated_requests: bool,

    /// whether or not if the middleware should expect a valid refresh token
    pub require_refresh_token: bool,

    /// List of API key scopes to validate when a API key is sent.
    pub scopes: ApiKeyScopes<'static>,
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

    pub fn get(&self) -> Result<(&str, String), Error> {
        let header = self.0.to_owned();
        let value = String::from_utf8(header.as_ref().to_vec()).map_err(|e| {
            error!(error = %e, "received invalid UTF-8 characters when trying to parse header value");
            sentry::capture_error(&e);

            Error::InvalidUtf8
        })?;

        let instance = Instance::get();
        match value.split_once(' ') {
            Some((_, val)) if val.contains(' ') => Err(Error::InvalidParts {
                why: Cow::Borrowed("received more than one space"),
            }),
            Some(("Bearer", val)) => Ok(("Bearer", val.to_owned())),
            Some(("Basic", val)) if instance.config.sessions.enable_basic_auth => Ok(("Basic", val.to_owned())),
            Some(("ApiKey", val)) => Ok(("ApiKey", val.to_owned())),
            Some((ty, _)) => Err(Error::UnknownType {
                received: ty.to_string(),
            }),

            None => Err(Error::MissingHeader),
        }
    }
}

impl Header for Authorization {
    fn name() -> &'static axum::http::HeaderName {
        &AUTHORIZATION
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(HeaderValue::from_bytes(self.0.as_ref()).unwrap()));
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum_extra::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .map(|h| Authorization(h.clone()))
            .ok_or_else(axum_extra::headers::Error::invalid)
    }
}

impl AsyncAuthorizeRequest<Body> for Middleware {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut req: Request<Body>) -> Self::Future {
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
        let span = info_span!("charted.sessions.authorize");

        Box::pin(
            async move {
                let header = match req.extract_parts::<TypedHeader<Authorization>>().await {
                    Ok(header) => header,
                    Err(e) => {
                        error!(error = %e, "unable to extract `Authorization` header");
                        match e.reason() {
                            TypedHeaderRejectionReason::Missing if allow_unauth_requests => return Ok(req),
                            TypedHeaderRejectionReason::Missing => return Err(Error::MissingHeader.into_response()),
                            TypedHeaderRejectionReason::Error(e) => {
                                sentry::capture_error(&e);
                                return Err(err(
                                    StatusCode::NOT_ACCEPTABLE,
                                    (
                                        ErrorCode::InvalidHttpHeader,
                                        "received an invalid `Authorization` header",
                                    ),
                                )
                                .into_response());
                            }

                            _ => unreachable!(),
                        }
                    }
                };

                let (ty, token) = header.get().map_err(IntoResponse::into_response)?;
                let instance = Instance::get();
                let mut sessions = instance.sessions.lock().await;
                let span = info_span!(
                    "charted.authz",
                    req.uri = req.uri().path(),
                    req.method = req.method().as_str(),
                    auth.ty = ty
                );

                let _guard = span.enter();
                match ty {
                    "Basic" if instance.config.sessions.enable_basic_auth => {
                        if require_refresh_token {
                            return Err(err(StatusCode::NOT_ACCEPTABLE, (ErrorCode::RefreshTokenRequired, "cannot use `Basic` authentication on a Bearer-only REST route")).into_response());
                        }

                        let span = info_span!(parent: &span, "charted.authz.basic");
                        let _guard = span.enter();
                        let decoded = STANDARD.decode(&token).map_err(|e| {
                            error!(error = %e, "unable to decode base64 from Authorization header");
                            sentry::capture_error(&e);

                            Error::Base64(e).into_response()
                        })?;

                        let decoded = String::from_utf8(decoded).map_err(|e| {
                            error!(error = %e, "received invalid UTF-8 characters when trying to parse header value");
                            sentry::capture_error(&e);

                            Error::InvalidUtf8.into_response()
                        })?;

                        let (username, password) = match decoded.split_once(':') {
                            Some((_, password)) if password.contains(':') => {
                                let idx = password.chars().position(|c| c == ':').unwrap();
                                return Err(Error::InvalidParts {
                                    why: Cow::Owned(format!("received more than one colon @ {idx}")),
                                }
                                .into_response());
                            }

                            Some(tup) => tup,
                            None => return Err(Error::InvalidParts { why: Cow::Borrowed("`Basic` authentication requires the header value to be a base64 string of [username:password]") }.into_response()),
                        };

                        let name = Name::new(username).map_err(|e| {
                            error!(error = %e, username, "received invalid `Name` for username");
                            sentry::capture_error(&e);

                            Error::InvalidParts { why: Cow::Owned(e.to_string()) }.into_response()
                        })?;

                        let user = match instance.controllers.users.get_by(&name).await {
                            Ok(Some(user)) => user,
                            Ok(None) => return Err(err(StatusCode::NOT_FOUND, (ErrorCode::EntityNotFound, "user with given username was not found", json!({"username":name}))).into_response()),
                            Err(_) => return Err(err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR).into_response()),
                        };

                        match instance.authz.authenticate(user.clone(), password.to_string()).await {
                            Ok(()) => {
                                req.extensions_mut().insert(Session { session: None, user });
                            }

                            Err(auth::Error::InvalidPassword) => return Err(Error::InvalidPassword.into_response()),
                            Err(e) => {
                                error!(error = %e, user.id, "failed to authenticate from authz backend");
                                sentry::capture_error(&e);

                                return Err(internal_server_error().into_response());
                            }
                        }
                    }

                    "Bearer" => {
                        let span = info_span!(parent: &span, "charted.authz.bearer");
                        let _guard = span.enter();

                        let decoded = decode::<HashMap<String, Value>>(
                            &token,
                            &DecodingKey::from_secret(instance.config.jwt_secret_key.as_ref()),
                            &Validation::new(Algorithm::HS512)
                        ).map_err(|e| {
                            error!(error = %e, "unable to decode JWT token");
                            sentry::capture_error(&e);

                            Error::Jwt(e).into_response()
                        })?;

                        let Some(Value::Number(uid)) = decoded.claims.get("user_id") else {
                            warn!("JWT token didn't have a `user_id` claim, marking as an unknown session");
                            return Err(Error::UnknownSession.into_response());
                        };

                        let Some(Value::String(sess_id)) = decoded.claims.get("session_id") else {
                            warn!("JWT token doesn't have a `session_id` claim, marking as an unknown session");
                            return Err(Error::UnknownSession.into_response());
                        };

                        let sess_id = Uuid::parse_str(sess_id).map_err(|e| {
                            error!(error = %e, session.id = sess_id, "unable to parse `session_id` as a UUID");
                            sentry::capture_error(&e);

                            internal_server_error().into_response()
                        })?;

                        let id = uid.as_u64().ok_or_else(||
                            err(
                                StatusCode::UNPROCESSABLE_ENTITY,
                                (
                                    ErrorCode::InvalidJwtClaim,
                                    "[user_id] jwt claim was not a valid `u64` value",
                                    json!({"value":uid})
                                )
                            ).into_response()
                        )?;

                        let user = match instance.controllers.users.get(id.try_into().unwrap()).await {
                            Ok(Some(user)) => user,
                            Ok(None) => return Err(err(StatusCode::NOT_FOUND, (ErrorCode::EntityNotFound, "user with given id doesn't exist", json!({"id":id}))).into_response()),
                            Err(_) => return Err(internal_server_error().into_response())
                        };

                        let Some(session) = sessions.from_user(id, sess_id).await.map_err(|e| {
                            error!(error = %e, user.id, session.id = %sess_id, "unable to get session from Redis");
                            sentry_eyre::capture_report(&e);

                            Error::UnknownSession.into_response()
                        })? else {
                            return Err(Error::UnknownSession.into_response());
                        };

                        let refresh_token = session.refresh_token.as_ref().unwrap();
                        if require_refresh_token && token.as_str() != refresh_token.as_str() {
                            return Err(Error::RefreshTokenRequired.into_response());
                        }

                        req.extensions_mut().insert(Session { session: Some(session), user });
                    }

                    "ApiKey" => todo!(),
                    _ => unreachable!(),
                }

                Ok(req)
            }
            .instrument(span),
        )
    }
}
