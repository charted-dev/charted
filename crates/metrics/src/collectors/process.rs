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

use crate::{registry::prometheus::create_metric_descriptor, Collector};
use charted_common::rust::Cast;
use erased_serde::Serialize;
use prometheus_client::{
    metrics::counter::ConstCounter,
    registry::{Descriptor, LocalMetric, Prefix},
    MaybeOwned,
};
use std::borrow::Cow;

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

    fn collect(&self) -> Box<dyn Serialize> {
        Box::new(ProcessMetrics { id: std::process::id() })
    }
}

impl prometheus_client::collector::Collector for ProcessCollector {
    fn collect<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (Cow<'a, Descriptor>, MaybeOwned<'a, Box<dyn LocalMetric>>)> + 'a> {
        let original_metrics = <Self as Collector>::collect(self);
        let metrics = original_metrics.cast::<ProcessMetrics>().unwrap();

        Box::new(
            [create_metric_descriptor(
                Cow::Owned(Descriptor::new(
                    "process_id",
                    "Returns the current process ID",
                    None,
                    Some(&Prefix::from(String::from("charted"))),
                    vec![],
                )),
                MaybeOwned::Owned(Box::new(ConstCounter::new(metrics.id as u64))),
            )]
            .into_iter(),
        )
    }
}
