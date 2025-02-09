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

//! Dependency Injection module.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};

#[derive(Debug, derive_more::Display)]
pub enum Error {
    #[display("object already exists")]
    ObjectAlreadyExists,

    #[display("failed to cast")]
    FailedCast,

    #[display("object is unavaliable")]
    ObjectUnavaliable,
}

impl std::error::Error for Error {}

/// A container that holds all the dependencies of the API server.
#[derive(Debug, Clone, Default)]
pub struct Container {
    objects: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    /// Installs a injectable of `I` into this container.
    pub fn install<I: Any + Send + Sync>(&mut self, object: I) -> Result<(), Error> {
        let id = TypeId::of::<I>();
        if self.objects.contains_key(&id) {
            return Err(Error::ObjectAlreadyExists);
        }

        self.objects.insert(id, Arc::new(object));
        Ok(())
    }

    /// Get a [`Injectable`] from this container, returns `None` if it can't be found.
    pub fn get<T: Any>(&self) -> Result<&T, Error> {
        self.objects
            .get(&TypeId::of::<T>())
            .ok_or(Error::FailedCast)?
            .downcast_ref()
            .ok_or(Error::ObjectUnavaliable)
    }
}
