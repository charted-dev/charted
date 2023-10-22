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

mod paths;

use charted_proc_macros::error;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Expr, ExprArray, ExprAssign, ExprLit, ExprMacro,
    ExprPath, Lit, Token,
};

/// Dynamically creates a [`Paths`](utoipa::openapi::Paths) object from any amount
/// of paths.
///
/// ## Example
/// ```no_run
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
    let args = parse_macro_input!(body as paths::AddPathArgs);
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
                    .description(concat!("Response object for ", #schema))
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
                                                    .description(Some(concat!("whether if this response [", concat!("Api", stringify!($schema)), "] was successful or not")))
                                                    .build()
                                            )
                                            .required("success")
                                            .property(
                                                "data",
                                                ::utoipa::openapi::Ref::from_schema_name(stringify!($schema))
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

                (#schema, ::utoipa::openapi::RefOr::T(__res))
            }
        }
    }
    .into()
}
