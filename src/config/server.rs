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

pub mod ratelimits;
pub mod ssl;

use crate::TRUTHY_REGEX;
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
pub struct Config {
    /// Host to bind onto. `127.0.0.1` is for internal, `0.0.0.0` is for public.
    #[serde(default = "__default_host")]
    pub host: String,

    /// Port to listen on.
    #[serde(default = "__default_port")]
    pub port: u16,

    /// Configures the use of HTTPS on the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<ssl::Config>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: __default_host(),
            port: __default_port(),
            ssl: None,
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            host: env!("CHARTED_SERVER_HOST", or_else: env!("HOST", or_else: __default_host())),
            port: env!("CHARTED_SERVER_PORT", to: u16, or_else: env!("PORT", to: u16, or_else: __default_port())),
            ssl: match env!("CHARTED_SERVER_SSL_ENABLE") {
                Ok(res) if TRUTHY_REGEX.is_match(&res) => Some(ssl::Config::try_from_env()?),
                Ok(_) => None,

                Err(std::env::VarError::NotUnicode(_)) => {
                    return Err(eyre!("expected valid utf-8 for `CHARTED_SERVER_SSL_ENABLE`"));
                }

                Err(_) => None,
            },
        })
    }
}

impl From<Config> for SocketAddr {
    fn from(value: Config) -> SocketAddr {
        format!("{}:{}", value.host, value.port).parse().unwrap()
    }
}

#[inline]
fn __default_host() -> String {
    String::from("0.0.0.0")
}

#[inline]
fn __default_port() -> u16 {
    3651
}
