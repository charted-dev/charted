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

mod error;
pub use error::*;

mod extract;
pub use extract::*;

use crate::Context;
use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, StatusCode},
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use charted_authz::InvalidPassword;
use charted_core::{
    api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
    BoxedFuture,
};
use charted_database::entities::{apikey, session, user, ApiKeyEntity, SessionEntity, UserEntity};
use charted_types::{name::Name, ApiKey, Ulid, User};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::{json, Value};
use std::{borrow::Cow, collections::HashMap, str::FromStr};
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{error, instrument, trace};

pub const JWT_ISS: &str = "Noelware";
pub const JWT_AUD: &str = "charted-server";
pub const JWT_ALGORITHM: Algorithm = Algorithm::HS512;
pub const JWT_UID_FIELD: &str = "uid";
pub const JWT_SID_FIELD: &str = "sid";

/// The middleware that can be attached to a single route.
#[derive(Clone, Default)]
pub struct Middleware {
    /// whether if the middleware should continue running without
    /// an `Authorization` header.
    pub allow_unauthorized_requests: bool,

    /// whether if a refresh token is required to be the input
    /// of a `Bearer` token.
    pub refresh_token_required: bool,

    /// a list of API key scopes. If none are specified, then it'll
    /// run like normal.
    pub scopes: ApiKeyScopes,
}

// SETTERS \\

impl Middleware {
    /// Appends a new [`ApiKeyScope`] for this middleware to check.
    pub fn with_scope<S: Into<ApiKeyScope>>(self, scope: S) -> Self {
        let mut bitfield = self.scopes;
        bitfield.add([scope.into()]);

        Self {
            scopes: bitfield,
            ..self
        }
    }
}

// IMPL \\
impl Middleware {
    #[instrument(
        name = "charted.server.authz.basic",
        skip_all,
        fields(req.uri = req.uri().path(), req.method = req.method().as_str())
    )]
    pub(self) async fn basic_auth<'ctx>(
        self,
        ctx: &'ctx Context,
        mut req: Request<Body>,
        contents: String,
    ) -> Result<Request<Body>, Response> {
        if self.refresh_token_required {
            return Err(as_response(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::RefreshTokenRequired,
                    "cannot use `Basic` authentication on this route",
                ),
            )));
        }

        if !ctx.config.sessions.enable_basic_auth {
            return Err(as_response(api::err(
                StatusCode::BAD_REQUEST,
                (
                    api::ErrorCode::BadRequest,
                    "instance has disabled the use of `Basic` authentication",
                ),
            )));
        }

        let decoded = String::from_utf8(
            STANDARD
                .decode(&contents)
                .inspect_err(|e| {
                    error!(error = %e, "failed to decode from base64");
                    sentry::capture_error(e);
                })
                .map_err(Error::DecodeBase64)
                .map_err(as_response)?,
        )
        .map_err(|_| Error::invalid_utf8())
        .map_err(as_response)?;

        let (username, password) = match decoded.split_once(':') {
            Some((_, pass)) if pass.contains(':') => {
                return Err(as_response(Error::msg(
                    "received more than one `:` in basic auth input",
                )))
            }

            Some(v) => v,
            None => {
                return Err(as_response(Error::msg(
                    "input must be in the form of 'username:password'",
                )))
            }
        };

        let user: Name = username
            .parse()
            .map_err(|e| Error::InvalidName {
                input: Cow::Owned(username.to_owned()),
                error: e,
            })
            .map_err(as_response)?;

        let Some(model) = UserEntity::find()
            .filter(user::Column::Username.eq(user))
            .one(&ctx.pool)
            .await
            .map_err(Error::Database)
            .map_err(as_response)?
        else {
            return Err(as_response(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with username doesn't exist",
                    json!({
                        "username": username,
                    }),
                ),
            )));
        };

        let auser: User = model.clone().into();
        match ctx
            .authz
            .authenticate(charted_authz::Request {
                user: auser.clone(),
                password: Cow::Borrowed(password),
                model,
            })
            .await
        {
            Ok(()) => {
                req.extensions_mut().insert(Session {
                    session: None,
                    user: auser,
                });

                Ok(req)
            }

            Err(e) => {
                if e.downcast_ref::<InvalidPassword>().is_some() {
                    return Err(as_response(Error::InvalidPassword));
                }

                error!(error = %e, "failed to authenticate from authz backend");
                sentry::capture_error(&*e);

                Err(as_response(api::system_failure_from_report(e)))
            }
        }
    }

    #[instrument(
        name = "charted.server.authz.bearer",
        skip_all,
        fields(
            req.uri = req.uri().path(),
            req.method = req.method().as_str(),
        )
    )]
    pub(self) async fn bearer_auth<'ctx>(
        self,
        ctx: &'ctx Context,
        mut req: Request<Body>,
        token: String,
    ) -> Result<Request<Body>, Response> {
        let key = DecodingKey::from_secret(ctx.config.jwt_secret_key.as_ref());
        let decoded = jsonwebtoken::decode::<HashMap<String, Value>>(&token, &key, &Validation::new(JWT_ALGORITHM))
            .inspect_err(|e| {
                error!(error = %e, "failed to decode JWT token");
                sentry::capture_error(e);
            })
            .map_err(Error::Jwt)
            .map_err(as_response)?;

        // All tokens created by us will always have `uid` and `sid` fields.
        let uid = decoded
            .claims
            .get(JWT_UID_FIELD)
            .filter(|x| matches!(x, Value::String(_)))
            .and_then(Value::as_str)
            .map(Ulid::new)
            .ok_or_else(|| as_response(Error::msg(format!("missing `{}` JWT claim", JWT_UID_FIELD))))?
            .map_err(Error::DecodeUlid)
            .map_err(as_response)?;

        let sid = decoded
            .claims
            .get(JWT_SID_FIELD)
            .filter(|x| matches!(x, Value::String(_)))
            .and_then(Value::as_str)
            .map(Ulid::new)
            .ok_or_else(|| as_response(Error::msg(format!("missing `{}` JWT claim", JWT_SID_FIELD))))?
            .map_err(Error::DecodeUlid)
            .map_err(as_response)?;

        let Some(session) = SessionEntity::find_by_id(sid)
            .filter(session::Column::Account.eq(uid))
            .one(&ctx.pool)
            .await
            .map_err(Error::Database)
            .map_err(as_response)?
        else {
            return Err(as_response(Error::UnknownSession));
        };

        if self.refresh_token_required && session.refresh_token != token {
            return Err(as_response(Error::RefreshTokenRequired));
        }

        let Some(user) = UserEntity::find_by_id(uid)
            .one(&ctx.pool)
            .await
            .map_err(Error::Database)
            .map_err(as_response)?
            .map(Into::<User>::into)
        else {
            return Err(as_response(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id doesn't exist",
                    json!({
                        "id": uid,
                    }),
                ),
            )));
        };

        req.extensions_mut().insert(Session {
            session: Some(session.into()),
            user,
        });

        Ok(req)
    }

    #[instrument(
        name = "charted.server.authz.apikey",
        skip_all,
        fields(
            req.uri = req.uri().path(),
            req.method = req.method().as_str(),
        )
    )]
    async fn apikey_auth<'ctx>(
        self,
        ctx: &'ctx Context,
        mut req: Request<Body>,
        token: String,
    ) -> Result<Request<Body>, Response> {
        if self.refresh_token_required {
            return Err(as_response(Error::RefreshTokenRequired));
        }

        let Some(apikey) = ApiKeyEntity::find()
            .filter(apikey::Column::Token.eq(token))
            .one(&ctx.pool)
            .await
            .map_err(Error::Database)
            .map_err(as_response)?
            .map(Into::<ApiKey>::into)
        else {
            return Err(as_response(api::err(
                StatusCode::NOT_FOUND,
                (api::ErrorCode::EntityNotFound, "api key from token was not found?"),
            )));
        };

        let scopes = apikey.bitfield();
        for (scope, bit) in self.scopes.flags() {
            trace!(%apikey.name, %scope, "checking if api key enabled the scope required for this route");
            if !scopes.contains(bit) {
                trace!(%apikey.name, %scope, "scope is not enabled!");
                return Err(as_response(api::err(
                    StatusCode::FORBIDDEN,
                    (
                        api::ErrorCode::AccessNotPermitted,
                        "api key doesn't have required scope for this route enabled",
                        json!({
                            "scope": scope,
                            "repr": bit,
                        }),
                    ),
                )));
            }
        }

        let Some(user) = UserEntity::find_by_id(apikey.owner)
            .one(&ctx.pool)
            .await
            .map_err(Error::Database)
            .map_err(as_response)?
            .map(Into::<User>::into)
        else {
            return Err(as_response(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id doesn't exist",
                    json!({
                        "id": apikey.owner,
                    }),
                ),
            )));
        };

        req.extensions_mut().insert(Session { session: None, user });
        Ok(req)
    }
}

