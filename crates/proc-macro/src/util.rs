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

use syn::{
    parse::{Parse, ParseStream},
    Attribute, Expr, ExprLit, Lit, Token,
};

/// Munch a comma token from a given [input].
pub fn munch_comma(input: ParseStream) -> syn::Result<()> {
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }

    Ok(())
}

/// Consume a [literal expression][ExprLit] from a parsed [expression][Expr].
pub fn parse_lit_expr(expr: Expr) -> syn::Result<String> {
    match expr {
        Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
        expr => Err(err!(expr, "expected literal string as input")),
    }
}

pub fn get_attr_opt<A: Parse, I: AsRef<str>>(attrs: &[Attribute], name: I) -> syn::Result<Option<A>> {
    let name = name.as_ref();
    match attrs
        .iter()
        .find(|el| match el.meta.path().get_ident() {
            Some(ident) => ident == name,
            None => false,
        })
        .map(|args| args.parse_args::<A>())
    {
        Some(Ok(attr)) => Ok(Some(attr)),
        Some(Err(_)) => Err(err!(format!("failed to parse `#[{name}]` attribute"))),
        None => Ok(None),
    }
}

pub fn get_attr<A: Parse, I: AsRef<str>>(attrs: &[Attribute], name: I) -> syn::Result<A> {
    match get_attr_opt(attrs, name.as_ref()) {
        Ok(Some(attr)) => Ok(attr),
        Ok(None) => Err(err!(format!("missing required attribute: `#[{}]`", name.as_ref()))),
        Err(err) => Err(err),
    }
}

macro_rules! err {
    ($message:literal) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), $message)
    };

    ($message:expr) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), $message)
    };

    ($span:ident, $message:expr) => {{
        use ::syn::spanned::Spanned;

        ::syn::Error::new(($span).span(), $message)
    }};

    ($span:ident, $message:literal) => {{
        use ::syn::spanned::Spanned;

        ::syn::Error::new(($span).span(), $message)
    }};
}

pub(crate) use err;

macro_rules! ident {
    ($ident:ident) => {
        ::proc_macro2::Ident::new(stringify!($ident), ::proc_macro2::Span::call_site())
    };
}

pub(crate) use ident;

macro_rules! into_compile_error {
    ($err:expr) => {
        match $err {
            Ok(value) => value,
            Err(err) => return err.into_compile_error().into(),
        }
    };
}

pub(crate) use into_compile_error;
