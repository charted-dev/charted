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

use charted_authz::{Authenticator, Request};
use charted_core::BoxedFuture;

/// Main implementation of the **local** session management
#[derive(Debug, Clone, Copy, Default)]
pub struct Backend {
    _priv: (),
}

impl Authenticator for Backend {
    fn authenticate<'a>(
        &'a self,
        Request {
            user: _,
            password: _,
            model: _,
        }: Request<'a>,
    ) -> BoxedFuture<'a, eyre::Result<()>> {
        Box::pin(async move { Ok(()) })
    }
}

/*
pub struct Backend;
impl charted_authz::Authenticator for Backend {
    fn authenticate<'u>(&'u self, user: &'u User, password: String) -> charted_core::BoxedFuture<'u, eyre::Result<()>> {
        Box::pin(async move {
            let Some(ref pass) = user.password else {
                return Err(eyre!(
                    "missing `password` field, did you migrate all users from previous backends?"
                ));
            };

            let hash = PasswordHash::new(pass)
                .inspect_err(|e| {
                    error!(error = %e, "failed to compute argon2 hash for password");
                })
                .map_err(|e|
                    /* since `password_hash::Error` doesn't implement `std::error::Error`,
                       we forward it to what `eyre!` uses instead */
                    eyre!(e))?;

            ARGON2
                .verify_password(password.as_ref(), &hash)
                .map_err(|_| charted_authz::InvalidPassword.into())
        })
    }
}
*/
