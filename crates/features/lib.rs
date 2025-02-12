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

use axum::Router;
use azalia::rust::AsArcAny;
use charted_core::BoxedFuture;
use charted_server::Context;
use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
    ops::Deref,
    sync::{Arc, LazyLock, Mutex},
};

static FEATURES: LazyLock<Mutex<HashMap<TypeId, Arc<dyn Feature>>>> = LazyLock::new(|| Mutex::new(azalia::hashmap!()));

fn all() -> HashMap<TypeId, Arc<dyn Feature>> {
    FEATURES.lock().unwrap().clone()
}

/// Returns a owned `F` if it is avaliable.
///
/// Due to dark magic, `F` is required to have the `Clone` requirement
/// so we can get `F` out of its internal `Arc` so it can be shared
/// and clone-able.
///
/// - Is this cursed? yes!
/// - Do I want to keep it this way? NO!
/// - Is there any other way? Probably! I don't want to think about it right now
///   since I could care less at this point.
pub fn get<F: Feature + Clone>() -> Option<F> {
    if let Some(feat) = all().get(&TypeId::of::<F>()).cloned() {
        let inner = feat.deref();
        debug_assert!(inner.is::<F>());

        return Some((unsafe { &*(inner as *const dyn Feature as *const F) }).clone());
    }

    None
}

/// Checks if `F` is in the feature list.
pub fn has<F: Feature + 'static>() -> bool {
    all().contains_key(&TypeId::of::<F>())
}

/// Adds a feature.
pub fn add<F: Feature + 'static>(feature: F) {
    let features = &mut *FEATURES.lock().unwrap();
    let id = TypeId::of::<F>();

    if features.contains_key(&id) {
        panic!("feature with type `{}` ({id:?}) already exists", type_name::<F>());
    }

    features.insert(id, Arc::new(feature));
}

/// Removes a feature.
pub fn remove<F: Feature + 'static>() -> bool {
    let features = &mut *FEATURES.lock().unwrap();
    if !features.contains_key(&TypeId::of::<F>()) {
        return false;
    }

    let _ = features.remove(&TypeId::of::<F>());
    true
}

/// Marker trait that can be enabled or disabled by the configuration file.
pub trait Feature: AsArcAny + Send + Sync + 'static {
    /// Does pre-initialization of this feature, if needed.
    fn init<'feature, 'cx: 'feature>(&'feature self, _cx: &'cx Context) -> BoxedFuture<'cx, eyre::Result<()>> {
        Box::pin(async { Ok(()) })
    }

    /// If this feature adds additional functionality to the REST server, then this
    /// is where you set the [`Router`] to be used.
    fn extend_router(&self) -> Option<Router<Context>> {
        None
    }

    /// If a feature ever extends the database, this is where all migrations
    /// can be initialized in.
    ///
    /// It must extend the [`sea_orm_migration::MigratorTrait`] trait so that
    /// CLI commands like `charted migrations` can work as needed.
    #[cfg(feature = "extends-database")]
    fn migrator<M: ::sea_orm_migration::MigratorTrait>(&self) -> Option<M> {
        None
    }

    /// Extends the OpenAPI document.
    #[cfg(feature = "extends-openapi")]
    fn extends_openapi<'feature, 'a: 'feature>(&'feature self, _doc: &'a mut ::utoipa::openapi::OpenApi) {}
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
    pub fn downcast<F: Feature>(&self) -> Option<&F> {
        if self.is::<F>() {
            // Safety: we ensured that `self` is `F`.
            Some(unsafe { &*(self as *const dyn Feature as *const F) })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, Clone, Copy, PartialEq)]
    struct AFeature {
        _priv: (),
    }

    impl Feature for AFeature {}

    struct __Drop<T: Feature>(T);
    impl<T: Feature> Drop for __Drop<T> {
        fn drop(&mut self) {
            remove::<T>();
        }
    }

    #[test]
    fn interior_mutability() {
        {
            let _guard = __Drop(AFeature::default());
            add(_guard.0);

            assert!(has::<AFeature>());
            assert!(get::<AFeature>().unwrap() == _guard.0);
        }

        assert!(!has::<AFeature>());
    }
}
