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

pub mod apikeys;
pub mod avatars;
pub mod repositories;
pub mod sessions;

use crate::{
    Env, commit_patch,
    ext::ResultExt,
    extract::{Json, Path},
    middleware::authn::{Factory, Options, Session},
    mk_into_responses,
    openapi::{EmptyApiResponse, UserResponse},
    ops::{self, db},
    routing::v1::Entrypoint,
};
use axum::{Extension, Router, extract::State, handler::Handler, http::StatusCode, routing};
use charted_core::{api, bitflags::ApiKeyScope};
use charted_database::entities::{UserEntity, user};
use charted_helm_charts::DataStoreExt;
use charted_types::{
    NameOrUlid, User,
    payloads::{CreateUserPayload, PatchUserPayload},
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde_json::json;
use std::borrow::Cow;
use validator::ValidateEmail;

pub fn create_router(env: &Env) -> Router<Env> {
    let router = match env.config.single_user {
        false => Router::new().route("/", routing::get(main).put(create)),
        true => Router::new().route("/", routing::get(main)),
    };

    let id_or_name = Router::new()
        .route("/", routing::get(fetch))
        .route("/avatar", routing::get(avatars::get_user_avatar))
        .route("/avatars/{hash}", routing::get(avatars::get_user_avatar_by_hash))
        .route("/repositories", routing::get(repositories::list_user_repositories));

    let at_me = {
        let base = match env.config.single_user {
            false => Router::new().route(
                "/",
                routing::get(get_self.layer(env.authn(Options::default().with_scope(ApiKeyScope::UserAccess))))
                    .patch(patch.layer(env.authn(Options::default().with_scope(ApiKeyScope::UserUpdate))))
                    .delete(delete.layer(env.authn(Options::default().with_scope(ApiKeyScope::UserDelete)))),
            ),
            true => Router::new().route(
                "/",
                routing::get(get_self.layer(env.authn(Options::default().with_scope(ApiKeyScope::UserAccess))))
                    .patch(patch.layer(env.authn(Options::default().with_scope(ApiKeyScope::UserUpdate)))),
            ),
        };

        base.nest("/apikeys", apikeys::create_router(env))
            .route(
                "/session",
                routing::get(sessions::fetch.layer(env.authn(Options::default())))
                    .delete(sessions::logout.layer(env.authn(Options::default()))),
            )
            .route(
                "/session/refresh",
                routing::post(sessions::refresh_session.layer(env.authn(Options {
                    require_refresh_token: true,
                    ..Default::default()
                }))),
            )
            .route(
                "/repositories",
                routing::get(
                    repositories::list_self_user_repositories
                        .layer(env.authn(Options::default().with_scope(ApiKeyScope::RepoAccess))),
                )
                .put(
                    repositories::create_user_repository
                        .layer(env.authn(Options::default().with_scope(ApiKeyScope::RepoCreate))),
                )
                .patch(
                    repositories::patch_user_repository
                        .layer(env.authn(Options::default().with_scope(ApiKeyScope::RepoUpdate))),
                )
                .delete(
                    repositories::delete.layer(env.authn(Options::default().with_scope(ApiKeyScope::RepoDelete))),
                ),
            )
            .route(
                "/avatar",
                routing::get(
                    avatars::get_self_user_avatar
                        .layer(env.authn(Options::default().with_scope(ApiKeyScope::UserAccess))),
                )
                .post(
                    avatars::upload_user_avatar.layer(
                        env.authn(
                            Options::default()
                                .with_scope(ApiKeyScope::UserAccess)
                                .with_scope(ApiKeyScope::UserAvatarUpdate),
                        ),
                    ),
                ),
            )
            .route(
                "/avatars/{hash}",
                routing::get(
                    avatars::get_self_user_avatar_by_hash
                        .layer(env.authn(Options::default().with_scope(ApiKeyScope::UserAccess))),
                ),
            )
    };

    router
        .nest("/@me", at_me)
        .nest("/{idOrName}", id_or_name)
        .route("/login", routing::post(sessions::login))
}

/// Entrypoint to the Users API.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users",
    operation_id = "users",
    tag = "Users",
    responses(Entrypoint)
)]
pub async fn main() -> api::Response<Entrypoint> {
    api::ok(StatusCode::OK, Entrypoint::new("Users"))
}

