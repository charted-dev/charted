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

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHash, PasswordHasher, PasswordVerifier,
};
use charted_authz::{Authenticator, Request};
use charted_core::{BoxedFuture, ARGON2};
use eyre::bail;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
};
use tracing::{instrument, trace, warn};

const ARGON2_PREFIX: &str = "$argon2id";

#[derive(Clone)]
pub struct Backend(HashMap<String, String>);
impl Debug for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Backend")
            .field("kind", &"static")
            .field("users", &self.0.keys())
            .finish()
    }
}

impl Backend {
    pub fn new(users: BTreeMap<String, String>) -> Self {
        let mut new_users = azalia::hashmap!();
        for (username, password) in users {
            if password.starts_with(ARGON2_PREFIX) {
                trace!(%username, "password was already hashed; checking if it is a valid hash");
                let hash = match PasswordHash::new(&password) {
                    Ok(v) => v,
                    Err(err) => {
                        warn!(%username, error = %err, "failed to parse hashed password; skipping entry!");
                        continue;
                    }
                };

                new_users.insert(username, hash.to_string());
                continue;
            }

            warn!(%username, "password for this user wasn't hashed! please set a hash from `charted admin authz hash-password <password>` and set the output as the value to keep a consistent hash!");

            let salt = SaltString::generate(&mut OsRng);
            let hash = match ARGON2.hash_password(password.as_bytes(), &salt) {
                Ok(v) => v,
                Err(err) => {
                    warn!(%username, error = %err, "failed to create hashed password; skipping entry!");
                    continue;
                }
            };

            new_users.insert(username, hash.to_string());
        }

        Self(new_users)
    }
}

impl Authenticator for Backend {
    #[instrument(name = "charted.authz.static.authenticate", skip_all, fields(%user.username))]
    fn authenticate<'a>(&'a self, Request { user, password, .. }: Request<'a>) -> BoxedFuture<'a, eyre::Result<()>> {
        Box::pin(async move {
            let Some(hashed) = self.0.get(user.username.as_str()) else {
                bail!("user {} doesn't exist in static mapping", user.username);
            };

            // We check if the hash is already valid, so `.unwrap()` is ok to use.
            let hash = PasswordHash::new(hashed).unwrap();
            ARGON2
                .verify_password(password.as_bytes(), &hash)
                .map_err(|_| charted_authz::InvalidPassword.into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use charted_authz::{Authenticator, InvalidPassword};
    use charted_database::entities::user::Model;
    use charted_types::User;
    use std::borrow::Cow;

    fn build_backend() -> Backend {
        Backend::new(azalia::btreemap! {
            // echo "noeliscutieuwu" | cargo cli admin authz hash-password --stdin
            "noel" => "$argon2id$v=19$m=19456,t=2,p=1$gIcVA4mVHgr8ZWkmDrtJlw$sb5ypFAvphFCGrJXy9fRI1Gb/2vGIH1FTzDax458+xY"
        })
    }

    fn build_request<'s>(username: &'s str, password: &'s str) -> Request<'s> {
        charted_authz::Request {
            password: Cow::Borrowed(password),
            model: Model {
                password: None,
                email: "charted@noelware.org".to_owned(),
                username: username.parse().unwrap(),
                verified_publisher: Default::default(),
                prefers_gravatar: Default::default(),
                gravatar_email: Default::default(),
                description: Default::default(),
                avatar_hash: Default::default(),
                created_at: Default::default(),
                updated_at: Default::default(),
                admin: Default::default(),
                name: Default::default(),
                id: Default::default(),
            },
            user: User {
                username: username.parse().unwrap(),
                verified_publisher: Default::default(),
                prefers_gravatar: Default::default(),
                gravatar_email: Default::default(),
                description: Default::default(),
                avatar_hash: Default::default(),
                created_at: Default::default(),
                updated_at: Default::default(),
                admin: Default::default(),
                name: Default::default(),
                id: Default::default(),
            },
        }
    }

    #[tokio::test]
    async fn test_authentication_backend() {
        let backend = build_backend();
        assert!(backend
            .authenticate(build_request("noel", "noeliscutieuwu"))
            .await
            .is_ok());

        let err = backend
            .authenticate(build_request("noel", "thetwinofboel"))
            .await
            .unwrap_err();

        assert!(err.is::<InvalidPassword>());

        let err = backend
            .authenticate(build_request("boel", "rootofallevil"))
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), "user boel doesn't exist in static mapping");
    }
}
