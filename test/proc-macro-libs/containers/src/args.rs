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

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Expr, Meta, Path, Result, Token,
};

/// Arguments to a `#[testcontainers]` proc-macro.
pub struct Args {
    /// [`ItemFn`] to use to configure the test beforehand.
    pub configure: Option<Path>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = Args { configure: None };
        let meta = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        while let Some(arg) = meta.iter().next() {
            match arg {
                Meta::NameValue(nv) => {
                    let ident = nv
                        .path
                        .get_ident()
                        .ok_or_else(|| Error::new(nv.span(), "expected ident as key"))?
                        .to_string()
                        .to_lowercase();

                    match ident.as_str() {
                        "configure" => {
                            if args.configure.is_some() {
                                return Err(Error::new_spanned(ident, "`configure` cannot be overwritten twice"));
                            }

                            let path = match nv.value.clone() {
                                Expr::Path(path) => path,
                                other => return Err(Error::new_spanned(other, "unknown expr for `configure`")),
                            };

                            args.configure = Some(path.path);
                        }
                        other => return Err(Error::new_spanned(other, "unknown attribute config key")),
                    }
                }
                other => {
                    return Err(Error::new_spanned(
                        other,
                        "unknown attribute in #[testcontainers] macro!",
                    ))
                }
            }
        }

        Ok(args)
    }
}
