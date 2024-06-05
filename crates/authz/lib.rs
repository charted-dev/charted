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

use charted_common::BoxedFuture;
use charted_entities::User;
use eyre::eyre;
use sqlx::{PgPool, Row};

/// `Authenticator` is the main interface you implement to crate custom authenticators
/// where it'll identify a user from a `user` object and check if the credentials are
/// valid.
pub trait Authenticator: Send + Sync {
    /// Authenticates a user with a given password. If the authenticator doesn't support passwords, then
    /// it can be safely ignored. `()` is returned as a indicator that this user is authenticated.
    fn authenticate(&self, user: User, password: String) -> BoxedFuture<eyre::Result<()>>;
}

/// Represents a provider to provide a password from `self`.
pub trait PasswordProvider: __private::Sealed + Send + Sync {
    /// Provides a password with a reference to the Postgres database.
    fn provide(&self, pool: PgPool) -> BoxedFuture<eyre::Result<Option<String>>>;
}

impl PasswordProvider for charted_entities::User {
    fn provide(&self, pool: PgPool) -> BoxedFuture<eyre::Result<Option<String>>> {
        Box::pin(async move {
            let Some(row) = sqlx::query("select password from users where id = $1;")
                .bind(self.id)
                .fetch_optional(&pool)
                .await?
            else {
                return Ok(None);
            };

            match row.try_get_raw("password") {
                Ok(data) => data
                    .as_str()
                    .map(|f| Some(f.to_owned()))
                    .map_err(|e| eyre!("received invalid utf-8?! ({e})")),

                Err(sqlx::Error::ColumnNotFound(_)) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
    }
}

mod __private {
    pub trait Sealed {}

    impl Sealed for charted_entities::User {}
}
