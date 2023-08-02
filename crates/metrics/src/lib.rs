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

pub mod collectors;
mod registry;

pub use registry::*;

use dyn_clone::{clone_trait_object, DynClone};
use erased_serde::Serialize;
use std::fmt::Debug;

/// The [`Collector`] abstraction allows you to wrap `Serialize`-impl structs
/// and wrap it around the Admin API, where statistics about the server is running
/// can be collected.
///
/// This can also be wrapped around a Prometheus collector (`prometheus_client::collector::Collector`)
/// to provide Prometheus metrics for this collector. This can be only available via the PrometheusMetricRegistry
/// implementation.
pub trait Collector: DynClone + Send + Sync {
    /// The name for this Collector.
    fn name(&self) -> &'static str;

    /// Collects all the data and returns a `Serialize`-trait bound thing.
    fn collect(&self) -> Box<dyn Serialize>;
}

impl Debug for dyn Collector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn Collector").finish_non_exhaustive()
    }
}

clone_trait_object!(Collector);

/// Represents a registry of [`Collector`]s that can be easily queried.
pub trait Registry: DynClone {
    /// Inserts a new [`Collector`] into this registry.
    fn insert(&mut self, collector: Box<dyn Collector>);

    /// Returns all of the collectors that this [`Registry`] owns.
    fn collectors(&self) -> Vec<Box<dyn Collector>>;
}

clone_trait_object!(Registry);
