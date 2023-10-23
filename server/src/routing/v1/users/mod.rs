// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use self::{
    avatars::GetCurrentUserAvatarRestController,
    repositories::{CreateUserRepositoryRestController, ListUserRepositoriesRestController},
    sessions::LoginRestController,
};
use super::EntrypointResponse;
use crate::{
    extract::Json,
    macros::controller,
    middleware::{Session, SessionAuth},
    models::res::{err, no_content, ok, ApiResponse, Empty},
    validation::{validate, validate_email},
    Server,
};
use axum::{extract::State, handler::Handler, http::StatusCode, routing, Extension, Router};
use charted_common::{
    extract::NameOrSnowflake,
    models::{
        entities::{ApiKeyScope, User},
        helm::ChartIndex,
        payloads::{CreateUserPayload, PatchUserPayload},
        Name,
    },
    VERSION,
};
use charted_database::controller::{users::UserDatabaseController, DbController};
use charted_openapi::generate_response_schema;
use charted_storage::Bytes;
use chrono::Local;
use remi_core::{StorageService, UploadRequest};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

pub fn create_router() -> Router<Server> {
    let main_route = routing::get(MainRestController::run)
        .put(CreateUserRestController::run)
        .patch(PatchUserRestController::run.layer(AsyncRequireAuthorizationLayer::new(
            SessionAuth::default().scope(ApiKeyScope::UserUpdate),
        )))
        .delete(DeleteUserRestController::run.layer(AsyncRequireAuthorizationLayer::new(
            SessionAuth::default().scope(ApiKeyScope::UserDelete),
        )));

    let id_or_name_router = Router::new()
        .route("/", routing::get(GetUserRestController::run))
        .route("/avatar", routing::get(GetCurrentUserAvatarRestController::run))
        .route("/repositories", routing::get(ListUserRepositoriesRestController::run));

    let me_router =
        Router::new()
            .route(
                "/",
                routing::get(GetSelfRestController::run.layer(AsyncRequireAuthorizationLayer::new(
                    SessionAuth::default().scope(ApiKeyScope::UserAccess),
                ))),
            )
            .route(
                "/avatar",
                routing::get(
                    avatars::me::GetMyCurrentAvatarRestController::run
                        .layer(AsyncRequireAuthorizationLayer::new(SessionAuth::default())),
                )
                .post(avatars::UploadUserAvatarRestController::run.layer(
                    AsyncRequireAuthorizationLayer::new(SessionAuth::default().scope(ApiKeyScope::UserAvatarUpdate)),
                )),
            )
            .route(
                "/repositories",
                routing::put(
                    CreateUserRepositoryRestController::run.layer(AsyncRequireAuthorizationLayer::new(
                        SessionAuth::default().scope(ApiKeyScope::RepoCreate),
                    )),
                ),
            );

    Router::new()
        .nest("/sessions", sessions::create_router())
        .route("/login", routing::post(LoginRestController::run))
        .nest("/@me", me_router)
        .nest("/:idOrName", id_or_name_router)
        .route("/", main_route)
}

/// Generic entrypoint route for the Users API.
#[controller(id = "users", tags("Users"), response(200, "Successful response", ("application/json", response!("EntrypointResponse"))))]
async fn main() {
    ok(
        StatusCode::OK,
        EntrypointResponse {
            message: "Welcome to the Users API".into(),
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}/api/users"),
        },
    )
}

pub struct UserResponse;
generate_response_schema!(UserResponse, schema = "User");

/// Creates a new user if the server allows registrations.
#[controller(
    method = put,
    tags("Users"),
    requestBody("Payload for creating a new user. `password` can be empty if the server's session manager is not Local", ("application/json", schema!("CreateUserPayload"))),
    response(200, "Successful response", ("application/json", response!("UserResponse"))),
    response(403, "Whether if this server doesn't allow registrations", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the `username` or `email` was taken.", ("application/json", response!("ApiErrorResponse")))
)]
async fn create_user(
    State(server): State<Server>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<ApiResponse<User>, ApiResponse> {
    if !server.config.registrations {
        return Err(err(
            StatusCode::FORBIDDEN,
            (
                "REGISTRATIONS_DISABLED",
                "This instance is not allowing registrations at this given moment.",
            )
                .into(),
        ));
    }

    if payload.password.is_none() {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            ("MISSING_PASSWORD", "Missing the password to create with.").into(),
        ));
    }

    let username = validate(payload.username.clone(), Name::validate)?;
    match sqlx::query("SELECT users.* FROM users WHERE username = $1;")
        .bind(username.as_str())
        .fetch_optional(&server.pool)
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    "USER_EXISTS",
                    format!("user with username {username} already exists!").as_str(),
                )
                    .into(),
            ))
        }

        Err(e) => {
            error!(username = tracing::field::display(username.clone()), %e, "failed to query user {username}");
            sentry::capture_error(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    let email = validate_email(payload.email.clone())?;
    match sqlx::query("select users.* from users where email = $1;")
        .bind(email.clone())
        .fetch_optional(&server.pool)
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                ("USER_EXISTS", "user with email received already exists").into(),
            ))
        }

        Err(e) => {
            error!(email, %e, "failed to query user with");
            sentry::capture_error(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    let password = payload.password.as_ref().unwrap();
    if password.len() < 8 {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                "PASSWORD_LENGTH_NOT_ACCEPTED",
                "password was expected to be 8 or more characters long",
            )
                .into(),
        ));
    }

    let id = server.snowflake.generate();
    let password = charted_common::server::hash_password(password.clone()).map_err(|_| {
        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    let user = User {
        created_at: Local::now(),
        updated_at: Local::now(),
        password: Some(password),
        username,
        email,
        id: i64::try_from(id.value()).unwrap(),

        ..Default::default()
    };

    let users = server.controllers.get::<UserDatabaseController>();
    let user = users.create(payload, user).await.map_err(|_| {
        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    let index = ChartIndex::default();
    let serialized = serde_yaml::to_string(&index).unwrap();
    server
        .storage
        .upload(
            format!("./metadata/{}/index.yaml", user.id),
            UploadRequest::default()
                .with_content_type(Some("text/yaml; charset=utf-8".into()))
                .with_data(Bytes::from(serialized))
                .seal(),
        )
        .await
        .map_err(|e| {
            error!(user.id, error = %e, "unable to upload [./metadata/{}/index.yaml] to storage service", user.id);
            sentry::capture_error(&e);

            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            )
        })?;

    Ok(ok(StatusCode::CREATED, user))
}

/// Retrieve a [`User`] object.
#[controller(
    tags("Users"),
    response(200, "Successful response", ("application/json", response!("UserResponse"))),
    response(400, "Invalid `idOrName` specified", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Unknown User", ("application/json", response!("ApiErrorResponse"))),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a [`Name`] or [`Snowflake`] identifier.")
)]
pub async fn get_user(
    State(Server { controllers, .. }): State<Server>,
    NameOrSnowflake(id_or_name): NameOrSnowflake,
) -> Result<ApiResponse<User>, ApiResponse> {
    let users = controllers.get::<UserDatabaseController>();
    match users.get_by_nos(id_or_name.clone()).await {
        Ok(Some(user)) => Ok(ok(StatusCode::OK, user)),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (
                "UNKNOWN_USER",
                format!("User with ID or name [{id_or_name}] was not found.").as_str(),
            )
                .into(),
        )),

        Err(_) => Err(err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )),
    }
}

