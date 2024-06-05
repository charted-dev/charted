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

use std::any::{Any, TypeId};

/// Represents a server feature that allows encapsulation between features.
pub trait Feature: Send + Sync {
    /// checks whenever if this feature is enabled or not
    fn enabled(&self) -> bool;
}

impl dyn Feature + 'static {
    /// Checks whenver if this [`Feature`] is enabled or not.
    pub fn enabled<T: Feature + 'static>(&self) -> bool {
        if self.is::<T>() {
            let downcasted = unsafe { self.downcast_unchecked::<T>() };
            downcasted.enabled()
        } else {
            false
        }
    }

    /// Compares if [`self`] is `T`, similar to [`Any::is`].
    ///
    /// This method might fail (as in, returns `false`) if `T` doesn't implement [`Feature`].
    ///
    /// [`Any::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
    pub fn is<T: Any>(&self) -> bool {
        let us = self.type_id();
        let other = TypeId::of::<T>();

        us == other
    }

    /// Attempts to downcast `T` from this `dyn Feature`.
    ///
    /// ## Example
    /// ```rust,ignore
    /// # use charted_features::Feature;
    /// # use std::sync::Arc;
    /// #
    /// #[derive(Default)]
    /// struct A;
    ///
    /// impl Feature for A { fn enabled(&self) -> bool { false } }
    ///
    /// // create a feature using `Arc` (this also works with `Box`).
    /// let feature: Arc<dyn Feature> = Arc::new(A::default());
    ///
    /// // `downcast` uses Feature::is::<A>() internally to check
    /// // if it is `A`, the second assertion will also be true.
    /// assert!(feature.downcast::<A>().is_some());
    /// assert!(feature.is::<A>());
    /// ```
    pub fn downcast<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(unsafe { self.downcast_unchecked() })
        } else {
            None
        }
    }

    /// This method is the same as [`Any::downcast_ref_unchecked`] but uses `dyn Feature`
    /// instead of [`dyn Any`].
    ///
    /// Since the purpose of this is for the `downcast` method, this is not public
    /// and probably never will be.
    unsafe fn downcast_unchecked<T: Any>(&self) -> &T {
        debug_assert!(self.is::<T>());

        // SAFETY: caller has ensured that `self` is `dyn Feature`.
        unsafe { &*(self as *const dyn Feature as *const T) }
    }
}
