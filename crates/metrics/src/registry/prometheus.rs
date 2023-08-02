// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use prometheus_client::{
    encoding::text,
    registry::{Descriptor, LocalMetric},
    MaybeOwned,
};

use crate::{Collector, Registry};
use std::{
    borrow::Cow,
    fmt::{Debug, Display, Write},
    sync::{Arc, Mutex},
};

/// Represents an error that can occur when [`PrometheusRegistry::write_metrics`] is called.
pub enum WriteError {
    /// Error occurs when the registry couldn't be unlocked.
    RegistryCannotBeUnlocked,

    /// Error when the [`text::encode`] method fails.
    Format(std::fmt::Error),
}

impl Debug for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::RegistryCannotBeUnlocked => write!(f, "registry couldn't be unlocked"),
            WriteError::Format(err) => write!(f, "write failed: {err}"),
        }
    }
}

impl Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::RegistryCannotBeUnlocked => write!(f, "registry couldn't be unlocked"),
            WriteError::Format(err) => write!(f, "write failed: {err}"),
        }
    }
}

impl std::error::Error for WriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WriteError::RegistryCannotBeUnlocked => None,
            WriteError::Format(err) => Some(err),
        }
    }
}

impl From<std::fmt::Error> for WriteError {
    fn from(value: std::fmt::Error) -> WriteError {
        WriteError::Format(value)
    }
}

#[derive(Clone)]
pub struct PrometheusRegistry {
    registry: Arc<Mutex<prometheus_client::registry::Registry>>,
    inner: Box<dyn Registry>,
}

impl PrometheusRegistry {
    /// Returns a Option variant that can contain a reference to the Mutex guard that
    /// contains the [Prometheus registry][prometheus_client::registry::Registry]. The `None` variant can only occur
    /// when the inner [`Mutex`] is poisoned.
    ///
    /// Since this ultimately locks the inner Mutex, this will block the thread that is trying to access
    /// the [Prometheus registry][prometheus_client::registry::Registry]! The guard is dropped when you're
    /// done accessing it.
    ///
    /// ### Returns
    /// Option variant that might contain a reference to a [Prometheus registry][prometheus_client::registry::Registry].
    pub fn registry(&self) -> Arc<Mutex<prometheus_client::registry::Registry>> {
        self.registry.clone()
    }

    /// Registers a Prometheus collector into the Prometheus registry that this collector
    /// registry holds. This won't do anything if the registry lock couldn't be locked.
    ///
    /// ## Arguments
    /// - `collector`: The [`prometheus_client::collector::Collector`] to insert into
    /// the inner Prometheus registry.
    pub fn register_collector(&self, collector: Box<dyn prometheus_client::collector::Collector>) {
        if let Ok(mut registry) = self.registry.try_lock() {
            registry.register_collector(collector);
        }
    }

    /// Returns a cloned value of the underlying [`Registry`] that this Prometheus
    /// registry is taking over.
    pub fn inner(&self) -> Box<dyn Registry> {
        self.inner.clone()
    }

    /// Same as [`PrometheusRegistry::inner`], but returns a mutable reference
    /// of the underlying [`Registry`].
    pub fn inner_mut(&mut self) -> &mut Box<dyn Registry> {
        &mut self.inner
    }

    /// Writes the Prometheus metrics in `buf`.
    ///
    /// ## Arguments
    /// - `buf`: [`Write`]-implemented value to write the contained metrics in.
    ///
    /// ## Returns
    /// [`Result`] variant that returns a unit, indicating success. Otherwise,
    /// a [`WriteError`] can occur. This can happen when the [`text::encode`] method
    /// fails or if the registry couldn't be unlocked.
    pub fn write_metrics<W: Write>(&self, buf: &mut W) -> Result<(), WriteError> {
        if let Ok(prometheus) = self.registry.lock() {
            text::encode(buf, &prometheus)?;
            return Ok(());
        }

        Err(WriteError::RegistryCannotBeUnlocked)
    }
}

impl Registry for PrometheusRegistry {
    fn insert(&mut self, collector: Box<dyn Collector>) {
        self.inner_mut().insert(collector);
    }

    fn collectors(&self) -> Vec<Box<dyn Collector>> {
        self.inner.collectors()
    }
}

