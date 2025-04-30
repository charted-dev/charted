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

use crate::{MetadataNamespace, OwnerRepoNamespace};
use charted_datastore::DataStore;
use charted_types::Ulid;

pub trait DataStoreExt: Sized {
    /// Builds a handle to the `metadata` datastore namespace.
    fn metadata<'storage, 'ext: 'storage>(&'ext self) -> MetadataNamespace<'storage>;

    /// Builds a handle to the <code>repositories/{[`owner`](Ulid)}/{[`repo`](Ulid)}</code> datastore namespace.
    fn owner_repo<'storage, 'ext: 'storage>(&'ext self, owner: Ulid, repo: Ulid) -> OwnerRepoNamespace<'storage>;
}

impl DataStoreExt for DataStore {
    fn metadata<'storage, 'ext: 'storage>(&'ext self) -> MetadataNamespace<'storage> {
        MetadataNamespace::new(self)
    }

    fn owner_repo<'storage, 'ext: 'storage>(&'ext self, owner: Ulid, repo: Ulid) -> OwnerRepoNamespace<'storage> {
        OwnerRepoNamespace::new(self, owner, repo)
    }
}
