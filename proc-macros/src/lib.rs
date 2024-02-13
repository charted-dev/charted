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

mod helpers;
mod openapi;
mod server;

use heck::ToPascalCase;
use helpers::{error, StringHelper};
use proc_macro::TokenStream;
use quote::quote;
use server::controller;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Expr, ExprArray, ExprAssign, ExprLit, ExprMacro,
    ExprPath, Ident, ItemFn, Lit, ReturnType, Token,
};
use utoipa::openapi::PathItemType;

/// Dynamically creates a [`Paths`](utoipa::openapi::Paths) object from any amount
/// of paths.
///
/// ## Example
/// ```rust,ignore
/// # use charted_openapi::add_paths;
/// #
/// add_paths! {
///     "/" => index();
/// }
///
/// fn index() -> utoipa::openapi::Paths {
///     // ....
///     # ::utoipa::openapi::PathsBuilder::new().build()
/// }
/// ```
#[proc_macro]
pub fn add_paths(body: TokenStream) -> TokenStream {
    let args = parse_macro_input!(body as openapi::AddPathArgs);
    let mut tt = proc_macro2::TokenStream::new();

    tt.extend(quote! {
        ::utoipa::openapi::PathsBuilder::new()
    });

    for (lhs, rhs) in args.elements.iter() {
        match rhs {
            Expr::Array(ExprArray { elems, .. }) => {
                let mut new_tt = proc_macro2::TokenStream::new();
                new_tt.extend(quote! {
                    let mut __paths = ::utoipa::openapi::path::PathItemBuilder::new();
                });

                for elem in elems.iter() {
                    if let Expr::Call(_) = elem.clone() {
                        new_tt.extend(quote! {
                            {
                                let (__item, __op) = #elem.operations.pop_first().unwrap();
                                __paths = __paths.operation(__item, __op);
                            }
                        });
                    }
                }

                new_tt.extend(quote! {
                    __paths.build()
                });

                tt.extend(quote! {
                    .path(#lhs, {
                        #new_tt
                    })
                });
            }

            Expr::Call(rhs) => {
                tt.extend(quote! {
                    .path(#lhs, #rhs)
                });
            }

            _ => unreachable!(),
        }
    }

    tt.extend(quote! {
        .build()
    });

    tt.into()
}

/// Functional prodecural macro to implement a `ToResponse` trait from a schema that should
/// represent an API response.
#[proc_macro]
pub fn generate_response_schema(body: TokenStream) -> TokenStream {
    let exprs = parse_macro_input!(body with Punctuated::<Expr, Token![,]>::parse_terminated);
    let mut iter = exprs.iter();

    let Some(Expr::Path(ExprPath { path, .. })) = iter.next() else {
        return error(exprs.span(), "expected a path").into();
    };

    let ident = match path.require_ident() {
        Ok(ident) => ident,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut content_type = String::from("application/json");
    let mut schema = ident.to_string();

    for arg in iter {
        let Expr::Assign(ExprAssign { left, right, .. }) = arg else {
            return error(arg.span(), "expected assignment, received {arg}").into();
        };

        let Expr::Path(ExprPath { path: ref name, .. }) = **left else {
            return error(
                left.span(),
                "expected lhs to be a path with a single ident, received {left}",
            )
            .into();
        };

        let name = match name.require_ident() {
            Ok(ident) => ident,
            Err(e) => return e.into_compile_error().into(),
        };

        match name.to_string().as_str() {
            "content" => {
                let Expr::Lit(ExprLit {
                    lit: Lit::Str(ref s), ..
                }) = **right
                else {
                    return error(
                        right.span(),
                        format!("expected rhs to be a literal string, received {right:?}"),
                    )
                    .into();
                };

                content_type = s.value();
            }

            "schema" => {
                if let Expr::Macro(ExprMacro { ref mac, .. }) = **right {
                    let tt: proc_macro2::TokenStream = match mac.parse_body() {
                        Ok(tt) => tt,
                        Err(e) => return e.into_compile_error().into(),
                    };

                    // check if `tt` can be used as an Ident
                    let ident = match syn::parse_str::<syn::Ident>(tt.to_string().as_str()) {
                        Ok(ident) => ident,
                        Err(e) => return e.into_compile_error().into(),
                    };

                    schema = ident.to_string();
                    continue;
                }

                let Expr::Lit(ExprLit {
                    lit: Lit::Str(ref s), ..
                }) = **right
                else {
                    return error(
                        right.span(),
                        format!("expected rhs to be a literal string, received {right:?}"),
                    )
                    .into();
                };

                schema = s.value();
            }

            n => {
                return error(
                    arg.span(),
                    format!("expected 'content' or 'schema' on lhs, but received '{n}'"),
                )
                .into()
            }
        }
    }

    quote! {
        #[automatically_derived]
        impl<'r> ::utoipa::ToResponse<'r> for #ident {
            fn response() -> (
                &'r str,
                ::utoipa::openapi::RefOr<::utoipa::openapi::Response>
            ) {
                let __res = ::utoipa::openapi::ResponseBuilder::new()
                    .description(concat!("Response object for ", stringify!(#schema)))
                    .content(
                        #content_type,
                        ::utoipa::openapi::ContentBuilder::new()
                            .schema(
                                ::utoipa::openapi::RefOr::T(
                                    ::utoipa::openapi::Schema::Object({
                                        let __obj = ::utoipa::openapi::ObjectBuilder::new()
                                            .property(
                                                "success",
                                                ::utoipa::openapi::ObjectBuilder::new()
                                                    .schema_type(::utoipa::openapi::SchemaType::Boolean)
                                                    .description(Some(concat!("whether if this response [", stringify!(#ident), "] was successful or not")))
                                                    .build()
                                            )
                                            .required("success")
                                            .property(
                                                "data",
                                                ::utoipa::openapi::Ref::from_schema_name(#schema)
                                            )
                                            .required("data")
                                            .build();

                                        __obj
                                    })
                                )
                            )
                            .build()
                    )
                    .build();

                let __schema_name = stringify!(#ident);
                (__schema_name, ::utoipa::openapi::RefOr::T(__res))
            }
        }
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
/// ```rust,ignore
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
