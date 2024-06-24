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
use charted_authz::{InvalidPasswordError, PasswordProvider};
use charted_common::BoxedFuture;
use charted_core::ARGON2;
use charted_entities::User;
use eyre::{eyre, Context};
use sqlx::PgPool;
use tracing::error;

/// Represents an implementation of [`charted_authz::Authenticator`] but uses the
/// local Postgres database for authenticating users.
#[derive(Clone)]
pub struct Authenticator {
    pool: PgPool,
}

impl Authenticator {
    /// Creates a new [`Authenticator`] instance to a owned instance of [`PgPool`].
    pub fn new(pool: PgPool) -> Authenticator {
        Authenticator { pool }
    }
}

impl charted_authz::Authenticator for Authenticator {
    fn authenticate<'u>(&'u self, user: &'u User, password: String) -> BoxedFuture<'u, eyre::Result<()>> {
        Box::pin(async move {
            let Some(pass) = user
                .provide(self.pool.clone())
                .await
                .context("failed to get password from database")?
            else {
                return Err(eyre!(
                    "unable to get password from database, were accounts not migrated?!"
                ));
            };

            let hash = PasswordHash::new(&password)
                .inspect_err(|e| error!(%e, "unable to compute argon2 hash for password: {e}"))
                .map_err(|e| eyre!(e))?;

            ARGON2
                .verify_password(pass.as_ref(), &hash)
                .map_err(|_| InvalidPasswordError)
                .context("invalid password")
        })
    }
}