struct CreateUserR;
mk_into_responses!(for CreateUserR {
    "201" => [ref(UserResponse)];
    "403" => [error(description("Instance doesn't allow registrations"))];
    "406" => [error(description("Session backend requires `password` or `email` wasn't a valid email address"))];
    "409" => [error(description("Either the `username` or `email` was taken by another user"))];
});

/// Creates a new user.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    post,

    path = "/v1/users",
    tag = "Users",
    operation_id = "createUser",
    request_body(
        description = "Request body for creating a new user on this instance. The `password` field can be omitted if the session backend isn't the local one.",
        content = ref("#/components/schemas/CreateUserPayload")
    ),
    responses(CreateUserR)
)]
pub async fn create(
    State(env): State<Env>,
    Json(CreateUserPayload {
        email,
        password,
        username,
    }): Json<CreateUserPayload>,
) -> api::Result<User> {
    if !env.config.registrations {
        return Err(api::err(
            StatusCode::FORBIDDEN,
            (
                api::ErrorCode::RegistrationsDisabled,
                "this instance has user registrations disabled",
            ),
        ));
    }

    if env.authz.is::<charted_authz_local::Backend>() && password.is_none() {
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

    if db::user::get_as_model(&env.db, NameOrUlid::Name(username.clone()))
        .await?
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

    if db::user::find(&env.db, |query| query.filter(user::Column::Email.eq(email.clone())))
        .await?
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

        Some(ops::hash_password(password).map_err(api::system_failure_from_report)?)
    } else {
        None
    };

    let id = env.ulid.generate().into_system_failure()?;
    let model = user::Model {
        verified_publisher: false,
        prefers_gravatar: false,
        gravatar_email: None,
        description: None,
        avatar_hash: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        password,
        username,
        email,
        admin: false,
        name: None,
        id: id.into(),
    };

    UserEntity::insert(model.clone().into_active_model())
        .exec(&env.db)
        .await
        .into_system_failure()?;

    let metadata = env.ds.metadata();
    if let Err(e) = metadata.create_chart_index(id.into()).await {
        error!(error = %e, "failed to create chart index, retrying later...");
        sentry::capture_error(&*e);
    }

    Ok(api::ok(StatusCode::CREATED, model.into()))
}

struct FetchUserR;
mk_into_responses!(for FetchUserR {
    "200" => [ref(UserResponse)];
    "400" => [error(description("Invalid ULID or name specified"))];
    "404" => [error(description("User not found"))];
});

/// Retrieve a single user by their ID or name.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}",
    tag = "Users",
    operation_id = "getUserByIdOrName",
    responses(FetchUserR),
    params(NameOrUlid)
)]
pub async fn fetch(State(env): State<Env>, Path(id_or_name): Path<NameOrUlid>) -> api::Result<User> {
    match db::user::get(&env.db, id_or_name.clone()).await? {
        Some(user) => Ok(api::ok(StatusCode::OK, user)),
        None => Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "user with id or name was not found",
                json!({"idOrName":id_or_name}),
            ),
        )),
    }
}

struct FetchSelfR;
mk_into_responses!(for FetchSelfR {
    "200" => [ref(UserResponse)];
    "4XX" => [error(description("Authentication failures"))];
});

/// Returns information about yourself via an authenticated request.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/@me",
    tag = "Users",
    operation_id = "getSelfUser",
    responses(FetchSelfR),
    security(
        ("ApiKey" = ["user:access"])
    )
)]
pub async fn get_self(Extension(Session { user, .. }): Extension<Session>) -> api::Response<User> {
    api::ok(StatusCode::OK, user)
}

