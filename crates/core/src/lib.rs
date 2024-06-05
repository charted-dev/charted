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

use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use azalia::lazy;
use eyre::eyre;
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use tracing::error;

pub mod avatars;
pub mod multipart;
pub mod openapi;
pub mod redis;
pub mod response;

pub static ARGON2: Lazy<Argon2<'static>> = lazy!(Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()));

/// Hashes a password into a Argon2-based password that is safely secure to store.
pub fn hash_password<P: AsRef<[u8]>>(password: P) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.as_ref(), &salt)
        .map(|hash| hash.to_string())
        .inspect_err(|e| {
            error!(error = %e, "unable to compute password");
        })
        .map_err(|e| eyre!(e))
}
