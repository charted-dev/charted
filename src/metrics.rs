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

use crate::Instance;
use charted_common::{lazy, COMMIT_HASH, VERSION};
use charted_config::metrics::Config;
use charted_entities::Distribution;
use charted_metrics::{prometheus::Prometheus, Collector, Disabled, Minimal, Registry};
use once_cell::sync::Lazy;
use prometheus_client::{
    encoding::EncodeMetric,
    metrics::{counter::ConstCounter, gauge::ConstGauge, histogram::Histogram, MetricType},
};
use serde::Serialize;
use std::{
    any::Any,
    sync::{atomic::Ordering, Arc},
};

pub static REQUEST_LATENCY_HISTOGRAM: Lazy<Histogram> = lazy!(Histogram::new(IntoIterator::into_iter([
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0
])));

#[derive(Debug, Default)]
pub struct ServerMetricsCollector;

#[derive(Serialize)]
pub struct ServerMetrics {
    distribution: Distribution,
    commit_hash: &'static str,
    requests: usize,
    version: &'static str,
}

impl Collector for ServerMetricsCollector {
    fn name(&self) -> &'static str {
        "server"
    }

    fn collect(&self) -> Box<dyn Any> {
        let instance = Instance::get();

        Box::new(ServerMetrics {
            distribution: Distribution::detect(),
            commit_hash: COMMIT_HASH,
            requests: instance.requests.load(Ordering::SeqCst),
            version: VERSION,
        })
    }

    fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize> {
        let instance = Instance::get();

        Box::new(ServerMetrics {
            distribution: Distribution::detect(),
            commit_hash: COMMIT_HASH,
            requests: instance.requests.load(Ordering::SeqCst),
            version: VERSION,
        })
    }
}

impl prometheus_client::collector::Collector for ServerMetricsCollector {
    fn encode(&self, mut encoder: prometheus_client::encoding::DescriptorEncoder) -> Result<(), std::fmt::Error> {
        let original = <Self as Collector>::collect(self);
        let metrics = original.downcast_ref::<ServerMetrics>().unwrap();

        {
            let gauge = ConstGauge::new(1i64);
            let mut encoder = encoder.encode_descriptor(
                "charted_server_distribution",
                "distribution kind",
                None,
                MetricType::Gauge,
            )?;

            gauge.encode(encoder.encode_family(&[("distribution", metrics.distribution.to_string())])?)?;
        }

        {
            let gauge = ConstGauge::new(1i64);
            let mut encoder =
                encoder.encode_descriptor("charted_commit_hash", "git commit hash", None, MetricType::Gauge)?;

            gauge.encode(encoder.encode_family(&[("commit", metrics.commit_hash)])?)?;
        }

        {
            let gauge = ConstGauge::new(1i64);
            let mut encoder =
                encoder.encode_descriptor("charted_version", "charted version", None, MetricType::Gauge)?;

            gauge.encode(encoder.encode_family(&[("version", metrics.version)])?)?;
        }

        ConstCounter::new(u64::try_from(metrics.requests).unwrap()).encode(encoder.encode_descriptor(
            "charted_server_requests",
            "amount of requests that were received",
            None,
            MetricType::Counter,
        )?)?;

        {
            let encoder = encoder.encode_descriptor(
                "charted_server_request_latency",
                "latency between each request",
                None,
                MetricType::Histogram,
            )?;

            (*REQUEST_LATENCY_HISTOGRAM).encode(encoder)?;
        }

        Ok(())
    }
}

pub fn new(config: &Config) -> Arc<dyn Registry> {
    let mut registry: Arc<dyn Registry> = match (config.enabled, config.prometheus) {
        (true, true) => Arc::new(Prometheus::new(Box::<Minimal>::default(), None)),
        (true, false) => Arc::new(Minimal::default()),
        (false, true) => Arc::new(Prometheus::new(Box::<Disabled>::default(), None)),
        (false, false) => Arc::new(Disabled::default()),
    };

    #[cfg(tokio_unstable)]
    let collectors: [Box<dyn Collector>; 3] = [
        Box::new(charted_metrics::collectors::ProcessCollector),
        Box::new(charted_metrics::collectors::TokioCollector),
        Box::new(crate::metrics::ServerMetricsCollector),
    ];

    #[cfg(not(tokio_unstable))]
    let collectors: [Box<dyn Collector>; 2] = [
        Box::new(charted_metrics::collectors::ProcessCollector),
        Box::new(crate::metrics::ServerMetricsCollector),
    ];

    for collector in collectors.into_iter() {
        // `get_mut` works here since we don't have any previous `Arc`/`Weak` clones
        // since it was just initialised.
        Arc::get_mut(&mut registry).unwrap().insert(collector);
    }

    registry
}
