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

use crate::Context;
use axum::Router;
use azalia::rust::AsArcAny;
use charted_core::BoxedFuture;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};
use utoipa::openapi::OpenApi;

pub type Collection = HashMap<TypeId, Arc<dyn Feature>>;

/// Implements a **server feature**. Read in the [`features/` folder] for documentation.
///
/// [`features/` folder]: https://github.com/charted-dev/charted/blob/main/features/README.md
pub trait Feature: AsArcAny + Send + Sync + 'static {
    /// Does pre-initialization of this feature once the server is booting up.
    fn init<'feat, 'cx: 'feat>(&self, cx: &'cx Context) -> BoxedFuture<'feat, eyre::Result<()>> {
        let _ = cx;
        Box::pin(async { Ok(()) })
    }

    /// If the server feature implements new routes or data types, then this is where
    /// you can extend the core OpenAPI document.
    fn extends_openapi(&self, doc: &mut OpenApi) {
        let _ = doc;
    }

    /// Extends the API router.
    ///
    /// It returns a tuple of `(path, router)`. The `path` must be a valid path.
    fn extend_router(&self) -> (&'static str, Router<Context>);
}

impl dyn Feature {
    /// Compares if [`self`] is `T`, similar to [`Any::is`].
    ///
    /// This method might fail (as in, returns `false`) if `T` doesn't implement
    /// [`Feature`].
    ///
    /// [`Any::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
    pub fn is<T: Any>(&self) -> bool {
        let us = self.type_id();
        let other = TypeId::of::<T>();

        us == other
    }

    /// Downcast `self` into type `F`, otherwise `None` is returned if `F` is not `self`.
    ///
    /// ## Example
    /// ```
    /// # use charted_server::feature::Feature;
    /// #
    /// pub struct MyFeature;
    /// impl Feature for MyFeature {
    ///     fn extend_router(&self) -> (&'static str, ::axum::routing::Router<charted_server::Context>) { todo!() }
    /// }
    ///
    /// let x: Box<dyn Feature> = Box::new(MyFeature);
    /// assert!(x.downcast::<MyFeature>().is_some());
    /// ```
    pub fn downcast<F: Feature>(&self) -> Option<&F> {
        if self.is::<F>() {
            // Safety: we ensured that `self` is `F`.
            Some(unsafe { &*(self as *const dyn Feature as *const F) })
        } else {
            None
        }
    }
}
