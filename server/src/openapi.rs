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

use crate::models::res::Error;
use charted_common::{
    models::{entities::*, helm::*, Distribution, Name, NameOrSnowflake},
    ID,
};
use utoipa::{
    openapi::{
        ArrayBuilder, ContentBuilder, ObjectBuilder, OpenApiBuilder, Ref, RefOr, Response, ResponseBuilder, Schema,
        SchemaType,
    },
    OpenApi, ToResponse,
};

#[derive(OpenApi)]
#[openapi(components(
    schemas(
        Error,
        NameOrSnowflake,
        Name,
        ID,
        Distribution,
        ChartSpecVersion,
        ChartType,
        ImportValue,
        StringOrImportValue,
        ChartDependency,
        ChartMaintainer,
        Chart,
        ChartIndexSpec,
        User,
        Repository,
        Organization,
        ApiKey,
        Member,
        RepositoryRelease,
        charted_sessions::Session,
        crate::routing::v1::features::FeaturesResponse,
        crate::routing::v1::main::MainResponse,
        crate::routing::v1::info::InfoResponse,
        crate::routing::v1::EntrypointResponse,
        crate::pagination::PaginatedOrganizationMember,
        crate::pagination::PaginatedRepositoryMember,
        crate::pagination::PaginatedOrganization,
        crate::pagination::PaginatedRepository,
        charted_common::server::pagination::PageInfo,
        charted_common::server::pagination::OrderBy
    ),
    responses(
        crate::pagination::OrganizationMemberPaginatedResponse,
        crate::pagination::RepositoryMemberPaginatedResponse,
        crate::routing::v1::users::sessions::SessionResponse,
        crate::pagination::OrganizationPaginatedResponse,
        crate::pagination::RepositoryPaginatedResponse,
        crate::routing::v1::features::FeaturesResponse,
        crate::routing::v1::indexes::ChartIndexResponse,
        crate::routing::v1::users::UserResponse,
        crate::routing::v1::main::MainResponse,
        crate::routing::v1::info::InfoResponse,
        crate::routing::v1::EntrypointResponse,
        ApiErrorResponse,
        EmptyApiResponse
    )
))]
pub(crate) struct MainOpenAPI;

macro_rules! add_paths {
    ($($path:literal: $fn:expr;)*) => {{
        ::utoipa::openapi::PathsBuilder::new()$(.path($path, $fn))*.build()
    }};
}

macro_rules! gen_response_schema_priv {
    ($ty:ty) => {
        $crate::openapi::gen_response_schema!($ty, content: "application/json");
    };

    ($ty:ty, schema: $schema:ty) => {
         impl<'r> ::utoipa::ToResponse<'r> for $ty {
            fn response() -> (&'r str, ::utoipa::openapi::RefOr<::utoipa::openapi::Response>) {
                (
                    concat!("Api", stringify!($ty)),
                    ::utoipa::openapi::RefOr::T(
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(concat!("Response object for ", stringify!($ty)).to_string())
                            .content("application/json", ::utoipa::openapi::ContentBuilder::new()
                                .schema(::utoipa::openapi::RefOr::T(::utoipa::openapi::Schema::Object({
                                    ::utoipa::openapi::ObjectBuilder::new()
                                        .property(
                                            "success",
                                            ::utoipa::openapi::ObjectBuilder::new()
                                                .schema_type(::utoipa::openapi::SchemaType::Boolean)
                                                .description(Some("Indicates whether if this response was a success or not"))
                                                .build()
                                        )
                                        .required("success")
                                        .property(
                                            "data",
                                            ::utoipa::openapi::Ref::from_schema_name(stringify!($schema))
                                        )
                                        .required("data")
                                        .build()
                                })))
                                .build()
                            )
                            .build(),
                    ),
                )
            }
        }
    };

    ($ty:ty, schema: $schema:literal) => {
         impl<'r> ::utoipa::ToResponse<'r> for $ty {
            fn response() -> (&'r str, ::utoipa::openapi::RefOr<::utoipa::openapi::Response>) {
                (
                    concat!("Api", stringify!($ty)),
                    ::utoipa::openapi::RefOr::T(
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(concat!("Response object for ", stringify!($ty)).to_string())
                            .content("application/json", ::utoipa::openapi::ContentBuilder::new()
                                .schema(::utoipa::openapi::RefOr::T(::utoipa::openapi::Schema::Object({
                                    ::utoipa::openapi::ObjectBuilder::new()
                                        .property(
                                            "success",
                                            ::utoipa::openapi::ObjectBuilder::new()
                                                .schema_type(::utoipa::openapi::SchemaType::Boolean)
                                                .description(Some("Indicates whether if this response was a success or not"))
                                                .build()
                                        )
                                        .required("success")
                                        .property(
                                            "data",
                                            ::utoipa::openapi::Ref::from_schema_name($schema)
                                        )
                                        .required("data")
                                        .build()
                                })))
                                .build()
                            )
                            .build(),
                    ),
                )
            }
        }
    };

    ($ty:ty, content: $content:literal) => {
        impl<'r> ::utoipa::ToResponse<'r> for $ty {
            fn response() -> (&'r str, ::utoipa::openapi::RefOr<::utoipa::openapi::Response>) {
                (
                    concat!("Api", stringify!($ty)),
                    ::utoipa::openapi::RefOr::T(
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(concat!("Response object for ", stringify!($ty)).to_string())
                            .content($content, ::utoipa::openapi::ContentBuilder::new()
                                .schema(::utoipa::openapi::RefOr::T(::utoipa::openapi::Schema::Object({
                                    ::utoipa::openapi::ObjectBuilder::new()
                                        .property(
                                            "success",
                                            ::utoipa::openapi::ObjectBuilder::new()
                                                .schema_type(::utoipa::openapi::SchemaType::Boolean)
                                                .description(Some("Indicates whether if this response was a success or not"))
                                                .build()
                                        )
                                        .required("success")
                                        .property(
                                            "data",
                                            ::utoipa::openapi::Ref::from_schema_name(stringify!($ty))
                                        )
                                        .required("data")
                                        .build()
                                })))
                                .build()
                            )
                            .build(),
                    ),
                )
            }
        }
    };
}

