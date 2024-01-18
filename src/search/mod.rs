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

pub mod elasticsearch;
pub mod meilisearch;

use crate::common::models::entities::{Organization, Repository, User};
use serde::Serialize;

/// Represents a indexable object that can be indexed into a backend.
pub trait Indexable: Serialize {
    /// Returns the name of the index that this object should be indexable as.
    fn index(&self) -> &'static str;

    /// Returns the `id` field that this `Indexable` object can be deleted.
    fn id(&self) -> &'static str {
        "id"
    }
}

/// Represents a searchable object that can be searched through a backend.
pub trait Searchable: Indexable {
    /// List of allowed fields that this [`Searchable`] object can represent. This is useful
    /// for filter queries.
    fn allowed_fields(&self) -> &'static [&'static str];
}

/// Represents a single backend for a [`Searchable`] + [`Indexable`] object.
#[async_trait]
pub trait Backend<S: Searchable>: Send + Sync {
    /// Surf through [`S`] and try to find results that best match the query.
    async fn search(&self, query: &str, obj: S) -> eyre::Result<()>;

    /// Perform a single index on `obj`.
    async fn index(&self, obj: S) -> eyre::Result<()>;

    /// Do a bulk index on multiple `objs`.
    async fn bulk_index(&self, objs: Vec<S>) -> eyre::Result<()>;

    /// Deletes the `obj` from the search backend.
    async fn delete(&self, obj: S) -> eyre::Result<()>;
}

macro_rules! impl_searchable {
    ($($ty:ty[index: $index:literal$(, id: $id:literal)?] {
        fields: [$($field:literal),*];
    }),*) => {
        $(
            impl Indexable for $ty {
                fn index(&self) -> &'static str {
                    $index
                }

                $(
                    fn id(&self) -> &'static str {
                        $id
                    }
                )?
            }

            impl Searchable for $ty {
                fn allowed_fields(&self) -> &'static [&'static str] {
                    &[
                        $($field,)*
                    ]
                }
            }
        )*
    };
}

impl_searchable!(
    User[index: "users"] {
        fields: ["verified_publisher", "description", "created_at", "updated_at", "username", "name", "id"];
    },

    Repository[index: "repository"] {
        fields: ["description", "deprecated", "created_at", "owner", "name", "type", "id"];
    },

    Organization[index: "organization"] {
        fields: ["verified_publisher", "display_name", "created_at", "updated_at", "name", "id"];
    }
);

/// Represents a joined backend that supports all the available [`Indexable`] objects at once.
pub struct JoinedBackend {
    /// Represents the backend for the [`Organization`] index.
    pub organizations: Box<dyn Backend<Organization>>,

    /// Represents the backend for the [`Repository`] index.
    pub repositories: Box<dyn Backend<Repository>>,

    /// Represents the backend for the [`User`] index.
    pub users: Box<dyn Backend<User>>,
}
