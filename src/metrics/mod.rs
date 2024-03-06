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

pub mod collectors;
pub mod registries;

use self::registries::{default::Default, prometheus::Prometheus};
use crate::{common::AsArcAny, config::Config, metrics::registries::disabled::Disabled};
use std::{
    any::{Any, TypeId},
    sync::Arc,
};

/// The [`Collector`] abstraction allows you to wrap `Serialize`-impl structs
/// and wrap it around the Admin API, where statistics about the server is running
/// can be collected.
///
/// This can also be wrapped around a Prometheus collector (`prometheus_client::collector::Collector`)
/// to provide Prometheus metrics for this collector. This can be only available via the PrometheusMetricRegistry
/// implementation.
pub trait Collector: Send + Sync {
    /// Returns the name of this [`Collector`].
    fn name(&self) -> &'static str;

    /// Collects all the data and returns anything.
    fn collect(&self) -> Box<dyn Any>;

    /// Collects all data, but returns `Serialize`-trait object.
    fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize>;
}

/// Represents a collection of [`Collector`]s.
pub trait Registry: AsArcAny + Send + Sync {
    /// Inserts a new [`Collector`] into this registry.
    fn insert(&mut self, collector: Box<dyn Collector>);

    /// Returns all of the collectors that this [`Registry`] owns.
    fn collectors(&self) -> &Vec<Box<dyn Collector>>;
}

impl dyn Registry {
    /// Compares if [`self`] is `T`, similar to [`Any::is`].
    ///
    /// This method might fail (as in, returns `false`) if `T` doesn't implement [`Registry`].
    ///
    /// [`Any::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
    pub fn is<T: Any>(&self) -> bool {
        // get the `TypeId` of the concrete type (`self` being whatever registry)
        let t = self.type_id();

        // get the `TypeId` of `T`.
        let other = TypeId::of::<T>();

        t == other
    }

    /// Attempts to downcast `T` from this `dyn Registry`.
    ///
    /// ## Example
    /// ```
    /// # use charted::metrics::{Registry, registries::disabled::Disabled};
    /// # use std::sync::Arc;
    /// #
    /// // create a registry using `Arc` (this also works with `Box`).
    /// let registry: Arc<dyn Registry> = Arc::new(Disabled::default());
    ///
    /// // `downcast` uses Registry::is::<Disabled>() internally to check
    /// // if it is `Disabled`, the second assertion will also be true.
    /// assert!(registry.downcast::<Disabled>().is_some());
    /// assert!(registry.is::<Disabled>());
    /// ```
    pub fn downcast<T: Registry>(&self) -> Option<&T> {
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
        unsafe { &*(self as *const dyn Registry as *const T) }
    }
}

/// Creates a new [`Registry`] for metrics to take place.
pub fn new(config: &Config) -> Arc<dyn Registry> {
    let mut registry: Arc<dyn Registry> = match (config.metrics.enabled, config.metrics.prometheus) {
        (true, true) => Arc::new(Prometheus::new(Box::<Default>::default(), None)),
        (true, false) => Arc::new(Default::default()),
        (false, true) => Arc::new(Prometheus::new(Box::<Disabled>::default(), None)),
        (false, false) => Arc::new(Disabled::default()),
    };

    #[cfg(tokio_unstable)]
    let collectors: [Box<dyn Collector>; 4] = [
        Box::new(crate::metrics::collectors::process::ProcessCollector),
        Box::new(crate::metrics::collectors::server::ServerMetricsCollector),
        Box::new(crate::metrics::collectors::tokio::TokioCollector),
        Box::new(crate::metrics::collectors::os::OperatingSystemCollector),
    ];

    #[cfg(not(tokio_unstable))]
    let collectors: [Box<dyn Collector>; 3] = [
        Box::new(crate::metrics::collectors::process::ProcessCollector),
        Box::new(crate::metrics::collectors::server::ServerMetricsCollector),
        Box::new(crate::metrics::collectors::os::OperatingSystemCollector),
    ];

    for collector in collectors.into_iter() {
        Arc::get_mut(&mut registry).unwrap().insert(collector);
    }

    registry
}

#[cfg(test)]
mod tests {
    use super::{
        registries::{default::Default, disabled::Disabled},
        Registry,
    };
    use std::sync::Arc;

    #[test]
    fn test_dyn_registry_is() {
        let registry: Box<dyn Registry> = Box::<Disabled>::default();
        assert!(registry.is::<Disabled>());
        assert!(!registry.is::<Default>());
    }

    #[test]
    fn test_dyn_registry_downcast() {
        let registry: Box<dyn Registry> = Box::<Disabled>::default();
        assert!(registry.downcast::<Disabled>().is_some());
        assert!(registry.downcast::<Default>().is_none());
    }

    #[test]
    fn test_dyn_registry_downcast_arc() {
        let registry: Arc<dyn Registry> = Arc::<Disabled>::default();
        assert!(registry.downcast::<Disabled>().is_some());
        assert!(registry.downcast::<Default>().is_none());
    }
}
