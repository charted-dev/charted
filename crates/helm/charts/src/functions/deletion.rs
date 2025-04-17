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

use azalia::remi::{StorageService, core::StorageService as _};
use charted_core::ResultExt;

pub async fn delete_chart(
    storage: &StorageService,
    owner: u64,
    repo: u64,
    version: impl AsRef<str> + Send,
) -> eyre::Result<()> {
    storage
        .delete(format!("./repositories/{owner}/{repo}/{}.tgz", version.as_ref()))
        .await
        .map(|_| ())
        .into_report()
}

pub async fn delete_chart_prov(
    storage: &StorageService,
    owner: u64,
    repo: u64,
    version: impl AsRef<str> + Send,
) -> eyre::Result<()> {
    storage
        .delete(format!("./repositories/{owner}/{repo}/{}.prov.tgz", version.as_ref()))
        .await
        .map(|_| ())
        .into_report()
}
