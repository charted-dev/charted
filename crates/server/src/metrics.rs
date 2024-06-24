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

use crate::ServerContext;
use azalia::lazy;
use charted_entities::Distribution;
use charted_metrics::{encode_counter, encode_gauge, encode_histogram};
use once_cell::sync::Lazy;
use prometheus_client::{
    encoding::{DescriptorEncoder, EncodeMetric},
    metrics::{counter::ConstCounter, gauge::ConstGauge, histogram::Histogram, MetricType},
};
use serde::Serialize;
use std::{any::Any, sync::atomic::Ordering};

pub static REQUEST_LATENCY_HISTOGRAM: Lazy<Histogram> = lazy! {
    Histogram::new([0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0].into_iter())
};

/// Collection of all the API server metrics.
#[derive(Default, Serialize)]
pub struct Metrics {
    /// Distribution type.
    pub distribution: Distribution,

    /// How many requests the API server has served.
    pub requests: usize,

    /// Version of `charted`.
    pub version: &'static str,
}

/// [`charted_metrics::Collector`] implementation for the API server.
#[derive(Debug, Default)]
pub struct Collector;
impl charted_metrics::Collector for Collector {
    fn name(&self) -> &'static str {
        "server"
    }

    fn collect(&self) -> Box<dyn Any> {
        let ctx = ServerContext::get();
        Box::new(Metrics {
            distribution: Distribution::detect(),
            requests: ctx.requests.load(Ordering::Relaxed),
            version: charted_common::version(),
        })
    }

    fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize> {
        let ctx = ServerContext::get();
        Box::new(Metrics {
            distribution: Distribution::detect(),
            requests: ctx.requests.load(Ordering::Relaxed),
            version: charted_common::version(),
        })
    }
}

impl prometheus_client::collector::Collector for Collector {
    fn encode(&self, mut encoder: DescriptorEncoder) -> Result<(), std::fmt::Error> {
        let original = <Self as charted_metrics::Collector>::collect(self);
        let me = original.downcast_ref::<Metrics>().ok_or(std::fmt::Error)?;

        encode_gauge(
            &mut encoder,
            ConstGauge::new(1i64),
            |encoder| {
                encoder.encode_descriptor(
                    "charted_server_distribution",
                    "distribution type",
                    None,
                    MetricType::Gauge,
                )
            },
            |gauge, mut encoder| gauge.encode(encoder.encode_family(&[("distribution", me.distribution.to_string())])?),
        )?;

        encode_gauge(
            &mut encoder,
            ConstGauge::new(1i64),
            |encoder| {
                encoder.encode_descriptor(
                    "charted_server_version",
                    "version of the `charted` binary",
                    None,
                    MetricType::Gauge,
                )
            },
            |gauge, mut encoder| gauge.encode(encoder.encode_family(&[("version", me.version.to_owned())])?),
        )?;

        encode_counter(
            &mut encoder,
            ConstCounter::new(u64::try_from(me.requests).map_err(|_| std::fmt::Error)?),
            |encoder| {
                encoder.encode_descriptor(
                    "charted_server_requests",
                    "how many requests the server has hit",
                    None,
                    MetricType::Counter,
                )
            },
            |counter, encoder| counter.encode(encoder),
        )?;

        // deref op for `Lazy<T>` will initialize if it hasn't already
        let histogram = &*REQUEST_LATENCY_HISTOGRAM;
        encode_histogram(
            &mut encoder,
            histogram.clone(),
            |encoder| {
                encoder.encode_descriptor(
                    "charted_server_request_latency",
                    "latency on how long a request takes",
                    None,
                    MetricType::Histogram,
                )
            },
            |histogram, encoder| histogram.encode(encoder),
        )?;

        Ok(())
    }
}
