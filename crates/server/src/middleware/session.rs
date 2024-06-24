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

mod error;
mod extractor;
pub use extractor::*;

use crate::ServerContext;
use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
    response::IntoResponse,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use charted_authz::InvalidPasswordError;
use charted_common::BoxedFuture;
use charted_core::response::{err, internal_server_error, ErrorCode};
use charted_database::controllers::{session, users, DbController};
use charted_entities::{ApiKeyScopes, Name};
use error::Error;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde_json::{json, Value};
use sqlx::types::Uuid;
use std::{borrow::Cow, collections::HashMap, str::FromStr};
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{error, instrument, warn};

pub const JWT_ISS: &str = "Noelware/charted-server";

/// Represents the tower middleware for charted's sessions system.
#[derive(Clone, Default)]
pub struct Middleware {
    /// whether to allow unauthorized requests to the route.
    pub allow_unauthorized_requests: bool,

    /// if a refresh token is required
    pub refresh_token_required: bool,

    /// list of scopes the route requires if [`AuthType`] is [`ApiKey`][AuthType::ApiKey].
    pub scopes: ApiKeyScopes,
}

impl Middleware {
    #[instrument(name = "charted.authz.basic", skip_all)]
    async fn on_basic_auth(
        self,
        mut req: Request<Body>,
        ctx: &ServerContext,
        token: String,
    ) -> Result<Request<Body>, Response<Body>> {
        if self.refresh_token_required {
            return Err(err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    ErrorCode::RefreshTokenRequired,
                    "cannot use `Basic` authentication on a Bearer-only route",
                ),
            )
            .into_response());
        }

        let decoded = String::from_utf8(
            STANDARD
                .decode(&token)
                .inspect_err(|e| {
                    error!(error = %e, "unable to decode base64 from `Authorization` header");
                    sentry::capture_error(e);
                })
                .map_err(|e| Error::Base64(e).into_response())?,
        )
        .map_err(|_| Error::invalid_utf8().into_response())?;

        let (username, password) = match decoded.split_once(':') {
            Some((_, pass)) if pass.contains(':') => {
                let idx = pass.chars().position(|c| c == ':').unwrap_or_default();
                return Err(Error::message(format!(
                    "received more than one colon in `Basic` authentication value in pos {idx}"
                ))
                .into_response());
            }

            Some(tup) => tup,
            None => {
                return Err(Error::message(
                    "`Basic` authentication requires the header to be in the form of 'username:password'",
                )
                .into_response())
            }
        };

        let username = username
            .parse::<Name>()
            .map_err(|e| Error::message(e.to_string()).into_response())?;

        let users = ctx.controllers.get::<users::DbController>().unwrap();
        let user = match users.get_by(&username).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(err(
                    StatusCode::NOT_FOUND,
                    (
                        ErrorCode::EntityNotFound,
                        "user with username doesn't exist",
                        json!({"username":username}),
                    ),
                )
                .into_response())
            }

            Err(_) => return Err(internal_server_error().into_response()),
        };

        match ctx.authz.authenticate(&user, password.to_owned()).await {
            Ok(()) => {
                req.extensions_mut().insert(Session { session: None, user });
                Ok(req)
            }

            Err(e) => {
                // if an authz backend reports a invalid password error, then use
                // a "Invalid Password" response instead of a Internal Server Error one.
                if e.downcast_ref::<InvalidPasswordError>().is_some() {
                    return Err(Error::InvalidPassword.into_response());
                }

                error!(error = %e, user.id, "failed to authenticate from authz backend");
                sentry::capture_error(&*e);

                Err(internal_server_error().into_response())
            }
        }
    }

    #[instrument(name = "charted.authz.bearer", skip_all)]
    async fn on_bearer_auth(
        self,
        mut req: Request<Body>,
        ctx: &ServerContext,
        token: String,
    ) -> Result<Request<Body>, Response<Body>> {
        let decode_key = DecodingKey::from_secret(ctx.config.jwt_secret_key.as_ref());
        let decoded =
            jsonwebtoken::decode::<HashMap<String, Value>>(&token, &decode_key, &Validation::new(Algorithm::HS512))
                .inspect_err(|e| {
                    error!(error = %e, "failed to decode JWT token");
                    sentry::capture_error(e);
                })
                .map_err(|e| Error::Jwt(e).into_response())?;

        let Some(Value::Number(uid)) = decoded.claims.get("user_id") else {
            warn!("JWT token was missing `user_id` claim");
            return Err(Error::UnknownSession.into_response());
        };

        let uid = uid.as_i64().ok_or_else(|| internal_server_error().into_response())?;
        let Some(Value::String(sess)) = decoded.claims.get("session_id") else {
            warn!("JWT token was missing `session_id` claim");
            return Err(Error::UnknownSession.into_response());
        };

        let session = sess.parse::<Uuid>().map_err(|e| {
            error!(error = %e, "received invalid uuid for session id");
            err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    ErrorCode::InvalidJwtClaim,
                    "`session_id` JWT claim was not a valid UUID",
                ),
            )
            .into_response()
        })?;

        let controller = ctx.controllers.get::<session::DbController>().unwrap();
        let Some(session) = controller
            .get(session)
            .await
            .map_err(|_| internal_server_error().into_response())?
        else {
            return Err(Error::UnknownSession.into_response());
        };

        // for clarity, let's assert the user is the same as the session it claims
        if session.user_id != uid {
            error!(
                session.user_id,
                user.id = uid,
                "FATAL: failed to assert `session.user_id` == `uid` -- things are being tampered?!"
            );

            return Err(Error::UnknownSession.into_response());
        }

        if self.refresh_token_required && session.refresh_token.as_ref().unwrap() != &token {
            return Err(Error::RefreshTokenRequired.into_response());
        }

        let users = ctx.controllers.get::<users::DbController>().unwrap();
        let user = match users.get(uid).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(err(
                    StatusCode::NOT_FOUND,
                    (
                        ErrorCode::EntityNotFound,
                        "user with id doesn't exist",
                        json!({"id":uid}),
                    ),
                )
                .into_response())
            }

            Err(_) => return Err(internal_server_error().into_response()),
        };

        req.extensions_mut().insert(Session {
            session: Some(session),
            user,
        });

        Ok(req)
    }

    async fn on_apikey_auth(
        self,
        _req: Request<Body>,
        _ctx: &ServerContext,
        _token: String,
    ) -> Result<Request<Body>, Response<Body>> {
        todo!()
    }
}

