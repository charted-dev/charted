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

use argon2::{PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use charted_common::server::ARGON2;
use charted_sessions::{SessionProvider, UserWithPassword};
use eyre::{eyre, Result};
use sqlx::PgPool;
use std::fmt::Debug;

#[derive(Clone)]
pub struct LocalSessionProvider {
    pool: PgPool,
}

unsafe impl Send for LocalSessionProvider {}

impl Debug for LocalSessionProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalSessionProvider").finish_non_exhaustive()
    }
}

impl LocalSessionProvider {
    pub fn new(pool: PgPool) -> LocalSessionProvider {
        LocalSessionProvider { pool }
    }
}

#[async_trait]
impl SessionProvider for LocalSessionProvider {
    #[tracing::instrument(name = "charted.sessions.local.authorize", skip_all, user.id = user.user().id, user.username = tracing::field::display(user.user().username))]
    async fn authorize(&mut self, password: String, user: &dyn UserWithPassword) -> Result<()> {
        let user = user.user();
        match user.password(self.pool.clone(), user.id as u64).await {
            Ok(Some(pass)) => {
                let hash = PasswordHash::new(&pass).map_err(|e| eyre!("unable to compute hash: {e}"))?;
                match ARGON2.verify_password(password.as_ref(), &hash) {
                    Ok(()) => Ok(()),
                    Err(e) => Err(eyre!("unable to verify password: {e}")),
                }
            }

            Ok(None) => Err(eyre!(
                "Internal Server Error: user @{} ({}) doens't contain a password!",
                user.username,
                user.id
            )),

            Err(e) => Err(eyre!("unable to retrieve password from database: {e}")),
        }
    }
}
