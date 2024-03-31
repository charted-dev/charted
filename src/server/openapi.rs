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

use crate::openapi::openapi;
use charted_common::lazy;
use charted_proc_macros::add_paths;
use charted_server::APIVersion;
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
        charted_entities::payloads::PatchUserConnectionsPayload::schema(),
        charted_entities::payloads::CreateOrganizationPayload::schema(),
        charted_entities::payloads::PatchOrganizationPayload::schema(),
        charted_entities::payloads::CreateRepositoryPayload::schema(),
        charted_entities::payloads::PatchRepositoryPayload::schema(),
        charted_entities::payloads::CreateApiKeyPayload::schema(),
        charted_entities::payloads::PatchApiKeyPayload::schema(),
        charted_entities::payloads::CreateUserPayload::schema(),
        charted_entities::payloads::PatchUserPayload::schema(),
        charted_entities::payloads::UserLoginPayload::schema(),
        // other schemas
        charted_server::pagination::PaginatedOrganization::schema(),
        charted_server::pagination::PaginatedRepository::schema(),
        charted_entities::helm::StringOrImportValue::schema(),
        charted_server::pagination::PaginatedMember::schema(),
        charted_server::pagination::PaginatedApiKey::schema(),
        charted_entities::helm::ChartSpecVersion::schema(),
        charted_entities::helm::ChartDependency::schema(),
        charted_entities::helm::ChartMaintainer::schema(),
        charted_entities::helm::ChartIndexSpec::schema(),
        charted_server::pagination::PageInfo::schema(),
        charted_entities::RepositoryRelease::schema(),
        charted_server::pagination::OrderBy::schema(),
        charted_entities::helm::ImportValue::schema(),
        charted_entities::helm::ChartIndex::schema(),
        charted_entities::UserConnections::schema(),
        charted_entities::helm::ChartType::schema(),
        charted_entities::NameOrSnowflake::schema(),
        charted_common::serde::Duration::schema(),
        charted_entities::Distribution::schema(),
        charted_entities::Organization::schema(),
        charted_entities::ApiKeyScope::schema(),
        charted_entities::helm::Chart::schema(),
        charted_entities::Repository::schema(),
        charted_entities::Version::schema(),
        charted_server::ErrorCode::schema(),
        charted_entities::Member::schema(),
        charted_entities::ApiKey::schema(),
        crate::sessions::Session::schema(),
        charted_entities::User::schema(),
        charted_entities::Name::schema(),
        charted_server::Error::schema(),
        charted_common::ID::schema(),
        version_req(),
        datetime()
    ])
    .responses_from_iter([
        crate::server::routing::v1::organization::OrganizationResponse::response(),
        crate::server::routing::v1::user::sessions::SessionResponse::response(),
        crate::server::routing::v1::repository::RepositoryResponse::response(),
        crate::server::routing::v1::repository::RepositoryResponse::response(),
        charted_server::pagination::OrganizationPaginatedResponse::response(),
        crate::server::routing::v1::features::FeaturesResponse::response(),
        charted_server::pagination::RepositoryPaginatedResponse::response(),
        charted_server::pagination::ApiKeyPaginatedResponse::response(),
        charted_server::pagination::MemberPaginatedResponse::response(),
        crate::server::routing::v1::apikey::ApiKeyResponse::response(),
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

fn version_req<'s>() -> (&'s str, RefOr<Schema>) {
    let obj = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .description(Some("Represents a semantic version (https://semver.org) requirement (i.e, `>=1.2.0`) that Helm and charted-server will only accept"))
            .build();

    ("VersionReq", RefOr::T(Schema::Object(obj)))
}
