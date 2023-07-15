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

mod args;

use args::Args;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error, ItemFn};

#[proc_macro_attribute]
pub fn testcontainers(args: TokenStream, rust: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let func = parse_macro_input!(rust as ItemFn);

    if func.sig.asyncness.is_none() {
        return Error::new(func.sig.span(), "missing `async` keyword in fn declaration")
            .to_compile_error()
            .into();
    }

    let header = quote! { #[::tokio::test] };
    let name = func.sig.ident;
    let actual_body = func.block;
    let configure_body = match args.configure {
        Some(path) => quote! { #path().await.unwrap(); },
        None => quote!(),
    };

    quote! {
        #header
        async fn #name() {
            #configure_body
            #actual_body
        }
    }
    .into()
}
