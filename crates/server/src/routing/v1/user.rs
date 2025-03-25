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
    Context, commit_patch,
    extract::{Json, Path},
    extract_refor_t, hash_password,
    middleware::authn::{self, Options, Session},
    modify_property,
    openapi::{ApiErrorResponse, ApiResponse, EmptyApiResponse},
    routing::v1::{Entrypoint, EntrypointResponse},
};
use axum::{Extension, Router, extract::State, handler::Handler, http::StatusCode, routing};
use charted_core::{api, bitflags::ApiKeyScope};
use charted_database::entities::{UserEntity, user};
use charted_types::{
    NameOrUlid, User,
    payloads::{CreateUserPayload, PatchUserPayload},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, sqlx::types::chrono,
};
use serde_json::json;
use std::{borrow::Cow, collections::BTreeMap};
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{Ref, RefOr, Response},
};
use validator::ValidateEmail;

pub type UserResponse = ApiResponse<User>;

pub fn create_router(cx: &Context) -> Router<Context> {
    let router = match cx.config.single_user {
        false => Router::new().route("/", routing::get(main).put(create)),
        true => Router::new().route("/", routing::get(main)),
    };

    let id_or_name = Router::new()
        .route("/", routing::get(fetch))
        .route("/repositories", routing::get(repositories::list_user_repositories))
        .route("/avatar", routing::get(avatars::get_user_avatar))
        .route("/avatars/{hash}", routing::get(avatars::get_user_avatar_by_hash));

    let at_me = {
        let mut base = Router::new();
        match cx.config.single_user {
            true => {
                base = base.route(
                    "/",
                    routing::get(get_self.layer(authn::new(
                        cx.to_owned(),
                        Options::default().with_scope(ApiKeyScope::UserAccess),
                    )))
                    .patch(patch.layer(authn::new(
                        cx.to_owned(),
                        Options::default().with_scope(ApiKeyScope::UserUpdate),
                    ))),
                )
            }

            false => {
                base = base.route(
                    "/",
                    routing::get(get_self.layer(authn::new(
                        cx.to_owned(),
                        Options::default().with_scope(ApiKeyScope::UserAccess),
                    )))
                    .patch(patch.layer(authn::new(
                        cx.to_owned(),
                        Options::default().with_scope(ApiKeyScope::UserUpdate),
                    )))
                    .delete(delete.layer(authn::new(
                        cx.to_owned(),
                        Options::default().with_scope(ApiKeyScope::UserDelete),
                    ))),
                )
            }
        }

        base.nest("/apikeys", apikeys::create_router(cx))
            .route(
                "/repositories",
                routing::get(
                    repositories::list_self_user_repositories.layer(authn::new(
                        cx.to_owned(),
                        Options::default()
                            .with_scope(ApiKeyScope::RepoAccess)
                            .with_scope(ApiKeyScope::UserAccess),
                    )),
                )
                .put(
                    repositories::create_user_repository.layer(authn::new(
                        cx.to_owned(),
                        Options::default()
                            .with_scope(ApiKeyScope::RepoCreate)
                            .with_scope(ApiKeyScope::UserAccess),
                    )),
                ),
            )
            .route(
                "/avatar",
                routing::get(avatars::get_self_user_avatar.layer(authn::new(
                    cx.to_owned(),
                    Options::default().with_scope(ApiKeyScope::UserAccess),
                )))
                .post(
                    avatars::upload_user_avatar.layer(authn::new(
                        cx.to_owned(),
                        Options::default()
                            .with_scope(ApiKeyScope::RepoCreate)
                            .with_scope(ApiKeyScope::UserAvatarUpdate),
                    )),
                ),
            )
            .route(
                "/avatars/{hash}",
                routing::get(
                    avatars::get_self_user_avatar_by_hash.layer(authn::new(
                        cx.to_owned(),
                        Options::default()
                            .with_scope(ApiKeyScope::RepoCreate)
                            .with_scope(ApiKeyScope::UserAccess),
                    )),
                ),
            )
    };

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
            "201" => Ref::from_response_name("UserResponse"),
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
        content = ref("#/components/schemas/CreateUserPayload")
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

    let id = cx
        .ulid_generator
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
            "200" => Ref::from_response_name("UserResponse"),
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
    params(NameOrUlid)
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
            "200" => Ref::from_response_name("UserResponse"),
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
    path = "/v1/users/@me",
    tag = "Users",
    operation_id = "getSelfUser",
    responses(FetchSelfR)
)]
pub async fn get_self(Extension(Session { user, .. }): Extension<Session>) -> api::Response<User> {
    api::ok(StatusCode::OK, user)
}

struct PatchUserR;
impl IntoResponses for PatchUserR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "204" => {
                let mut response = extract_refor_t!(EmptyApiResponse::response().1);
                modify_property!(response; description("Patch was successful"));

                response
            },

            "4XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Any occurrence when authentication fails or if the patch couldn't be applied"));

                response
            }
        }
    }
}

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
    responses(PatchUserR)
)]
pub async fn patch(
    State(cx): State<Context>,
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
    let mut model = UserEntity::find_by_id(user.id)
        .one(&cx.pool)
        .await
        .map_err(api::system_failure)?
        .map(Into::<user::ActiveModel>::into)
        .unwrap();

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
        } else if UserEntity::find()
            .filter(user::Column::Email.eq(email.to_owned()))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
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
        if UserEntity::find()
            .filter(user::Column::Username.eq(username.to_owned()))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
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
        if !cx.authz.is::<charted_authz_local::Backend>() {
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
            model.password = crate::hash_password(password)
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
        .update(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, %user.username, "failed to commit changes for patch");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)
        .map(|_| api::no_content())
}

/// Delete yourself.
#[cfg_attr(debug_assertions, axum::debug_handler)]
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
pub async fn delete(State(cx): State<Context>, Extension(Session { user, .. }): Extension<Session>) -> api::Result<()> {
    UserEntity::delete_by_id(user.id)
        .exec(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, %user.id, %user.username, "failed to delete user");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)
        .map(|_| api::no_content())
}
