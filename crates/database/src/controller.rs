// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

pub trait Controller: Send + Sync {
    /// Payload for creating a `Entity`.
    type CreatePayload;

    /// Payload for patching a `Entity`.
    type PatchPayload;

    /// Entity type itself.
    type Entity;
}

/// A registry of [`Controller`]s.
#[derive(Debug, Clone)]
pub struct Registry {
    // `dyn Any` is used instead of `dyn Controller` is because we need the
    // GATs inlined and we can't do that since they can be different types.
    registered: HashMap<TypeId, Arc<dyn Any + Send + Sync + 'static>>,
}

impl Registry {
    pub fn insert<T: Controller + 'static>(&mut self, controller: T) {
        self.registered.insert(controller.type_id(), Arc::new(controller));
    }

    pub fn get<T: Controller + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.registered.get(&type_id).and_then(|s| s.downcast_ref())
    }
}

#[cfg(test)]
mod tests {}
