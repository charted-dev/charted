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
use std::fmt::Debug;

#[derive(Clone, Default)]
pub struct DefaultRegistry {
    collectors: Vec<Box<dyn Collector>>,
}

impl Debug for DefaultRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultRegistry")
            .field("collectors", &self.collectors.len())
            .finish()
    }
}

impl Registry for DefaultRegistry {
    fn insert(&mut self, collector: Box<dyn Collector>) {
        self.collectors.push(collector);
    }

    fn collectors(&self) -> Vec<Box<dyn Collector>> {
        self.collectors.clone()
    }
}
