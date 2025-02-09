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

//! # 🐻‍❄️📦 `charted_client`

#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]

mod client;
mod error;
pub mod types;

pub use client::*;
pub use error::*;

// use charted_core::api;
// use reqwest::Url;

// #[derive(Debug, Clone)]
// pub struct Client {
//     version: api::Version,
//     inner: reqwest::Client,
//     base: Url,
// }

// impl Client {
//     /// Creates a new client instance.
//     pub fn new(base: Url) -> Client {
//         Client {
//             version: api::Version::V1,
//             inner: reqwest::ClientBuilder::new().build().unwrap(),
//             base,
//         }
//     }

//     /// Sets the API version.
//     pub fn with_api_version<V: Into<api::Version>>(self, version: V) -> Self {
//         Self {
//             version: version.into(),
//             ..self
//         }
//     }

//     pub fn with_base<U: TryFrom<Url>>(self, url: U) ->
// }

// impl Default for Client {
//     fn default() -> Self {
//         Self::new(Url::parse("https://charts.noelware.org/api").unwrap())
//     }
// }

// impl From<reqwest::Client> for Client {
//     fn from(value: reqwest::Client) -> Self {
//         Self {
//             inner: value,
//             ..Default::default()
//         }
//     }
// }
