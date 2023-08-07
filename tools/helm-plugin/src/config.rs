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

use charted_common::models::NameOrSnowflake;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub repository: String,
    pub registry: Option<String>,
}

impl Config {
    /// Normalize the `repository` key in a `.charted.yaml` file and returns as a
    /// [`NameOrSnowflake`].
    pub fn normalize_repository(&self) -> Result<NameOrSnowflake, &'static str> {
        match self.repository.parse::<u64>() {
            Ok(uid) => Ok(NameOrSnowflake::Snowflake(uid)),
            Err(_) => match self.repository.split_once('/') {
                Some((_, repo)) if repo.contains('/') => Err("received more than one slash"),
                Some((owner, repo)) => Ok(NameOrSnowflake::Name(format!("{owner}/{repo}"))),
                _ => Err("missing slash in repository name; expected 'owner/repo' syntax"),
            },
        }
    }
}
