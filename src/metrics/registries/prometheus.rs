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

use crate::metrics::Registry;
use prometheus_client::encoding::text;
use std::{
    fmt::Write,
    sync::{Arc, RwLock},
};

pub struct Prometheus {
    registry: Arc<RwLock<prometheus_client::registry::Registry>>,
    inner: Box<dyn Registry>,
}

impl Prometheus {
    /// Creates a new [`Prometheus`] registry.
    pub fn new(registry: Box<dyn Registry>, prom: Option<prometheus_client::registry::Registry>) -> Prometheus {
        Prometheus {
            registry: Arc::new(RwLock::new(prom.unwrap_or_default())),
            inner: registry,
        }
    }

    /// Registers a Prometheus collector into the Prometheus registry that this collector
    /// registry holds. This won't do anything if the registry lock couldn't be locked.
    ///
    /// ## Arguments
    /// - `collector`: The [`prometheus_client::collector::Collector`] to insert into
    /// the inner Prometheus registry.
    pub fn register_collector(&self, collector: Box<dyn prometheus_client::collector::Collector>) {
        if let Ok(mut registry) = self.registry.try_write() {
            registry.register_collector(collector);
        }
    }

    /// Writes the Prometheus metrics in `buf`.
    ///
    /// ## Arguments
    /// - `buf`: [`Write`]-implemented value to write the contained metrics in.
    ///
    /// ## Returns
    /// [`Result`] variant that returns a unit, indicating success. Otherwise, this can happen when the
    /// [`text::encode`] method fails.
    pub fn write_to<W: Write>(&self, buf: &mut W) -> Result<(), std::fmt::Error> {
        let registry = self.registry.read().unwrap();
        text::encode(buf, &registry)
    }
}

impl Registry for Prometheus {
    fn insert(&mut self, collector: Box<dyn crate::metrics::Collector>) {
        self.inner.insert(collector);
    }

    fn collectors(&self) -> &Vec<Box<dyn crate::metrics::Collector>> {
        self.inner.collectors()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{registries::disabled::Disabled, Collector};
    use prometheus_client::{
        encoding::EncodeMetric,
        metrics::{counter::ConstCounter, MetricType},
    };

    #[derive(Debug)]
    pub struct MyCollector(u64);
    impl Collector for MyCollector {
        fn name(&self) -> &'static str {
            "collector"
        }

        fn collect(&self) -> Box<dyn std::any::Any> {
            Box::new(self.0)
        }

        fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize> {
            Box::new(self.0)
        }
    }

    impl prometheus_client::collector::Collector for MyCollector {
        fn encode(&self, mut encoder: prometheus_client::encoding::DescriptorEncoder) -> Result<(), std::fmt::Error> {
            ConstCounter::new(self.0 + 1).encode(encoder.encode_descriptor(
                "counter",
                "This is a counter",
                None,
                MetricType::Counter,
            )?)
        }
    }

    #[test]
    fn test_registry() {
        let mut registry = Prometheus::new(Box::<Disabled>::default(), None);
        {
            let lock = registry.registry.clone();
            let guard = lock.write();
            assert!(guard.is_ok());

            let mut guard = guard.unwrap();
            registry.insert(Box::new(MyCollector(0)));
            guard.register_collector(Box::new(MyCollector(0)));
        }

        let mut buf = String::new();
        registry.write_to(&mut buf).unwrap();

        assert_eq!(
            "# HELP counter This is a counter
# TYPE counter counter
counter_total 1
# EOF
",
            buf
        );
    }
}
