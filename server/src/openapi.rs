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

use charted_common::{
    models::{entities::*, helm::*, Distribution, Name, NameOrSnowflake},
    ID,
};
use utoipa::{
    openapi::{ContentBuilder, ObjectBuilder, OpenApiBuilder, RefOr, Response, ResponseBuilder, Schema, SchemaType},
    OpenApi, ToResponse,
};

#[derive(OpenApi)]
#[openapi(components(schemas(
    // Entities
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

    // Responses
    crate::routing::v1::features::FeaturesResponse,
    crate::routing::v1::main::MainResponse,
    crate::routing::v1::info::InfoResponse,

    // Payloads
    charted_database::controllers::users::CreateUserPayload
), responses(
    crate::routing::v1::features::FeaturesResponse,
    crate::routing::v1::main::MainResponse,
    crate::routing::v1::info::InfoResponse,
    EmptyApiResponse
)))]
pub(crate) struct ChartedOpenAPI;

macro_rules! add_paths {
    ($($path:literal: $fn:expr;)*) => {{
        ::utoipa::openapi::PathsBuilder::new()$(.path($path, $fn))*.build()
    }};
}

macro_rules! gen_response_schema_priv {
    ($ty:ty) => {
        $crate::openapi::gen_response_schema!($ty, content: "application/json");
    };

    ($ty:ty, content: $content:literal) => {
        impl<'r> ::utoipa::ToResponse<'r> for $ty {
            fn response() -> (&'r str, ::utoipa::openapi::RefOr<::utoipa::openapi::Response>) {
                (
                    concat!("Api", stringify!($ty)),
                    ::utoipa::openapi::RefOr::T(
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(concat!("Response object for ", stringify!($ty), ".").to_string())
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

impl<'r> ToResponse<'r> for EmptyApiResponse {
    fn response() -> (&'r str, RefOr<Response>) {
        (
            "EmptyApiResponse",
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

pub fn document() -> utoipa::openapi::OpenApi {
    let mut openapi = charted_openapi::openapi();
    let charted = ChartedOpenAPI::openapi();
    openapi.merge(charted);

    // now, let's merge our paths
    openapi.merge(
        OpenApiBuilder::new()
            .paths(add_paths! {
                "/features": crate::routing::v1::features::paths();
                "/info": crate::routing::v1::info::paths();
                "/": crate::routing::v1::main::paths();
            })
            .build(),
    );

    openapi
}
