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

pub mod ratelimits;
pub mod ssl;

use crate::util;
use azalia::config::{
    env::{self, TryFromEnv},
    merge::Merge,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, net::SocketAddr};

pub const HEADERS: &str = "CHARTED_SERVER_HEADERS";
pub const HOST: &[&str; 2] = &["CHARTED_SERVER_HOST", "HOST"];
pub const PORT: &[&str; 2] = &["CHARTED_SERVER_PORT", "PORT"];

/// ## `[server]` table
/// This configures the HTTP service that the API server creates.
#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// A list of headers to append to all responses.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,

    /// The host to bind towards.
    #[serde(default = "__default_host")]
    pub host: String,

    /// Port to listen on.
    #[serde(default = "__default_port")]
    pub port: u16,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<ssl::Config>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            headers: BTreeMap::new(),
            host: __default_host(),
            port: __default_port(),
            ssl: None,
        }
    }
}

impl Config {
    pub fn to_socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }
}

impl TryFromEnv for Config {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        Ok(Config {
            headers: env::try_parse(HEADERS).unwrap_or_else(|_| Default::default()),
            host: env::try_parse_or_else(HOST[0], env::try_parse_or_else(HOST[1], __default_host())?)?,
            port: env::try_parse_or_else(PORT[0], env::try_parse_or_else(PORT[1], __default_port())?)?,
            ssl: match util::bool_env(ssl::ENABLED) {
                Ok(true) => ssl::Config::try_from_env().map(Some)?,
                Ok(false) => None,
                Err(e) => return Err(e),
            },
        })
    }
}

#[inline]
fn __default_host() -> String {
    String::from("0.0.0.0")
}

const fn __default_port() -> u16 {
    3651
}
