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

use crate::util;
use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

mod kw {
    syn::custom_keyword!(index);
    syn::custom_keyword!(field);
    syn::custom_keyword!(id);
}

pub struct Attr {
    pub index: String,
    pub field: Option<String>,
    pub id: Ident,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Attr> {
        if input.is_empty() {
            return Err(util::err!("cannot use empty body as a valid `#[indexable]` attribute"));
        }

        let mut attr = Attr {
            index: "{unknown}".into(),
            field: None,
            id: util::ident!(id),
        };

        while !input.is_empty() {
            // lookahead from what the compiler gave us
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::index) {
                let tok = input.parse::<kw::index>()?;
                if attr.index != "{unknown}" {
                    return Err(util::err!(tok, "`index` field is already set"));
                }

                // munch `=`
                input.parse::<Token![=]>()?;

                attr.index = util::parse_lit_expr(input.parse()?)?;
                util::munch_comma(input)?;

                continue;
            } else if lookahead.peek(kw::field) {
                let tok: kw::field = input.parse()?;
                if attr.field.is_some() {
                    return Err(util::err!(tok, "`field` is already set previously"));
                }

                input.parse::<Token![=]>()?;

                attr.field = Some(util::parse_lit_expr(input.parse()?)?);
                util::munch_comma(input)?;

                continue;
            } else if lookahead.peek(kw::id) {
                input.parse::<kw::id>()?;
                input.parse::<Token![=]>()?;

                attr.id = input.parse()?;
                util::munch_comma(input)?;

                continue;
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(attr)
    }
}
