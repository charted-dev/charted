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

use charted_types::{ApiKey, Organization, Repository, User};
use serde_json::Value;
use utoipa::{
    PartialSchema, ToResponse, ToSchema,
    openapi::{
        ArrayBuilder, ContentBuilder, ObjectBuilder, Ref, RefOr, Response, ResponseBuilder, Schema, Type,
        schema::SchemaType,
    },
};

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

macro_rules! gen_api_response_types {
    ($($Ty:ty)+) => {$(
        $crate::__macro_support::paste! {
            #[doc = concat!("Response datatype for object `", stringify!($Ty), "`.")]
            pub struct [<$Ty Response>];

            impl<'r> $crate::__macro_support::utoipa::ToResponse<'r> for [<$Ty Response>] {
                fn response() -> (
                    &'r str,
                    $crate::__macro_support::utoipa::openapi::RefOr<
                        $crate::__macro_support::utoipa::openapi::Response
                    >
                ) {
                    use $crate::__macro_support::utoipa::{
                        PartialSchema,
                        openapi::{
                            RefOr,
                            Schema,
                            Response,
                            Content,
                            Object,
                        }
                    };

                    const __NAME: &'static str = concat!(
                        stringify!($Ty),
                        "Response"
                    );

                    let RefOr::T(Schema::Object(response)) = <
                        ::charted_core::api::Response<()> as PartialSchema
                    >::schema() else {
                        unreachable!();
                    };

                    // Safety: the derive macro for `api::Response` will always have
                    // a `success` field.
                    let success = unsafe { response.properties.get("sucesss").unwrap_unchecked() };
                    let errors = unsafe { response.properties.get("errors").unwrap_unchecked() };

                    (
                        __NAME,
                        Response::builder()
                            .description(concat!(
                                "Response datatype for object `",
                                concat!(
                                    stringify!($Ty),
                                    "Response"
                                ),
                                "`."
                            ))
                            .content(
                                "application/json",
                                Content::builder()
                                    .schema(Some(RefOr::T(Schema::Object({
                                        let __object = Object::builder()
                                            .property("success", success.to_owned())
                                            .required("success")
                                            .property("data", RefOr::Ref(Ref::from_schema_name(<$Ty as ToSchema>::name())))
                                            .property("errors", errors.to_owned());

                                        __object.build()
                                    }))))
                                    .build()
                            )
                            .build()
                            .into()
                    )
                }
            }
        }
    )+};
}

gen_api_response_types! {
    Organization
    Repository
    ApiKey
    User
}
