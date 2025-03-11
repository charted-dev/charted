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

pub mod avatars;
pub mod repositories;
pub mod sessions;

use crate::{
    Context, ULID_GENERATOR,
    extract::{Json, Path},
    extract_refor_t, hash_password,
    middleware::sessions::Session,
    modify_property,
    openapi::{ApiErrorResponse, ApiResponse},
    routing::v1::{Entrypoint, EntrypointResponse},
};
use axum::{Extension, Router, extract::State, http::StatusCode, routing};
use charted_core::{api, bitflags::ApiKeyScope};
use charted_database::entities::{UserEntity, user};
use charted_types::{NameOrUlid, User, payloads::CreateUserPayload};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, sqlx::types::chrono};
use serde_json::json;
use std::collections::BTreeMap;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{RefOr, Response},
};
use validator::ValidateEmail;

pub type UserResponse = ApiResponse<User>;

pub fn create_router(cx: &Context) -> Router<Context> {
    let router = match cx.config.single_user {
        false => Router::new().route("/", routing::get(main).put(create)),
        true => Router::new().route("/", routing::get(main)),
    };

    let id_or_name = Router::new().route("/", routing::get(fetch));
    let at_me = Router::new().route(
        "/",
        routing::get(get_self).layer(AsyncRequireAuthorizationLayer::new(
            crate::middleware::sessions::Middleware::default().with_scope(ApiKeyScope::UserAccess),
        )),
    );

    router.nest("/@me", at_me).nest("/{idOrName}", id_or_name)
}

/// Entrypoint to the Users API.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users",
    operation_id = "users",
    tag = "Users",
    responses(EntrypointResponse)
)]
pub async fn main() -> api::Response<Entrypoint> {
    api::ok(StatusCode::OK, Entrypoint::new("Users"))
}

struct CreateUserR;
impl IntoResponses for CreateUserR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap!(
            "201" => {
                let mut response = extract_refor_t!(UserResponse::response().1);
                modify_property!(response; description("User was successfully created"));

                response
            },

            "403" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Instance doesn't allow registrations"));

                response
            },

            "406" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Session backend required the `password` field or `email` was not a valid, proper email address"));

                response
            },

            "409" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("If a user already has the `username` or `email` taken."));

                response
            },

            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Failure"));

                response
            }
        )
    }
}

/// Creates a new user.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(
    name = "charted.server.ops.create[user]",
    skip_all,
    fields(
        user.username = %username,
        user.email = email
    )
)]
#[utoipa::path(
    post,

    path = "/v1/users",
    tag = "Users",
    operation_id = "createUser",
    request_body(
        description = "Request body for creating a new user on this instance. The `password` field can be omitted if the session backend isn't the local one.",
        content = ref("CreateUserPayload")
    ),
    responses(CreateUserR)
)]
pub async fn create(
    State(cx): State<Context>,
    Json(CreateUserPayload {
        email,
        password,
        username,
    }): Json<CreateUserPayload>,
) -> api::Result<User> {
    if !cx.config.registrations {
        return Err(api::err(
            StatusCode::FORBIDDEN,
            (
                api::ErrorCode::RegistrationsDisabled,
                "this instance has user registrations disabled",
            ),
        ));
    }

    if cx.authz.is::<charted_authz_local::Backend>() && password.is_none() {
        return Err(api::err(
            StatusCode::NOT_ACCEPTABLE,
            (
                api::ErrorCode::MissingPassword,
                "authentication backend requires you to include a password for this new account",
            ),
        ));
    }

    if !email.validate_email() {
        return Err(api::err(
            StatusCode::NOT_ACCEPTABLE,
            (
                api::ErrorCode::ValidationFailed,
                "`email` is not a valid email",
                json!({"email":&email}),
            ),
        ));
    }

    if UserEntity::find()
        .filter(user::Column::Username.eq(username.clone()))
        .one(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, user.username = %username, "failed to find user by username");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?
        .is_some()
    {
        return Err(api::err(
            StatusCode::CONFLICT,
            (
                api::ErrorCode::EntityAlreadyExists,
                "a user with `username` already exists",
                json!({"username":username.as_str()}),
            ),
        ));
    }

    if UserEntity::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, user.email = %username, "failed to find user by email");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?
        .is_some()
    {
        return Err(api::err(
            StatusCode::CONFLICT,
            (
                api::ErrorCode::EntityAlreadyExists,
                "a user with `email` already exists",
                json!({"email":&email}),
            ),
        ));
    }

    let password = if let Some(ref password) = password {
        if password.len() < 8 {
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::InvalidPassword,
                    "`password` length was expected to be 8 characters or longer",
                ),
            ));
        }

        Some(hash_password(password).map_err(api::system_failure_from_report)?)
    } else {
        None
    };

    let id = ULID_GENERATOR
        .generate()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!("received monotonic overflow -- please inspect this as fast you can!!!!!");
        })
        .map_err(api::system_failure)?;

    let model = user::Model {
        verified_publisher: false,
        prefers_gravatar: false,
        gravatar_email: None,
        description: None,
        avatar_hash: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        password,
        username,
        email,
        admin: false,
        name: None,
        id: id.into(),
    };

    UserEntity::insert(model.clone().into_active_model())
        .exec(&cx.pool)
        .await
        .map_err(api::system_failure)?;

    if let Err(e) = charted_helm_charts::create_chart_index(&cx.storage, id.into()).await {
        error!(error = %e, "failed to create chart index, retrying later...");
        sentry::capture_error(&*e);
    }

    Ok(api::ok(StatusCode::CREATED, model.into()))
}

