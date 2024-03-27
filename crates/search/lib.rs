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

pub use charted_proc_macros::{Indexable, Searchable};

use async_trait::async_trait;
use std::borrow::Cow;

/// Represents a trait that allows to be indexed into a search backend. It must implement [`Serialize`] as
/// a backend will take a `dyn Indexable` to allow indexing.
pub trait Indexable: erased_serde::Serialize + Send {
    /// Returns the name of the index that this object should be in
    fn index(&self) -> &'static str;

    /// Returns the `id` field that this [`Indexable`] object can be used to delete
    /// or patch the entity in the search backend.
    fn id(&self) -> &'static str {
        "id"
    }
}

/// Represents a trait that is allowed to be searchable. It must implement the [`Indexable`]
/// trait as we need to know the index ahead of time.
pub trait Searchable: Indexable + Send {
    /// Returns an immutable slice that contains the allowed fields that this [`Searchable`] object
    /// can contain.
    fn allowed_fields<'s>(&self) -> &'s [&'s str];
}

/// Represents a backend that will allow to send requests with full-text search capabilities.
#[async_trait]
pub trait Backend: Send + Sync {
    /// Performs a search on a [`Indexable`] object based off the `query`, which will return something
    /// that is serializable.
    async fn search(
        &self,
        index: Cow<'static, str>,
        query: Cow<'static, str>,
    ) -> eyre::Result<&dyn erased_serde::Serialize>;

    /// Deletes a [`Indexable`] object from the search database.
    async fn delete(&self, obj: &dyn Indexable) -> eyre::Result<()>;

    /// Indexes a single [`Searchable`] object.
    async fn index(&self, obj: &dyn Searchable) -> eyre::Result<()>;
}
