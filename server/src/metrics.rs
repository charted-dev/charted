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

use charted_common::{models::Distribution, COMMIT_HASH, VERSION};
use charted_metrics::{prometheus::create_metric_descriptor, Collector};
use erased_serde::Serialize as ErasedSerialize;
use prometheus_client::{
    metrics::gauge::ConstGauge,
    registry::{Descriptor, LocalMetric, Prefix},
    MaybeOwned,
};
use serde::{Deserialize, Serialize};
use std::{any::Any, borrow::Cow, sync::atomic::Ordering};

use crate::SERVER;

/// VENDOR is the current vendor of charted-server. If you're making a fork,
/// you can set this to whatever you want. We don't mind! :)
const VENDOR: &str = "Noelware, LLC.";

#[derive(Debug, Clone, Copy, Default)]
pub struct ServerMetricsCollector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMetrics {
    distribution: Distribution,
    commit_hash: &'static str,
    requests: usize,
    version: &'static str,
    vendor: &'static str,
}

impl Collector for ServerMetricsCollector {
    fn name(&self) -> &'static str {
        "server"
    }

    fn collect(&self) -> Box<dyn Any> {
        let server = SERVER.get().unwrap();

        Box::new(ServerMetrics {
            distribution: Distribution::detect(),
            commit_hash: COMMIT_HASH,
            requests: server.requests.load(Ordering::SeqCst),
            version: VERSION,
            vendor: VENDOR,
        })
    }

    fn collect_serialized(&self) -> Box<dyn ErasedSerialize> {
        let server = SERVER.get().unwrap();

        Box::new(ServerMetrics {
            distribution: Distribution::detect(),
            commit_hash: COMMIT_HASH,
            requests: server.requests.load(Ordering::SeqCst),
            version: VERSION,
            vendor: VENDOR,
        })
    }
}

impl prometheus_client::collector::Collector for ServerMetricsCollector {
    fn collect<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (Cow<'a, Descriptor>, MaybeOwned<'a, Box<dyn LocalMetric>>)> + 'a> {
        let original = <Self as Collector>::collect(self);
        let metrics = original.downcast_ref::<ServerMetrics>().unwrap();

        Box::new(
            [
                create_metric_descriptor(
                    Cow::Owned(Descriptor::new(
                        "distribution",
                        "Distribution kind",
                        None,
                        Some(&Prefix::from(String::from("charted"))),
                        vec![(
                            Cow::Borrowed("distribution"),
                            Cow::Owned(metrics.distribution.to_string()),
                        )],
                    )),
                    MaybeOwned::Owned(Box::new(ConstGauge::new(1))),
                ),
                create_metric_descriptor(
                    Cow::Owned(Descriptor::new(
                        "commit_hash",
                        "Git commit hash",
                        None,
                        Some(&Prefix::from(String::from("charted"))),
                        vec![(Cow::Borrowed("commit"), Cow::Borrowed(metrics.commit_hash))],
                    )),
                    MaybeOwned::Owned(Box::new(ConstGauge::new(1))),
                ),
                create_metric_descriptor(
                    Cow::Owned(Descriptor::new(
                        "requests",
                        "How many requests were collected",
                        None,
                        Some(&Prefix::from(String::from("charted"))),
                        vec![],
                    )),
                    MaybeOwned::Owned(Box::new(ConstGauge::new(metrics.requests as i64))),
                ),
                create_metric_descriptor(
                    Cow::Owned(Descriptor::new(
                        "version",
                        "Version of the API server",
                        None,
                        Some(&Prefix::from(String::from("charted"))),
                        vec![(Cow::Borrowed("version"), Cow::Borrowed(metrics.version))],
                    )),
                    MaybeOwned::Owned(Box::new(ConstGauge::new(1))),
                ),
                create_metric_descriptor(
                    Cow::Owned(Descriptor::new(
                        "vendor",
                        "Vendor for this software",
                        None,
                        Some(&Prefix::from(String::from("charted"))),
                        vec![(Cow::Borrowed("vendor"), Cow::Borrowed(metrics.vendor))],
                    )),
                    MaybeOwned::Owned(Box::new(ConstGauge::new(1))),
                ),
            ]
            .into_iter(),
        )
    }
}
