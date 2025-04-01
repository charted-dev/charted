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

mod extract;
pub use extract::*;

mod error;

use crate::Context;
use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse, Response},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use charted_authz::InvalidPassword;
use charted_core::{
    BoxedFuture, api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
};
use charted_database::entities::{ApiKeyEntity, SessionEntity, UserEntity, apikey, session, user};
use charted_types::{ApiKey, Ulid, User, name::Name};
use error::Error;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::{Value, json};
use std::{borrow::Cow, collections::HashMap, str::FromStr};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

/// The `iss` field value.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.1>
pub const JWT_ISS: &str = "Noelware";

/// The `aud` field value.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.3>
pub const JWT_AUD: &str = "charted-server";

/// JWT algorithm to use.
pub const JWT_ALGORITHM: Algorithm = Algorithm::HS512;

/// JWT claim for the expiration.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.4>
pub const JWT_EXP: &str = "exp";

/// Claim name for a user ID.
pub const JWT_UID_FIELD: &str = "uid";

/// Claim name for a session ID.
pub const JWT_SID_FIELD: &str = "sid";

#[derive(Clone, Default)]
pub struct Options {
    /// whether if the rest endpoint requires a valid refresh token
    pub require_refresh_token: bool,

    /// whether if the rest endpoint is ok with no prior authorization.
    pub allow_unauthorized: bool,

    /// a list of api key scopes that the middleware should check
    /// if it was ran from the api key path
    pub scopes: ApiKeyScopes,
}

impl Options {
    /// Append a scope to this middleware.
    pub fn with_scope(mut self, scope: impl Into<ApiKeyScope>) -> Self {
        self.scopes.add([scope.into()]);
        self
    }
}

/// Creates a [`AsyncRequireAuthorizationLayer`] that will use the authn handler to
/// handle authorization between in-flight requests.
pub fn new(ctx: Context, options: Options) -> AsyncRequireAuthorizationLayer<Handler> {
    AsyncRequireAuthorizationLayer::new(Handler { options, context: ctx })
}

#[derive(Clone)]
pub struct Handler {
    options: Options,
    context: Context,
}

impl Handler {
    #[instrument(
        name = "charted.server.authn.basic",
        skip_all,
        fields(req.uri = req.uri().path(), req.method = req.method().as_str())
    )]
    async fn handle_basic_auth(
        self,
        ctx: Context,
        mut req: Request<Body>,
        token: String,
    ) -> Result<Request<Body>, Response> {
        if self.options.require_refresh_token {
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
                .decode(&token)
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
                    None,
                )));
            }

            Some(v) => v,
            None => {
                return Err(as_response(Error::msg(
                    "input must be in the form of 'username:password'",
                    None,
                )));
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
        name = "charted.server.authn.bearer",
        skip_all,
        fields(req.uri = req.uri().path(), req.method = req.method().as_str())
    )]
    async fn handle_bearer_auth(
        self,
        ctx: Context,
        mut req: Request<Body>,
        token: String,
    ) -> Result<Request<Body>, Response> {
        let key = DecodingKey::from_secret(ctx.config.jwt_secret_key.as_ref());
        let validation = create_validation();
        let decoded = jsonwebtoken::decode::<HashMap<String, Value>>(&token, &key, &validation)
            .inspect_err(|e| {
                error!(error = %e, "failed to decode JWT token");
                sentry::capture_error(e);
            })
            .map_err(Error::Jwt)
            .map_err(as_response)?;

        trace!("JWT claims: {:?}", decoded.claims);

        // All tokens created by us will always have `uid` and `sid` fields.
        let uid = decoded
            .claims
            .get(JWT_UID_FIELD)
            .filter(|x| matches!(x, Value::String(_)))
            .and_then(Value::as_str)
            .map(Ulid::new)
            .ok_or_else(|| {
                as_response(Error::msg(
                    format!("missing `{}` JWT claim", JWT_UID_FIELD),
                    Some(api::ErrorCode::InvalidJwtClaim),
                ))
            })?
            .map_err(Error::DecodeUlid)
            .map_err(as_response)?;

        let sid = extract_sid_from_token(&decoded.claims).map_err(as_response)?;

        let Some(session) = SessionEntity::find_by_id(sid)
            .filter(session::Column::Account.eq(uid))
            .one(&ctx.pool)
            .await
            .map_err(Error::Database)
            .map_err(as_response)?
        else {
            return Err(as_response(Error::UnknownSession));
        };

        if self.options.require_refresh_token && session.refresh_token != token {
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
        name = "charted.server.authn.apikey",
        skip_all,
        fields(req.uri = req.uri().path(), req.method = req.method().as_str())
    )]
    async fn handle_apikey_auth(
        self,
        ctx: Context,
        mut req: Request<Body>,
        token: String,
    ) -> Result<Request<Body>, Response> {
        if self.options.require_refresh_token {
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
        for (scope, bit) in self.options.scopes.flags() {
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

impl AsyncAuthorizeRequest<Body> for Handler {
    type Future = BoxedFuture<'static, Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>;
    type RequestBody = Body;
    type ResponseBody = Body;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        let headers = request.headers();
        let Some(header) = headers.get(AUTHORIZATION) else {
            if self.options.allow_unauthorized {
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
                    Err(as_response(Error::msg(
                        format!("received extra space at {space} when parsing header"),
                        None,
                    )))
                });
            }

            Some((ty, value)) => match ty.parse::<AuthType>() {
                Ok(ty) => (ty, value),
                Err(e) => {
                    return Box::pin(async { Err(as_response(Error::UnknownAuthType(Cow::Owned(e)))) });
                }
            },

            None => {
                return Box::pin(async {
                    Err(as_response(Error::msg(
                        "auth header must be in the form of 'Type Value', i.e, 'ApiKey hjdjshdjs'",
                        None,
                    )))
                });
            }
        };

        let context = self.context.clone();
        let me = self.clone();
        match ty {
            AuthType::Basic => Box::pin(me.handle_basic_auth(context, request, value.to_owned())),
            AuthType::Bearer => Box::pin(me.handle_bearer_auth(context, request, value.to_owned())),
            AuthType::ApiKey => Box::pin(me.handle_apikey_auth(context, request, value.to_owned())),
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
    /// Returns a slice of the avaliable [`AuthType`]s. If `basic` is false, then
    /// [`AuthType::Basic`] will not be avaliable.
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

/// Extracts the `sid` field, which in returns the session ID.
// while the lint is correct, in this case, we can't do `Box<...>`
// since that's not the signature that we expect
#[allow(clippy::result_large_err)]
pub fn extract_sid_from_token(claims: &HashMap<String, Value>) -> Result<Ulid, Error> {
    claims
        .get(JWT_SID_FIELD)
        .filter(|x| matches!(x, Value::String(_)))
        .and_then(Value::as_str)
        .map(Ulid::new)
        .ok_or_else(|| {
            Error::msg(
                format!("missing `{}` JWT claim", JWT_SID_FIELD),
                Some(api::ErrorCode::InvalidJwtClaim),
            )
        })?
        .map_err(Error::DecodeUlid)
}

pub fn create_validation() -> Validation {
    let mut validation = Validation::new(JWT_ALGORITHM);
    validation.set_audience(&[JWT_AUD]);
    validation.set_issuer(&[JWT_ISS]);
    validation.set_required_spec_claims(&[JWT_EXP, "aud", "iss"]);

    validation
}

#[inline(never)]
#[cold]
fn as_response<R: IntoResponse>(e: R) -> Response {
    e.into_response()
}

#[cfg(test)]
mod tests;
