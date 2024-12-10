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
pub use error::*;

mod extract;
pub use extract::*;

use crate::{ops, ServerContext};
use axum::{
    body::Body,
    extract::Request,
    http::{header::AUTHORIZATION, Response, StatusCode},
    response::IntoResponse,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use charted_authz::InvalidPassword;
use charted_core::{
    api::{self, internal_server_error},
    bitflags::{ApiKeyScope, ApiKeyScopes},
    BoxedFuture,
};
use charted_database::{
    connection,
    schema::{postgresql, sqlite},
};
use charted_types::{name::Name, Ulid};
use diesel::sqlite::Sqlite;
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::{json, Value};
use std::{borrow::Cow, collections::HashMap, str::FromStr};
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{error, instrument, trace};

pub const JWT_ISS: &str = "Noelware";
pub const JWT_AUS: &str = "charted-server";

#[derive(Clone, Default)]
pub struct Middleware {
    allow_unauthorized_requests: bool,
    refresh_token_required: bool,
    scopes: ApiKeyScopes,
}

impl Middleware {
    pub fn allow_unauthorized_requests(self, yes: bool) -> Self {
        Self {
            allow_unauthorized_requests: yes,
            ..self
        }
    }

    pub fn refresh_token_required(self, yes: bool) -> Self {
        Self {
            refresh_token_required: yes,
            ..self
        }
    }

    pub fn scopes<I: IntoIterator<Item = ApiKeyScope>>(self, scopes: I) -> Self {
        let mut bitfield = self.scopes;
        bitfield.add(scopes);

        Self {
            scopes: bitfield,
            ..self
        }
    }
}

impl Middleware {
    /// Performs basic authentication if the server has enabled it.
    #[instrument(
        name = "charted.server.authz.basic",
        skip_all,
        fields(
            req.uri = req.uri().path(),
            req.method = req.method().as_str()
        )
    )]
    async fn basic_auth(
        self,
        mut req: Request<Body>,
        ctx: &ServerContext,
        token: String,
    ) -> Result<Request<Body>, Response<Body>> {
        if self.refresh_token_required {
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::RefreshTokenRequired,
                    "cannot use basic authentication scheme on this route",
                ),
            )
            .into_response());
        }

        let decoded = String::from_utf8(
            STANDARD
                .decode(&token)
                .inspect_err(|e| {
                    error!(error = %e, "failed to decode base64 from authorization header");
                    sentry::capture_error(e);
                })
                .map_err(Error::DecodeBase64)
                .map_err(IntoResponse::into_response)?,
        )
        .map_err(|_| Error::invalid_utf8())
        .map_err(IntoResponse::into_response)?;

        let (username, password) = match decoded.split_once(':') {
            Some((_, pass)) if pass.contains(':') => {
                let idx = pass.chars().position(|c| c == ':').unwrap_or_default();
                return Err(Error::msg(format!("received more than one ':' @ pos {idx}")).into_response());
            }

            Some(tuple) => tuple,
            None => {
                return Err(
                    Error::msg("basic authentication requires the syntax of 'username:password'").into_response(),
                )
            }
        };

        let user = username
            .parse::<Name>()
            .map_err(|e| Error::msg(e.to_string()).into_response())?;

        let Some(user) = ops::db::user::get(ctx, user.clone())
            .await
            .map_err(Error::Unknown)
            .map_err(IntoResponse::into_response)?
        else {
            return Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id doesn't exist",
                    json!({"username":user}),
                ),
            )
            .into_response());
        };

        match ctx.authz.authenticate(&user, password.to_owned()).await {
            Ok(()) => {
                req.extensions_mut().insert(extract::Session { session: None, user });
                Ok(req)
            }

            Err(e) => {
                if e.downcast_ref::<InvalidPassword>().is_some() {
                    return Err(Error::InvalidPassword.into_response());
                }

                error!(error = %e, "failed to authenticate from authz backend");
                sentry::capture_error(&*e);

                Err(internal_server_error().into_response())
            }
        }
    }

    /// Performs JWT-based authentication for `Bearer` tokens.
    #[instrument(name = "charted.server.authz.bearer", skip_all)]
    async fn bearer_auth(
        self,
        mut req: Request<Body>,
        ctx: &ServerContext,
        token: String,
    ) -> Result<Request<Body>, Response<Body>> {
        let key = DecodingKey::from_secret(ctx.config.jwt_secret_key.as_ref());
        let decoded = jsonwebtoken::decode::<HashMap<String, Value>>(
            &token,
            &key,
            &Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .inspect_err(|e| {
            error!(error = %e, "failed to decode JWT token");
            sentry::capture_error(e);
        })
        .map_err(Error::Jwt)
        .map_err(IntoResponse::into_response)?;

        // All JWT tokens created by the server will always have a `user_id` which
        // will always be a valid ULID.
        let uid = decoded
            .claims
            .get("user_id")
            .filter(|x| matches!(x, Value::String(_)))
            .and_then(Value::as_str)
            .map(Ulid::new)
            .ok_or_else(|| Error::msg("missing `user_id` JWT claim").into_response())?
            .map_err(Error::DecodeUlid)
            .map_err(IntoResponse::into_response)?;

        let session = decoded
            .claims
            .get("session_id")
            .filter(|x| matches!(x, Value::String(_)))
            .and_then(Value::as_str)
            .map(Ulid::new)
            .ok_or_else(|| Error::msg("missing `session_id` JWT claim").into_response())?
            .map_err(Error::DecodeUlid)
            .map_err(IntoResponse::into_response)?;

        let mut conn = ctx
            .pool
            .get()
            .inspect_err(|e| {
                error!(error = %e, "failed to get database connection");
                sentry::capture_error(e);
            })
            .map_err(|_| api::internal_server_error().into_response())?;

        let session = connection!(@raw conn {
            PostgreSQL(conn) => conn.build_transaction().read_only().run::<charted_types::Session, Error, _>(|txn| {
                use postgresql::sessions::{dsl::*, table};
                use diesel::pg::Pg;

                table
                    .select(<charted_types::Session as SelectableHelper<Pg>>::as_select())
                    .filter(owner.eq(uid))
                    .filter(id.eq(&session))
                    .first(txn)
                    .map_err(Into::into)
            });

            SQLite(conn) => conn.immediate_transaction(|txn| {
                use sqlite::sessions::{dsl::*, table};

                table
                    .select(<charted_types::Session as SelectableHelper<Sqlite>>::as_select())
                    .filter(owner.eq(uid))
                    .filter(id.eq(&session))
                    .first(txn)
                    .map_err(Into::into)
            });
        })
        .map_err(|e| match e {
            Error::Database(diesel::result::Error::NotFound) => api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "session with id doesn't exist",
                    json!({"session":session}),
                ),
            )
            .into_response(),

            err => err.into_response(),
        })?;

        if session.owner != uid {
            error!("FATAL: assertion of `session.owner` == {uid} failed");
            return Err(Error::UnknownSession.into_response());
        }

        if self.refresh_token_required && session.refresh_token != token {
            return Err(Error::RefreshTokenRequired.into_response());
        }

        let Some(user) = ops::db::user::get(ctx, uid)
            .await
            .map_err(Error::Unknown)
            .map_err(IntoResponse::into_response)?
        else {
            return Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id doesn't exist",
                    json!({"user":uid}),
                ),
            )
            .into_response());
        };

        req.extensions_mut().insert(Session {
            session: Some(session),
            user,
        });

        Ok(req)
    }

    /// Performs API Key-based authentication
    #[instrument(name = "charted.server.authz.apikey", skip_all)]
    async fn apikey_auth(
        self,
        mut req: Request<Body>,
        ctx: &ServerContext,
        token: String,
    ) -> Result<Request<Body>, Response<Body>> {
        if self.refresh_token_required {
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::RefreshTokenRequired,
                    "cannot use api key authentication on a bearer-only route",
                ),
            )
            .into_response());
        }

        let Some(apikey) = ops::db::apikey::get(ctx, token, None::<!>)
            .await
            .map_err(Error::Unknown)
            .map_err(IntoResponse::into_response)?
        else {
            return Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "api key with received token was not found",
                ),
            )
            .into_response());
        };

        let scopes = apikey.bitfield();
        for (scope, bit) in self.scopes.flags() {
            trace!(%apikey.name, "checking if api key has scope [{scope}] enabled");
            if !scopes.contains(bit) {
                trace!(%apikey.name, %scope, "api key scope is not enabled");
                return Err(api::err(
                    StatusCode::FORBIDDEN,
                    (
                        api::ErrorCode::AccessNotPermitted,
                        "api key doesn't have access to this route due to not enabling the required flag",
                        json!({"scope":scope,"$repr":bit}),
                    ),
                )
                .into_response());
            }
        }

        let Some(user) = ops::db::user::get(ctx, apikey.owner)
            .await
            .map_err(Error::Unknown)
            .map_err(IntoResponse::into_response)?
        else {
            return Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id doesn't exist",
                    json!({"user":apikey.owner}),
                ),
            )
            .into_response());
        };

        req.extensions_mut().insert(extract::Session { session: None, user });
        Ok(req)
    }
}

