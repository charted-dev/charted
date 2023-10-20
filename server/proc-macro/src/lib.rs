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

use charted_proc_macros::{
    error,
    helpers::{self, StringHelper},
};
use heck::ToPascalCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Expr, Ident, ItemFn, Lit, ReturnType};
use utoipa::openapi::PathItemType;

mod controller;
mod test;

/// Simple macro to return a reference to a [`Schema`][utoipa::openapi::Schema].
#[proc_macro]
pub fn schema(body: TokenStream) -> TokenStream {
    let args = parse_macro_input!(body as Expr);
    quote! {
        ::utoipa::openapi::RefOr::Ref(::utoipa::openapi::schema::Ref::T(#args))
    }
    .into()
}

/// Simple macro to return a [`Ref`][utoipa::openapi::RefOr::Ref] to a schema.
#[proc_macro]
pub fn ref_schema(body: TokenStream) -> TokenStream {
    let args = parse_macro_input!(body as Lit);
    quote! {
        ::utoipa::openapi::RefOr::Ref(::utoipa::openapi::schema::Ref::from_schema_name(#args))
    }
    .into()
}

/// Simple macro to return a [`Ref`][utoipa::openapi::RefOr::Ref] to a response.
#[proc_macro]
pub fn response(body: TokenStream) -> TokenStream {
    let args = parse_macro_input!(body as Lit);
    quote! {
        ::utoipa::openapi::RefOr::Ref(::utoipa::openapi::schema::Ref::from_response_name(#args))
    }
    .into()
}

/// Proc-macro to register a charted-server REST endpoint. This was made in attempt to
/// build out a hieroitical layer between routes.
///
/// ## How does it work?
/// It works by grouping APIs together into a single module with a function that points to
/// a path and a method. This is used by the `#[handler]` attribute macro:
///
/// ```no_run
/// # use charted_proc_macros::{response, handler};
/// # use charted_openapi::security::*;
/// #
/// #[controller(
///     id = "features",
///     description = "REST endpoint to retrieve this instance's features.",
///     responses(200, {
///         description("Successful response.");
///         contentType("application/json", response!("ApiFeaturesResponse"));
///     })
/// )]
/// pub async fn features() -> impl IntoResponse {}
/// ```
///
/// This will generate a [Handler][axum::handler::Handler] with the arguments that
/// implement [`FromRequestParts`][axum::extract::FromRequestParts] or [`FromRequest`][axum::extract::FromRequest]. Even
/// though that implementing our own handlers, the arguments and return type are validated to check
/// if it is a valid element.
///
/// The struct name will always be the ID with `RestController` at the end, i.e, `FeaturesRestController`.
#[proc_macro_attribute]
pub fn controller(attr: TokenStream, body: TokenStream) -> TokenStream {
    let func = parse_macro_input!(body as ItemFn);
    if func.sig.unsafety.is_some() {
        return error(func.sig.span(), "`unsafe` is not allowed in `#[controller]` functions").into();
    }

    if func.sig.constness.is_some() {
        return error(func.sig.span(), "`const` is not allowed in `#[controller]` functions").into();
    }

    if func.sig.abi.is_some() {
        return error(
            func.sig.span(),
            "ABI signatures is not allowed in `#[controller]` functions",
        )
        .into();
    }

    if func.sig.asyncness.is_none() {
        return error(func.sig.span(), "`async` is required in `#[controller]` functions").into();
    }

    let mut args = parse_macro_input!(attr as controller::Args);
    if args.description.is_none() {
        args.description = Some(match helpers::extract_doc_comments(func.attrs.clone().iter()) {
            Ok(comments) => comments.join("\n"),
            Err(e) => return e.to_compile_error().into(),
        });
    }

    if args.id.is_empty() {
        args.id = func.sig.ident.to_string();
    }

    let return_type = match func.sig.output.clone() {
        ReturnType::Default => quote!(impl ::axum::response::IntoResponse),
        ReturnType::Type(_, ty) => quote!(#ty),
    };

    let body = func.block.clone();
    let tags: helpers::VecHelper<String> = args.tags.into();
    let item_type: helpers::PathItemType = args.item_type.unwrap_or(PathItemType::Get).into();
    let responses: helpers::collections::BTreeMap<u16, helpers::Response> = args.responses.into();
    let parameters: helpers::collections::BTreeMap<StringHelper, helpers::Parameter> = {
        let params = args.parameters.clone();
        let mut h = std::collections::BTreeMap::<StringHelper, helpers::Parameter>::new();

        for (key, value) in params.iter() {
            h.insert(helpers::StringHelper::from(key.clone()), value.clone());
        }

        helpers::collections::BTreeMap(h)
    };

    let is_deprecated_tt = match args.is_deprecated.clone().flatten() {
        Some(_) => quote!(Some(::utoipa::openapi::Deprecated::True)),
        None => quote!(None),
    };

    let request_body = match args.request_body.clone() {
        Some(req) => quote!(Some(#req)),
        None => quote!(None),
    };

    let ident_name = format!("{}RestController", func.sig.ident.clone()).to_pascal_case();
    let struct_name = Ident::new(&ident_name, func.sig.ident.span());
    let desc_doc_comments = match args.description.clone() {
        Some(desc) => desc.split('\n').map(|f| quote!(#[doc = #f])).collect::<Vec<_>>(),
        None => vec![],
    };

    let desc = args.description.map(|f| f.trim().to_string());
    let deprecated_attr = match args.is_deprecated.as_ref() {
        Some(Some(desc)) => quote!(#[deprecated = #desc]),
        Some(None) => quote!(#[deprecated]),
        None => quote!(),
    };

    let id = args.id.clone();
    let args = func.sig.inputs.clone();
    quote! {
        #(#desc_doc_comments)*
        #deprecated_attr
        pub struct #struct_name;

        impl ::std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(#ident_name).finish()
            }
        }

        impl ::core::marker::Copy for #struct_name {}
        impl ::core::clone::Clone for #struct_name {
            fn clone(&self) -> #struct_name {
                *self
            }
        }

        impl ::core::default::Default for #struct_name {
            fn default() -> #struct_name {
                #struct_name
            }
        }

        impl #struct_name {
            #[doc = " Generates a new [PathItem][utoipa::openapi::path::PathItem] for this rest controller."]
            pub fn paths() -> ::utoipa::openapi::path::PathItem {
                let mut builder = ::utoipa::openapi::path::PathItemBuilder::new();
                let responses: ::std::collections::BTreeMap<u16, ::utoipa::openapi::Response> = { #responses };
                let parameters: ::std::collections::BTreeMap<::std::string::String, ::utoipa::openapi::path::Parameter> = { #parameters };
                let request_body: ::core::option::Option<::utoipa::openapi::request_body::RequestBody> = #request_body;

                let mut op = ::utoipa::openapi::path::OperationBuilder::new()
                    .description(Some(#desc))
                    .operation_id(Some(#id))
                    .tags(Some({ #tags }))
                    .deprecated(#is_deprecated_tt);

                for (status, resp) in responses.clone().into_iter() {
                    let status_str = status.to_string();
                    op = op.response(status_str, resp);
                }

                for parameter in parameters.clone().values() {
                    op = op.parameter(parameter.clone());
                }

                builder.operation(#item_type, op.build()).build()
            }

            #[doc = " Runs this REST controller"]
            pub async fn run(#args) -> #return_type #body
        }
    }.into()
}