/// Returns a [User] from an authenticated request.
#[controller(
    tags("Users"),
    securityRequirements(("ApiKey", ["users:access"]), ("Bearer", []), ("Basic", [])),
    response(200, "Returns the current authenticated user's metadata", ("application/json", response!("UserResponse"))),
    response(400, "If the request body was invalid (i.e, validation errors)", ("application/json", response!("ApiErrorResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid.", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the request body contained invalid data, or if the session header contained invalid data", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_self(Extension(Session { user, .. }): Extension<Session>) -> ApiResponse<User> {
    ok(StatusCode::OK, user)
}

/// Patches the current authenticated user's metadata.
#[controller(
    method = patch,
    tags("Users"),
    securityRequirements(("ApiKey", ["users:update"]), ("Bearer", []), ("Basic", [])),
    response(204, "Successful response", ("application/json", response!("EmptyApiResponse"))),
    response(400, "If the request body was invalid (i.e, validation errors)", ("application/json", response!("ApiErrorResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid.", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the request body contained invalid data, or if the session header contained invalid data", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn patch_user(
    State(server): State<Server>,
    Extension(Session { user, .. }): Extension<Session>,
    payload: Json<PatchUserPayload>,
) -> Result<ApiResponse, ApiResponse> {
    validate(payload.clone(), PatchUserPayload::validate)?;

    // validate username (since it doesn't validate it D:)
    if let Some(username) = payload.username.clone() {
        validate(username.clone(), Name::validate)?;

        // check if username exists
        match sqlx::query("SELECT users.* FROM users WHERE username = $1;")
            .bind(username.as_str())
            .fetch_optional(&server.pool)
            .await
        {
            Ok(None) => {}
            Ok(Some(_)) => {
                return Err(err(
                    StatusCode::CONFLICT,
                    (
                        "USER_EXISTS",
                        format!("unable to patch: user with username {username} already exists").as_str(),
                    )
                        .into(),
                ))
            }

            Err(e) => {
                error!(username = tracing::field::display(username.clone()), %e, "failed to query user {username}");
                sentry::capture_error(&e);

                return Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ));
            }
        }
    }

    // check if an email already exists by an another user
    if let Some(email) = payload.email.clone() {
        match sqlx::query("select users.* from users where email = $1;")
            .bind(email.clone())
            .fetch_optional(&server.pool)
            .await
        {
            Ok(None) => {}
            Ok(Some(_)) => {
                return Err(err(
                    StatusCode::CONFLICT,
                    ("USER_EXISTS", "user with email received already exists").into(),
                ))
            }

            Err(e) => {
                error!(email, %e, "failed to query user with");
                sentry::capture_error(&e);

                return Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ));
            }
        }
    }

    let users = server.controllers.get::<UserDatabaseController>();
    users
        .patch(u64::try_from(user.id).unwrap(), payload.clone())
        .await
        .map_err(|_| {
            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            )
        })?;

    Ok(no_content())
}

#[controller(
    method = delete,
    tags("Users"),
    securityRequirements(("ApiKey", ["users:delete"]), ("Bearer", []), ("Basic", [])),
    response(201, "Successful response", ("application/json", response!("EmptyApiResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid or expired", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn delete_user(
    State(server): State<Server>,
    Extension(Session {
        user,
        session: _session,
    }): Extension<Session>,
) -> Result<ApiResponse, ApiResponse> {
    let users = server.controllers.get::<UserDatabaseController>();
    users.delete(u64::try_from(user.id).unwrap()).await.map_err(|_| {
        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    // in a background task, delete all entities and helm charts
    // that existed with that user.
    tokio::spawn(async move {
        let _helm_charts = server.helm_charts.clone();
        let _pool = server.pool.clone();
        let _user = user.clone();
    });

    Ok(ok(StatusCode::ACCEPTED, Empty))
}
