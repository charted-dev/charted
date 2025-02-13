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

mod modifiers;

use modifiers::*;
use serde_json::Value;
use utoipa::{
    openapi::{
        schema::SchemaType, ArrayBuilder, ContentBuilder, ObjectBuilder, Ref, RefOr, Response, ResponseBuilder, Schema,
        Type,
    },
    OpenApi, PartialSchema, ToResponse, ToSchema,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "charted-server",
        description = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
        version = charted_core::VERSION,
        terms_of_service = "https://charts.noelware.org/legal/tos",
        license(
            identifier = "Apache-2.0",
            name = "Apache 2.0 License",
            url = "https://apache.org/licenses/LICENSE-2.0"
        ),
        contact(
            name = "Noelware, LLC.",
            email = "team@noelware.org",
            url = "https://noelware.org"
        )
    ),
    modifiers(
        &UpdatePathsToIncludeDefaultVersion,
        &IncludeErrorProneDatatypes,
        &SecuritySchemes,
        &ResponseModifiers
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
            charted_types::name::Name,
            charted_types::VersionReq,
            charted_types::Version
        ),
        responses(

        )
    ),
    tags(
        (
            name = "Main",
            description = "Represents all the main routes that don't tie to any entity"
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

/// Represents a generic empty API response, please do not use this in actual code,
/// it is only meant for utoipa for OpenAPI code generation.
pub struct EmptyApiResponse;

impl PartialSchema for EmptyApiResponse {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .property(
                "success",
                RefOr::T(Schema::Object({
                    ObjectBuilder::new()
                        .schema_type(SchemaType::Type(Type::Boolean))
                        .description(Some("Whether if this request was a success"))
                        .build()
                })),
            )
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for EmptyApiResponse {}

impl<'r> ToResponse<'r> for EmptyApiResponse {
    fn response() -> (&'r str, RefOr<Response>) {
        let response = ResponseBuilder::new()
            .description("API response that doesn't contain any data")
            .content(
                "application/json",
                ContentBuilder::new().schema(Some(EmptyApiResponse::schema())).build(),
            )
            .build();

        ("EmptyApiResponse", RefOr::T(response))
    }
}

/// Represents a generic API error response object. Please do not use this in actual code,
/// it is only meant for OpenAPI code generation.
pub struct ApiErrorResponse;

impl PartialSchema for ApiErrorResponse {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .property(
                "success",
                RefOr::T(Schema::Object({
                    ObjectBuilder::new()
                        .schema_type(SchemaType::Type(Type::Boolean))
                        .description(Some("Whether if this request was a success or not (always false)"))
                        .default(Some(Value::Bool(false)))
                        .build()
                })),
            )
            .property(
                "errors",
                RefOr::T(Schema::Array({
                    ArrayBuilder::new()
                        .description(Some(
                            "List of errors that happened. This can be represented as a stacktrace",
                        ))
                        .items(RefOr::Ref(Ref::from_schema_name("Error")))
                        .build()
                })),
            )
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for ApiErrorResponse {}

impl<'r> ToResponse<'r> for ApiErrorResponse {
    fn response() -> (&'r str, RefOr<Response>) {
        let response = ResponseBuilder::new()
            .description("API response that is returned during a error path")
            .content(
                "application/json",
                ContentBuilder::new().schema(Some(ApiErrorResponse::schema())).build(),
            )
            .build();

        ("ApiErrorResponse", RefOr::T(response))
    }
}

#[cfg(test)]
mod tests {
    /// A sanity check for all tests if all the references that are
    /// used are all correct and avaliable.
    #[test]
    fn sanity_check_if_all_references_are_correct() {}
}
