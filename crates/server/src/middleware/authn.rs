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
mod extract;

use crate::{Env, ops};
use axum::{
    body::Body,
    http::{Request, Response, StatusCode, header::AUTHORIZATION},
    response::IntoResponse,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use charted_authz::InvalidPassword;
use charted_core::{
    BoxedFuture, api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
};
use charted_database::entities::{ApiKeyEntity, SessionEntity, UserEntity, apikey, session, user};
use charted_types::{ApiKey, User, name::Name};
use error::Error;
pub use extract::Session;
use jsonwebtoken::TokenData;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::{borrow::Cow, str::FromStr};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

/// Options that configures the [`Authn`] middleware.
#[derive(Clone, Default)]
pub struct Options {
    /// whether if this request requires a valid refresh token.
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

/// Factory trait that allows building new authn middleware
/// without cloning [`Env`] each time.
pub trait Factory {
    /// Creates a [`AsyncRequireAuthorizationLayer`] that will use the authn handler to
    /// handle authorization between in-flight requests.
    fn authn(&self, options: Options) -> AsyncRequireAuthorizationLayer<Authn>;
}

impl Factory for Env {
    fn authn(&self, options: Options) -> AsyncRequireAuthorizationLayer<Authn> {
        AsyncRequireAuthorizationLayer::new(Authn {
            options,
            env: self.clone(),
        })
    }
}

#[derive(Clone)]
pub struct Authn {
    env: Env,
    options: Options,
}

impl AsyncAuthorizeRequest<Body> for Authn {
    type Future = BoxedFuture<'static, Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>;
    type RequestBody = Body;
    type ResponseBody = Body;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        // it has to be cloned for each inflight request because Rust is so sensitive
        // that `&mut self` will outlive a static, allocated future but whatever
        // I guess.
        let me = self.clone();
        let env = me.env.clone();
        let options = me.options.clone();

        Box::pin(async move {
            let headers = request.headers();
            let Some(header) = headers.get(AUTHORIZATION) else {
                if options.allow_unauthorized {
                    return Ok(request);
                }

                bail!(Error::MissingAuthorizationHeader)
            };

            let Ok(value) = String::from_utf8(header.as_ref().to_vec()).inspect_err(|e| {
                error!(error = %e, "failed to validate UTF-8 contents in header");
            }) else {
                bail!(Error::invalid_utf8())
            };

            let (ty, value) = match value.split_once(' ') {
                Some((_, value)) if value.contains(' ') => {
                    let space = value.chars().position(|x| x == ' ').unwrap_or_default();
                    bail!(Error::msg(
                        format!("received extra space at {space} when parsing header"),
                        None,
                    ))
                }

                Some((ty, value)) => match ty.parse::<AuthType>() {
                    Ok(ty) => (ty, value),
                    Err(e) => {
                        bail!(Error::UnknownAuthType(Cow::Owned(e)))
                    }
                },

                None => {
                    bail!(Error::msg(
                        "auth header must be in the form of 'Type Value', i.e, 'ApiKey hjdjshdjs'",
                        None,
                    ));
                }
            };

            // sorted from most likely to unlikely
            match ty {
                AuthType::ApiKey => me.api_key_auth(env, request, value.to_owned()).await,
                AuthType::Bearer => me.bearer_auth(env, request, value.to_owned()).await,
                AuthType::Basic if env.config.sessions.enable_basic_auth => {
                    me.basic_auth(env, request, value.to_owned()).await
                }

                _ => bail!(api::err(
                    StatusCode::PRECONDITION_FAILED,
                    (
                        api::ErrorCode::UnsupportedAuthorizationKind,
                        "instance has disabled the use of `Basic` authentication",
                    ),
                )),
            }
        })
    }
}

impl Authn {
    #[instrument(
        name = "charted.server.authz.basic",
        skip_all,
        fields(req.uri = req.uri().path(), req.method = req.method().as_str())
    )]
    async fn basic_auth(
        &self,
        env: Env,
        mut req: Request<Body>,
        content: String,
    ) -> Result<Request<Body>, Response<Body>> {
        if self.options.require_refresh_token {
            bail!(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::RefreshTokenRequired,
                    "cannot use `Basic` authentication on this route",
                ),
            ))
        }

        let decoded = String::from_utf8(
            STANDARD
                .decode(&content)
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
                bail!(Error::msg("received more than one `:` in basic auth input", None));
            }

            Some(v) => v,
            None => {
                bail!(Error::msg("input must be in the form of 'username:password'", None));
            }
        };

        let username: Name = username
            .parse()
            .map_err(|e| Error::InvalidName {
                input: Cow::Owned(username.to_owned()),
                error: e,
            })
            .map_err(as_response)?;

        let (model, user): (_, User) = match UserEntity::find()
            .filter(user::Column::Username.eq(username.clone()))
            .one(&env.db)
            .await
            .map_err(|e| as_response(Error::Database(e)))
        {
            Ok(Some(model)) => (model.clone(), model.into()),
            Ok(None) => bail!(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with username doesn't exist",
                    json!({
                        "username": username,
                    }),
                ),
            )),

            Err(e) => return Err(e),
        };

        let email = model.email.to_string();
        let request = charted_authz::Request {
            password: Cow::Borrowed(password),
            user: user.clone(),
            model,
        };

        match env.authz.authenticate(request).await {
            Ok(()) => {
                sentry::configure_scope(|scope| {
                    scope.set_user(Some(sentry::User {
                        username: Some(user.username.as_str().to_owned()),
                        email: Some(email),
                        id: Some(user.id.to_string()),

                        ..Default::default()
                    }));
                });

                req.extensions_mut().insert(Session { session: None, user });
                Ok(req)
            }

            Err(e) if e.downcast_ref::<InvalidPassword>().is_some() => bail!(Error::InvalidPassword),
            Err(e) => {
                error!(error = %e, %user.username, %user.id, "failed to authenticate user from authz backend");
                sentry::capture_error(&*e);

                bail!(api::system_failure_from_report(e))
            }
        }
    }

    #[instrument(
        name = "charted.server.authz.bearer",
        skip_all,
        fields(req.uri = req.uri().path(), req.method = req.method().as_str())
    )]
    async fn bearer_auth(
        &self,
        env: Env,
        mut req: Request<Body>,
        content: String,
    ) -> Result<Request<Body>, Response<Body>> {
        let TokenData { claims, .. } =
            ops::jwt::decode_jwt(&env, &content).map_err(|e| as_response(Error::Jwt(e)))?;

        trace!("decoded JWT claims: {claims:?}");

        let Some(session) = SessionEntity::find_by_id(claims.sid)
            .filter(session::Column::Account.eq(claims.uid))
            .one(&env.db)
            .await
            .map_err(|e| as_response(Error::Database(e)))?
        else {
            bail!(Error::UnknownSession)
        };

        if self.options.require_refresh_token && session.refresh_token != content {
            bail!(Error::RefreshTokenRequired)
        }

        let Some(user) = UserEntity::find_by_id(claims.uid)
            .one(&env.db)
            .await
            .map_err(|e| as_response(Error::Database(e)))?
            .map(Into::<User>::into)
        else {
            return Err(as_response(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id doesn't exist",
                    json!({
                        "id": claims.uid,
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
        fields(req.uri = req.uri().path(), req.method = req.method().as_str())
    )]
    async fn api_key_auth(
        &self,
        env: Env,
        mut req: Request<Body>,
        content: String,
    ) -> Result<Request<Body>, Response<Body>> {
        if self.options.require_refresh_token {
            bail!(Error::RefreshTokenRequired)
        }

        let Some(apikey) = ApiKeyEntity::find()
            .filter(apikey::Column::Token.eq(content))
            .one(&env.db)
            .await
            .map_err(|e| as_response(Error::Database(e)))?
            .map(Into::<ApiKey>::into)
        else {
            bail!(api::err(
                StatusCode::NOT_FOUND,
                (api::ErrorCode::EntityNotFound, "api key from token was not found"),
            ))
        };

        let scopes = apikey.bitfield();
        for (scope, bit) in self.options.scopes.flags() {
            debug!(%apikey.name, %apikey.owner, %scope, "checking if api key has scope enabled");
            if !scopes.contains(bit) {
                debug!(%apikey.name, %apikey.owner, %scope, "user's api key doesn't have scope enabled");
                bail!(api::err(
                    StatusCode::FORBIDDEN,
                    (
                        api::ErrorCode::AccessNotPermitted,
                        "api key doesn't have required scope for this route enabled",
                        json!({
                            "scope": scope,
                            "repr": bit,
                        }),
                    ),
                ))
            }
        }

        let Some(user) = UserEntity::find_by_id(apikey.owner)
            .one(&env.db)
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

#[inline(never)]
#[cold] // responses are only created if errors are being used
fn as_response<R: IntoResponse>(e: R) -> Response<Body> {
    e.into_response()
}

/// A early-return bail for authn-usage only.
macro_rules! bail {
    ($($tt:tt)*) => {
        return ::core::result::Result::Err(as_response($($tt)*))
    };
}

pub(in crate::middleware::authn) use bail;

#[cfg(test)]
mod tests;
