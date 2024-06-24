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

use crate::{encode_counter, Collector};
use azalia::hashmap;
use charted_common::serde::Duration;
use prometheus_client::{
    encoding::{DescriptorEncoder, EncodeMetric},
    metrics::{counter::ConstCounter, gauge::ConstGauge, MetricType},
    registry::Unit,
};
use serde::Serialize;
use std::{any::Any, collections::HashMap, fmt::Error};
use tokio::runtime::Handle;

#[derive(Debug, Default)]
pub struct TokioCollector;

#[derive(Debug, Clone, Serialize)]
pub struct TokioMetrics {
    blocking_threads: usize,
    file_descriptors: u64,
    active_tasks: usize,
    workers: HashMap<usize, WorkerMetrics>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkerMetrics {
    total_busy_duration: Duration,
    poll_count: u64,
}

impl Collector for TokioCollector {
    fn name(&self) -> &'static str {
        "tokio"
    }

    fn collect(&self) -> Box<dyn Any> {
        let handle = Handle::current();
        let metrics = handle.metrics();

        Box::new(TokioMetrics {
            file_descriptors: metrics.io_driver_fd_registered_count() - metrics.io_driver_fd_deregistered_count(),
            blocking_threads: metrics.num_blocking_threads(),
            active_tasks: metrics.active_tasks_count(),
            workers: {
                let mut h = hashmap!(usize, WorkerMetrics);
                for worker in 0..metrics.num_workers() {
                    h.insert(
                        worker,
                        WorkerMetrics {
                            total_busy_duration: Duration(metrics.worker_total_busy_duration(worker)),
                            poll_count: metrics.worker_poll_count(worker),
                        },
                    );
                }

                h
            },
        })
    }

    fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize> {
        let handle = Handle::current();
        let metrics = handle.metrics();

        Box::new(TokioMetrics {
            file_descriptors: metrics.io_driver_fd_registered_count() - metrics.io_driver_fd_deregistered_count(),
            blocking_threads: metrics.num_blocking_threads(),
            active_tasks: metrics.active_tasks_count(),
            workers: {
                let mut h = hashmap!(usize, WorkerMetrics);
                for worker in 0..metrics.num_workers() {
                    h.insert(
                        worker,
                        WorkerMetrics {
                            total_busy_duration: Duration(metrics.worker_total_busy_duration(worker)),
                            poll_count: metrics.worker_poll_count(worker),
                        },
                    );
                }

                h
            },
        })
    }
}

impl prometheus_client::collector::Collector for TokioCollector {
    fn encode(&self, mut encoder: DescriptorEncoder) -> Result<(), Error> {
        let original = <Self as Collector>::collect(self);
        let metrics = original.downcast_ref::<TokioMetrics>().unwrap();

        encode_counter(
            &mut encoder,
            ConstCounter::new(metrics.blocking_threads as u64),
            |encoder| {
                encoder.encode_descriptor(
                    "charted_runtime_blocking_threads",
                    "number of additional threads spawned by the Tokio runtime",
                    None,
                    MetricType::Counter,
                )
            },
            |counter, encoder| counter.encode(encoder),
        )?;

        encode_counter(
            &mut encoder,
            ConstCounter::new(metrics.file_descriptors),
            |encoder| {
                encoder.encode_descriptor(
                    "charted_runtime_file_descriptors",
                    "number of file descriptors the runtime's I/O driver has",
                    None,
                    MetricType::Counter,
                )
            },
            |counter, encoder| counter.encode(encoder),
        )?;

        encode_counter(
            &mut encoder,
            ConstCounter::new(metrics.active_tasks as u64),
            |encoder| {
                encoder.encode_descriptor(
                    "charted_runtime_active_tasks",
                    "number of active tasks in the runtime",
                    None,
                    MetricType::Counter,
                )
            },
            |counter, encoder| counter.encode(encoder),
        )?;

        for (worker, metrics) in &metrics.workers {
            ConstGauge::<i64>::new(metrics.total_busy_duration.as_secs().try_into().unwrap()).encode(
                encoder.encode_descriptor(
                    &format!("charted_worker_{worker}_total_busy_duration"),
                    "returns the amount (in seconds) of time the given worker thread has been busy",
                    Some(&Unit::Seconds),
                    MetricType::Gauge,
                )?,
            )?;

            ConstCounter::new(metrics.poll_count).encode(encoder.encode_descriptor(
                &format!("charted_worker_{worker}_poll_count"),
                "returns the number of tasks the given worker thread has polled.",
                None,
                MetricType::Counter,
            )?)?;
        }

        Ok(())
    }
}
