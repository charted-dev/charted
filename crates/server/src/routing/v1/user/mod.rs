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

use super::Entrypoint;
use crate::{
    extract::{Json, Path},
    hash_password,
    middleware::session::{self, Session},
    openapi::{ApiErrorResponse, EmptyApiResponse},
    ops, NameOrUlid, ServerContext,
};
use axum::{extract::State, http::StatusCode, routing, Extension, Router};
use charted_core::{api, bitflags::ApiKeyScope};
use charted_database::{
    connection,
    schema::{postgresql, sqlite},
};
use charted_types::{
    payloads::user::{CreateUserPayload, PatchUserPayload},
    User,
};
use eyre::Context;
use serde_json::json;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use tracing::{error, instrument};
use validator::ValidateEmail;

pub fn create_router() -> Router<ServerContext> {
    let id_or_name = Router::new().route("/", routing::get(get_user));
    let at_me = Router::new()
        .route(
            "/",
            routing::get(get_self).layer(AsyncRequireAuthorizationLayer::new(
                session::Middleware::default().scopes([ApiKeyScope::UserAccess]),
            )),
        )
        .nest("/apikeys", apikeys::create_router());

    Router::new()
        .route("/", routing::get(main).put(create_user))
        .nest("/@me", at_me)
        .nest("/{idOrName}", id_or_name)
}

/// Entrypoint to the Users API.
#[utoipa::path(
    get,
    path = "/v1/users",
    operation_id = "users",
    tag = "Users",
    responses(
        (
            status = 200,
            description = "Entrypoint response",
            body = api::Response<Entrypoint>,
            content_type = "application/json"
        )
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn main() -> api::Response<Entrypoint> {
    api::ok(StatusCode::OK, Entrypoint::new("Users"))
}

#[utoipa::path(
    post,
    path = "/v1/users",
    tag = "Users",
    operation_id = "createUser",
    request_body(
        content = ref("CreateUserPayload"),
        description = "Payload for creating a new user. The `password` field can be omitted if the session backend is not `Local`.",
        content_type = "application/json"
    ),
    responses(
        (
            status = 201,
            description = "User has been created",
            body = api::Response<User>,
            content_type = "application/json"
        ),
        (
            status = 403,
            description = "Returned if the server doesn't allow user registrations or if this is a single-user registry",
            body = ApiErrorResponse,
            content_type = "application/json"
        ),
        (
            status = 406,
            description = "Returned if the authentication backend requires a `password` field or the `email` field is not a valid email",
            body = ApiErrorResponse,
            content_type = "application/json"
        ),
        (
            status = 409,
            description = "Returned if the `username` or `email` provided is already registered",
            body = ApiErrorResponse,
            content_type = "application/json"
        )
    )
)]
#[instrument(
    name = "charted.server.ops.v1.createUser",
    skip_all,
    fields(user.name = %username)
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn create_user(
    State(cx): State<ServerContext>,
    Json(CreateUserPayload {
        email,
        password,
        username,
    }): Json<CreateUserPayload>,
) -> api::Result<User> {
    if !cx.config.registrations || cx.config.single_user {
        return Err(api::err(
            StatusCode::FORBIDDEN,
            (
                api::ErrorCode::RegistrationsDisabled,
                "this instance has user registrations disabled",
            ),
        ));
    }

    if cx.authz.as_ref().downcast::<charted_authz_local::Backend>().is_some() && password.is_none() {
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

    let mut conn = cx
        .pool
        .get()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!(error = %e, "failed to get db connection");
        })
        .map_err(|_| api::internal_server_error())?;

    // Check if we already have this `User` by their username
    {
        let exists = connection!(@raw conn {
            PostgreSQL(conn) => conn.build_transaction().read_only().run::<_, eyre::Report, _>(|txn| {
                use postgresql::users::{dsl, table};
                use diesel::pg::Pg;

                match table.select(<User as SelectableHelper<Pg>>::as_select()).filter(dsl::username.eq(&username)).first(txn) {
                    Ok(_) => Ok(true),
                    Err(diesel::result::Error::NotFound) => Ok(false),
                    Err(e) => Err(eyre::Report::from(e))
                }
            });

            SQLite(conn) => conn.immediate_transaction(|txn| {
                use sqlite::users::{dsl, table};
                use diesel::sqlite::Sqlite;

                match table.select(<User as SelectableHelper<Sqlite>>::as_select()).filter(dsl::username.eq(&username)).first(txn) {
                    Ok(_) => Ok(true),
                    Err(diesel::result::Error::NotFound) => Ok(false),
                    Err(e) => Err(eyre::Report::from(e))
                }
            });
        }).inspect_err(|e| {
            sentry_eyre::capture_report(e);
            error!(user.name = %username, error = %e, "failed to query user by username");
        }).map_err(|_| api::internal_server_error())?;

        if exists {
            return Err(api::err(
                StatusCode::CONFLICT,
                (
                    api::ErrorCode::EntityAlreadyExists,
                    "a user with `username` already exists",
                    json!({"username":username.as_str()}),
                ),
            ));
        }
    }

    // Check if we already have this `User` by their email address
    {
        let exists = connection!(@raw conn {
            PostgreSQL(conn) => conn.build_transaction().read_only().run::<_, eyre::Report, _>(|txn| {
                use postgresql::users::{dsl, table};
                use diesel::pg::Pg;

                match table.select(<User as SelectableHelper<Pg>>::as_select()).filter(dsl::email.eq(&email)).first(txn) {
                    Ok(_) => Ok(true),
                    Err(diesel::result::Error::NotFound) => Ok(false),
                    Err(e) => Err(eyre::Report::from(e))
                }
            });

            SQLite(conn) => conn.immediate_transaction(|txn| {
                use sqlite::users::{dsl, table};
                use diesel::sqlite::Sqlite;

                match table.select(<User as SelectableHelper<Sqlite>>::as_select()).filter(dsl::email.eq(&email)).first(txn) {
                    Ok(_) => Ok(true),
                    Err(diesel::result::Error::NotFound) => Ok(false),
                    Err(e) => Err(eyre::Report::from(e))
                }
            });
        })
        .inspect_err(|e| {
            sentry_eyre::capture_report(e);
            error!(user.email = email, error = %e, "failed to query user by email");
        })
        .map_err(|_| api::internal_server_error())?;

        if exists {
            return Err(api::err(
                StatusCode::CONFLICT,
                (
                    api::ErrorCode::EntityAlreadyExists,
                    "a user with the `email` given already exists",
                    json!({"email":email}),
                ),
            ));
        }
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

        Some(hash_password(password).map_err(|_| api::internal_server_error())?)
    } else {
        None
    };

    let id = cx
        .ulid_gen
        .generate()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!("received monotonic overflow -- please inspect this as fast you can!!!!!");
        })
        .map_err(|_| api::internal_server_error())?;

    let user = User {
        verified_publisher: false,
        prefers_gravatar: false,
        gravatar_email: None,
        description: None,
        avatar_hash: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        password,
        username,
        email,
        admin: false,
        name: None,
        id: id.into(),
    };

    connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_write().run::<_, eyre::Report, _>(|txn| {
            use postgresql::users::table;

            diesel::insert_into(table)
                .values(&user)
                .execute(txn)
                .context("failed to insert user into database")
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::users::table;

            diesel::insert_into(table)
                .values(&user)
                .execute(txn)
                .context("failed to insert user into database")
        });
    })
    .inspect_err(|e| {
        error!(error = %e, "failed to persist user into database");
        sentry_eyre::capture_report(e);
    })
    .map_err(|_| api::internal_server_error())?;

    ops::charts::create_index(&cx, &user)
        .await
        .map_err(|_| api::internal_server_error())?;

    // TODO(@auguwu): if this is a single organization registry, then add
    // them as an organization member

    Ok(api::ok(StatusCode::CREATED, user))
}

