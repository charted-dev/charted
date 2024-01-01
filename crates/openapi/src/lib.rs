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

pub use charted_openapi_proc_macro as macros;
pub use charted_openapi_proc_macro::add_paths;

use charted_common::VERSION;
use charted_config::Config;
use once_cell::sync::Lazy;
use utoipa::{
    openapi::{
        external_docs::ExternalDocsBuilder,
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
        ArrayBuilder, ComponentsBuilder, ContactBuilder, ContentBuilder, InfoBuilder, LicenseBuilder, ObjectBuilder,
        OpenApi, OpenApiBuilder, Ref, RefOr, Response, ResponseBuilder, Schema, SchemaType, ServerBuilder,
    },
    ToResponse,
};

pub static API_KEY_SCHEME: Lazy<SecurityScheme> =
    Lazy::new(|| SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("ApiKey"))));

pub static BEARER_SCHEME: Lazy<SecurityScheme> = Lazy::new(|| {
    let http = HttpBuilder::new()
        .scheme(HttpAuthScheme::Bearer)
        .bearer_format("Bearer")
        .description(Some(
            "JWT-signed session key that is used to safely identify someone for 2 days with a refresh token",
        ))
        .build();

    SecurityScheme::Http(http)
});

pub static BASIC_SCHEME: Lazy<SecurityScheme> = Lazy::new(|| {
    let http = HttpBuilder::new()
        .scheme(HttpAuthScheme::Basic)
        .description(Some(
            "Basic is only meant for testing the API, do not use this for anything else.",
        ))
        .build();

    SecurityScheme::Http(http)
});

/// Creates a new [`OpenAPI`] object.
pub fn openapi() -> OpenApi {
    let config = Config::get();
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
        .url(format!("https://charts.noelware.org/docs/server/{VERSION}"))
        .description(Some("Main documentation for charted-server"))
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

    let api_key = &*API_KEY_SCHEME;
    let bearer_scheme = &*BEARER_SCHEME;
    let basic_scheme = &*BASIC_SCHEME;
    let components = ComponentsBuilder::new()
        .security_scheme("ApiKey", api_key.clone())
        .security_scheme("Bearer", bearer_scheme.clone())
        .security_scheme("Basic", basic_scheme.clone())
        .build();

    OpenApiBuilder::new()
        .info(info)
        .external_docs(Some(docs))
        .servers(Some([ServerBuilder::new()
            .url(format!("http://{}", config.server.addr()))
            .build()]))
        .components(Some(components))
        .build()
}

// /// Re-usuable functional macro to create a [Paths][utoipa::openapi::Paths] object easily.
// ///
// /// ## Example
// /// ```no_run
// /// # use charted_openapi::add_paths;
// /// #
// /// add_paths! {
// ///     "/" => index();
// /// }
// ///
// /// fn index() -> utoipa::openapi::Paths {
// ///     // ....
// ///     # ::utoipa::openapi::PathsBuilder::new().build()
// /// }
// /// ```
// #[macro_export]
// macro_rules! add_paths {
//     ($($path:expr => $fn:expr;)*) => {{
//         ::utoipa::openapi::PathsBuilder::new()$(.path($path, $fn))*.build()
//     }};
// }

/*
    let mut paths = PathItemBuilder::new();
    let operations = vec![
        MainRestController::paths().operations.pop_first().unwrap(),
        CreateUserRestController::paths().operations.pop_first().unwrap(),
        PatchUserRestController::paths().operations.pop_first().unwrap(),
    ];

    for (item, op) in operations.iter() {
        paths = paths.operation(item.clone(), op.clone());
    }

    paths.build()
*/

pub use charted_openapi_proc_macro::generate_response_schema;

// #[macro_export]
// macro_rules! generate_response_schema {
//     ($ty:ty, content: $content:literal, schema: $schema:expr) => {
//         impl<'r> ::utoipa::ToResponse<'r> for $ty {
//             fn response() -> (
//                 &'r str,
//                 ::utoipa::openapi::RefOr<::utoipa::openapi::Response>
//             ) {
//                 let __response = ::utoipa::openapi::ResponseBuilder::new()
//                     .description(concat!("Response object for ", stringify!(schema)))
//                     .content(
//                         "application/json",
//                         ::utoipa::openapi::ContentBuilder::new()
//                             .schema(
//                                 ::utoipa::openapi::RefOr::T(
//                                     ::utoipa::openapi::Schema::Object({
//                                         let __obj = ::utoipa::openapi::ObjectBuilder::new()
//                                             .property(
//                                                 "success",
//                                                 ::utoipa::openapi::ObjectBuilder::new()
//                                                     .schema_type(::utoipa::openapi::SchemaType::Boolean)
//                                                     .description(Some(concat!("whether if this response [", concat!("Api", stringify!($schema)), "] was successful or not")))
//                                                     .build()
//                                             )
//                                             .required("success")
//                                             .property(
//                                                 "data",
//                                                 ::utoipa::openapi::Ref::from_schema_name(stringify!($schema))
//                                             )
//                                             .required("data")
//                                             .build();

//                                         __obj
//                                     })
//                                 )
//                             )
//                             .build()
//                     )
//                     .build();

//                 (
//                     concat!("Api", stringify!($schema)),
//                     ::utoipa::openapi::RefOr::T(__response)
//                 )
//             }
//         }
//     };

//     ($ty:ty, content: $content:literal) => {
//         $crate::generate_response_schema!($ty, content: $content, schema: stringify!($ty));
//     };

//     ($ty:ty, schema: $schema:expr) => {
//         $crate::generate_response_schema!($ty, content: "application/json", schema: $schema);
//     };

//     ($ty:ty) => {
//         $crate::generate_response_schema!($ty, content: "application/json", schema: stringify!($ty));
//     }
// }

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