struct FetchUserR;
impl IntoResponses for FetchUserR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => {
                let mut response = extract_refor_t!(UserResponse::response().1);
                modify_property!(response; description("A single user was found by name or id."));

                response
            },

            "400" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Invalid ID or name specified"));

                response
            },

            "404" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("User not found"));

                response
            }
        }
    }
}

/// Retrieve a single user by their ID or name.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[user]", skip_all, fields(%id_or_name))]
#[utoipa::path(
    get,

    path = "/v1/users/{idOrName}",
    tag = "Users",
    operation_id = "getUserByIdOrName",
    responses(FetchUserR),
    params(
        ("idOrName" = NameOrUlid, Path)
    )
)]
pub async fn fetch(State(cx): State<Context>, Path(id_or_name): Path<NameOrUlid>) -> api::Result<User> {
    match id_or_name {
        NameOrUlid::Name(ref name) => match UserEntity::find()
            .filter(user::Column::Username.eq(name.to_owned()))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
        {
            Some(user) => Ok(api::ok(StatusCode::OK, user)),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )),
        },

        NameOrUlid::Ulid(id) => match UserEntity::find_by_id(id)
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
        {
            Some(user) => Ok(api::ok(StatusCode::OK, user)),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )),
        },
    }
}

struct FetchSelfR;
impl IntoResponses for FetchSelfR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => extract_refor_t!(UserResponse::response().1),
            "4XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("The occurrence when authentication fails"));

                response
            }
        }
    }
}

/// Returns information about yourself via an authenticated request.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}",
    tag = "Users",
    operation_id = "getSelfUser",
    responses(FetchSelfR)
)]
pub async fn get_self(Extension(Session { user, .. }): Extension<Session>) -> api::Response<User> {
    api::ok(StatusCode::OK, user)
}

