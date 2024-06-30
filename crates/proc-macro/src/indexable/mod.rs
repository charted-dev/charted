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
use syn::DeriveInput;

pub fn expand(
    DeriveInput {
        attrs, ident, generics, ..
    }: &DeriveInput,
) -> TokenStream {
    let attr::Attr { field, id, index } = util::into_compile_error!(util::get_attr(attrs.as_slice(), "indexable"));

    quote! {
        #[automatically_derived]
        impl #generics ::charted_search::Indexable for #ident {
            fn index<'a>(&self) -> ::std::borrow::Cow<'a, str> {
                ::std::borrow::Cow::Borrowed(#index)
            }

            fn id_field<'a>(&self) -> ::std::borrow::Cow<'a, str> {
                match #field {
                    Some(value) => ::std::borrow::Cow::Borrowed(value),
                    None => ::std::borrow::Cow::Borrowed("id")
                }
            }

            fn id(&self) -> i64 {
                self.#id
            }
        }
    }
    .into()
}