/*
                    "ApiKey" => {
                        if require_refresh_token {
                            return Err(err(StatusCode::NOT_ACCEPTABLE, (ErrorCode::RefreshTokenRequired, "cannot use `ApiKey` authentication on a Bearer-only REST route")).into_response());
                        }

                        let span = info_span!(parent: &span, "charted.authz.apikey");
                        let _guard = span.enter();

                        let Some(apikey) = sqlx::query_as::<sqlx::Postgres, ApiKey>("select api_keys.* from api_keys where token = ?")
                            .bind(token)
                            .fetch_optional(&instance.pool)
                            .await
                            .map_err(|e| {
                                error!(error = %e, "failed to find API key");
                                sentry::capture_error(&e);

                                internal_server_error().into_response()
                            })? else {
                                return Err(err(
                                    StatusCode::NOT_FOUND,
                                    (ErrorCode::EntityNotFound, "api key was not found with associated token")
                                ).into_response());
                            };

                        let scopes = apikey.bitfield();
                        for (scope, value) in scopes.flags() {
                            trace!(%apikey.name, apikey.id, "checking if scope [{scope}] is enabled ({value})");
                            if !scopes.contains(*value) {
                                trace!(%apikey.name, apikey.id, flag = *scope, "...flag is not enabled");
                                return Err(err(
                                    StatusCode::FORBIDDEN,
                                    (
                                        ErrorCode::AccessNotPermitted,
                                        "access to route is not permitted since required flag is not enabled",
                                        json!({"flag":*scope})
                                    )
                                ).into_response());
                            }
                        }

                        let user = match instance.controllers.users.get(apikey.owner).await {
                            Ok(Some(user)) => user,
                            Ok(None) => return Err(err(StatusCode::NOT_FOUND, (ErrorCode::EntityNotFound, "user with given id doesn't exist", json!({"id":apikey.owner}))).into_response()),
                            Err(_) => return Err(internal_server_error().into_response())
                        };

                        req.extensions_mut().insert(Session { session: None, user });
                    }

                    _ => unreachable!(),
                }

                Ok(req)
            }
            .instrument(span),
        )
    }
}

*/

