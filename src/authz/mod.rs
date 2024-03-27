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

pub mod ldap;
pub mod local;

use charted_entities::User;
use sqlx::{PgPool, Row};
use std::fmt::Display;

/// Represents an error that could've happened when authenticating via an authz [`Backend`].
#[derive(Debug)]
pub enum Error {
    /// given password was not the right one
    InvalidPassword,

    /// generic [`eyre::Report`] that was generated, this indicates that
    /// something didn't go right
    Eyre(eyre::Report),

    /// error that came through LDAP, this should never be received if the backend
    /// is not the LDAP one.
    Ldap(ldap3::LdapError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error as E;

        match self {
            E::InvalidPassword => f.write_str("received incorrect password"),
            E::Ldap(err) => Display::fmt(err, f),
            E::Eyre(err) => Display::fmt(err, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error as E;

        match self {
            E::Eyre(e) => Some(e.as_ref()),
            E::Ldap(e) => Some(e),
            _ => None,
        }
    }
}

impl From<eyre::Report> for Error {
    fn from(value: eyre::Report) -> Self {
        Self::Eyre(value)
    }
}

impl From<ldap3::LdapError> for Error {
    fn from(value: ldap3::LdapError) -> Self {
        Self::Ldap(value)
    }
}

/// Represents an auth backend that allows to authenticate users.
#[async_trait]
pub trait Backend: Send + Sync {
    /// Authenticate a user. If it returns `()`, then authentication was a success.
    async fn authenticate(&self, user: User, password: String) -> Result<(), Error>;

    /// Checks whenever if this is the local backend.
    fn is_local(&self) -> bool {
        false
    }
}

/// Represents a provider for providing a password for a user.
#[async_trait]
pub trait PasswordProvider: private::Sealed + Send + Sync {
    async fn provide_password(&self, pool: PgPool) -> eyre::Result<Option<String>>;
}

#[async_trait]
impl PasswordProvider for User {
    async fn provide_password(&self, pool: PgPool) -> eyre::Result<Option<String>> {
        let Some(row) = sqlx::query("SELECT password FROM users WHERE id = $1;")
            .bind(self.id)
            .fetch_optional(&pool)
            .await?
        else {
            return Ok(None);
        };

        match row.try_get_raw("password") {
            Ok(data) => data.as_str().map(|f| Some(f.to_owned())).map_err(|e| eyre!(e)),
            Err(sqlx::Error::ColumnNotFound(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for charted_entities::User {}
}
