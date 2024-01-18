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

pub use charted_proc_macros::generate_response_schema;

use crate::{lazy, VERSION};
use once_cell::sync::Lazy;
use utoipa::{
    openapi::{
        external_docs::ExternalDocsBuilder,
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
        ArrayBuilder, ComponentsBuilder, ContactBuilder, ContentBuilder, InfoBuilder, LicenseBuilder, ObjectBuilder,
        OpenApi, OpenApiBuilder, Ref, RefOr, Response, ResponseBuilder, Schema, SchemaType,
    },
    ToResponse,
};

/// List of [`SecuritySchemes`] that are available.
pub static SCHEMES: Lazy<Vec<(String, SecurityScheme)>> = lazy!(vec![
    (String::from("ApiKey"), SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("ApiKey")))),
    (String::from("Bearer"), SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .description(Some("Signed JWT that is made to safely be authenticated"))
            .build()
    )),
    (String::from("Basic"), SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Basic)
            .description(Some("> WARN: On some instances, this is disabled\n\nAllows the use of the HTTP Basic Auth scheme to use authenticated endpoints as a user."))
            .build()
    ))
]);

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
        .url(format!("https://charts.noelware.org/docs/server/{}", crate::VERSION))
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
