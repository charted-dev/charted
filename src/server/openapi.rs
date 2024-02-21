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

use super::version::APIVersion;
use crate::{lazy, openapi::openapi};
use charted_proc_macros::add_paths;
use once_cell::sync::Lazy;
use utoipa::{
    openapi::{
        Components, ComponentsBuilder, KnownFormat, ObjectBuilder, OpenApiBuilder, Paths, RefOr, Schema, SchemaFormat,
        SchemaType,
    },
    OpenApi, ToResponse, ToSchema,
};

static COMPONENTS: Lazy<Components> = lazy!(ComponentsBuilder::new()
    .schemas_from_iter([
        // response schemas
        crate::server::routing::v1::features::FeaturesResponse::schema(),
        crate::server::routing::v1::EntrypointResponse::schema(),
        crate::server::routing::v1::info::InfoResponse::schema(),
        crate::server::routing::v1::main::MainResponse::schema(),
        // request body schemas
        crate::common::models::payloads::PatchUserConnectionsPayload::schema(),
        crate::common::models::payloads::CreateOrganizationPayload::schema(),
        crate::common::models::payloads::PatchOrganizationPayload::schema(),
        crate::common::models::payloads::CreateRepositoryPayload::schema(),
        crate::common::models::payloads::PatchRepositoryPayload::schema(),
        crate::common::models::payloads::CreateApiKeyPayload::schema(),
        crate::common::models::payloads::PatchApiKeyPayload::schema(),
        crate::common::models::payloads::CreateUserPayload::schema(),
        crate::common::models::payloads::PatchUserPayload::schema(),
        crate::common::models::payloads::UserLoginPayload::schema(),
        // other schemas
        crate::common::models::entities::RepositoryRelease::schema(),
        crate::server::pagination::PaginatedOrganization::schema(),
        crate::common::models::helm::StringOrImportValue::schema(),
        crate::common::models::entities::UserConnections::schema(),
        crate::server::pagination::PaginatedRepository::schema(),
        crate::common::models::entities::Organization::schema(),
        crate::common::models::helm::ChartSpecVersion::schema(),
        crate::common::models::helm::ChartMaintainer::schema(),
        crate::common::models::helm::ChartDependency::schema(),
        crate::common::models::entities::ApiKeyScope::schema(),
        crate::common::models::entities::Repository::schema(),
        crate::common::models::helm::ChartIndexSpec::schema(),
        crate::server::pagination::PaginatedMember::schema(),
        crate::server::pagination::PaginatedApiKey::schema(),
        crate::common::models::helm::ImportValue::schema(),
        crate::common::models::entities::Member::schema(),
        crate::common::models::helm::ChartIndex::schema(),
        crate::common::models::entities::ApiKey::schema(),
        crate::common::models::helm::ChartType::schema(),
        crate::common::models::NameOrSnowflake::schema(),
        crate::server::models::res::ErrorCode::schema(),
        crate::common::models::entities::User::schema(),
        crate::server::pagination::PageInfo::schema(),
        crate::common::models::helm::Chart::schema(),
        crate::server::pagination::OrderBy::schema(),
        crate::server::extract::VersionReq::schema(),
        crate::server::models::res::Error::schema(),
        crate::server::extract::Version::schema(),
        crate::common::models::Name::schema(),
        crate::sessions::Session::schema(),
        crate::common::ID::schema(),
        datetime()
    ])
    .responses_from_iter([
        crate::server::routing::v1::user::sessions::SessionResponse::response(),
        crate::server::routing::v1::repository::RepositoryResponse::response(),
        crate::server::routing::v1::repository::RepositoryResponse::response(),
        crate::server::pagination::OrganizationPaginatedResponse::response(),
        crate::server::routing::v1::features::FeaturesResponse::response(),
        crate::server::pagination::RepositoryPaginatedResponse::response(),
        crate::server::routing::v1::apikey::ApiKeyResponse::response(),
        crate::server::pagination::ApiKeyPaginatedResponse::response(),
        crate::server::pagination::MemberPaginatedResponse::response(),
        crate::server::routing::v1::user::UserResponse::response(),
        crate::server::routing::v1::EntrypointResponse::response(),
        crate::server::routing::v1::info::InfoResponse::response(),
        crate::server::routing::v1::main::MainResponse::response(),
        crate::server::routing::v1::user::UserResponse::response(),
        crate::openapi::ApiErrorResponse::response(),
        crate::openapi::EmptyApiResponse::response(),
    ])
    .build());

