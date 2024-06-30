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

mod controller;
mod external;
mod indexable;
mod searchable;
mod util;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

/// Derive-based procedural macro to implement the `Searchable` trait from the `charted_search` crate.
#[proc_macro_derive(Searchable, attributes(search))]
pub fn searchable(body: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(body as DeriveInput);
    match &input.data {
        Data::Struct(struct_) => searchable::expand(&input, struct_),
        Data::Enum(_) => util::err!("enums are not supported in `#[derive(Indexable)]`")
            .into_compile_error()
            .into(),

        Data::Union(_) => util::err!("unions are not supported in `#[derive(Indexable)]`")
            .into_compile_error()
            .into(),
    }
}

/// Derive-based procedural macro to implement the `Indexable` trait from the `charted_search` crate.
#[proc_macro_derive(Indexable, attributes(indexable))]
pub fn indexable(body: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(body as DeriveInput);
    match &input.data {
        Data::Struct(_) => indexable::expand(&input),
        Data::Enum(_) => util::err!("enums are not supported in `#[derive(Indexable)]`")
            .into_compile_error()
            .into(),
        Data::Union(_) => util::err!("unions are not supported in `#[derive(Indexable)]`")
            .into_compile_error()
            .into(),
    }
}
