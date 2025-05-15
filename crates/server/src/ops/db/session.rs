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

use super::user;
use crate::{
    Env,
    ext::ResultExt,
    ops::jwt::{self, Claims},
};
use axum::http::StatusCode;
use charted_authz::InvalidPassword;
use charted_core::api;
use charted_database as db;
use charted_database::entities::{SessionEntity, session};
use charted_types::{
    NameOrUlid, Session, User,
    payloads::{Login, UserLoginPayload},
};
use chrono::{DateTime, Days, TimeZone, Utc};
use jsonwebtoken::{TokenData, errors::ErrorKind};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde_json::json;
use std::borrow::Cow;
use validator::ValidateEmail;

#[instrument(
    name = "charted.server.ops.login",
    skip_all,
    fields(%login)
)]
pub async fn login(
    env: &Env,
    UserLoginPayload { login, password }: &UserLoginPayload,
) -> Result<Session, api::Response> {
    let (user, model) = (match login {
        Login::Username(name) => user::get_with_model(&env.db, NameOrUlid::Name(name.clone())).await,
        Login::Email(email) => {
            if !email.validate_email() {
                return Err(api::err(
                    StatusCode::NOT_ACCEPTABLE,
                    (api::ErrorCode::ValidationFailed, "invalid email address"),
                ));
            }

            user::find(&env.db, |query| {
                query.filter(db::entities::user::Column::Email.eq(email.clone()))
            })
            .await
            .map(|model| model.map(|model| (model.clone().into(), model)))
        }
    })?
    .ok_or_else(|| {
        api::err(
            StatusCode::NOT_FOUND,
            (api::ErrorCode::EntityNotFound, "user was not found"),
        )
    })?;

    let request = charted_authz::Request {
        password: Cow::Borrowed(password.as_str()),
        model,
        user: user.clone(),
    };

    env.authz.authenticate(request).await.map_err(|e| {
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

    let id = env.ulid.generate().into_system_failure()?;
    let now = Utc::now();
    let access_token = jwt::encode_jwt(env, Claims {
        iss: String::from(jwt::ISS_VALUE),
        aud: String::from(jwt::AUD_VALUE),
        exp: to_seconds(now + Days::new(1)).try_into().into_system_failure()?,
        uid: user.id,
        sid: id.into(),
    })
    .into_system_failure()?;

    let refresh_token = jwt::encode_jwt(env, Claims {
        iss: String::from(jwt::ISS_VALUE),
        aud: String::from(jwt::AUD_VALUE),
        exp: to_seconds(now + Days::new(7)).try_into().into_system_failure()?,
        uid: user.id,
        sid: id.into(),
    })
    .into_system_failure()?;

    let model = session::Model {
        refresh_token,
        access_token,
        account: user.id,
        id: id.into(),
    };

    SessionEntity::insert(model.clone().into_active_model())
        .exec(&env.db)
        .await
        .into_system_failure()?;

    Ok(model.into())
}

#[instrument(
    name = "charted.server.ops.logout",
    skip_all,
    fields(%session.id, %session.owner)
)]
pub async fn logout(env: &Env, session: Session) -> Result<(), api::Response> {
    let TokenData { claims, .. } = jwt::decode_jwt(env, &session.access_token.unwrap()).into_system_failure()?;

    if SessionEntity::find_by_id(claims.sid)
        .filter(session::Column::Account.eq(session.owner))
        .one(&env.db)
        .await
        .into_system_failure()?
        .is_none()
    {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "session with id was not found for user",
                json!({"session":claims.sid, "user":session.owner}),
            ),
        ));
    }

    SessionEntity::delete_by_id(claims.sid)
        .filter(session::Column::Account.eq(session.owner))
        .exec(&env.db)
        .await
        .map(|_| ())
        .inspect_err(|e| {
            error!(error = %e, %session.id, user.id = %session.owner, from = %claims.sid, "failed to delete session");
            sentry::capture_error(e);
        })
        .into_system_failure()
}

#[instrument(
    name = "charted.server.ops.refreshSession",
    skip_all,
    fields(%session.id, %session.owner)
)]
pub async fn refresh_session(env: &Env, session: Session, user: User) -> Result<Session, api::Response> {
    match jwt::decode_jwt(env, session.refresh_token.as_ref().unwrap()) {
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

    // Perform a logout.
    logout(env, session.clone()).await?;

    let id = env.ulid.generate().into_system_failure()?;
    let now = Utc::now();
    let access_token = jwt::encode_jwt(env, Claims {
        iss: String::from(jwt::ISS_VALUE),
        aud: String::from(jwt::AUD_VALUE),
        exp: to_seconds(now + Days::new(1)).try_into().into_system_failure()?,
        uid: user.id,
        sid: id.into(),
    })
    .into_system_failure()?;

    let refresh_token = jwt::encode_jwt(env, Claims {
        iss: String::from(jwt::ISS_VALUE),
        aud: String::from(jwt::AUD_VALUE),
        exp: to_seconds(now + Days::new(7)).try_into().into_system_failure()?,
        uid: user.id,
        sid: id.into(),
    })
    .into_system_failure()?;

    let model = session::Model {
        refresh_token,
        access_token,
        account: user.id,
        id: id.into(),
    };

    SessionEntity::insert(model.clone().into_active_model())
        .exec(&env.db)
        .await
        .into_system_failure()?;

    Ok(model.into())
}

fn to_seconds<Tz: TimeZone>(dt: DateTime<Tz>) -> i64 {
    dt.timestamp_millis()
        .checked_div(1000)
        .expect("timestamp overflow occurred or was zero")
}
