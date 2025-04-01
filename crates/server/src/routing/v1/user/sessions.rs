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

use crate::{
    Context,
    extract::Json,
    extract_refor_t,
    middleware::authn::{self, JWT_ALGORITHM, JWT_AUD, JWT_ISS, Session},
    modify_property,
    openapi::ApiErrorResponse,
};
use axum::{Extension, extract::State, http::StatusCode};
use charted_authz::InvalidPassword;
use charted_core::api;
use charted_database::entities::{SessionEntity, UserEntity, session, user};
use charted_types::{
    User,
    payloads::{Login, UserLoginPayload},
};
use chrono::{DateTime, TimeZone, Utc, naive::Days};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde_json::json;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
};
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{Ref, RefOr, Response},
};
use validator::ValidateEmail;

struct LoginR;
impl IntoResponses for LoginR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_schema_name("SessionResponse"),
            "403" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("invalid password"));

                response
            },

            "404" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("user was not found by their username or email"));

                response
            },

            "406" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("`email` was not formatted properly"));

                response
            }
        }
    }
}

/// Creates a new session.
#[utoipa::path(
    post,

    path = "/v1/users/login",
    tags = ["Users", "Users/Sessions"],
    operation_id = "login",
    request_body(
        content = ref("#/components/schemas/UserLoginPayload"),
        content_type = "application/json"
    ),
    responses(LoginR)
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn login(
    State(cx): State<Context>,
    Json(UserLoginPayload { login, password }): Json<UserLoginPayload>,
) -> api::Result<charted_types::Session> {
    let Some((model, user)) = (match login {
        Login::Username(ref name) => UserEntity::find().filter(user::Column::Username.eq(name.clone())),
        Login::Email(ref email) => {
            if !email.validate_email() {
                return Err(api::err(
                    StatusCode::NOT_ACCEPTABLE,
                    (api::ErrorCode::ValidationFailed, "invalid email address"),
                ));
            }

            UserEntity::find().filter(user::Column::Email.eq(email.clone()))
        }
    })
    .one(&cx.pool)
    .await
    .inspect_err(|e| {
        error!(error = %e, ?login, "failed to query user");
        sentry::capture_error(e);
    })
    .map_err(api::system_failure)?
    .map(|model| (model.clone(), Into::<User>::into(model))) else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (api::ErrorCode::EntityNotFound, "user was not found"),
        ));
    };

    let request = charted_authz::Request {
        user: user.clone(),
        model,
        password: Cow::Owned(password),
    };

    cx.authz.authenticate(request).await.map_err(|e| {
        if e.is::<InvalidPassword>() {
            return api::err(
                StatusCode::FORBIDDEN,
                (api::ErrorCode::InvalidPassword, "invalid password given"),
            );
        }

        error!(error = %e, %user.id, "failed to complete authentication from backend");
        sentry::capture_error(&*e);

        api::system_failure_from_report(e)
    })?;

    // create session
    let id = cx
        .ulid_generator
        .generate()
        .inspect_err(|e| {
            error!(error = %e, %user.id, "failed to generate session id");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    let now = Utc::now();
    let access_token = jsonwebtoken::encode(
        &Header {
            alg: JWT_ALGORITHM,
            ..Default::default()
        },
        &json!({
            "iss": JWT_ISS,
            "aud": JWT_AUD,
            "uid": user.id,
            "sid": id,
            "exp": to_seconds(now + Days::new(2))
        }),
        &EncodingKey::from_secret(cx.config.jwt_secret_key.as_ref()),
    )
    .inspect_err(|e| {
        error!(error = %e, session.id = %id, %user.id, "failed to generate refresh token");
        sentry::capture_error(e);
    })
    .map_err(api::system_failure)?;

    let refresh_token = jsonwebtoken::encode(
        &Header {
            alg: JWT_ALGORITHM,
            ..Default::default()
        },
        &json!({
            "iss": JWT_ISS,
            "aud": JWT_AUD,
            "uid": user.id,
            "sid": id,
            "exp": to_seconds(now + Days::new(7))
        }),
        &EncodingKey::from_secret(cx.config.jwt_secret_key.as_ref()),
    )
    .inspect_err(|e| {
        error!(error = %e, session.id = %id, %user.id, "failed to generate refresh token");
        sentry::capture_error(e);
    })
    .map_err(api::system_failure)?;

    let model = session::Model {
        refresh_token,
        access_token,
        account: user.id,
        id: id.into(),
    };

    SessionEntity::insert(model.clone().into_active_model())
        .exec(&cx.pool)
        .await
        .map_err(api::system_failure)?;

    Ok(api::ok(StatusCode::CREATED, model.into()))
}

struct FetchSessionR;
impl IntoResponses for FetchSessionR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_schema_name("SessionResponse"),
            "4xx" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("authorization failed for some reason"));

                response
            }
        }
    }
}

/// Retrieve information about this session.
///
/// Useless on its own but useful for testing out session authentication.
#[utoipa::path(
    get,

    path = "/v1/users/@me/session",
    tags = ["Users", "Users/Sessions"],
    operation_id = "getUserSession",
    responses(FetchSessionR)
)]
pub async fn fetch(Extension(Session { session, .. }): Extension<Session>) -> api::Result<charted_types::Session> {
    let Some(session) = session else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "authentication didn't use a bearer token",
            ),
        ));
    };

    Ok(api::ok(StatusCode::OK, session.sanitize()))
}

