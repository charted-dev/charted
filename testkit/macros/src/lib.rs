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
use proc_macro2::Span;
use quote::quote;
use std::fmt::Display;
use syn::{parse_macro_input, spanned::Spanned, ItemFn};

/// Represents an attribute-based procedural macro that helps create a test that
/// includes a test context.
///
/// ## Example
/// ```rust,ignore
/// # use charted_testkit_macros::integ_test;
/// #
/// #[integ_test(
///     display_name = "A test that could fail", // a display name when printing
///     docker = false,                          // disable running Docker containers for this test
///     should_panic = true,                     // in same vein of `std::test`'s `expect_panic`
/// )]
/// async fn my_test(context: charted_testkit::TestContext) -> eyre::Result<()> {
///     # panic!("i panic'd!")
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn integ_test(attrs: TokenStream, body: TokenStream) -> TokenStream {
    let fn_body: proc_macro2::TokenStream = body.clone().into();
    let attrs = parse_macro_input!(attrs as Args);
    let input = parse_macro_input!(body as ItemFn);

    if input.sig.asyncness.is_none() {
        return error(
            input.sig.span(),
            format!("function `{}` needs `async` for `#[test]` to work", input.sig.ident),
        );
    }

    if input.sig.abi.is_some() {
        return error(input.sig.span(), "ABI methods are not allowed in `#[test]`");
    }

    if input.sig.constness.is_some() {
        return error(input.sig.span(), "Using `const fn` is not supported in `#[test]`");
    }

    let name = input.sig.ident;
    let display_name = match &attrs.display_name.is_empty() {
        true => quote!(concat!(#name)),
        false => {
            let name = attrs.display_name;
            quote!(#name)
        }
    };

    quote! {
        #[test]
        fn #name() -> ::eyre::Result<()> {
            // First, build the Tokio runtime. Since we can't derive `#[tokio::test]`, this
            // is only for our use case.
            let runtime = ::tokio::runtime::Builder::new_current_thread()
                .worker_threads(1)
                .thread_name("test-worker-pool")
                .enable_all()
                .build()
                .unwrap(); // panic if we can't build the runtime

            // Next, we need to build a TestContext that the test
            // can interact with to know the current state of this test.
            let ctx = ::charted_testkit::TestContext::create(concat!("test [", #display_name, "] in ", file!()));
            runtime.block_on(async move {
                let __caller = #fn_body;
                __caller(&ctx).await;
            })
                .map(|_| Ok(()))
        }
    }
    .into()
}

fn error<I: Display>(span: Span, message: I) -> TokenStream {
    syn::Error::new(span, message).into_compile_error().into()
}
