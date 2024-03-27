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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    DataStruct, DeriveInput, Result,
};

#[derive(Default)]
struct Args {
    skip: bool,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Default::default());
        }

        let mut me = Args::default();
        if input.peek(kw::skip) {
            input.parse::<kw::skip>()?;
            me.skip = true;
        }

        Ok(me)
    }
}

pub fn expand(input: &DeriveInput, struct_: &DataStruct) -> TokenStream {
    let generics = &input.generics;
    let name = &input.ident;

    let mut fields = Vec::new();
    for field in &struct_.fields {
        let Some(ref ident) = field.ident else {
            continue;
        };

        // If there is no `#[searchable]`, then we will assume we can index it :3
        let Some(attr) = field.attrs.iter().find(|s| match s.meta.path().get_ident() {
            Some(ident) => ident == "search",
            None => false,
        }) else {
            fields.push(ident.to_string());
            continue;
        };

        // If there was, let's try to parse `#[search(skip)]`, if we can. `Args`' Parse impl
        // will only check if `(skip)` is available and will not do any error handling as it
        // won't care what it wants.
        let args = attr.parse_args::<Args>().unwrap();
        if args.skip {
            continue;
        }

        fields.push(ident.to_string());
    }

    quote! {
        #[automatically_derived]
        impl #generics ::charted_search::Searchable for #name #generics {
            fn allowed_fields<'s>(&self) -> &'s [&'s str] {
                &[
                    #(#fields,)*
                ]
            }
        }
    }
}

mod kw {
    syn::custom_keyword!(skip);
}
