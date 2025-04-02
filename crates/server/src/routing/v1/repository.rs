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

pub mod releases;

use super::Entrypoint;
use crate::{
    Context,
    extract::Path,
    extract_refor_t,
    middleware::authn::{self, Options},
    modify_property,
    openapi::ApiErrorResponse,
    routing::v1::EntrypointResponse,
};
use axum::{Router, extract::State, handler::Handler, http::StatusCode, routing};
use charted_core::{
    api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
};
use charted_database::entities::{RepositoryEntity, repository};
use charted_types::{NameOrUlid, Repository};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::collections::BTreeMap;
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{Ref, RefOr, Response},
};

pub fn create_router(cx: &Context) -> Router<Context> {
    Router::new().route("/", routing::get(main)).route(
        "/{owner}/{repo}",
        routing::get(fetch.layer(authn::new(cx.to_owned(), Options {
            allow_unauthorized: true,
            scopes: ApiKeyScopes::new(ApiKeyScope::RepoAccess.into()),
            require_refresh_token: false,
        }))),
    )
}

/// Entrypoint handler to the Repositories API.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/v1/repositories",
    operation_id = "repositories",
    tag = "Repositories",
    responses(EntrypointResponse)
)]
pub async fn main() -> api::Response<Entrypoint> {
    api::ok(StatusCode::OK, Entrypoint::new("Repositories"))
}
struct FetchRepoR;
impl IntoResponses for FetchRepoR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_response_name("RepositoryResponse"),
            "400" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Invalid ID or name specified"));

                response
            },

            "404" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Repository not found"));

                response
            }
        }
    }
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[repository]", skip_all, fields(%owner, %repo))]
#[utoipa::path(
    get,
    path = "/v1/repositories/{owner}/{repo}",
    tag = "Users",
    operation_id = "getRepositoryByIdOrName",
    responses(FetchRepoR),
    params(NameOrUlid)
)]
pub async fn fetch(
    State(cx): State<Context>,
    Path((owner, repo)): Path<(NameOrUlid, NameOrUlid)>,
) -> api::Result<Repository> {
    let user = super::user::fetch(State(cx.clone()), Path(owner)).await?.data.unwrap();
    match repo {
        NameOrUlid::Name(ref name) => match RepositoryEntity::find()
            .filter(repository::Column::Name.eq(name.clone()))
            .filter(repository::Column::Owner.eq(user.id))
            .filter(repository::Column::Private.eq(false))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<Repository>::into)
        {
            Some(repo) => Ok(api::ok(StatusCode::OK, repo)),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "repository with id or name was not found",
                    json!({"idOrName":repo}),
                ),
            )),
        },

        NameOrUlid::Ulid(id) => match RepositoryEntity::find_by_id(id)
            .filter(repository::Column::Owner.eq(user.id))
            .filter(repository::Column::Private.eq(false))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::into)
        {
            Some(repo) => Ok(api::ok(StatusCode::OK, repo)),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "repository with id or name was not found",
                    json!({"idOrName":repo}),
                ),
            )),
        },
    }
}

/*
struct FetchSelfR;
impl IntoResponses for FetchSelfR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_response_name("UserResponse"),
            "4XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("The occurrence when authentication fails"));

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
    responses(FetchSelfR),
    security(
        ("ApiKey" = ["user:access"])
    )
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
                modify_property!(response.description("Patch was successful"));

                response
            },

            "4XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Any occurrence when authentication fails or if the patch couldn't be applied"));

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
    responses(PatchUserR),
    security(
        ("ApiKey" = ["user:update"])
    )
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
pub async fn delete(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
) -> api::Result<()> {
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

*/
