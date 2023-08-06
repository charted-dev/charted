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

use crate::Server;
use axum::{routing, Router};

pub fn create_router() -> Router<Server> {
    Router::new()
        .route("/", routing::put(|| async {}).patch(|| async {}).delete(|| async {}))
        .route("/@me", routing::get(get::me::route))
        .route("/:idOrName", routing::get(get::user))
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
            Ok(None) => Err(err(StatusCode::NOT_FOUND, ("UNKNOWN_USER", "heck").into())),
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
        use axum::response::IntoResponse;
        use utoipa::openapi::{path::OperationBuilder, PathItem, PathItemType};

        pub async fn route() -> impl IntoResponse {}

        pub fn paths() -> PathItem {
            PathItem::new(PathItemType::Get, OperationBuilder::new())
        }
    }
}
