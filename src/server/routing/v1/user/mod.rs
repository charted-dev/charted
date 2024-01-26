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

pub mod avatars;
pub mod repositories;
pub mod sessions;

use crate::{
    common::models::{
        entities::{ApiKeyScope, ApiKeyScopes, User},
        helm::ChartIndex,
        payloads::{CreateUserPayload, PatchUserPayload},
        Name, NameOrSnowflake,
    },
    db::controllers::DbController,
    openapi::generate_response_schema,
    server::{
        controller,
        extract::Json,
        hash_password,
        middleware::session::{Middleware, Session},
        models::res::{
            err, internal_server_error, no_content, ok, ApiResponse, ErrorCode, Result, INTERNAL_SERVER_ERROR,
        },
        routing::v1::EntrypointResponse,
        validation::{validate, validate_email},
    },
    Instance, VERSION,
};
use axum::{
    extract::{Path, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use chrono::Local;
use remi::{Bytes, StorageService, UploadRequest};
use serde_json::json;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

pub struct UserResponse;
generate_response_schema!(UserResponse, schema = "User");

pub fn create_router() -> Router<Instance> {
    let nos = Router::new().route("/", routing::get(GetUserRestController::run));
    let me = Router::new().route(
        "/",
        routing::get(
            GetSelfRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                scopes: ApiKeyScopes::init(ApiKeyScope::UserAccess.into()),
                ..Default::default()
            })),
        ),
    );

    Router::new()
        .route(
            "/",
            routing::get(MainRestController::run)
                .put(CreateUserRestController::run)
                .patch(
                    PatchRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                        scopes: ApiKeyScopes::init(ApiKeyScope::UserUpdate.into()),
                        ..Default::default()
                    })),
                )
                .delete(
                    DeleteSelfRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                        scopes: ApiKeyScopes::init(ApiKeyScope::UserDelete.into()),
                        ..Default::default()
                    })),
                ),
        )
        .nest("/@me", me)
        .nest("/:idOrName", nos)
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

/// Creates a new user if the server allows registrations.
#[controller(
    method = put,
    tags("Users"),
    requestBody("Payload for creating a new user. `password` can be empty if the server's session manager is not Local", ("application/json", schema!("CreateUserPayload"))),
    response(200, "Successful response", ("application/json", response!("UserResponse"))),
    response(403, "Whether if this server doesn't allow registrations", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the `username` or `email` was taken.", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn create_user(
    State(Instance {
        controllers,
        snowflake,
        storage,
        config,
        authz,
        pool,
        ..
    }): State<Instance>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<User> {
    if !config.registrations {
        return Err(err(
            StatusCode::FORBIDDEN,
            (
                ErrorCode::RegistrationsDisabled,
                "this instance disabled registrations!",
            ),
        ));
    }

    validate(&payload, CreateUserPayload::validate)?;

    if authz.is_local() && payload.password.is_none() {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                ErrorCode::MissingPassword,
                "authz backend requires you to have a password backed to this account",
            ),
        ));
    }

    validate(&payload.username, Name::validate)?;
    match controllers.users.get_by(&payload.username).await {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    ErrorCode::EntityAlreadyExists,
                    "entity with given username already exists",
                    json!({"username":payload.username}),
                ),
            ))
        }

        Err(_) => return Err(err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR)),
    }

    validate_email(payload.email.clone())?;
    match sqlx::query("select users.id from users where email = $1;")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    ErrorCode::EntityAlreadyExists,
                    "entity with given email already exists",
                    json!({"email":payload.email}),
                ),
            ))
        }

        Err(e) => {
            error!(error = %e, user.email = payload.email, "unable to query user by email");
            sentry::capture_error(&e);

            return Err(err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR));
        }
    }

    let password = if let Some(ref pass) = payload.password {
        if pass.len() < 8 {
            return Err(err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    ErrorCode::InvalidPassword,
                    "password was expected to be 8 characters or longer",
                ),
            ));
        }

        Some(hash_password(pass).map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR))?)
    } else {
        None
    };

    let id = snowflake.generate();
    let user = User {
        created_at: Local::now(),
        updated_at: Local::now(),
        password,
        username: payload.username.clone(),
        email: payload.email.clone(),
        id: id.value().try_into().unwrap(),

        ..Default::default()
    };

    controllers
        .users
        .create(payload, &user)
        .await
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR))?;

    // create the chart index for this user
    let index = ChartIndex::default();
    let serialized = serde_yaml::to_string(&index).unwrap();
    storage
        .upload(
            format!("./metadata/{}/index.yaml", user.id),
            UploadRequest::default()
                .with_content_type(Some("text/yaml; charset=utf-8"))
                .with_data(Bytes::from(serialized)),
        )
        .await
        .map_err(|e| {
            error!(error = %e, user.id, "unable to create chart index for user");
            sentry::capture_error(&e);

            err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR)
        })?;

    Ok(ok(StatusCode::CREATED, user))
}