/// Locate a user by their ID or username.
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}",
    tags = ["Users"],
    operation_id = "getUserByIdOrName",
    params(
        (
            "idOrName" = NameOrUlid,
            Path,

            description = "Parameter that can take a `Name` or `Ulid`"
        )
    ),
    responses(
        (
            status = 200,
            description = "A single user found",
            body = api::Response<User>,
            content_type = "application/json"
        ),
        (
            status = 400,
            description = "Invalid ID or name specified",
            body = ApiErrorResponse,
            content_type = "application/json"
        ),
        (
            status = 404,
            description = "Entity Not Found",
            body = ApiErrorResponse,
            content_type = "application/json"
        )
    )
)]
pub async fn get_user(State(cx): State<ServerContext>, Path(id_or_name): Path<NameOrUlid>) -> api::Result<User> {
    match ops::db::user::get(&cx, id_or_name.clone()).await {
        Ok(Some(user)) => Ok(api::ok(StatusCode::OK, user)),
        Ok(None) => Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "user with id or name was not found",
                json!({"idOrName":id_or_name}),
            ),
        )),

        Err(_) => Err(api::internal_server_error()),
    }
}

/// Returns information about yourself via an authenticated request.
#[utoipa::path(
    get,
    path = "/v1/users/@me",
    operation_id = "getSelfUser",
    tags = ["Users"],
    responses(
        (
            status = 200,
            description = "A single user found",
            body = api::Response<User>,
            content_type = "application/json"
        ),
        (
            status = 4XX,
            description = "Any occurrence when authentication fails",
            body = ApiErrorResponse,
            content_type = "application/json"
        )
    )
)]
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