/// Constructs a new [`PrometheusRegistry`].
///
/// ### Type Arguments
/// - `I`: Inner [`Registry`] implementation.
///
/// ### Arguments
/// - `registry`: Inner [`Registry`] implementation to implement [`Registry::collect`].
/// - `prom_registry`: Option variant to use a [`prometheus_client::registry::Registry`] instead
/// of the default one, which will use `Default::default`.
///
/// ### Returns
/// A [`Registry`] implementation that allows you to bind [`Collector`]s with Prometheus'
/// [collector abstraction][prometheus_client::collector::Collector].
///
/// ## Examples
/// ```
/// # use charted_metrics::{prometheus::new, disabled::DisabledRegistry};
/// # use charted_metrics::Registry;
/// #
/// let registry = new(Box::new(DisabledRegistry), None);
/// assert_eq!(registry.collectors().len(), 0);
/// ```
pub fn new(
    registry: Box<dyn Registry>,
    prom_registry: Option<prometheus_client::registry::Registry>,
) -> PrometheusRegistry {
    PrometheusRegistry {
        registry: Arc::new(Mutex::new(
            prom_registry.map_or_else(prometheus_client::registry::Registry::default, |a| a),
        )),
        inner: registry,
    }
}

/// Method to properly cast `descriptor` and `metric` into a tuple that Prometheus'
/// collector abstraction can support. Doing this when using `Box::new([...].into_iter())`
/// will lose the lifetime reference (`'a`), and will be `'_`.
///
/// ## Why?
/// For collectors to be implemented, we want to create Prometheus supported
/// collectors, and writing something like:
///
/// ```no_run
/// let _: (Cow<'a, Descriptor>, MaybeOwned<'a, Box<dyn LocalMetric>>) = (
///     ...
/// );
/// ```
///
/// can be pretty daunting, so this is just a utility method to do so!
///
/// ### Arguments
/// - `descriptor`: [Clone-on-write][std::borrow::Cow] variant.
/// - `metric`:     [MaybeOwned] variant to a dynamic local metric.
pub fn create_metric_descriptor<'a>(
    descriptor: Cow<'a, Descriptor>,
    metric: MaybeOwned<'a, Box<dyn LocalMetric>>,
) -> (Cow<'a, Descriptor>, MaybeOwned<'a, Box<dyn LocalMetric>>) {
    (descriptor, metric)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disabled::DisabledRegistry;
    use prometheus_client::{
        metrics::counter::ConstCounter,
        registry::{Descriptor, LocalMetric},
        MaybeOwned,
    };
    use std::borrow::Cow;

    #[derive(Debug, Clone, Copy)]
    pub struct MyCollector(u64);

    impl Collector for MyCollector {
        fn name(&self) -> &'static str {
            "collector"
        }

        fn collect(&self) -> Box<dyn erased_serde::Serialize> {
            Box::new(self.0)
        }
    }

    impl prometheus_client::collector::Collector for MyCollector {
        fn collect<'a>(
            &'a self,
        ) -> Box<
            dyn Iterator<
                    Item = (
                        std::borrow::Cow<'a, prometheus_client::registry::Descriptor>,
                        prometheus_client::MaybeOwned<'a, Box<dyn prometheus_client::registry::LocalMetric>>,
                    ),
                > + 'a,
        > {
            let c: Box<dyn LocalMetric> = Box::new(ConstCounter::new(self.0 + 1));
            let descriptor = Descriptor::new("counter", "This is a counter", None, None, vec![]);

            Box::new(std::iter::once((Cow::Owned(descriptor), MaybeOwned::Owned(c))))
        }
    }

    #[test]
    fn test_registry() {
        let mut registry = new(Box::<DisabledRegistry>::default(), None);

        {
            let prom_registry = registry.registry();
            let guard = prom_registry.lock();
            assert!(guard.is_ok());

            let mut guard = guard.unwrap();
            let collector = MyCollector(0);
            registry.insert(Box::new(collector));
            guard.register_collector(Box::new(collector));
        }

        let mut buf = String::new();
        registry.write_metrics(&mut buf).unwrap();

        let expected = "# HELP counter This is a counter.
# TYPE counter counter
counter_total 1
# EOF
";

        assert_eq!(expected, buf.as_str());
    }
}
