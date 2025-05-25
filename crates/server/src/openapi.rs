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

mod addons;
mod types;

use addons::{IncludeDefaultVersionWithoutPrefix, IncludeErrorProneSchemas};
pub use types::{
    ApiErrorResponse, ApiKeyResponse, EmptyApiResponse, ListApiKeyResponse, ListOrganizationResponse,
    ListRepositoryResponse, OrganizationResponse, RepositoryResponse, SessionResponse, Url, UrlResponse,
    UserResponse,
};
use utoipa::{
    Modify, OpenApi,
    openapi::{
        ComponentsBuilder,
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

#[derive(OpenApi)]
#[openapi(
    modifiers(
        &IncludeDefaultVersionWithoutPrefix,
        &IncludeErrorProneSchemas
    ),
    info(
        title = "charted-server",
        description = "🐻‍❄️📦 Free, open-source way to distribute Helm charts across the world",
        version = charted_core::VERSION,
        terms_of_service = "https://charts.noelware.org/legal/tos",
        license(identifier = "Apache-2.0", name = "Apache 2.0 License"),
        contact(
            name = "Noelware, LLC.",
            email = "team@noelware.org",
            url = "https://noelware.org"
        )
    ),
    components(
        schemas(
            //                          request bodies                          \\
            charted_types::payloads::CreateRepositoryReleasePayload,
            charted_types::payloads::PatchRepositoryReleasePayload,
            charted_types::payloads::CreateOrganizationPayload,
            charted_types::payloads::PatchOrganizationPayload,
            charted_types::payloads::CreateRepositoryPayload,
            charted_types::payloads::PatchRepositoryPayload,
            charted_types::payloads::CreateApiKeyPayload,
            charted_types::payloads::PatchApiKeyPayload,
            charted_types::payloads::CreateUserPayload,
            charted_types::payloads::PatchUserPayload,

            //                                scopes                            \\
            charted_core::bitflags::ApiKeyScope,

            //                              helm types                          \\
            charted_helm_types::StringOrImportValue,
            charted_helm_types::ChartSpecVersion,
            charted_helm_types::ChartMaintainer,
            charted_helm_types::ChartDependency,
            charted_helm_types::ChartIndexSpec,
            charted_helm_types::ImportValue,
            charted_helm_types::ChartIndex,
            charted_helm_types::ChartType,
            charted_helm_types::Chart,

            //                                entities                          \\
            charted_types::RepositoryRelease,
            charted_types::RepositoryMember,
            charted_types::Repository,

            charted_types::OrganizationMember,
            charted_types::Organization,

            charted_types::UserConnections,
            charted_types::Session,
            charted_types::ApiKey,
            charted_types::User,

            charted_core::api::ErrorCode,
            charted_core::api::Error,
            charted_core::Distribution,
            charted_core::BuildInfo,

            charted_types::NameOrUlid,
            charted_types::name::Name,
            charted_types::VersionReq,
            charted_types::Version,

            charted_types::payloads::UserLoginPayload,
            charted_types::QueryableVersion,

            charted_feature::Metadata,
            charted_feature::Deprecation,

            crate::routing::v1::main::Main,
            crate::routing::v1::Entrypoint,
            crate::pagination::Ordering,
            crate::pagination::PaginationRequest
        ),
        responses(
            ApiErrorResponse,
            ApiKeyResponse,
            EmptyApiResponse,
            OrganizationResponse,
            RepositoryResponse,
            UserResponse,
            ListApiKeyResponse,
            ListOrganizationResponse,
            ListRepositoryResponse,
            UrlResponse,
            SessionResponse,

            crate::routing::v1::main::MainResponse,
            crate::routing::v1::indexes::ChartIndexResponse,
            crate::routing::v1::EntrypointResponse,
        )
    ),
    paths(
        crate::routing::v1::repository::releases::get_single_release_provenance,
        crate::routing::v1::repository::releases::get_single_release_tarball,
        crate::routing::v1::repository::releases::get_single_release,
        crate::routing::v1::repository::releases::fetch_releases,
        crate::routing::v1::repository::fetch,
        crate::routing::v1::repository::main,

        crate::routing::v1::user::sessions::login,
        crate::routing::v1::user::sessions::logout,
        crate::routing::v1::user::sessions::fetch,
        crate::routing::v1::user::sessions::refresh_session,

        crate::routing::v1::user::avatars::get_self_user_avatar_by_hash,
        crate::routing::v1::user::avatars::get_user_avatar_by_hash,
        crate::routing::v1::user::avatars::get_self_user_avatar,
        crate::routing::v1::user::avatars::upload_user_avatar,
        crate::routing::v1::user::avatars::get_user_avatar,

        crate::routing::v1::user::apikeys::delete,
        crate::routing::v1::user::apikeys::create,
        crate::routing::v1::user::apikeys::patch,
        crate::routing::v1::user::apikeys::fetch,
        crate::routing::v1::user::apikeys::list,

        crate::routing::v1::user::repositories::list_self_user_repositories,
        crate::routing::v1::user::repositories::list_user_repositories,
        crate::routing::v1::user::repositories::create_user_repository,

        crate::routing::v1::user::get_self,
        crate::routing::v1::user::create,
        crate::routing::v1::user::delete,
        crate::routing::v1::user::patch,
        crate::routing::v1::user::fetch,
        crate::routing::v1::user::main,

        crate::routing::v1::healthz::healthz,
        crate::routing::v1::indexes::fetch,
        crate::routing::v1::main::main,
    ),
    tags(
        (
            name = "Main"
        ),
        (
            name = "Users",
            description = "Endpoints that create, modify, delete, or fetch user metadata"
        ),
        (
            name = "Users/Avatars",
            description = "Endpoints that can create, modify, delete, and fetch user avatars"
        ),
        (
            name = "Users/Sessions",
            description = "Endpoints that allow to login as a user and get an access token."
        ),
        (
            name = "API Keys",
            description = "Endpoints that allow authenticating users with a secret key that is trusted by the server."
        ),
        (
            name = "Repositories",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository metadata"
        ),
        (
            name = "Repository/Releases",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository releases"
        ),
        (
            name = "Repository/Members",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository members"
        ),
        (
            name = "Organizations",
            description = "Endpoints that create, modify, delete, or fetch organization metadata"
        ),
        (
            name = "Organization/Members",
            description = "Endpoints that create, modify, delete, or fetch organization members"
        ),
    ),
    servers(
        (
            url = "https://charts.noelware.org/api/v{version}",
            description = "Production Server",
            variables(
                ("version" = (
                    default = "1",
                    description = "Revision of the HTTP specification",
                    enum_values("1")
                ))
            )
        )
    ),
    external_docs(url = "https://charts.noelware.org/docs/server/latest")
)]
pub struct Document;

impl Document {
    /// Returns a prettified JSON document of the OpenAPI document.
    pub fn to_json_pretty() -> serde_json::Result<String> {
        serde_json::to_string_pretty(&Document::openapi())
    }
}

impl Modify for Document {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = Into::<ComponentsBuilder>::into(unsafe { openapi.components.take().unwrap_unchecked() })
            .security_scheme(
                "ApiKey",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description("ApiKey", "A generated token by the server granted to a user"))),
            )
            .security_scheme(
                "Bearer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Basic)
                        .description(Some("A signed JWT that the server created for a session via the `PUT /v1/users/login` REST endpoint"))
                        .build(),
                ),
            )
            .security_scheme("Basic", SecurityScheme::Http(HttpBuilder::new().description(Some("Follows the **Basic** authentication scheme that most HTTP servers support, based off [RFC7617](https://www.rfc-editor.org/rfc/rfc7617)")).build()));

        openapi.components = Some(components.build());
    }
}

#[cfg(test)]
mod tests;