impl AsyncAuthorizeRequest<Body> for Middleware {
    type ResponseBody = Body;
    type RequestBody = Body;
    type Future = BoxedFuture<'static, Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        let headers = request.headers();

        let Some(header) = headers.get(AUTHORIZATION) else {
            if self.allow_unauthorized_requests {
                return Box::pin(noop(request));
            }

            return Box::pin(async { Err(as_response(Error::MissingAuthorizationHeader)) });
        };

        let Ok(value) = String::from_utf8(header.as_ref().to_vec()).inspect_err(|err| {
            error!(error = %err, "failed to validate UTF-8 contents in header");
            sentry::capture_error(err);
        }) else {
            return Box::pin(async { Err(as_response(Error::invalid_utf8())) });
        };

        let (ty, value) = match value.split_once(' ') {
            Some((_, value)) if value.contains(' ') => {
                let space = value.chars().position(|x| x == ' ').unwrap_or_default();
                return Box::pin(async move {
                    Err(as_response(Error::msg(format!(
                        "received extra space at {space} when parsing header"
                    ))))
                });
            }

            Some((ty, value)) => match ty.parse::<AuthType>() {
                Ok(ty) => (ty, value),
                Err(e) => return Box::pin(async { Err(as_response(Error::UnknownAuthType(Cow::Owned(e)))) }),
            },

            None => {
                return Box::pin(async {
                    Err(as_response(Error::msg(
                        "auth header must be in the form of 'Type Value', i.e, 'ApiKey hjdjshdjs'",
                    )))
                })
            }
        };

        let ctx = Context::get();
        let me = self.to_owned();
        match ty {
            AuthType::Basic => Box::pin(me.basic_auth(ctx, request, value.to_owned())),
            AuthType::Bearer => Box::pin(me.bearer_auth(ctx, request, value.to_owned())),

            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthType {
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

async fn noop(request: Request<Body>) -> Result<Request<Body>, Response<Body>> {
    Ok(request)
}

#[inline(never)]
#[cold]
fn as_response<R: IntoResponse>(e: R) -> Response {
    e.into_response()
}

#[cfg(test)]
mod tests;
