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

use charted_common::models::Name;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub repository: String,
    pub registry: Option<String>,
    pub readme: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerRepoOrSnowflake {
    Snowflake(u64),
    Repository((Name, Name)),
}

impl Config {
    /// Normalize the `repository` key in a `.charted.yaml` file and returns as a
    /// [`NameOrSnowflake`].
    pub fn normalize_repository(&self) -> Result<OwnerRepoOrSnowflake, String> {
        match self.repository.parse::<u64>() {
            Ok(uid) => Ok(OwnerRepoOrSnowflake::Snowflake(uid)),
            Err(_) => match self.repository.split_once('/') {
                Some((_, repo)) if repo.contains('/') => Err("received more than one slash".into()),
                Some((owner, repo)) => {
                    let owner = Name::new(owner).map_err(|e| e.to_string())?;
                    let repo = Name::new(repo).map_err(|e| e.to_string())?;

                    Ok(OwnerRepoOrSnowflake::Repository((owner, repo)))
                }
                _ => Err("missing slash in repository name; expected 'owner/repo' syntax".into()),
            },
        }
    }
}
