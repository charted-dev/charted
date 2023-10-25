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

use crate::Collector;
use erased_serde::Serialize;
use prometheus_client::{
    encoding::EncodeMetric,
    metrics::{gauge::ConstGauge, MetricType},
};
use std::any::Any;

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
    fn encode(&self, mut encoder: prometheus_client::encoding::DescriptorEncoder) -> Result<(), std::fmt::Error> {
        // SAFETY: We know that OperatingSystemCollector returns `OperatingSystemMetrics`
        // when called via `collect(&self)`
        let original = <Self as Collector>::collect(self);
        let metrics = original.downcast_ref::<OperatingSystemMetrics<'_>>().unwrap();

        {
            let gauge = ConstGauge::new(1i64);
            let mut encoder =
                encoder.encode_descriptor("charted_os_name", "operating system name", None, MetricType::Gauge)?;

            gauge.encode(encoder.encode_family(&[("name", metrics.name)])?)?;
        }

        {
            let gauge = ConstGauge::new(1i64);
            let mut encoder = encoder.encode_descriptor(
                "charted_os_arch",
                "operating system architecture",
                None,
                MetricType::Gauge,
            )?;

            gauge.encode(encoder.encode_family(&[("architecture", metrics.arch)])?)?;
        }

        Ok(())
    }
}