pub(crate) use gen_response_schema_priv as gen_response_schema;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct EmptyApiResponse;

// not meant to be used directly
pub(crate) struct ApiErrorResponse;

impl<'r> ToResponse<'r> for EmptyApiResponse {
    fn response() -> (&'r str, RefOr<Response>) {
        (
            "ApiEmptyResponse",
            RefOr::T(
                ResponseBuilder::new()
                    .description("Empty response object")
                    .content(
                        "application/json",
                        ContentBuilder::new()
                            .schema(RefOr::T(Schema::Object(
                                ObjectBuilder::new()
                                    .property(
                                        "success",
                                        ObjectBuilder::new()
                                            .schema_type(SchemaType::Boolean)
                                            .description(Some(
                                                "Indicates whether if this response was a success or not",
                                            ))
                                            .build(),
                                    )
                                    .required("success")
                                    .build(),
                            )))
                            .build(),
                    )
                    .build(),
            ),
        )
    }
}

impl<'r> ToResponse<'r> for ApiErrorResponse {
    fn response() -> (&'r str, utoipa::openapi::RefOr<utoipa::openapi::response::Response>) {
        (
            "ApiErrorResponse",
            RefOr::T(
                ResponseBuilder::new()
                    .description("Response of when something went wrong. Used in all non 200 status codes.")
                    .content(
                        "application/json",
                        ContentBuilder::new()
                            .schema(RefOr::T(Schema::Object(
                                ObjectBuilder::new()
                                    .description(Some("Schema definition for ApiErrorResponse"))
                                    .property(
                                        "success",
                                        RefOr::T(Schema::Object(
                                            ObjectBuilder::new()
                                                .schema_type(SchemaType::Boolean)
                                                .description(Some(
                                                    "Whether if the request succeeded. This will always be `false`",
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
                                    .required("errors")
                                    .build(),
                            )))
                            .build(),
                    )
                    .build(),
            ),
        )
    }
}

pub fn document() -> utoipa::openapi::OpenApi {
    let mut openapi = charted_openapi::openapi();
    openapi.merge(MainOpenAPI::openapi());

    // now, let's merge our paths
    openapi.merge(
        OpenApiBuilder::new()
            .paths(add_paths! {
                // repositories

                // organizations

                // api keys

                // users
                "/users/{idOrName}/repositories": crate::routing::v1::users::repositories::ListUserRepositoriesRestController::paths();
                "/users/sessions/refresh-token": crate::routing::v1::users::sessions::RefreshSessionTokenRestController::paths();
                "/users/{idOrName}/avatar": crate::routing::v1::users::avatars::GetCurrentUserAvatarRestController::paths();
                "/users/@me/repositories": crate::routing::v1::users::repositories::CreateUserRepositoryRestController::paths();
                "/users/sessions/logout": crate::routing::v1::users::sessions::LogoutRestController::paths();
                "/users/@me/avatar": crate::routing::v1::users::avatars::me::GetMyAvatarRestController::paths();
                "/users/{idOrName}": crate::routing::v1::users::GetUserRestController::paths();
                "/users/login": crate::routing::v1::users::sessions::LoginRestController::paths();
                "/users/@me": crate::routing::v1::users::GetSelfRestController::paths();
                "/users": crate::routing::v1::users::paths();

                // main
                "/indexes/{idOrName}": crate::routing::v1::indexes::GetIndexRestController::paths();
                "/heartbeat": crate::routing::v1::heartbeat::HeartbeatRestController::paths();
                "/features": crate::routing::v1::features::FeaturesRestController::paths();
                "/info": crate::routing::v1::info::InfoRestController::paths();
                "/": crate::routing::v1::main::MainRestController::paths();
            })
            .build(),
    );

    openapi
}
