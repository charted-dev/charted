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
