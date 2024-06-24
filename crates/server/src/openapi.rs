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

use azalia::lazy;
use charted_common::VERSION;
use once_cell::sync::Lazy;
use utoipa::{
    openapi::{
        external_docs::ExternalDocsBuilder,
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
        Components, ComponentsBuilder, Contact, ContactBuilder, ExternalDocs, InfoBuilder, License, LicenseBuilder,
        OpenApi, OpenApiBuilder, RefOr, Schema,
    },
    ToSchema,
};

static SCHEMAS: Lazy<Vec<(&str, RefOr<Schema>)>> = lazy!([
    /* REQUEST BODY */
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
    /* OTHER */
    // pagination
    crate::pagination::PaginatedRepositoryRelease::schema(),
    crate::pagination::PaginatedOrganization::schema(),
    crate::pagination::PaginatedRepository::schema(),
    crate::pagination::PaginatedApiKey::schema(),
    crate::pagination::PaginatedMember::schema(),
    // helm-specific
    charted_entities::helm::StringOrImportValue::schema(),
    charted_entities::helm::ChartSpecVersion::schema(),
    charted_entities::helm::ChartIndexSpec::schema(),
    charted_entities::helm::ImportValue::schema(),
    charted_entities::helm::ChartType::schema(),
    charted_entities::helm::Chart::schema(),
    // entities
]
.into_iter()
.collect());

pub fn components() -> Components {
    ComponentsBuilder::new()
        .schemas_from_iter(Lazy::force(&SCHEMAS).clone())
        .build()
}

/*
static COMPONENTS: Lazy<Components> = lazy!(ComponentsBuilder::new()
    .schemas_from_iter([
        // response schemas
        crate::server::routing::v1::features::FeaturesResponse::schema(),
        crate::server::routing::v1::EntrypointResponse::schema(),
        crate::server::routing::v1::info::InfoResponse::schema(),
        crate::server::routing::v1::main::MainResponse::schema(),
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
        charted_server::pagination::RepositoryPaginatedResponse::response(),
        crate::server::routing::v1::indexes::ChartIndexResponse::response(),
        crate::server::routing::v1::features::FeaturesResponse::response(),
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
*/

/// Returns a slice of all the [`SecurityScheme`]s that the API server supports.
pub fn schemes() -> [(&'static str, SecurityScheme); 3] {
    [
        (
            "ApiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("ApiKey"))),
        ),
        (
            "Bearer",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .description(Some(
                        "Signed JWT token created by the server for safe authentication between mutual trust",
                    ))
                    .build(),
            ),
        ),
        ("Basic", SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Basic)
                .description(Some(
                    "Allows to authenticate via a basic `username:password` scheme. This is not enabled on most instances and is very unsecure."
                ))
                .build()
            )
        ),
    ]
}

fn license() -> License {
    LicenseBuilder::new()
        .name("Apache-2.0")
        .url(Some("https://apache.org/licenses/LICENSE-2.0"))
        .build()
}

fn contact() -> Contact {
    ContactBuilder::new()
        .email(Some("team@noelware.org"))
        .name(Some("Noelware, LLC."))
        .url(Some("https://noelware.org"))
        .build()
}

/// Allows to reference an API object to the primary documentation site.
pub fn object_doc(object: &str) -> ExternalDocs {
    ExternalDocsBuilder::new()
        .url(format!(
            "https://charts.noelware.org/docs/server/{VERSION}/api/reference/org.noelware.charted.{object}",
        ))
        .description(Some(format!(
            "Reference to the `org.noelware.charted.{object}` API object"
        )))
        .build()
}

