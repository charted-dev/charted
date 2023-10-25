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

use crate::Registry;

/// Represents a disabled [`Registry`] where no collectors can be registered
/// and collect metrics.
#[derive(Debug, Clone, Copy, Default)]
pub struct DisabledRegistry;

impl Registry for DisabledRegistry {
    fn insert(&mut self, _collector: Box<dyn crate::Collector>) {}
    fn collectors(&self) -> Vec<Box<dyn crate::Collector>> {
        vec![]
    }
}
