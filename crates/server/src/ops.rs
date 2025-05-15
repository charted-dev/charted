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

pub mod avatars;
pub mod db;
pub mod jwt;

use argon2::{
    PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use charted_core::ARGON2;

pub fn hash_password<P: AsRef<[u8]>>(password: P) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let password = password.as_ref();

    ARGON2
        .hash_password(password, &salt)
        .map(|hash| hash.to_string())
        .inspect_err(|e| {
            error!(error = %e, "failed to compute argon2 password");
        })
        // since `argon2::Error` doesn't implement `std::error::Error`,
        // we implicitlly pass it into the `eyre!` macro, which will create
        // an adhoc error.
        .map_err(|e| eyre::eyre!(e))
}
