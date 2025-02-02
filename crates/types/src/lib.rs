// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

//! # 🐻‍❄️📦 `charted_types`
//! This crate is just a generic crate that exports all newtype wrappers for the
//! API server and database entities.

#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]

pub mod name;
pub mod payloads;

mod entities;
mod newtypes;

pub use entities::*;
pub use newtypes::*;

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_sea_orm {
    ($($item:item)*) => {
        #[cfg(feature = "__internal_db")]
        #[doc(hidden)]
        const _: () = {
            $($item)*
        };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_openapi {
    ($($item:item)*) => {
        #[cfg(feature = "openapi")]
        #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
        const _: () = {
            $($item)*
        };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_jsonschema {
    ($($item:item)*) => {
        #[cfg(feature = "jsonschema")]
        #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "jsonschema")))]
        const _: () = {
            $($item)*
        };
    };
}

// Not public API, used by macros in this crate.
#[doc(hidden)]
pub mod __private {
    pub use paste::paste;
}
