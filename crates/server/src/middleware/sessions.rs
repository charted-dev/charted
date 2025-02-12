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
use charted_authz::InvalidPassword;
pub use error::*;

mod extract;
pub use extract::*;

use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, StatusCode},
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use charted_core::{
    api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
    BoxedFuture,
};
use charted_database::entities::{user, UserEntity};
use charted_types::{name::Name, User};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::{borrow::Cow, str::FromStr};
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{error, instrument};

use crate::Context;

pub const JWT_ISS: &str = "Noelware";
pub const JWT_AUD: &str = "charted-server";

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
}

/*
impl Middleware {
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
*/

impl AsyncAuthorizeRequest<Body> for Middleware {
    type ResponseBody = Body;
    type RequestBody = Body;
    type Future = BoxedFuture<'static, Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        let ctx = Context::get();
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

        let me = self.to_owned();
        match ty {
            AuthType::Basic => Box::pin(me.basic_auth(ctx, request, value.to_owned())),
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::set;
//     use axum::{http::Response, routing, Router};
//     use azalia::remi::StorageService;
//     use charted_config::{sessions::Backend, Config};
//     use sea_orm::DatabaseConnection;
//     use serde::de::DeserializeOwned;
//     use std::sync::{atomic::AtomicUsize, Arc};
//     use tower::{Service, ServiceExt};
//     use tower_http::auth::AsyncRequireAuthorizationLayer;

//     fn config(basic_auth: bool) -> Config {
//         Config {
//             jwt_secret_key: Default::default(),
//             registrations: Default::default(),
//             base_url: Default::default(),
//             database: Default::default(),
//             logging: Default::default(),
//             sentry_dsn: Default::default(),
//             server: Default::default(),
//             sessions: charted_config::sessions::Config {
//                 enable_basic_auth: basic_auth,
//                 backend: Backend::Local,
//             },
//             single_org: Default::default(),
//             single_user: Default::default(),
//             storage: Default::default(),
//             tracing: Default::default(),
//         }
//     }

//     fn create_context(basic_auth: bool) -> Context {
//         let ctx = Context {
//             requests: AtomicUsize::new(0),
//             storage: StorageService::__non_exhaustive,
//             config: config(basic_auth),
//             authz: Arc::new(charted_authz_static::Backend::new(azalia::btreemap! {
//                 // echo "noeliscutieuwu" | cargo cli admin authz hash-password --stdin
//                 "noel" => "$argon2id$v=19$m=19456,t=2,p=1$gIcVA4mVHgr8ZWkmDrtJlw$sb5ypFAvphFCGrJXy9fRI1Gb/2vGIH1FTzDax458+xY"
//             })),

//             pool: DatabaseConnection::Disconnected,
//         };

//         set(ctx.clone());
//         ctx
//     }

//     #[tokio::test]
//     async fn disallow_basic_auth() {
//         let mut router = router(Middleware::default(), false);
//         let mut service = router.as_service::<Body>();

//         let service = service.ready().await.unwrap();
//         let res = service
//             .call(Request::post("/echo").body(Body::empty()).unwrap())
//             .await
//             .unwrap();

//         assert_eq!(res.status(), StatusCode::NOT_ACCEPTABLE);

//         // let body = consume_body::<api::Response>(res.into_body()).await;
//         // assert_eq!(body, Error::MissingAuthorizationHeader.into());

//         let res = service
//             .call(
//                 Request::post("/echo")
//                     .header(AUTHORIZATION, "Basic bm9lbDpub2VsaXNjdXRpZXV3dQo=")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         dbg!(consume_body::<api::Response>(res.into_body()).await);
//         // assert_eq!(res.status(), StatusCode::NOT_ACCEPTABLE);
//         // assert_eq!(
//         //     consume_body::<api::Response>(res.into_body()).await,
//         //     api::Response {
//         //         status: StatusCode::NOT_ACCEPTABLE,
//         //         success: false,
//         //         data: None,
//         //         errors: vec![(
//         //             api::ErrorCode::RefreshTokenRequired,
//         //             "cannot use `Basic` authentication on this route",
//         //         )
//         //             .into()]
//         //     }
//         // );
//     }

//     #[tokio::test]
//     async fn allow_unauthorized_requests() {
//         let mut router = router(
//             Middleware {
//                 allow_unauthorized_requests: true,
//                 ..Default::default()
//             },
//             false,
//         );

//         let mut service = router.as_service::<Body>();
//         let request = Request::post("/echo").body(Body::empty()).unwrap();

//         let res = service.ready().await.unwrap().call(request).await.unwrap();
//         assert_eq!(res.status(), StatusCode::OK);
//     }

//     async fn echo(req: axum::extract::Request) -> impl IntoResponse {
//         (StatusCode::OK, Response::new(req.into_body()))
//     }

//     async fn consume_body<T: DeserializeOwned>(body: Body) -> T {
//         let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
//         serde_json::from_slice(&bytes).unwrap()
//     }

//     fn router(middleware: Middleware, basic_auth: bool) -> Router<()> {
//         Router::new()
//             .route(
//                 "/echo",
//                 routing::post(echo).layer(AsyncRequireAuthorizationLayer::new(middleware)),
//             )
//             .with_state(create_context(basic_auth))
//     }
// }
