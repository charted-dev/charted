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

use super::common;
use azalia::config::{TryFromEnv, env, merge::Merge};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Untagged enumeration to determine if a value is a [`PathBuf`] or a [`String`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, derive_more::From, derive_more::Display)]
#[serde(untagged)]
pub enum StringOrPath {
    #[display("{}", _0.display())]
    Path(PathBuf),
    String(String),
}

impl Merge for StringOrPath {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::Path(p1), Self::Path(p2)) => {
                p1.merge(p2);
            }

            (Self::String(s1), Self::String(s2)) => {
                s1.merge(s2);
            }

            (me, other) => {
                *me = other;
            }
        }
    }
}

/// ## `[database.sqlite]`
///
/// This database driver uses the almighty, holy [SQLite](https://sqlite.org). This is mainly used
/// for development, evaluation purposes, or if PostgreSQL is too heavy for your
/// use-cases.
#[derive(Debug, Clone, Merge, Serialize, Deserialize, derive_more::Deref, derive_more::Display)]
#[display("sqlite://{}?mode=rwc", self.path)]
pub struct Config {
    #[serde(flatten)]
    #[deref]
    pub common: common::Config,

    /// Path to the SQLite database. By default, this will be in `./data/charted.db`.
    ///
    /// The [official Docker image](https://docker.noelware.org/~/charted/server) will overwrite this path to `/var/lib/noelware/charted/data/charted.db`.
    #[serde(default = "__db_path")]
    pub path: StringOrPath,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            common: common::Config::default(),
            path: __db_path(),
        }
    }
}

const PATH: &str = "CHARTED_DATABASE_PATH";

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Config;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            common: common::Config::try_from_env()?,
            path: env!(PATH).map(|p| StringOrPath::Path(p.into())).unwrap_or(__db_path()),
        })
    }
}

#[inline]
fn __db_path() -> StringOrPath {
    PathBuf::from("./data/charted.db").into()
}
