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

mod modifiers;
use modifiers::*;

use serde_json::Value;
use std::borrow::Cow;
use utoipa::{
    openapi::{
        schema::SchemaType, ArrayBuilder, ContentBuilder, ObjectBuilder, Ref, RefOr, Response, ResponseBuilder, Schema,
        Type,
    },
    OpenApi, PartialSchema, ToResponse, ToSchema,
};

#[derive(OpenApi)]
#[openapi(
    modifiers(
        &UpdatePathsToIncludeDefaultVersion,
        &IncludeErrorProneDatatypes,
        &SecuritySchemes,
        &ResponseModifiers,
    ),
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
    components(
        schemas(
            // ==== Request Bodies ====
            charted_types::payloads::repository::release::CreateRepositoryReleasePayload,
            charted_types::payloads::repository::release::PatchRepositoryReleasePayload,
            charted_types::payloads::organization::CreateOrganizationPayload,
            charted_types::payloads::organization::PatchOrganizationPayload,
            charted_types::payloads::repository::CreateRepositoryPayload,
            charted_types::payloads::repository::PatchRepositoryPayload,
            charted_types::payloads::apikey::CreateApiKeyPayload,
            charted_types::payloads::apikey::PatchApiKeyPayload,
            charted_types::payloads::user::CreateUserPayload,
            charted_types::payloads::user::PatchUserPayload,

            // ==== Response Datatypes ====
            crate::routing::v1::info::Info,
            crate::routing::v1::main::Main,
            crate::routing::v1::Entrypoint,

            // ==== Helm ====
            charted_types::helm::StringOrImportValue,
            charted_types::helm::ChartSpecVersion,
            charted_types::helm::ChartMaintainer,
            charted_types::helm::ChartDependency,
            charted_types::helm::ChartIndexSpec,
            charted_types::helm::ImportValue,
            charted_types::helm::ChartIndex,
            charted_types::helm::ChartType,
            charted_types::helm::Chart,

            // ==== Entities ====
            charted_types::RepositoryRelease,
            charted_types::RepositoryMember,
            charted_types::Repository,

            charted_types::OrganizationMember,
            charted_types::Organization,

            charted_types::UserConnections,
            charted_types::Session,
            charted_types::ApiKey,
            charted_types::User,

            // // ==== API Entities ====
            charted_core::api::ErrorCode,
            charted_core::api::Error,

            // ==== Generic ====
            //charted_core::serde::Duration,
            charted_core::Distribution,
            charted_types::name::Name,
            charted_types::VersionReq,
            crate::types::NameOrUlid,
            // charted_types::DateTime,
            charted_types::Version,
            charted_types::Ulid
        ),
        responses(
            EmptyApiResponse,
            ApiErrorResponse
        )
    ),
    paths(
        crate::routing::v1::user::create_user,
        crate::routing::v1::user::main,

        crate::routing::v1::heartbeat::heartbeat,
        crate::routing::v1::index::get_chart_index,
        crate::routing::v1::info::info,
        crate::routing::v1::main::main,
    ),
    servers(
        (
            url = "https://charts.noelware.org/api/v{version}",
            description = "Official, Production Service by Noelware, LLC.",
            variables(
                ("version" = (
                    default = "1",
                    description = "API revision of the charted HTTP specification",
                    enum_values("1")
                ))
            )
        )
    ),
    external_docs(
        url = "https://charts.noelware.org/docs/server/latest",
        description = "charted-server :: Documentation"
    )
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

impl ToSchema for EmptyApiResponse {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("EmptyApiResponse")
    }
}

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

impl ToSchema for ApiErrorResponse {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("ApiErrorResponse")
    }
}

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
