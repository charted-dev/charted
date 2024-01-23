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

pub use charted_proc_macros::controller;

use crate::lazy;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;

pub mod extract;
pub mod middleware;
pub mod models;
pub mod multipart;
pub mod openapi;
pub mod pagination;
pub mod routing;
pub mod validation;
pub mod version;

/// Represents the Hoshi distribution that was built from the `--cfg "bundle_web"` Rust flag.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(bundle_web, derive(rustembed::RustEmbed))]
#[cfg_attr(bundle_web, folder = "dist/")]
pub struct Hoshi;

impl Hoshi {
    #[allow(unused)] // it is only used in Hoshi::handler, so it's fine if it is unused.
    const INDEX_HTML: &'static str = "index.html";

    /// Checks whenever if [`Hoshi`] was built or not. This will just return the
    /// value from `--cfg "bundle_web"`.
    pub fn built() -> bool {
        cfg!(bundle_web)
    }

    #[cfg(bundle_web)]
    pub async fn handler(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
        use axum::response::IntoResponse;

        let path = uri.path().trim_start_matches('/');
        if path.is_empty() || path == INDEX_HTML {
            let asset = Hoshi::get(INDEX_HTML).expect("missing 'index.html' file?!");
            let content = remi::Bytes::from(asset.data.into_owned());

            return (
                [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
                content,
            )
                .into_response();
        }

        match Hoshi::get(path) {
            Some(file) => (
                [(axum::http::header::CONTENT_TYPE, file.metadata.mimetype())],
                remi::Bytes::from(file.data.into_owned()),
            )
                .into_response(),

            None if path.contains('.') => crate::server::models::res::err(
                axum::http::StatusCode::NOT_FOUND,
                (
                    crate::server::models::res::ErrorCode::HandlerNotFound,
                    "route was not found",
                ),
            )
            .into_response(),

            // let vue-router handle it.
            None => {
                let asset = Hoshi::get(INDEX_HTML).expect("missing 'index.html' file?!");
                let content = remi::Bytes::from(asset.data.into_owned());

                (
                    [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
                    content,
                )
                    .into_response();
            }
        }
    }
}

/// Static [`Argon2`] instance that is used for the API server.
pub static ARGON2: Lazy<Argon2<'static>> = lazy!(Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()));

/// Hashes a password into a Argon2-based password that is safely secure to store.
pub fn hash_password<P: Into<String>>(password: P) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.into().as_ref(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| {
            error!(error = %e, "unable to compute password");
            eyre!("unable to compute password: {e}")
        })
}