pub struct Document;
impl Document {
    fn format(version: APIVersion, key: &'static str) -> String {
        if key == "/" {
            return format!("/{version}");
        }

        format!("/{version}{key}")
    }

    /// [`Paths`] of all available [`APIVersion::V1`] endpoints.
    pub fn v1() -> Paths {
        add_paths! {
            // ~~~~~~~~~~~ API KEYS ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/apikeys/{idOrName}") => [];
            Document::format(APIVersion::V1, "/apikeys/all") => crate::server::routing::v1::apikey::ListAllApikeysRestController::paths();
            Document::format(APIVersion::V1, "/apikeys") => [
                crate::server::routing::v1::apikey::EntrypointRestController::paths()
            ];

            // ~~~~~~~~~~~ ORGANIZATIONS ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/organizations/{idOrName}/repositories") => crate::server::routing::v1::organization::repositories::ListOrgRepositoriesRestController::paths();
            Document::format(APIVersion::V1, "/organizations/{idOrName}/icon/{hash}") => crate::server::routing::v1::organization::icons::GetOrgIconByHashRestController::paths();
            Document::format(APIVersion::V1, "/organizations/{idOrName}/icon") => crate::server::routing::v1::organization::icons::GetCurrentOrgIconRestController::paths();
            Document::format(APIVersion::V1, "/organizations/{idOrName}") => crate::server::routing::v1::organization::GetOrgByIdOrNameRestController::paths();
            Document::format(APIVersion::V1, "/organizations") => crate::server::routing::v1::organization::EntrypointRestController::paths();

            // ~~~~~~~~~~~ REPOSITORIES ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/repositories/{owner}/{name}") => crate::server::routing::v1::repository::GetRepoByOwnerAndNameRestController::paths();
            Document::format(APIVersion::V1, "/repositories/{id}") => [
                crate::server::routing::v1::repository::GetRepoByIdRestController::paths()
            ];

            Document::format(APIVersion::V1, "/repositories") => crate::server::routing::v1::repository::EntrypointRestController::paths();

            // ~~~~~~~~~~~     USERS     ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/users/{idOrName}/avatar/{hash}") => crate::server::routing::v1::user::avatars::GetUserAvatarByHashRestController::paths();
            Document::format(APIVersion::V1, "/users/{idOrName}/repositories") => crate::server::routing::v1::user::repositories::ListUserRepositoriesRestController::paths();
            Document::format(APIVersion::V1, "/users/{idOrName}/avatar") => crate::server::routing::v1::user::avatars::GetCurrentUserAvatarRestController::paths();
            Document::format(APIVersion::V1, "/users/{idOrName}") => crate::server::routing::v1::user::GetUserRestController::paths();

            Document::format(APIVersion::V1, "/users/@me/avatar/{hash}") => crate::server::routing::v1::user::avatars::GetSelfUserAvatarByHashRestController::paths();
            Document::format(APIVersion::V1, "/users/@me/avatar") => [
                crate::server::routing::v1::user::avatars::GetSelfUserAvatarRestController::paths(),
                crate::server::routing::v1::user::avatars::UploadAvatarRestController::paths()
            ];

            Document::format(APIVersion::V1, "/users/@me/repositories") => crate::server::routing::v1::user::repositories::CreateUserRepositoryRestController::paths();
            Document::format(APIVersion::V1, "/users/@me") => crate::server::routing::v1::user::GetSelfRestController::paths();
            Document::format(APIVersion::V1, "/users") => [
                crate::server::routing::v1::user::DeleteSelfRestController::paths(),
                crate::server::routing::v1::user::CreateUserRestController::paths(),
                crate::server::routing::v1::user::PatchRestController::paths(),
                crate::server::routing::v1::user::MainRestController::paths()
            ];

            // ~~~~~~~~~~~      MAIN     ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/index/{idOrName}") => crate::server::routing::v1::indexes::GetChartIndexRestController::paths();
            Document::format(APIVersion::V1, "/heartbeat") => crate::server::routing::v1::heartbeat::HeartbeatRestController::paths();
            Document::format(APIVersion::V1, "/features") => crate::server::routing::v1::features::FeaturesRestController::paths();
            Document::format(APIVersion::V1, "/info") => crate::server::routing::v1::info::InfoRestController::paths();
            Document::format(APIVersion::V1, "/") => crate::server::routing::v1::main::MainRestController::paths();
        }
    }

