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

mod creation;
pub use creation::*;

mod deletion;
pub use deletion::*;

mod fetchers;
pub use fetchers::*;
mod sort_versions;
pub use sort_versions::*;

mod upload;
use azalia::remi::StorageService;
use tracing::warn;
pub use upload::*;

/// Initializes the storage service to contain the following directories:
///
/// * `$DATA_DIR/metadata` - used for holding user/organization indexes
/// * `$DATA_DIR/repositories` - used for holding repository metadata (charts, readmes,
///   etc)
#[tracing::instrument(name = "charted.helm.charts.initialize", skip_all)]
pub async fn init(storage: &StorageService) -> eyre::Result<()> {
    if let StorageService::Filesystem(fs) = storage {
        let paths = [fs.normalize("./metadata")?.unwrap(), fs.normalize("./repositories")?.unwrap()];
        for path in paths {
            if !tokio::fs::try_exists(&path).await? {
                warn!(path = %path.display(), "creating directory as it doesn't exist");
                tokio::fs::create_dir_all(path).await?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
