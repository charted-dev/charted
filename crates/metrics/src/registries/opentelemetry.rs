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

use crate::{Collector, Registry};
use opentelemetry::metrics::{Meter, MeterProvider};
use std::{borrow::Cow, ops::Deref};

pub struct OpenTelemetry {
    meter: Meter,
    inner: Box<dyn Registry>,
}

impl OpenTelemetry {
    /// Creates a new [`OpenTelemetry`] registry, where a [`MeterProvider`] can provide a [`Meter`] that
    /// this registry can create
    pub fn new<M: MeterProvider + 'static>(provider: &M, registry: Box<dyn Registry>) -> OpenTelemetry {
        OpenTelemetry {
            inner: registry,
            meter: provider.versioned_meter(
                "charted-server",
                Some(Cow::Borrowed("0.0.0-devel.0")),
                None::<Cow<'static, str>>,
                None,
            ),
        }
    }

    /// Returns a reference to a dynamic [`Registry`] that the OpenTelemetry registry owns.
    pub fn inner(&self) -> &dyn Registry {
        self.inner.as_ref()
    }
}

impl Deref for OpenTelemetry {
    type Target = Meter;

    fn deref(&self) -> &Self::Target {
        &self.meter
    }
}

impl Registry for OpenTelemetry {
    fn collectors(&self) -> &Vec<Box<dyn Collector>> {
        self.inner.collectors()
    }

    fn insert(&mut self, collector: Box<dyn Collector>) {
        self.inner.insert(collector);
    }
}