    /// [`Paths`] for all available recent API version endpoints.
    pub fn latest() -> Paths {
        add_paths! {
            // ~~~~~~~~~~~ API KEYS ~~~~~~~~~~~~~~~~~~~
            "/apikeys/{idOrName}" => [];
            "/apikeys/all" => crate::server::routing::v1::apikey::ListAllApikeysRestController::paths();
            "/apikeys" => [
                crate::server::routing::v1::apikey::EntrypointRestController::paths()
            ];

            // ~~~~~~~~~~~ ORGANIZATIONS ~~~~~~~~~~~~~~~~~~~
            "/organizations/{idOrName}/repositories" => crate::server::routing::v1::organization::repositories::ListOrgRepositoriesRestController::paths();
            "/organizations/{idOrName}/icon/{hash}" => crate::server::routing::v1::organization::icons::GetOrgIconByHashRestController::paths();
            "/organizations/{idOrName}/icon" => crate::server::routing::v1::organization::icons::GetCurrentOrgIconRestController::paths();
            "/organizations/{idOrName}" => crate::server::routing::v1::organization::GetOrgByIdOrNameRestController::paths();
            "/organizations" => crate::server::routing::v1::organization::EntrypointRestController::paths();

            // ~~~~~~~~~~~ REPOSITORIES ~~~~~~~~~~~~~~~~~~~
            "/repositories/{owner}/{name}" => crate::server::routing::v1::repository::GetRepoByOwnerAndNameRestController::paths();
            "/repositories/{id}" => [
                crate::server::routing::v1::repository::GetRepoByIdRestController::paths()
            ];

            "/repositories" => crate::server::routing::v1::repository::EntrypointRestController::paths();

            // ~~~~~~~~~~~     USERS     ~~~~~~~~~~~~~~~~~~~
            "/users/{idOrName}/avatar/{hash}" => crate::server::routing::v1::user::avatars::GetUserAvatarByHashRestController::paths();
            "/users/{idOrName}/repositories" => crate::server::routing::v1::user::repositories::ListUserRepositoriesRestController::paths();
            "/users/{idOrName}/avatar" => crate::server::routing::v1::user::avatars::GetCurrentUserAvatarRestController::paths();
            "/users/{idOrName}" => crate::server::routing::v1::user::GetUserRestController::paths();

            "/users/@me/avatar/{hash}" => crate::server::routing::v1::user::avatars::GetSelfUserAvatarByHashRestController::paths();
            "/users/@me/avatar" => [
                crate::server::routing::v1::user::avatars::GetSelfUserAvatarRestController::paths(),
                crate::server::routing::v1::user::avatars::UploadAvatarRestController::paths()
            ];

            "/users/@me/repositories" => crate::server::routing::v1::user::repositories::CreateUserRepositoryRestController::paths();
            "/users/@me" => crate::server::routing::v1::user::GetSelfRestController::paths();
            "/users" => [
                crate::server::routing::v1::user::DeleteSelfRestController::paths(),
                crate::server::routing::v1::user::CreateUserRestController::paths(),
                crate::server::routing::v1::user::PatchRestController::paths(),
                crate::server::routing::v1::user::MainRestController::paths()
            ];

            // ~~~~~~~~~~~      MAIN     ~~~~~~~~~~~~~~~~~~~
            "/index/{idOrName}" => crate::server::routing::v1::indexes::GetChartIndexRestController::paths();
            "/heartbeat" => crate::server::routing::v1::heartbeat::HeartbeatRestController::paths();
            "/features" => crate::server::routing::v1::features::FeaturesRestController::paths();
            "/info" => crate::server::routing::v1::info::InfoRestController::paths();
            "/" => crate::server::routing::v1::main::MainRestController::paths();
        }
    }
}

impl OpenApi for Document {
    fn openapi() -> utoipa::openapi::OpenApi {
        let mut builder = OpenApiBuilder::new().components(Some(COMPONENTS.clone())).build();
        builder.paths.paths.extend(Document::latest().paths);
        builder.paths.paths.extend(Document::v1().paths);

        let mut us = openapi();
        us.merge(builder);

        us
    }
}

fn datetime<'s>() -> (&'s str, RefOr<Schema>) {
    (
        "DateTime",
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
                .description(Some("RFC3339-encoded string that represents a datetime"))
                .read_only(Some(true))
                .build(),
        )),
    )
}
