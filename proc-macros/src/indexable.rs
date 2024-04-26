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

use crate::{error, helpers::OptionHelper};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    DeriveInput, Error, Expr, ExprLit, Ident, Lit, Result, Token,
};

pub struct Attributes {
    pub index: String,
    pub id_field: Option<String>,
    pub id: Ident,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Attributes {
                index: "<unknown>".into(),
                id_field: None,
                id: Ident::new("id", Span::call_site()),
            });
        }

        let mut me = Attributes {
            index: "<unknown>".into(),
            id_field: None,
            id: Ident::new("id", Span::call_site()),
        };

        while !input.is_empty() {
            if input.peek(kw::index) {
                let tok = input.parse::<kw::index>()?;
                if me.index != "<unknown>" {
                    return Err(Error::new(tok.span(), "`index` field was already initialized"));
                }

                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;
                let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = expr else {
                    return Err(Error::new(expr.span(), "expected literal string as result"));
                };

                me.index = s.value();
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }

                continue;
            }

            if input.peek(kw::id_field) {
                let tok = input.parse::<kw::id_field>()?;
                if me.id_field.is_some() {
                    return Err(Error::new(tok.span(), "`id_field` was previously set"));
                }

                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;
                let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = expr else {
                    return Err(Error::new(expr.span(), "expected literal string as result"));
                };

                me.id_field = Some(s.value());
                continue;
            }

            if input.peek(kw::id) {
                input.parse::<kw::id>()?;
                input.parse::<Token![=]>()?;

                let ident: Ident = input.parse()?;
                me.id = ident;
                continue;
            }
        }

        Ok(me)
    }
}

mod kw {
    syn::custom_keyword!(id_field);
    syn::custom_keyword!(index);
    syn::custom_keyword!(id);
}

pub(crate) fn expand(derive: &DeriveInput) -> TokenStream {
    let generics = &derive.generics;
    let name = &derive.ident;
    let args = match derive
        .attrs
        .iter()
        .find(|el| match el.meta.path().get_ident() {
            Some(ident) => ident == "indexable",
            None => false,
        })
        .map(|args| args.parse_args::<Attributes>())
    {
        Some(Ok(args)) => args,
        Some(Err(err)) => {
            return error(
                derive.span(),
                format!("failed to parse `#[indexable]` attribute: {err}"),
            )
        }

        None => return error(derive.span(), "missing `#[indexable]` attribute"),
    };

    let id_field = &OptionHelper(args.id_field);
    let index = &args.index;
    let id = &args.id;
    quote! {
        #[automatically_derived]
        impl #generics ::charted_search::Indexable for #name {
            fn index(&self) -> &'static str { #index }
            fn id_field(&self) -> &'static str {
                let __value: Option<&str> = #id_field;
                match __value {
                    Some(value) => value,
                    None => "id"
                }
            }

            fn id(&self) -> i64 {
                self.#id
            }
        }
    }
}
