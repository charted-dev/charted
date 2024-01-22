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

pub mod collectors;
pub mod registries;

use std::any::Any;

use crate::common::AsArcAny;

/// The [`Collector`] abstraction allows you to wrap `Serialize`-impl structs
/// and wrap it around the Admin API, where statistics about the server is running
/// can be collected.
///
/// This can also be wrapped around a Prometheus collector (`prometheus_client::collector::Collector`)
/// to provide Prometheus metrics for this collector. This can be only available via the PrometheusMetricRegistry
/// implementation.
pub trait Collector: Send + Sync {
    /// Returns the name of this [`Collector`].
    fn name(&self) -> &'static str;

    /// Collects all the data and returns anything.
    fn collect(&self) -> Box<dyn Any>;

    /// Collects all data, but returns `Serialize`-trait object.
    fn collect_serialized(&self) -> Box<dyn erased_serde::Serialize>;
}

/// Represents a collection of [`Collector`]s.
pub trait Registry: AsArcAny + Send + Sync {
    /// Inserts a new [`Collector`] into this registry.
    fn insert(&mut self, collector: Box<dyn Collector>);

    /// Returns all of the collectors that this [`Registry`] owns.
    fn collectors(&self) -> &Vec<Box<dyn Collector>>;
}
