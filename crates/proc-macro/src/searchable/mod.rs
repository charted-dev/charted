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

mod attr;

use crate::util;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput};

pub fn expand(
    DeriveInput { generics, ident, .. }: &DeriveInput,
    DataStruct { fields, .. }: &DataStruct,
) -> TokenStream {
    let mut searchable_fields = Vec::new();
    for field in fields {
        let Some(ref ident) = field.ident else {
            continue;
        };

        let Some(attr) = util::into_compile_error!(util::get_attr_opt::<attr::Attr, _>(&field.attrs, "search")) else {
            searchable_fields.push(ident.to_string());
            continue;
        };

        if !attr.skip {
            searchable_fields.push(ident.to_string());
        }
    }

    quote! {
        #[automatically_derived]
        impl #generics ::charted_search::Searchable for #ident #generics {
            fn allowed_fields<'s>(&self) -> &'s [&'s str] {
                &[
                    #(#fields,)#
                ]
            }
        }
    }
    .into()
}
