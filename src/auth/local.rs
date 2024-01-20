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
use sqlx::PgPool;

use crate::{common::models::entities::User, server::ARGON2};

use super::PasswordProvider;

/// Represents the `local` backend itself.
#[derive(Clone)]
pub struct Backend(PgPool);

impl Backend {
    /// Creates a new [`Backend`] instance to a reference of the PostgreSQL connection pool.
    pub fn new(pool: PgPool) -> Backend {
        Backend(pool)
    }
}

#[async_trait]
impl super::Backend for Backend {
    async fn authenticate(&self, user: User) -> eyre::Result<()> {
        match user.provide_password(self.0.clone()).await {
            Ok(Some(pass)) => {
                let hash = PasswordHash::new(&pass).map_err(|e| {
                    error!(error = %e, "unable to compute hash");
                    eyre!("unable to compute hash: {e}")
                })?;

                ARGON2.verify_password(pass.as_ref(), &hash).map_err(|e| {
                    error!(error = %e, "unable to verify password");
                    eyre!("unable to verify password: {e}")
                })
            }

            Ok(None) => Err(eyre!("internal server error: user @{} ({}) doesn't contain a password field! did you forget to migrate your users?", user.username, user.id)),
            Err(e) => Err(e)
        }
    }
}
