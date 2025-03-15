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
use crate::util;
use azalia::config::{TryFromEnv, env, merge::Merge};
use serde::{Deserialize, Serialize};
use url::Url;

/// ## `[database.postgresql]`
///
/// This database driver will use [PostgreSQL](https://postgresql.org). This driver
/// is recommended to be used for production use cases and better reliability.
#[derive(Debug, Clone, Merge, Serialize, Deserialize, derive_more::Display, derive_more::Deref)]
#[serde(deny_unknown_fields)]
#[display("{}", self.url)]
pub struct Config {
    #[serde(flatten)]
    #[deref]
    pub common: common::Config,

    /// The password to use for authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// The username to use for authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Database schema to select when querying objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// Database URL to connect to.
    pub url: Url,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            username: Default::default(),
            password: Default::default(),
            common: Default::default(),
            schema: Default::default(),
            url: Url::parse("postgresql://localhost:5432/charted?application_name=charted-server").unwrap(),
        }
    }
}

pub const PASSWORD: &str = "CHARTED_DATABASE_PASSWORD";
pub const USERNAME: &str = "CHARTED_DATABASE_USERNAME";
pub const SCHEMA: &str = "CHARTED_DATABASE_SCHEMA";
pub const HOST: &str = "CHARTED_DATABASE_HOST";
pub const PORT: &str = "CHARTED_DATABASE_PORT";

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Config;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            password: env!(PASSWORD).ok(),
            username: env!(USERNAME).ok(),
            schema: env!(SCHEMA).ok(),
            common: common::Config::try_from_env()?,
            url: util::env_from_str(
                common::URL,
                Url::parse("postgresql://localhost:5432/charted?application_name=charted-server").unwrap(),
            )?,
        })
    }
}