pub fn document() -> OpenApi {
    OpenApiBuilder::new()
        .info(
            InfoBuilder::new()
                .title("charted-server")
                .description(Some(
                    "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
                ))
                .version(VERSION)
                .terms_of_service(Some("https://charts.noelware.org/legal/tos"))
                .contact(Some(contact()))
                .license(Some(license()))
                .build(),
        )
        .external_docs(Some(
            ExternalDocsBuilder::new()
                .url(format!(
                    "https://charts.noelware.org/docs/server/{}",
                    charted_common::VERSION
                ))
                .description(Some("Main documentation source for charted-server"))
                .build(),
        ))
        .build()
}

/*
/// Returns the [`OpenApi`] object.
pub fn openapi() -> OpenApi {
    let license = LicenseBuilder::new()
        .name("Apache 2.0")
        .url(Some("https://www.apache.org/licenses/LICENSE-2.0"))
        .build();

    let contact = ContactBuilder::new()
        .name(Some("Noelware, LLC."))
        .url(Some("https://noelware.org"))
        .email(Some("team@noelware.org"))
        .build();

    let docs = ExternalDocsBuilder::new()
        .url(format!("https://charts.noelware.org/docs/server/{}", VERSION))
        .description(Some("Main documentation source for charted-server"))
        .build();

    let info = InfoBuilder::new()
        .title("charted-server")
        .description(Some(
            "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
        ))
        .version(VERSION)
        .terms_of_service(Some("https://charts.noelware.org/legal/tos"))
        .license(Some(license))
        .contact(Some(contact))
        .build();

    let mut components = ComponentsBuilder::new();
    for (name, scheme) in SCHEMES.iter() {
        components = components.security_scheme(name, scheme.clone());
    }

    OpenApiBuilder::new()
        .info(info)
        .external_docs(Some(docs))
        .components(Some(components.build()))
        .build()
}

/// Represents a generic empty API response, please do not use this in actual code,
/// it is only meant for utoipa for OpenAPI code generation.
pub struct EmptyApiResponse;
impl<'r> ToResponse<'r> for EmptyApiResponse {
    fn response() -> (&'r str, RefOr<Response>) {
        let response = ResponseBuilder::new()
            .description("API response that doesn't contain any data")
            .content(
                "application/json",
                ContentBuilder::new()
                    .schema(RefOr::T(Schema::Object({
                        let builder = ObjectBuilder::new()
                            .property(
                                "success",
                                RefOr::T(Schema::Object(
                                    ObjectBuilder::new()
                                        .schema_type(SchemaType::Boolean)
                                        .description(Some(
                                            "whether if this response [EmptyApiResponse] was a success or not",
                                        ))
                                        .build(),
                                )),
                            )
                            .required("success");

                        builder.build()
                    })))
                    .build(),
            )
            .build();

        ("EmptyApiResponse", RefOr::T(response))
    }
}

/// Represents a generic API error response object. Please do not use this in actual code,
/// it is only meant for OpenAPI code generation.
pub struct ApiErrorResponse;
impl<'r> ToResponse<'r> for ApiErrorResponse {
    fn response() -> (&'r str, RefOr<Response>) {
        let response = ResponseBuilder::new()
            .description("API response that doesn't contain any data")
            .content(
                "application/json",
                ContentBuilder::new()
                    .schema(RefOr::T(Schema::Object({
                        let builder = ObjectBuilder::new()
                            .property(
                                "success",
                                RefOr::T(Schema::Object(
                                    ObjectBuilder::new()
                                        .schema_type(SchemaType::Boolean)
                                        .description(Some(
                                            "whether if this response [ApiErrorResponse] was a success or not",
                                        ))
                                        .build(),
                                )),
                            )
                            .required("success")
                            .property(
                                "errors",
                                RefOr::T(Schema::Array(
                                    ArrayBuilder::new()
                                        .description(Some("List of errors on why the request failed."))
                                        .items(RefOr::Ref(Ref::from_schema_name("Error")))
                                        .build(),
                                )),
                            )
                            .required("errors");

                        builder.build()
                    })))
                    .build(),
            )
            .build();

        ("ApiErrorResponse", RefOr::T(response))
    }
}

*/
