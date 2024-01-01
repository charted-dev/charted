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

use crate::Collector;
use erased_serde::Serialize;
use prometheus_client::{
    encoding::EncodeMetric,
    metrics::{counter::ConstCounter, MetricType},
};
use std::any::Any;

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessCollector;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessMetrics {
    id: u32,
}

impl Collector for ProcessCollector {
    fn name(&self) -> &'static str {
        "process"
    }

    fn collect(&self) -> Box<dyn Any> {
        Box::new(ProcessMetrics { id: std::process::id() })
    }

    fn collect_serialized(&self) -> Box<dyn Serialize> {
        Box::new(ProcessMetrics { id: std::process::id() })
    }
}

impl prometheus_client::collector::Collector for ProcessCollector {
    fn encode(&self, mut encoder: prometheus_client::encoding::DescriptorEncoder) -> Result<(), std::fmt::Error> {
        // SAFETY: We know that ProcessCollector returns `ProcessMetrics`
        // when called via `collect(&self)`
        let original = <Self as Collector>::collect(self);
        let metrics = original.downcast_ref::<ProcessMetrics>().unwrap();

        {
            let counter = ConstCounter::new(metrics.id as u64);
            counter.encode(encoder.encode_descriptor(
                "charted_process_pid",
                "current process id",
                None,
                MetricType::Counter,
            )?)?;
        }

        Ok(())
    }
}
