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

use std::fmt::Display;

use charted_common::BoxedFuture;
use charted_entities::User;
use eyre::eyre;
use sqlx::{PgPool, Row};

/// Error which represents the user passed in the wrong password.
#[derive(Debug)]
pub struct InvalidPasswordError;

impl Display for InvalidPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid password")
    }
}

impl std::error::Error for InvalidPasswordError {}

/// `Authenticator` is the main interface you implement to crate custom authenticators
/// where it'll identify a user from a `user` object and check if the credentials are
/// valid.
pub trait Authenticator: Send + Sync {
    /// Authenticates a user with a given password. If the authenticator doesn't support passwords, then
    /// it can be safely ignored. `()` is returned as a indicator that this user is authenticated.
    fn authenticate<'u>(&'u self, user: &'u User, password: String) -> BoxedFuture<'u, eyre::Result<()>>;
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

// #[cfg(test)]
// mod tests {
//     use crate::{Authenticator, InvalidPasswordError};
//     use charted_common::box_pin;
//     use charted_entities::User;
//     use eyre::Context;

//     struct MyAuthenticator;
//     impl Authenticator for MyAuthenticator {
//         fn authenticate<'u>(
//             &'u self,
//             _user: &'u charted_entities::User,
//             _password: String,
//         ) -> charted_common::BoxedFuture<'u, eyre::Result<()>> {
//             box_pin!({ Err(InvalidPasswordError).context("w") })
//         }
//     }

//     #[tokio::test]
//     async fn test_invalid_password_deref() {
//         let res = MyAuthenticator::authenticate(&MyAuthenticator, &User::default(), "weow fluff".into()).await;
//         assert!(res.is_err());

//         let err = res.unwrap_err();
//         assert!(err.downcast_ref::<InvalidPasswordError>().is_some());
//     }
// }
