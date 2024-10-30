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

use axum::Router;
use azalia::rust::AsArcAny;
use charted_app::Context;
use charted_core::BoxedFuture;
use std::any::{Any, TypeId};

/// Represents a feature that can be enabled or disabled by the `features` object
/// in the API server configuration file.
///
/// For now, this is a marker trait for the `ServerContext` object to determine
/// a list of features enabled.
pub trait Feature: AsArcAny + Send + Sync {
    // If the feature requires to be initialized before being in use, then this is
    // method that does pre-initialization.
    fn init<'feat, 'cx>(&'feat self, _cx: &'cx Context) -> BoxedFuture<'cx, eyre::Result<()>>
    where
        'cx: 'feat,
    {
        Box::pin(async { Ok(()) })
    }

    /// Extends the API router to include endpoints.
    fn extend_router(&self) -> Router<Context> {
        Router::new()
    }

    /// Extends the database with a given [`DbPool`][charted_database::DbPool].
    ///
    /// This is mainly meant to run database migrations.
    #[cfg(feature = "extends-db")]
    fn extends_db<'feat, 'a>(&'feat self, _pool: &'a charted_database::DbPool) -> BoxedFuture<'a, eyre::Result<()>>
    where
        'a: 'feat,
    {
        Box::pin(async { Ok(()) })
    }

    /// Extends the OpenAPI document.
    #[cfg(feature = "extends-openapi")]
    fn extends_openapi<'feat, 'a>(&'feat self, _openapi: &'a mut utoipa::openapi::OpenApi)
    where
        'a: 'feat,
    {
    }
}

impl dyn Feature + 'static {
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

    /// Downcast `self` into [`F`], otherwise `None` is returned if `F` is not `self`.
    ///
    /// ## Example
    /// ```
    /// # use charted_features::Feature;
    /// #
    /// pub struct MyFeature;
    /// impl Feature for MyFeature {}
    ///
    /// let x: Box<dyn Feature> = Box::new(MyFeature);
    /// assert!(x.downcast::<MyFeature>().is_some());
    /// ```
    pub fn downcast<F: Feature + 'static>(&self) -> Option<&F> {
        if self.is::<F>() {
            // Safety: we ensured that `self` is `F`.
            Some(unsafe { &*(self as *const dyn Feature as *const F) })
        } else {
            None
        }
    }
}
