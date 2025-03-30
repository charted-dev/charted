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

//! # 🐻‍❄️📦 `charted_authz`
//! This crate holds the `Authenticator` trait, which other implementations
//! in `crates/authz` can use to safely authenticate a user.

use azalia::rust::AsArcAny;
use charted_core::BoxedFuture;
use charted_types::User;
use std::borrow::Cow;

/// Request object for the [`Authenticator::authenticate`] method.
#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub user: User,
    pub password: Cow<'a, str>,
    pub model: charted_database::entities::user::Model,
}

/// Error type to safely throw in a [`Authenticator`] implementation
/// when a invalid password is given.
#[derive(Debug, derive_more::Display)]
#[display("invalid password")]
pub struct InvalidPassword;
impl std::error::Error for InvalidPassword {}

/// Safely authenticate a user from any source.
pub trait Authenticator: AsArcAny + Send + Sync {
    fn authenticate<'a>(&'a self, request: Request<'a>) -> BoxedFuture<'a, eyre::Result<()>>;
}

azalia::impl_dyn_any!(Authenticator);

#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy;
    impl Authenticator for Dummy {
        fn authenticate<'a>(&'a self, _: Request<'a>) -> BoxedFuture<'a, eyre::Result<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[test]
    fn dyn_authenticator_is() {
        let me = Dummy;

        assert!(<dyn Authenticator>::is::<Dummy>(&me));
        assert!(!(<dyn Authenticator>::is::<String>(&me)));
    }

    #[test]
    fn dyn_authenticator_downcast() {
        let me: Box<dyn Authenticator> = Box::new(Dummy);

        assert!(me.downcast::<Dummy>().is_some());
        assert!(me.downcast::<String>().is_none());
    }
}
