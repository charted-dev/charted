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

use std::fmt::Debug;

use crate::Session;
use async_trait::async_trait;
use charted_common::models::entities::User;
use eyre::Result;
use sqlx::{Error, PgPool, Row};

/// The [`SessionProvider`] abstraction allows you to provide a session with
/// the current authenticated user's details that was passed in.
#[async_trait]
pub trait SessionProvider: Send + Sync {
    /// Authorizes a user and returns a [`Session`] object, if it succeeded.
    async fn authorize(&self, password: String, user: &dyn UserWithPassword) -> Result<Session>;
}

impl Debug for Box<dyn SessionProvider> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Box<dyn SessionProvider>").finish_non_exhaustive()
    }
}

/// Trait to get a user's password. This cannot be implemented by
/// other structs.
///
/// This was necessary to not expose a `password` field in the User struct, as
/// it can be leaked through APIs and having an `.sanitized` method on it
/// didn't seem right, only on the `Session` struct.
#[async_trait]
pub trait UserWithPassword: private::Sealed + Send + Sync {
    /// Returns the [`User`] that this trait impl is holding.
    fn user(self) -> User;

    /// Retrieve a user's password. Can return `None` if it is not stored in the
    /// database.
    async fn password(&self, pool: PgPool, id: u64) -> Result<Option<String>>;
}

#[async_trait]
impl UserWithPassword for User {
    fn user(self) -> User {
        self
    }

    async fn password(&self, pool: PgPool, id: u64) -> Result<Option<String>> {
        let Some(row) = sqlx::query("SELECT password FROM users WHERE id = $1;")
            .bind(id as i64)
            .fetch_optional(&pool)
            .await?
        else {
            return Ok(None);
        };

        match row.try_get_raw("password") {
            Ok(r) => r.as_str().map(|f| Some(f.to_string())).map_err(|e| eyre::eyre!("{e}")),
            Err(e) => match e {
                Error::ColumnNotFound(_) => Ok(None),
                _ => Err(e.into()),
            },
        }
    }
}

mod private {
    use charted_common::models::entities::User;

    pub trait Sealed {}
    impl Sealed for User {}
}
