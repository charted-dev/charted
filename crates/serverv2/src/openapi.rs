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
    ListRepositoryResponse, OrganizationResponse, RepositoryResponse, Url, UrlResponse, UserResponse,
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

            charted_feature::Metadata,
            charted_feature::Deprecation,
            crate::routing::v1::main::Main,
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

            crate::routing::v1::main::MainResponse,
            crate::routing::v1::indexes::ChartIndexResponse,
        )
    ),
    paths(
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
