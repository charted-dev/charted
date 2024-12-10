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

use crate::ServerContext;
use axum::extract::State;

#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatars",
    tag = "Users/Avatars",
    operation_id = "getUserAvatars"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_all_user_avatars(State(_): State<ServerContext>) {}

#[utoipa::path(
    get,
    path = "/v1/users/@me/avatars",
    tag = "Users/Avatars",
    operation_id = "getSelfUserAvatars"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_all_self_user_avatars() {}

#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatar",
    tag = "Users/Avatars",
    operation_id = "getUserCurrentAvatar"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_current_user_avatar() {}

#[utoipa::path(
    get,
    path = "/v1/users/@me/avatar",
    tag = "Users/Avatars",
    operation_id = "getSelfUserAvatar"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_self_user_avatar() {}

#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatar/{hash}",
    tag = "Users/Avatars",
    operation_id = "getUserAvatar"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_user_avatar() {}

#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatar/{hash}",
    tag = "Users/Avatars",
    operation_id = "getSelfUserAvatar"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_self_avatar() {}

#[utoipa::path(
    post,
    path = "/v1/users/{idOrName}/avatar",
    tag = "Users/Avatars",
    operation_id = "uploadUserAvatar"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn upload_avatar() {}

#[utoipa::path(
    delete,
    path = "/v1/users/{idOrName}/avatar/{hash}",
    tag = "Users/Avatars",
    operation_id = "deleteUserAvatar"
)]
pub async fn delete_avatar() {}
