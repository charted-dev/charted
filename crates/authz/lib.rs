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

use charted_core::BoxedFuture;
use charted_types::User;
use std::error::Error;

/// [`Error`] that represents that the password given is invalid.
///
/// ## Example
/// ```
/// # use charted_authz::Authenticator;
/// #
/// # #[derive(Default)]
/// # struct A;
/// # impl Authenticator for A {
/// #     fn authenticate(&self) -> ::charted_core::BoxedFuture<::eyre::Result<()>> {
/// #         Box::pin(async { Err(::charted_authz::InvalidPassword.into()) })
/// #     }
/// # }
/// #
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let auth = A::default();
/// let res = auth.authenticate().await;
/// assert!(res.is_err());
///
/// let err = res.unwrap_err();
/// assert!(err.downcast_ref::<charted_authz::InvalidPassword>().is_some());
/// # }
/// ```
#[derive(Debug, derive_more::Display)]
#[display("invalid password given")]
pub struct InvalidPassword;
impl Error for InvalidPassword {}

/// Trait that allows to build an authenticator that allows to authenticate users.
pub trait Authenticator: Send + Sync {
    fn authenticate(&self, user: User, password: String) -> BoxedFuture<eyre::Result<()>>;
}
