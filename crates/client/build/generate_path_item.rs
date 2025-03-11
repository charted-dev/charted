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

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, FnArg, Ident, Type};
use utoipa::openapi::{Deprecated, PathItem, RefOr, Schema};

pub fn generate(endpoint: &str, path: &PathItem) -> TokenStream {
    let mut tt = TokenStream::new();

    for operation in [&path.get, &path.put, &path.post, &path.delete, &path.patch] {
        let Some(op) = operation else {
            continue;
        };

        let comments: Vec<Attribute> = match (&op.summary, &op.description) {
            (Some(summary), None) => vec![syn::parse_quote!(#[doc = #summary])],
            (None, Some(desc)) => description_as_doc_comments(desc),
            _ => Vec::new(),
        };

        let op_id = Ident::new(op.operation_id.as_ref().unwrap(), Span::call_site());
        let deprecation: Option<Attribute> =
            op.deprecated
                .as_ref()
                .filter(|x| matches!(*x, Deprecated::True))
                .map(|_| {
                    syn::parse_quote! {
                        #[deprecated]
                    }
                });

        let params: Vec<FnArg> = op
            .parameters
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|param| {
                let name = Ident::new(&param.name, Span::call_site());
                let rust_type = get_rust_type_of(&param.schema.as_ref().expect("all parameters must have a schema"));

                syn::parse_quote! {
                    #name: #rust_type
                }
            })
            .collect();

        tt.extend(quote! {
            #(#comments)*
            #deprecation?
            pub fn #op_id(
                &self,
                #(#params,)*
            ) -> ::crate::Result<(), ()> {
                todo!()
            }
        });
    }

    tt
}

fn description_as_doc_comments(x: &str) -> Vec<Attribute> {
    x.lines().map(|x| syn::parse_quote! { #[doc = #x] }).collect()
}

fn get_rust_type_of(schema: &RefOr<Schema>) -> Type {
    todo!()
}