impl AsyncAuthorizeRequest<Body> for Middleware {
    type ResponseBody = Body;
    type RequestBody = Body;
    type Future = BoxedFuture<'static, Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: axum::http::Request<Body>) -> Self::Future {
        let ctx = ServerContext::get();
        let headers = request.headers();

        let Some(header) = headers.get(AUTHORIZATION) else {
            if self.allow_unauthorized_requests {
                return Box::pin(noop(request));
            }

            return Box::pin(error(Error::MissingAuthorizationHeader));
        };

        let Ok(value) = String::from_utf8(header.as_ref().to_vec()).inspect_err(|err| {
            error!(error = %err, "failed to validate UTF-8 contents in header");
            sentry::capture_error(err);
        }) else {
            return Box::pin(error(Error::invalid_utf8()));
        };

        let (ty, value) = match value.split_once(' ') {
            Some((_, value)) if value.contains(' ') => {
                let space = value.chars().position(|x| x == ' ').unwrap_or_default();
                return Box::pin(error(Error::msg(format!(
                    "received extra space at {space} when parsing header"
                ))));
            }

            Some((ty, value)) => match ty.parse::<AuthType>() {
                Ok(ty) => (ty, value),
                Err(e) => return Box::pin(error(Error::UnknownAuthenticationType(Cow::Owned(e)))),
            },

            None => {
                return Box::pin(error(Error::msg(
                    "auth header must be in the form of 'Type Value', i.e, 'ApiKey hjdjshdjs'",
                )))
            }
        };

        match ty {
            AuthType::Bearer => Box::pin(self.clone().bearer_auth(request, ctx, value.to_owned())),
            AuthType::ApiKey => Box::pin(self.clone().apikey_auth(request, ctx, value.to_owned())),
            AuthType::Basic => Box::pin(self.clone().basic_auth(request, ctx, value.to_owned())),
        }
    }
}

async fn noop(request: Request<Body>) -> Result<Request<Body>, Response<Body>> {
    Ok(request)
}

#[cold]
async fn error(error: Error) -> Result<Request<Body>, Response<Body>> {
    Err(error.into_response())
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
