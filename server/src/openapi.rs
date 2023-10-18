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

use crate::APIVersion;
use charted_common::{
    lazy,
    models::{
        entities::{ApiKey, Member, Organization, Repository, RepositoryRelease, User},
        helm::{
            Chart, ChartDependency, ChartMaintainer, ChartSpecVersion, ChartType, ImportValue, StringOrImportValue,
        },
        Distribution, Name, NameOrSnowflake,
    },
    ID,
};
use charted_openapi::{openapi, ApiErrorResponse, EmptyApiResponse};
use once_cell::sync::Lazy;
use utoipa::{
    openapi::{Components, ComponentsBuilder, OpenApiBuilder, Paths},
    OpenApi, ToResponse, ToSchema,
};

macro_rules! path {
    ($version:expr, { $($key:literal => $fn:expr;)* }) => {
        ::charted_openapi::add_paths! {
            $(
                Document::format($version, $key) => $fn;
            )*
        }
    };

    ($($key:literal => $fn:expr;)*) => {
        ::charted_openapi::add_paths! {
            $(
                $key => $fn;
            )*
        }
    };
}

static COMPONENTS: Lazy<Components> = lazy!(ComponentsBuilder::new()
    .schemas_from_iter([
        crate::routing::v1::features::FeaturesResponse::schema(),
        charted_common::server::pagination::PageInfo::schema(),
        charted_common::server::pagination::OrderBy::schema(),
        crate::pagination::PaginatedOrganization::schema(),
        crate::pagination::PaginatedRepository::schema(),
        crate::routing::v1::main::MainResponse::schema(),
        crate::routing::v1::info::InfoResponse::schema(),
        crate::routing::v1::EntrypointResponse::schema(),
        crate::pagination::PaginatedMember::schema(),
        crate::extract::VersionReq::schema(),
        crate::models::res::Error::schema(),
        charted_sessions::Session::schema(),
        crate::extract::Version::schema(),
        StringOrImportValue::schema(),
        RepositoryRelease::schema(),
        ChartSpecVersion::schema(),
        ChartDependency::schema(),
        NameOrSnowflake::schema(),
        ChartMaintainer::schema(),
        Organization::schema(),
        Distribution::schema(),
        ImportValue::schema(),
        Repository::schema(),
        ChartType::schema(),
        Member::schema(),
        ApiKey::schema(),
        Chart::schema(),
        Name::schema(),
        User::schema(),
        ID::schema(),
    ])
    .responses_from_iter([
        crate::routing::v1::users::sessions::SessionResponse::response(),
        crate::pagination::OrganizationPaginatedResponse::response(),
        crate::pagination::OrganizationPaginatedResponse::response(),
        crate::pagination::RepositoryPaginatedResponse::response(),
        crate::routing::v1::features::FeaturesResponse::response(),
        crate::pagination::MemberPaginatedResponse::response(),
        crate::routing::v1::users::UserResponse::response(),
        crate::routing::v1::main::MainResponse::response(),
        crate::routing::v1::info::InfoResponse::response(),
        crate::routing::v1::EntrypointResponse::response(),
        ApiErrorResponse::response(),
        EmptyApiResponse::response()
    ])
    .build());

/// Represents an [`OpenAPI`] document for charted-server.
pub struct Document;

impl Document {
    fn format(version: APIVersion, key: &'static str) -> String {
        format!("/{version}{key}")
    }

    /// Returns a [`Paths`] object for API version v1.
    pub fn v1() -> Paths {
        path!(APIVersion::V1, {
            // organizations

            // repositories

            // api keys
            "/apikeys" => crate::routing::v1::apikeys::EntrypointRestController::paths();

            // users
            "/users/{idOrName}/repositories" => crate::routing::v1::users::repositories::ListUserRepositoriesRestController::paths();
            "/users/sessions/refresh-token" => crate::routing::v1::users::sessions::RefreshSessionTokenRestController::paths();
            "/users/{idOrName}/avatar" => crate::routing::v1::users::avatars::GetCurrentUserAvatarRestController::paths();
            "/users/@me/repositories" => crate::routing::v1::users::repositories::CreateUserRepositoryRestController::paths();
            "/users/sessions/logout" => crate::routing::v1::users::sessions::LogoutRestController::paths();
            "/users/@me/avatar" => crate::routing::v1::users::avatars::me::paths();
            "/users/{idOrName}" => crate::routing::v1::users::GetUserRestController::paths();
            "/users/login" => crate::routing::v1::users::sessions::LoginRestController::paths();
            "/users/@me" => crate::routing::v1::users::GetSelfRestController::paths();
            "/users" => crate::routing::v1::users::paths();

            // main
            "/indexes/{idOrName}" => crate::routing::v1::indexes::GetIndexRestController::paths();
            "/heartbeat" => crate::routing::v1::heartbeat::HeartbeatRestController::paths();
            "/features" => crate::routing::v1::features::FeaturesRestController::paths();
            "/info" => crate::routing::v1::info::InfoRestController::paths();
            "/" => crate::routing::v1::main::MainRestController::paths();
        })
    }

    /// Returns a [`Paths`] object for the latest API version.
    pub fn paths() -> Paths {
        path! {
            // organizations

            // repositories

            // api keys
            "/apikeys" => crate::routing::v1::apikeys::EntrypointRestController::paths();

            // users
            "/users/{idOrName}/repositories" => crate::routing::v1::users::repositories::ListUserRepositoriesRestController::paths();
            "/users/sessions/refresh-token" => crate::routing::v1::users::sessions::RefreshSessionTokenRestController::paths();
            "/users/{idOrName}/avatar" => crate::routing::v1::users::avatars::GetCurrentUserAvatarRestController::paths();
            "/users/@me/repositories" => crate::routing::v1::users::repositories::CreateUserRepositoryRestController::paths();
            "/users/sessions/logout" => crate::routing::v1::users::sessions::LogoutRestController::paths();
            "/users/@me/avatar" => crate::routing::v1::users::avatars::me::paths();
            "/users/{idOrName}" => crate::routing::v1::users::GetUserRestController::paths();
            "/users/login" => crate::routing::v1::users::sessions::LoginRestController::paths();
            "/users/@me" => crate::routing::v1::users::GetSelfRestController::paths();
            "/users" => crate::routing::v1::users::paths();

            // main
            "/indexes/{idOrName}" => crate::routing::v1::indexes::GetIndexRestController::paths();
            "/heartbeat" => crate::routing::v1::heartbeat::HeartbeatRestController::paths();
            "/features" => crate::routing::v1::features::FeaturesRestController::paths();
            "/info" => crate::routing::v1::info::InfoRestController::paths();
            "/" => crate::routing::v1::main::MainRestController::paths();
        }
    }
}

impl OpenApi for Document {
    fn openapi() -> utoipa::openapi::OpenApi {
        let builder = OpenApiBuilder::new()
            .components(Some(COMPONENTS.clone()))
            .paths(Document::paths())
            .paths(Document::v1())
            .build();

        let mut us = openapi();
        us.merge(builder);

        us
    }
}