struct PatchUserR;
mk_into_responses!(for PatchUserR {
    "204" => [ref(with "application/json" => EmptyApiResponse;
        description("Patch was successful");
    )];

    "4XX" => [error(description("Authentication failures"))];
});

/// Patch the authenticated user's metadata.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    patch,

    path = "/v1/users/@me",
    operation_id = "patchSelf",
    tag = "Users",
    request_body(
        content_type = "application/json",
        description = "Payload object for patching user metadata",
        content = ref("#/components/schemas/PatchUserPayload")
    ),
    responses(PatchUserR),
    security(
        ("ApiKey" = ["user:patch"])
    )
)]
pub async fn patch(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
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
    let mut model = db::user::get_as_model(&env.db, NameOrUlid::Ulid(user.id))
        .await?
        .unwrap()
        .into_active_model();

    let mut errors = Vec::new();

    commit_patch!(model of bool: old.prefers_gravatar => prefers_gravatar; [user]);
    commit_patch!(model of string?: old.gravatar_email => gravatar_email);
    commit_patch!(model of string?: old.description => description; validate that len < 140 [errors]);

    if let Some(email) = email.as_deref() {
        if !email.validate_email() {
            errors.push(api::Error {
                code: api::ErrorCode::ValidationFailed,
                message: Cow::Borrowed("invalid email address"),
                details: Some(json!({
                    "path": "email",
                    "email": email
                })),
            });
        } else if db::user::find(&env.db, |query| query.filter(user::Column::Email.eq(email.to_owned())))
            .await?
            .is_some()
        {
            errors.push(api::Error {
                code: api::ErrorCode::EntityAlreadyExists,
                message: Cow::Borrowed("an existing user already exists with that email"),
                details: Some(json!({
                    "path": "email",
                    "email": email
                })),
            });
        } else {
            model.email = ActiveValue::set(email.to_owned());
        }
    }

    if let Some(username) = username {
        if db::user::get_as_model(&env.db, NameOrUlid::Name(username.clone()))
            .await?
            .is_some()
        {
            errors.push(api::Error {
                code: api::ErrorCode::EntityAlreadyExists,
                message: Cow::Borrowed("an existing user already exists with that name"),
                details: Some(json!({
                    "path": "username",
                    "username": &username
                })),
            });
        } else {
            model.username = ActiveValue::set(username);
        }
    }

    if let Some(password) = password.as_deref() {
        if !env.authz.is::<charted_authz_local::Backend>() {
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::InvalidBody,
                    "user.password: authz backend doesn't require this field",
                ),
            ));
        }

        if password.len() < 8 {
            let len = password.len();
            errors.push(api::Error {
                code: api::ErrorCode::InvalidPassword,
                message: Cow::Borrowed("length of password was expected to be 8 characters or longer"),
                details: Some(json!({
                    "path": "password",
                    "expected": 8,
                    "received": [len - 8, len]
                })),
            });
        } else {
            model.password = ops::hash_password(password)
                .map_err(api::system_failure_from_report)
                .map(|s| ActiveValue::set(Some(s)))?;
        }
    }

    commit_patch!(model of string?: old.name => name; validate that len < 64 [errors]);

    if !errors.is_empty() {
        return Err(api::Response {
            headers: axum::http::HeaderMap::new(),
            success: false,
            status: StatusCode::CONFLICT,
            errors,
            data: None::<()>,
        });
    }

    model
        .update(&env.db)
        .await
        .inspect_err(|e| {
            error!(error = %e, %user.username, "failed to commit changes for patch");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)
        .map(|_| api::no_content())
}

/// Delete yourself.
#[axum::debug_handler]
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
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
) -> api::Result<()> {
    UserEntity::delete_by_id(user.id)
        .exec(&env.db)
        .await
        .inspect_err(|e| {
            error!(error = %e, %user.id, %user.username, "failed to delete user");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)
        .map(|_| api::no_content())
}
