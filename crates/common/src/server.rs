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

use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use eyre::{eyre, Result};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use tracing::error;

/// Static [`Argon2`] instance that is used for the API server.
pub static ARGON2: Lazy<Argon2<'static>> =
    Lazy::new(|| Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()));

/// Hashes a password with Argon2 from the global [`ARGON2`] instance.
pub fn hash_password(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    match ARGON2.hash_password(password.as_ref(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => {
            error!(%e, "unable to compute password");

            Err(eyre!("unable to compute password, try again later"))
        }
    }
}
