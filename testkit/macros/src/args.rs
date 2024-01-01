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

use std::{borrow::Cow, collections::HashMap};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Expr, ExprLit, ExprPath, Lit, Path, Result, Token,
};

#[derive(Debug, Default)]
pub struct Container {
    /// Image that will be parsed as a Docker image.
    pub image: String,

    /// List of container arguments to append.
    pub args: Vec<String>,

    /// Mapping of the enviornment variables on that image.
    pub env: HashMap<String, String>,
}

impl Parse for Container {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = Container::default();
        while !input.is_empty() {}

        Ok(args)
    }
}

/// Represents the arguments that could be passed in a `#[test]` proc-macro.
#[derive(Debug, Default)]
pub struct Args {
    /// Display name for the test, by default, this will be the function's name.
    pub display_name: Cow<'static, str>,

    /// Represents a container that is spun up. This will be spun up as well
    /// Postgres and Redis, since those two are required.
    pub containers: Vec<Container>,

    /// Customized `teardown` function for this specific test. This will be ran
    /// after all containers are destroyed but before the test is dropped.
    pub destroy_fn: Option<Path>,

    /// Customized `setup` function for this specific test. This will be ran
    /// after all containers are spun up and before a test is invoked.
    pub setup_fn: Option<Path>,

    /// Whether or not if this test should be ignored by the Tokio test runner.
    pub ignored: Option<Option<String>>, // none = false, some(none) = true without message, some(some(<msg>)) = true with message

    /// Whether or not if Docker usage is permitted. If `docker` is not installed
    /// on the host system, then the test will be ignored and will print on the first
    /// test:
    ///
    /// ```text
    /// > `docker` is not installed on the host system, most integration tests
    /// > will be disabled! Be warned... :ghost::ghost:
    /// ```
    pub docker: bool,
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(display_name);
    custom_keyword!(destroy_fn);
    custom_keyword!(container);
    custom_keyword!(setup_fn);
    custom_keyword!(ignored);
    custom_keyword!(docker);
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = Args::default();
        while !input.is_empty() {
            if input.peek(kw::display_name) {
                let span = input.parse::<kw::display_name>()?;
                if !args.display_name.is_empty() {
                    return Err(Error::new(span.span(), "`display_name` was mentioned more than once"));
                }

                input.parse::<Token![=]>()?;
                let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = input.parse()? else {
                    return Err(Error::new(span.span(), "expected literal string after the equals sign"));
                };

                args.display_name = Cow::Owned(s.value());
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }

                continue;
            }

            if input.peek(kw::destroy_fn) {
                let span = input.parse::<kw::destroy_fn>()?;
                if args.destroy_fn.is_some() {
                    return Err(Error::new(span.span(), "`destroy_fn` was already set to a path"));
                }

                input.parse::<Token![=]>()?;
                let Expr::Path(ExprPath { path, .. }) = input.parse()? else {
                    return Err(Error::new(span.span(), "expected path after the equals sign"));
                };

                args.destroy_fn = Some(path);
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }

                continue;
            }
        }

        Ok(args)
    }
}
