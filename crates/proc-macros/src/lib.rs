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

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Token, Type, TypePath,
};

/// Generates a [`ToResponse`][utoipa::ToResponse] trait from a schema that should
/// represent the [API response schema].
///
/// ## Example
/// ```ignore
/// // Generate based off the name
/// generate_api_response!(MyResponse); // => ToResponse for `api::Response<MyResponse>`
///
/// // Generate based off a schema
/// generate_api_response!(MyResponse for User); // => ToResponse for `api::Response<User>`
/// ```
///
/// [API response schema]: https://charts.noelware.org/docs/server/latest/api#responses
#[proc_macro]
pub fn generate_api_response(tt: TokenStream) -> TokenStream {
    struct GenerateApiResponse {
        ident: Ident,
        schema: Option<TypePath>,
    }

    impl Parse for GenerateApiResponse {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(GenerateApiResponse {
                ident: input.parse()?,
                schema: if input.is_empty() {
                    None
                } else {
                    input.parse::<Token![for]>()?;

                    // We only want Type::Path variants (i.e, `ChartIndex`, `::weow::fluff`)
                    let ty = input.parse::<Type>()?;
                    match ty {
                        Type::Path(path) => Some(path),
                        _ => None,
                    }
                },
            })
        }
    }

    let GenerateApiResponse { ident, schema } = parse_macro_input!(tt as GenerateApiResponse);
    let schema = schema.unwrap_or(TypePath {
        qself: None,
        path: ident.clone().into(),
    });

    quote! {
        #[automatically_derived]
        impl<'r> ::utoipa::ToResponse<'r> for #ident {
            fn response() -> (&'r str, ::utoipa::openapi::RefOr<::utoipa::openapi::Response>) {
                let __datatype_schema = <#schema as ::utoipa::ToSchema>::name();
                let __schema = ::utoipa::openapi::Schema::Object({
                    let __object = ::utoipa::openapi::ObjectBuilder::new()
                        .property(
                            "success",
                            ::utoipa::openapi::ObjectBuilder::new()
                                .description(Some("whether if this request was successful"))
                                .schema_type(
                                    ::utoipa::openapi::schema::SchemaType::Type(
                                        ::utoipa::openapi::schema::Type::Boolean
                                    )
                                )
                                .build()
                        )
                        .required("success")
                        .property(
                            "data",
                            ::utoipa::openapi::RefOr::Ref(
                                ::utoipa::openapi::Ref::from_schema_name(__datatype_schema)
                            )
                        )
                        .required("data")
                        .build();

                    __object
                });

                let __response = ::utoipa::openapi::ResponseBuilder::new()
                    .content(
                        "application/json",
                        ::utoipa::openapi::ContentBuilder::new()
                            .schema(::core::option::Option::Some(__schema))
                            .build()
                    )
                    .build();

                let __name = stringify!(#ident);
                (__name, ::utoipa::openapi::RefOr::T(__response))
            }
        }
    }
    .into()
}
