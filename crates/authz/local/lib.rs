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

use argon2::{PasswordHash, PasswordVerifier};
use charted_authz::{Authenticator, Request};
use charted_core::{ARGON2, BoxedFuture};
use eyre::{bail, eyre};
use tracing::{error, instrument};

/// Main implementation of the **local** session management
#[derive(Debug, Clone, Copy, Default)]
pub struct Backend {
    _priv: (),
}

impl Authenticator for Backend {
    #[instrument(name = "charted.authz.local.authenticate", skip_all, fields(%user.username, %user.id))]
    fn authenticate<'a>(
        &'a self,
        Request { user, password, model }: Request<'a>,
    ) -> BoxedFuture<'a, eyre::Result<()>> {
        Box::pin(async move {
            let Some(ref pass) = model.password else {
                bail!("missing `password` field for user, did you migrate all users from previous backends?")
            };

            let hash = PasswordHash::new(pass)
                .inspect_err(|e| {
                    error!(error = %e, "failed to compute argon2 hash");
                })
                .map_err(|e| eyre!(e))?;

            ARGON2
                .verify_password(password.as_bytes(), &hash)
                .map_err(|_| charted_authz::InvalidPassword.into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use charted_authz::InvalidPassword;
    use charted_database::entities::user::Model;
    use charted_types::User;
    use std::borrow::Cow;

    fn build_request<'s>(username: &'s str, password: &'s str) -> Request<'s> {
        charted_authz::Request {
            password: Cow::Borrowed(password),
            model: Model {
                // echo "noeliscutieuwu" | cargo cli admin authz hash-password --stdin
                password: Some(String::from(
                    "$argon2id$v=19$m=19456,t=2,p=1$gIcVA4mVHgr8ZWkmDrtJlw$sb5ypFAvphFCGrJXy9fRI1Gb/2vGIH1FTzDax458+xY",
                )),

                email: "noel@noelware.org".to_owned(),
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
    async fn authenticate() {
        let backend = Backend::default();
        assert!(
            backend
                .authenticate(build_request("noel", "noeliscutieuwu"))
                .await
                .is_ok()
        );

        let err = backend
            .authenticate(build_request("noel", "thetwinofboel"))
            .await
            .unwrap_err();

        assert!(err.is::<InvalidPassword>());
    }
}
