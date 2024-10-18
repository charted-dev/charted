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
use std::{
    any::{Any, TypeId},
    error::Error,
};

/// [`Error`] that represents that the password given is invalid.
#[derive(Debug, derive_more::Display)]
#[display("invalid password given")]
pub struct InvalidPassword;
impl Error for InvalidPassword {}

/// Trait that allows to build an authenticator that allows to authenticate users.
pub trait Authenticator: Send + Sync {
    /// Authenticate a given [`User`] with the password given.
    fn authenticate<'u>(&'u self, user: &'u User, password: String) -> BoxedFuture<'u, eyre::Result<()>>;
}

impl dyn Authenticator {
    /// Compares if [`self`] is `T`, similar to [`Any::is`].
    ///
    /// This method might fail (as in, returns `false`) if `T` doesn't implement [`Authenticator`].
    ///
    /// [`Any::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
    pub fn is<T: Any>(&self) -> bool {
        // get the `TypeId` of the concrete type (`self` being whatever authenticator is avaliable)
        let t = self.type_id();

        // get the `TypeId` of `T`.
        let other = TypeId::of::<T>();

        t == other
    }

    /// Attempts to downcast `T` from this <code>dyn [`Authenticator`]</code>.
    pub fn downcast<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            // Safety: we checked if `T` is `dyn Registry`.
            Some(unsafe { self.downcast_unchecked() })
        } else {
            None
        }
    }

    /// This method is the same as [`Any::downcast_ref_unchecked`] but uses `dyn Registry`
    /// instead of [`dyn Any`].
    ///
    /// Since the purpose of this is for the `downcast` method, this is not public
    /// and probably never will be.
    unsafe fn downcast_unchecked<T: Any>(&self) -> &T {
        debug_assert!(self.is::<T>());

        // SAFETY: caller has ensured that `self` is `dyn Registry`.
        unsafe { &*(self as *const dyn Authenticator as *const T) }
    }
}
