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
    spanned::Spanned,
    Error, Expr, ExprLit, Lit, Result, Token,
};

#[derive(Debug, Default)]
pub struct AddPathArgs {
    //                 lhs   rhs
    pub elements: Vec<(Expr, Expr)>,
}

impl Parse for AddPathArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut me = AddPathArgs::default();
        while !input.is_empty() {
            let lhs = input.parse::<Expr>()?;

            // if it is a call expr, then we can continue
            if let Expr::Call(_) = lhs.clone() {
                input.parse::<Token![=>]>()?;

                let rhs = input.parse::<Expr>()?;
                if let Expr::Call(_) = rhs.clone() {
                    input.parse::<Token![;]>()?;
                    me.elements.push((lhs.clone(), rhs));

                    continue;
                }

                // an array is meant to have multiple paths that should be
                // on the same path
                if let Expr::Array(_) = rhs.clone() {
                    input.parse::<Token![;]>()?;
                    me.elements.push((lhs.clone(), rhs));

                    continue;
                }

                return Err(Error::new(rhs.span(), "expected call expression for right-hand side"));
            }

            // if it is a literal, then we can continue
            if let Expr::Lit(ExprLit { lit: Lit::Str(_), .. }) = lhs.clone() {
                input.parse::<Token![=>]>()?;

                let rhs = input.parse::<Expr>()?;
                if let Expr::Call(_) = rhs.clone() {
                    input.parse::<Token![;]>()?;
                    me.elements.push((lhs.clone(), rhs));

                    continue;
                }

                // an array is meant to have multiple paths that should be
                // on the same path
                if let Expr::Array(_) = rhs.clone() {
                    input.parse::<Token![;]>()?;
                    me.elements.push((lhs.clone(), rhs));

                    continue;
                }

                return Err(Error::new(rhs.span(), "expected call expression for right-hand side"));
            }

            dbg!(lhs.clone());
            return Err(Error::new(
                lhs.span(),
                "expected Path or literal string for left-hand side",
            ));
        }

        Ok(me)
    }
}
