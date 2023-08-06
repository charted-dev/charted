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
pub mod crud;
pub mod repositories;
pub mod sessions;

use crate::{openapi::gen_response_schema, Server};
use axum::Router;
use charted_common::models::entities::User;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;

#[derive(Debug, Clone, OpenApi)]
#[openapi(components(responses(UserResponse)))]
pub struct UsersOpenAPI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(flatten)]
    user: User,
}

gen_response_schema!(UserResponse, schema: "User");

pub fn create_router(_server: Server) -> Router<Server> {
    Router::new()
        .merge(crud::create_router())
        .nest("/avatars", avatars::create_router())
        .nest("/sessions", sessions::create_router())
        .nest("/repositories", repositories::create_router())
}
