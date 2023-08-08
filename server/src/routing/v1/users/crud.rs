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

use crate::{middleware::SessionAuth, models::res::ok, openapi::gen_response_schema, Server};
use axum::{
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{self, MethodFilter},
    Router,
};
use charted_common::VERSION;
use serde::{Deserialize, Serialize};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        request_body::RequestBodyBuilder,
        ContentBuilder, PathItem, PathItemType, Ref, RefOr, ResponseBuilder,
    },
    ToSchema,
};

/// Response object for `GET /users`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MainUserResponse {
    /// The message, which will always be "Hello, world!"
    message: String,

    /// Documentation URL for this generic entrypoint response.
    docs: String,
}

gen_response_schema!(MainUserResponse);

impl Default for MainUserResponse {
    fn default() -> Self {
        MainUserResponse {
            message: "Welcome to the Users API!".into(),
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}/api/reference/users"),
        }
    }
}

pub fn create_router(server: Server) -> Router<Server> {
    Router::new()
        .route(
            "/",
            routing::get(main)
                .put(|| async {})
                .on(
                    MethodFilter::PATCH,
                    (|| async {}).layer(AsyncRequireAuthorizationLayer::new(SessionAuth)),
                )
                .on(
                    MethodFilter::DELETE,
                    (|| async {}).layer(AsyncRequireAuthorizationLayer::new(SessionAuth)),
                ),
        )
        .route(
            "/@me",
            routing::get(get::me::route).route_layer(AsyncRequireAuthorizationLayer::new(SessionAuth)),
        )
        .route("/:idOrName", routing::get(get::user))
        .with_state(server.clone())
}

pub async fn main() -> impl IntoResponse {
    ok(StatusCode::OK, MainUserResponse::default())
}

pub fn paths() -> PathItem {
    PathItemBuilder::new()
        .operation(
            PathItemType::Get,
            OperationBuilder::new()
                .operation_id(Some("users"))
                .description(Some("Generic main entrypoint for the Users API."))
                .response(
                    "200",
                    ResponseBuilder::new()
                        .description("200 OK - Successful response for users")
                        .content(
                            "application/json",
                            ContentBuilder::new()
                                .schema(RefOr::Ref(Ref::from_response_name("MainUserResponse")))
                                .build(),
                        ),
                ),
        )
        .operation(
            PathItemType::Put,
            OperationBuilder::new()
                .operation_id(Some("create_user"))
                .description(Some("Creates a new [User]"))
                .request_body(Some(
                    RequestBodyBuilder::new()
                        .description(Some("Payload for creating a new user."))
                        .content(
                            "application/json",
                            ContentBuilder::new()
                                .schema(RefOr::Ref(Ref::from_schema_name("CreateUserPayload")))
                                .build(),
                        )
                        .build(),
                ))
                .response(
                    "201",
                    ResponseBuilder::new()
                        .description("201 Created: Newly created [User].")
                        .content(
                            "application/json",
                            ContentBuilder::new()
                                .schema(RefOr::Ref(Ref::from_response_name("ApiUserResponse")))
                                .build(),
                        ),
                )
                .response(
                    "403",
                    ResponseBuilder::new()
                        .description("403 Forbidden: Whether or not this server doesn't allow new users to be created.")
                        .content(
                            "application/json",
                            ContentBuilder::new()
                                .schema(RefOr::Ref(Ref::from_response_name("ApiErrorResponse")))
                                .build(),
                        ),
                )
                .response(
                    "406",
                    ResponseBuilder::new()
                        .description("406 Not Acceptable: If the session manager for this server is local, then this is indicated if the `password` field in the [CreateUserPayload] object was not found.")
                        .content(
                            "application/json",
                            ContentBuilder::new()
                                .schema(RefOr::Ref(Ref::from_response_name("ApiErrorResponse")))
                                .build(),
                        ),
                )
                .build(),
        )
        .build()
}

pub mod get {
    use crate::{
        models::res::{err, ok, ApiResponse},
        Server,
    };
    use axum::{
        extract::{Path, State},
        http::StatusCode,
    };
    use charted_common::models::{entities::User, NameOrSnowflake};
    use charted_database::{controllers::users::UserDatabaseController, extensions::snowflake::SnowflakeExt};
    use utoipa::openapi::{
        path::OperationBuilder, ContentBuilder, PathItem, PathItemType, Ref, RefOr, ResponseBuilder,
    };

    pub async fn user(
        State(server): State<Server>,
        Path(id_or_name): Path<NameOrSnowflake>,
    ) -> Result<ApiResponse<User>, ApiResponse> {
        let id_or_name = match id_or_name.is_valid() {
            Ok(()) => id_or_name,
            Err(why) => {
                error!(error = why, "received invalid 'idOrName' parameter");
                return Err(err(StatusCode::NOT_ACCEPTABLE, ("INVALID_ID_OR_NAME", why).into()));
            }
        };

        let users = server.controller::<UserDatabaseController>();
        match users.get_with_id_or_name(id_or_name.clone()).await {
            Ok(Some(user)) => Ok(ok(StatusCode::OK, user)),
            Ok(None) => Err(err(
                StatusCode::NOT_FOUND,
                ("UNKNOWN_USER", format!("unable to find user {id_or_name}").as_str()).into(),
            )),
            Err(e) => {
                error!(%e, "unable to get user with idOrName {id_or_name}");
                Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ))
            }
        }
    }

    pub fn paths() -> PathItem {
        PathItem::new(
            PathItemType::Get,
            OperationBuilder::new()
                .operation_id(Some("get_user"))
                .description(Some("Returns a [User] with a given user ID or username"))
                .response(
                    "200",
                    ResponseBuilder::new()
                        .description("200 OK - Successful response for get_user")
                        .content(
                            "application/json",
                            ContentBuilder::new()
                                .schema(RefOr::Ref(Ref::from_response_name("ApiUserResponse")))
                                .build(),
                        ),
                ),
        )
    }

    pub mod me {
        use crate::{middleware::Session, models::res::ok};
        use axum::{http::StatusCode, response::IntoResponse, Extension};
        use utoipa::openapi::{
            path::OperationBuilder, ContentBuilder, PathItem, PathItemType, Ref, RefOr, ResponseBuilder,
        };

        pub async fn route(Extension(Session { user, .. }): Extension<Session>) -> impl IntoResponse {
            ok(StatusCode::OK, user)
        }

        pub fn paths() -> PathItem {
            PathItem::new(
                PathItemType::Get,
                OperationBuilder::new()
                    .operation_id(Some("get_myself"))
                    .description(Some("Returns a [User] with any given authentication details"))
                    .response(
                        "200",
                        ResponseBuilder::new()
                            .description("200 OK: Successful response for `get_myself`")
                            .content(
                                "application/json",
                                ContentBuilder::new()
                                    .schema(RefOr::Ref(Ref::from_response_name("ApiUserResponse")))
                                    .build(),
                            ),
                    ),
            )
        }
    }
}