/// Logs you out from the session and destroys it.
#[utoipa::path(
    delete,

    path = "/v1/users/@me/session",
    tags = ["Users", "Users/Sessions"],
    operation_id = "logout",
    responses(LoginR)
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn logout(
    State(cx): State<Context>,
    Extension(Session { session, .. }): Extension<Session>,
) -> api::Result<()> {
    let Some(session) = session else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "authentication didn't use a bearer token",
            ),
        ));
    };

    let key = DecodingKey::from_secret(cx.config.jwt_secret_key.as_ref());
    let decoded = jsonwebtoken::decode::<HashMap<String, serde_json::Value>>(
        &session.access_token.unwrap(),
        &key,
        &Validation::new(JWT_ALGORITHM),
    )
    .inspect_err(|e| {
        error!(error = %e, %session.id, "failed to decode JWT token; entry is severed");
        sentry::capture_error(e);
    })
    .map_err(api::system_failure)?;

    let sid = authn::extract_sid_from_token(&decoded.claims).map_err(Into::<api::Response>::into)?;

    // sanity check just in case
    if SessionEntity::find_by_id(sid)
        .filter(session::Column::Account.eq(session.owner))
        .one(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, %session.id, user.id = %session.owner, from = %sid, "failed to find session");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?
        .is_none()
    {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "session with `sid` was not found for user",
                json!({"sid":sid, "user":session.owner}),
            ),
        ));
    }

    SessionEntity::delete_by_id(sid)
        .filter(session::Column::Account.eq(session.owner))
        .exec(&cx.pool)
        .await
        .map(|_| api::from_default(StatusCode::ACCEPTED))
        .inspect_err(|e| {
            error!(error = %e, %session.id, user.id = %session.owner, from = %sid, "failed to delete session");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)
}

struct RefreshSessionR;
impl IntoResponses for RefreshSessionR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "201" => Ref::from_schema_name("SessionResponse"),
            "4xx" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("authorization failed for some reason"));

                response
            }
        }
    }
}

/// Refresh a session by destroying the old session and creating a new one.
#[utoipa::path(
    post,

    path = "/v1/users/@me/session/refresh",
    tags = ["Users", "Users/Sessions"],
    operation_id = "refreshSessionToken",
    responses(RefreshSessionR)
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn refresh_session(
    State(cx): State<Context>,
    Extension(Session { session, user }): Extension<Session>,
) -> api::Result<charted_types::Session> {
    let Some(session) = session else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "authentication didn't use a bearer token",
            ),
        ));
    };

    // Check if the refresh token is still alive.
    let key = DecodingKey::from_secret(cx.config.jwt_secret_key.as_ref());
    match jsonwebtoken::decode::<HashMap<String, serde_json::Value>>(
        session.refresh_token.as_ref().unwrap(),
        &key,
        &Validation::new(JWT_ALGORITHM),
    ) {
        Ok(_) => {}
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => {
                return Err(api::err(
                    StatusCode::GONE,
                    (api::ErrorCode::SessionExpired, "session had expired"),
                ));
            }

            _ => {
                error!(error = %err, %session.id, "failed to decode JWT token");
                sentry::capture_error(&err);

                return Err(api::system_failure(err));
            }
        },
    }

    // we can still call `logout` since it's just a regular function
    logout(
        State(cx.clone()),
        Extension(Session {
            session: Some(session.clone()),
            user: user.clone(),
        }),
    )
    .await?;

    // create session
    let id = cx
        .ulid_generator
        .generate()
        .inspect_err(|e| {
            error!(error = %e, %user.id, "failed to generate session id");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    let now = Utc::now();
    let access_token = jsonwebtoken::encode(
        &Header {
            alg: JWT_ALGORITHM,
            ..Default::default()
        },
        &json!({
            "iss": JWT_ISS,
            "aud": JWT_AUD,
            "uid": user.id,
            "sid": id,
            "exp": to_seconds(now + Days::new(2))
        }),
        &EncodingKey::from_secret(cx.config.jwt_secret_key.as_ref()),
    )
    .inspect_err(|e| {
        error!(error = %e, session.id = %id, %user.id, "failed to generate refresh token");
        sentry::capture_error(e);
    })
    .map_err(api::system_failure)?;

    let refresh_token = jsonwebtoken::encode(
        &Header {
            alg: JWT_ALGORITHM,
            ..Default::default()
        },
        &json!({
            "iss": JWT_ISS,
            "aud": JWT_AUD,
            "uid": user.id,
            "sid": id,
            authn::JWT_EXP: to_seconds(now + Days::new(7))
        }),
        &EncodingKey::from_secret(cx.config.jwt_secret_key.as_ref()),
    )
    .inspect_err(|e| {
        error!(error = %e, session.id = %id, %user.id, "failed to generate refresh token");
        sentry::capture_error(e);
    })
    .map_err(api::system_failure)?;

    let model = session::Model {
        refresh_token,
        access_token,
        account: user.id,
        id: id.into(),
    };

    SessionEntity::insert(model.clone().into_active_model())
        .exec(&cx.pool)
        .await
        .map_err(api::system_failure)?;

    Ok(api::ok(StatusCode::CREATED, model.into()))
}

fn to_seconds<Tz: TimeZone>(dt: DateTime<Tz>) -> i64 {
    dt.timestamp_millis()
        .checked_div(1000)
        .expect("timestamp overflow occurred or was zero")
}