impl AsyncAuthorizeRequest<Body> for Middleware {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = BoxedFuture<'static, Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>;

    #[instrument(
        name = "charted.authz.middleware",
        skip_all,
        fields(
            request.uri = request.uri().path(),
            request.method = request.method().as_str(),
            request.version = ?request.version()
        )
    )]
    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        let ctx = ServerContext::get();
        let headers = request.headers();
        let Some(header) = headers.get(AUTHORIZATION) else {
            if self.allow_unauthorized_requests {
                return Box::pin(async move { Ok(request) });
            }

            return Box::pin(async move { Err(Error::MissingHeader.into_response()) });
        };

        let Ok(value) = String::from_utf8(header.as_ref().to_vec()).inspect_err(|err| {
            error!(error = %err, "failed to validate utf-8 when parsing header value");
            sentry::capture_error(err);
        }) else {
            return Box::pin(async move { Err(Error::message("received invalid utf-8").into_response()) });
        };

        let Some((ty, value)) = value.split_once(' ') else {
            return Box::pin(async move {
                Err(Error::message("when looking at `Authorization` header, received extra space(s)").into_response())
            });
        };

        if value.contains(' ') {
            return Box::pin(async move {
                Err(Error::message("when looking at `Authorization` header, received extra space(s)").into_response())
            });
        }

        let ty = match ty.parse::<AuthType>() {
            Ok(ty) => ty,
            Err(ty) => return Box::pin(async move { Err(Error::UnknownAuthType(Cow::Owned(ty)).into_response()) }),
        };

        match ty {
            AuthType::ApiKey => Box::pin(Middleware::on_apikey_auth(self.clone(), request, ctx, value.to_owned())),
            AuthType::Bearer => Box::pin(Middleware::on_bearer_auth(self.clone(), request, ctx, value.to_owned())),
            AuthType::Basic if ctx.config.sessions.enable_basic_auth => {
                Box::pin(Middleware::on_basic_auth(self.clone(), request, ctx, value.to_owned()))
            }

            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthType {
    /// Unknown authentication type received from `Authorization` header.
    Unknown(String),

    /// `Bearer` authentication type, typically a JWT token that was *possibly*
    /// created by the server.
    Bearer,

    /// `ApiKey` authentication type, created from the API keys API.
    ApiKey,

    /// `Basic` authentication type, if enabled; allows to send HTTP requests
    /// with basic credentials (`user:pass` in b64).
    Basic,
}

impl AuthType {
    /// Returns a slice of the avaliable [`AuthType`]s. If `basic` is false, then [`AuthType::Basic`]
    /// will not be avaliable.
    pub const fn values(basic: bool) -> &'static [AuthType] {
        if basic {
            &[AuthType::ApiKey, AuthType::Basic, AuthType::Bearer]
        } else {
            &[AuthType::ApiKey, AuthType::Bearer]
        }
    }
}

impl FromStr for AuthType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_ascii_lowercase() {
            "apikey" => Ok(Self::ApiKey),
            "bearer" => Ok(Self::Bearer),
            "basic" => Ok(Self::Basic),
            _ => Err(s.to_owned()),
        }
    }
}