/*
pub async fn get_self(Extension(Session { user, .. }): Extension<Session>) -> api::Response<User> {
    api::ok(StatusCode::OK, user)
}

/// Patch metadata about the current user.
#[utoipa::path(
    patch,
    path = "/v1/users/@me",
    operation_id = "patchSelf",
    tag = "Users",
    request_body(
        content_type = "application/json",
        description = "Update payload for the `User` entity",
        content = ref("PatchUserPayload")
    ),
    responses(
        (
            status = 204,
            description = "Patch was successfully reflected",
            body = EmptyApiResponse,
            content_type = "application/json"
        ),
        (
            status = 4XX,
            description = "Any occurrence when authentication fails or if the patch couldn't be reflected",
            body = ApiErrorResponse,
            content_type = "application/json"
        )
    )
)]
pub async fn patch(
    State(cx): State<ServerContext>,
    Extension(Session { mut user, .. }): Extension<Session>,
    Json(PatchUserPayload {
        prefers_gravatar,
        gravatar_email,
        description,
        username,
        password,
        email,
        name,
    }): Json<PatchUserPayload>,
) -> api::Result<()> {
    if let Some(prefers_gravatar) = prefers_gravatar {
        if user.prefers_gravatar != prefers_gravatar {
            user.prefers_gravatar = prefers_gravatar;
        }
    }

    if let Some(gravatar_email) = gravatar_email.as_deref() {
        // if `old` == None, then update the description
        // if `old` == Some(..) && `old` != `gravatar_email`, commit update
        // if `old` == Some(..) && `old` == `""`, commit as `None`
        let old = user.gravatar_email.as_deref();
        if old.is_none() && !gravatar_email.is_empty() {
            user.gravatar_email = Some(gravatar_email.to_owned());
        } else if let Some(old) = old
            && !old.is_empty()
            && old != gravatar_email
        {
            user.gravatar_email = Some(gravatar_email.to_owned());
        } else if gravatar_email.is_empty() {
            user.description = None;
        }
    }

    if let Some(description) = description {
        if description.len() > 140 {
            let len = description.len();
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::ValidationFailed,
                    "expected `description` to be less than 140 characters",
                    json!({
                        "expected": 140,
                        "received": {
                            "over": len - 140,
                            "length": len
                        }
                    }),
                ),
            ));
        }

        // if `old` == None, then update the description
        // if `old` == Some(..) && `old` != `descroption`, commit update
        // if `old` == Some(..) && `old` == `""`, commit as `None`
        let old = user.description.as_deref();
        if old.is_none() {
            user.description = Some(description);
        } else if let Some(old) = old
            && !old.is_empty()
            && old != description
        {
            user.description = Some(description);
        } else if description.is_empty() {
            user.description = None;
        }
    }

    if let Some(username) = username {
        // We need to validate that the username isn't already taken, so we will get a
        // temporary connection.
        match ops::db::user::get(&cx, NameOrUlid::Name(username.clone())).await {
            Ok(None) => {}
            Ok(Some(_)) => {
                return Err(api::err(
                    StatusCode::CONFLICT,
                    (
                        api::ErrorCode::EntityAlreadyExists,
                        "user with username already exists",
                        json!({"username":&username}),
                    ),
                ))
            }

            Err(e) => return Err(api::system_failure(e)),
        };

        // In deserialization of the request body, it'll validate that
        // the name is correct anyway, so it is ok to set it here without
        // even more validation.
        user.username = username;
    }

    if let Some(password) = password.as_deref() {
        let authz = cx.authz.as_ref();
        if authz.downcast::<charted_authz_local::Backend>().is_none() {
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::InvalidBody,
                    "`password` is only supported on the local authz backend",
                ),
            ));
        }

        if password.len() < 8 {
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::InvalidPassword,
                    "`password` length was expected to be 8 characters or longer",
                ),
            ));
        }

        user.password = Some(hash_password(password).map_err(|_| api::internal_server_error())?);
    }

    let mut conn = cx
        .pool
        .get()
        .inspect_err(|e| {
            sentry::capture_error(e);
            tracing::error!(error = %e, "failed to establish database connection");
        })
        .map_err(|_| api::internal_server_error())?;

    charted_database::connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().run(|txn| {
            use postgresql::users::{dsl, table};

            diesel::update(table.filter(dsl::id.eq(user.id)))
                .set(user.into_pg())
                .execute(txn)
                .map(|_| ())
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::users::{dsl, table};

            diesel::update(table.filter(dsl::id.eq(user.id)))
                .set(user.into_sqlite())
                .execute(txn)
                .map(|_| ())
        });
    })
    .inspect_err(|e| {
        sentry::capture_error(e);
        tracing::error!(error = %e, "failed to update user");
    })
    .map_err(|_| api::internal_server_error())?;

    Ok(api::no_content())
}

#[utoipa::path(
    delete,

    path = "/v1/users/@me",
    operation_id = "deleteSelf",
    tag = "Users",
    responses(
        (
            status = 204,
            description = "User is scheduled for deletion and will be deleted",
            body = EmptyApiResponse,
            content_type = "application/json"
        )
    )
)]
pub async fn delete(
    State(cx): State<ServerContext>,
    Extension(Session { user, .. }): Extension<Session>,
) -> api::Result<()> {
    ops::db::user::delete(cx, user)
        .await
        .inspect_err(|e| {
            sentry_eyre::capture_report(e);
            tracing::error!(error = %e, "failed to delete user");
        })
        .map_err(|_| api::internal_server_error())?;

    Ok(api::no_content())
}
*/
