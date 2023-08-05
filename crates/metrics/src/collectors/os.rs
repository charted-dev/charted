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

use crate::{prometheus::create_metric_descriptor, Collector};
use erased_serde::Serialize;
use prometheus_client::{
    metrics::gauge::ConstGauge,
    registry::{Descriptor, Prefix},
    MaybeOwned,
};
use std::{any::Any, borrow::Cow};

#[derive(Debug, Clone, Copy, Default)]
pub struct OperatingSystemCollector;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystemMetrics<'a> {
    arch: &'a str,
    name: &'a str,
}

impl Collector for OperatingSystemCollector {
    fn name(&self) -> &'static str {
        "os"
    }

    fn collect(&self) -> Box<dyn Any> {
        Box::new(OperatingSystemMetrics {
            arch: charted_common::os::architecture(),
            name: charted_common::os::os_name(),
        })
    }

    fn collect_serialized(&self) -> Box<dyn Serialize> {
        Box::new(OperatingSystemMetrics {
            arch: charted_common::os::architecture(),
            name: charted_common::os::os_name(),
        })
    }
}

impl prometheus_client::collector::Collector for OperatingSystemCollector {
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
        // SAFETY: We know that OperatingSystemCollector::collect(self) implements
        // OperatingSystemMetrics.
        let original_metrics = <Self as Collector>::collect(self);
        let metrics = original_metrics.downcast_ref::<OperatingSystemMetrics<'_>>().unwrap();

        Box::new(
            [
                create_metric_descriptor(
                    Cow::Owned(Descriptor::new(
                        "os_name",
                        "Returns the OS name for this system",
                        None,
                        Some(&Prefix::from(String::from("charted"))),
                        vec![(Cow::Owned("name".to_owned()), Cow::Owned(metrics.name.to_owned()))],
                    )),
                    MaybeOwned::Owned(Box::new(ConstGauge::new(1))),
                ),
                create_metric_descriptor(
                    Cow::Owned(Descriptor::new(
                        "os_arch",
                        "Returns the OS architecture for this system",
                        None,
                        Some(&Prefix::from(String::from("charted"))),
                        vec![(
                            Cow::Owned("distribution".to_owned()),
                            Cow::Owned(metrics.arch.to_owned()),
                        )],
                    )),
                    MaybeOwned::Owned(Box::new(ConstGauge::new(1))),
                ),
            ]
            .into_iter(),
        )
    }
}
