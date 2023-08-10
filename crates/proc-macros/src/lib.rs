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

mod helpers;
mod paths;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Lit};

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

/// Functional procedural macro to implement a method that returns
/// a [`PathItem`], with some custom syntax.
///
/// ## Examples
/// ```no_run
/// # use charted_proc_macros::{paths, response};
/// #
/// paths! {
///    Get {
///       operation_id("features");
///       description("REST handler to retrieve this server's features that were enabled or disabled by the server administrators");
///
///       responses(StatusCode::OK, "application/json") {
///          description("Successful response!");
///          schema(response!("ApiFeaturesResponse"));
///       }
///    }
/// }
/// ```
#[proc_macro]
pub fn paths(body: TokenStream) -> TokenStream {
    let args = parse_macro_input!(body as paths::Args);
    let operations = args
        .operations
        .iter()
        .map(|(ty, op)| {
            (
                helpers::PathItemType::from(ty.clone()),
                helpers::Operation::from(op.clone()),
            )
        })
        .map(|(ty, op)| quote! { .operation(#ty, #op) })
        .collect::<Vec<_>>();

    let pathitem = quote! {
        ::utoipa::openapi::path::PathItemBuilder::new()
            #(#operations)*
            .build()
    };

    quote! {
        pub fn paths() -> ::utoipa::openapi::PathItem {
            #pathitem
        }
    }
    .into()
}