/// Retrieve a user by their ID or username.
#[controller(
    tags("Users"),
    response(200, "Successful response", ("application/json", response!("UserResponse"))),
    response(400, "Invalid `idOrName` specified", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Unknown User", ("application/json", response!("ApiErrorResponse"))),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a [`Name`] or [`Snowflake`] identifier.")
)]
pub async fn get_user(
    State(Instance { controllers, .. }): State<Instance>,
    Path(nos): Path<NameOrSnowflake>,
) -> Result<User> {
    validate(&nos, NameOrSnowflake::validate)?;
    match controllers.users.get_by(&nos).await {
        Ok(Some(user)) => Ok(ok(StatusCode::OK, user)),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "user with id or username doesn't exist",
                json!({"idOrName":nos}),
            ),
        )),

        Err(_) => Err(err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR)),
    }
}

/// Returns a [User] from an authenticated request.
#[controller(
    tags("Users"),
    securityRequirements(("ApiKey", ["users:access"]), ("Bearer", []), ("Basic", [])),
    response(200, "Returns the authenticated user", ("application/json", response!("UserResponse"))),
    response(400, "If the request body was invalid (i.e, validation errors)", ("application/json", response!("ApiErrorResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid.", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the request body contained invalid data, or if the session header contained invalid data", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_self(Extension(Session { user, .. }): Extension<Session>) -> ApiResponse<User> {
    ok(StatusCode::OK, user)
}

/// Patch the current authenticated user's metadata about themselves.
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
pub async fn patch(
    State(Instance { controllers, pool, .. }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(payload): Json<PatchUserPayload>,
) -> Result<()> {
    validate(&payload, PatchUserPayload::validate)?;

    if let Some(ref username) = payload.username {
        validate(username, Name::validate)?;

        match controllers
            .users
            .exists_by(NameOrSnowflake::Name(username.clone()))
            .await
        {
            Ok(false) => {}
            Ok(true) => {
                return Err(err(
                    StatusCode::CONFLICT,
                    (
                        ErrorCode::EntityAlreadyExists,
                        "unable to patch `username`: user with given username already exists",
                        json!({"username":username}),
                    ),
                ))
            }

            Err(_) => return Err(internal_server_error()),
        }
    }

    if let Some(ref email) = payload.email {
        validate_email(email.clone())?;

        match sqlx::query("select count(1) from users where email = $1;")
            .bind(email)
            .fetch_optional(&pool)
            .await
        {
            Ok(_) => {
                return Err(err(
                    StatusCode::CONFLICT,
                    (
                        ErrorCode::EntityAlreadyExists,
                        "unable to patch `email`: user with given email already exists",
                        json!({"email":email}),
                    ),
                ))
            }

            Err(sqlx::Error::ColumnNotFound(_)) => {}
            Err(_) => return Err(internal_server_error()),
        }
    }

    controllers
        .users
        .patch(user.id, payload)
        .await
        .map(|_| no_content())
        .map_err(|_| internal_server_error())
}

/// Delete your user and the data attached to you from the service
#[controller(
    method = delete,
    tags("Users"),
    securityRequirements(("ApiKey", ["users:delete"]), ("Bearer", []), ("Basic", [])),
    response(201, "Successful response", ("application/json", response!("EmptyApiResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid or expired", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn delete_self(
    State(Instance { controllers, .. }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
) -> Result<()> {
    // TODO(@auguwu): create a background job on deleting all data
    controllers
        .users
        .delete(user.id)
        .await
        .map(|()| ok(StatusCode::ACCEPTED, ()))
        .map_err(|_| internal_server_error())
}
