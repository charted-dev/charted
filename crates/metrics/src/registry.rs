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

pub mod default;
pub mod disabled;
pub mod prometheus;

use charted_config::Config;

/// Represents a registry of [`Collector`]s that can be easily queried.
#[derive(Debug, Clone)]
pub enum Registry {
    Prometheus(prometheus::PrometheusRegistry),
    Disabled(disabled::DisabledRegistry),
    Default(default::DefaultRegistry),
}

impl crate::Registry for Registry {
    fn collectors(&self) -> Vec<Box<dyn crate::Collector>> {
        match self {
            Self::Prometheus(prom) => prom.collectors(),
            Self::Default(def) => def.collectors(),
            Self::Disabled(_) => vec![],
        }
    }

    fn insert(&mut self, collector: Box<dyn crate::Collector>) {
        match self {
            Self::Prometheus(prom) => prom.insert(collector),
            Self::Default(def) => def.insert(collector),
            Self::Disabled(_) => {}
        }
    }
}

impl Registry {
    pub fn configure(config: Config, extra: Vec<Box<dyn crate::Collector>>) -> Registry {
        match (config.metrics.enabled, config.metrics.prometheus) {
            (b, true) => {
                let mut registry = prometheus_client::registry::Registry::default();
                registry.register_collector(Box::<crate::collectors::os::OperatingSystemCollector>::default());
                registry.register_collector(Box::<crate::collectors::process::ProcessCollector>::default());

                #[cfg(tokio_unstable)]
                registry.register_collector(Box::<crate::collectors::tokio::TokioCollector>::default());

                let mut prom = prometheus::new(
                    match b {
                        true => Box::<default::DefaultRegistry>::default(),
                        false => Box::<disabled::DisabledRegistry>::default(),
                    },
                    Some(registry),
                );

                #[cfg(tokio_unstable)]
                register_metrics(&mut prom, {
                    let mut collectors: Vec<Box<dyn crate::Collector>> = vec![
                        Box::<crate::collectors::os::OperatingSystemCollector>::default(),
                        Box::<crate::collectors::process::ProcessCollector>::default(),
                        Box::<crate::collectors::tokio::TokioCollector>::default(),
                    ];

                    collectors.extend(extra);
                    collectors
                });

                #[cfg(not(tokio_unstable))]
                register_metrics(&mut prom, {
                    let mut collectors: Vec<Box<dyn crate::Collector>> = vec![
                        Box::<crate::collectors::os::OperatingSystemCollector>::default(),
                        Box::<crate::collectors::process::ProcessCollector>::default(),
                    ];

                    collectors.extend(extra);
                    collectors
                });

                Registry::Prometheus(prom)
            }

            (true, false) => {
                let mut registry = default::DefaultRegistry::default();
                #[cfg(tokio_unstable)]
                register_metrics(&mut registry, {
                    let mut collectors: Vec<Box<dyn crate::Collector>> = vec![
                        Box::<crate::collectors::os::OperatingSystemCollector>::default(),
                        Box::<crate::collectors::process::ProcessCollector>::default(),
                        Box::<crate::collectors::tokio::TokioCollector>::default(),
                    ];

                    collectors.extend(extra);
                    collectors
                });

                #[cfg(not(tokio_unstable))]
                register_metrics(&mut registry, {
                    let mut collectors: Vec<Box<dyn crate::Collector>> = vec![
                        Box::<crate::collectors::os::OperatingSystemCollector>::default(),
                        Box::<crate::collectors::process::ProcessCollector>::default(),
                    ];

                    collectors.extend(extra);
                    collectors
                });

                Registry::Default(registry)
            }
            (false, false) => Registry::Disabled(disabled::DisabledRegistry),
        }
    }
}

fn register_metrics(registry: &mut dyn crate::Registry, collectors: Vec<Box<dyn crate::Collector>>) {
    for collector in collectors.clone().into_iter() {
        registry.insert(collector);
    }
}
