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

use syn::parse::Parse;

/// Defines a module for the `skip` keyword for the `#[searchable]` attribute proc-macro in a `#[derive(Searchable)]`.
mod kw {
    syn::custom_keyword!(skip);
}

#[derive(Default)]
pub struct Attr {
    pub skip: bool,
}

impl Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Default::default());
        }

        let mut me = Attr::default();
        if input.peek(kw::skip) {
            input.parse::<kw::skip>()?;
            me.skip = true;
        }

        Ok(me)
    }
}
