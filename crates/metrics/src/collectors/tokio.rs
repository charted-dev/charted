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
use prometheus_client::{
    metrics::counter::ConstCounter,
    registry::{Descriptor, LocalMetric, Prefix},
    MaybeOwned,
};
use std::{any::Any, borrow::Cow, collections::HashMap};
use tokio::runtime::Handle;

#[derive(Debug, Clone, Copy, Default)]
pub struct TokioCollector;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokioMetrics {
    num_workers: usize,
    blocking_threads: usize,
    active_tasks: usize,
    file_descriptors: u64,
    workers: HashMap<usize, TokioWorkerMetrics>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokioWorkerMetrics {
    poll_count: u64,
    total_busy_duration: charted_common::serde::duration::Duration,
}

impl Collector for TokioCollector {
    fn name(&self) -> &'static str {
        "tokio"
    }

    #[cfg(tokio_unstable)]
    fn collect(&self) -> Box<dyn Any> {
        let handle = Handle::current();
        let metrics = handle.metrics();

        Box::new(TokioMetrics {
            file_descriptors: metrics.io_driver_fd_registered_count() - metrics.io_driver_fd_deregistered_count(),
            blocking_threads: metrics.num_blocking_threads(),
            active_tasks: metrics.active_tasks_count(),
            num_workers: metrics.num_workers(),
            workers: {
                let mut map = HashMap::new();
                for worker in 0..metrics.num_workers() {
                    map.insert(
                        worker,
                        TokioWorkerMetrics {
                            total_busy_duration: charted_common::serde::duration::Duration(
                                metrics.worker_total_busy_duration(worker),
                            ),
                            poll_count: metrics.worker_poll_count(worker),
                        },
                    );
                }

                map
            },
        })
    }

    #[cfg(not(tokio_unstable))]
    fn collect(&self) -> Box<dyn Any> {
        unreachable!()
    }

    #[cfg(tokio_unstable)]
    fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize> {
        let handle = Handle::current();
        let metrics = handle.metrics();

        Box::new(TokioMetrics {
            file_descriptors: metrics.io_driver_fd_registered_count() - metrics.io_driver_fd_deregistered_count(),
            blocking_threads: metrics.num_blocking_threads(),
            active_tasks: metrics.active_tasks_count(),
            num_workers: metrics.num_workers(),
            workers: {
                let mut map = HashMap::new();
                for worker in 0..metrics.num_workers() {
                    map.insert(
                        worker,
                        TokioWorkerMetrics {
                            total_busy_duration: charted_common::serde::duration::Duration(
                                metrics.worker_total_busy_duration(worker),
                            ),
                            poll_count: metrics.worker_poll_count(worker),
                        },
                    );
                }

                map
            },
        })
    }

    #[cfg(not(tokio_unstable))]
    fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize> {
        unreachable!()
    }
}

impl prometheus_client::collector::Collector for TokioCollector {
    #[allow(clippy::type_complexity)]
    fn collect<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (Cow<'a, Descriptor>, MaybeOwned<'a, Box<dyn LocalMetric>>)> + 'a> {
        let original = <Self as Collector>::collect(self);
        let metrics = original.downcast_ref::<TokioMetrics>().unwrap();

        let mut descriptors: Vec<(Cow<'a, Descriptor>, MaybeOwned<'a, Box<dyn LocalMetric>>)> = vec![
            create_metric_descriptor(
                Cow::Owned(Descriptor::new(
                    "tokio_io_file_descriptors",
                    "Total file descriptors that the Tokio I/O driver has used",
                    None,
                    Some(&Prefix::from(String::from("charted"))),
                    vec![],
                )),
                MaybeOwned::Owned(Box::new(ConstCounter::new(metrics.file_descriptors))),
            ),
            create_metric_descriptor(
                Cow::Owned(Descriptor::new(
                    "tokio_blocking_threads",
                    "Total blocking threads in the current Tokio runtime",
                    None,
                    Some(&Prefix::from(String::from("charted"))),
                    vec![],
                )),
                MaybeOwned::Owned(Box::new(ConstCounter::new(metrics.blocking_threads as u64))),
            ),
            create_metric_descriptor(
                Cow::Owned(Descriptor::new(
                    "tokio_active_threads",
                    "Total amount of active tasks the runtime is holding onto",
                    None,
                    Some(&Prefix::from(String::from("charted"))),
                    vec![],
                )),
                MaybeOwned::Owned(Box::new(ConstCounter::new(metrics.active_tasks as u64))),
            ),
            create_metric_descriptor(
                Cow::Owned(Descriptor::new(
                    "tokio_workers",
                    "Total count of workers available",
                    None,
                    Some(&Prefix::from(String::from("charted"))),
                    vec![],
                )),
                MaybeOwned::Owned(Box::new(ConstCounter::new(metrics.num_workers as u64))),
            ),
        ];

        for (worker, metric) in metrics.workers.clone().iter() {
            // TODO(@auguwu): implement TokioWorkerMetrics::total_busy_duration
            descriptors.push(create_metric_descriptor(
                Cow::Owned(Descriptor::new(
                    "tokio_worker_poll_count",
                    "Amount of poll events this specific Tokio worker has polled",
                    None,
                    Some(&Prefix::from(String::from("charted"))),
                    vec![(Cow::Borrowed("worker"), Cow::Owned(worker.to_string()))],
                )),
                MaybeOwned::Owned(Box::new(ConstCounter::new(metric.poll_count))),
            ));
        }

        Box::new(descriptors.into_iter())
    }
}
