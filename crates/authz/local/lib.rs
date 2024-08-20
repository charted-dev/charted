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

use argon2::{PasswordHash, PasswordVerifier};
use charted_core::ARGON2;
use charted_types::User;
use eyre::eyre;
use tracing::error;

pub struct Backend;
impl charted_authz::Authenticator for Backend {
    fn authenticate(&self, user: User, password: String) -> charted_core::BoxedFuture<eyre::Result<()>> {
        Box::pin(async move {
            let Some(pass) = user.password else {
                return Err(eyre!(
                    "missing `password` field, did you migrate all users from previous backends?"
                ));
            };

            let hash = PasswordHash::new(&pass)
                .inspect_err(|e| {
                    error!(error = %e, "failed to compute argon2 hash for password");
                })
                .map_err(|e|
                    /* since `password_hash::Error` doesn't implement `std::error::Error`, we forward it to what `eyre!` uses instead */
                    eyre!(e)
                )?;

            ARGON2
                .verify_password(password.as_ref(), &hash)
                .map_err(|_| charted_authz::InvalidPassword.into())
        })
    }
}
