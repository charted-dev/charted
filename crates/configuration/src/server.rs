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

pub const HOST: &[&str; 2] = &["CHARTED_SERVER_HOST", "HOST"];
pub const PORT: &[&str; 2] = &["CHARTED_SERVER_PORT", "PORT"];

/*
use crate::helpers;
use azalia::{
    config::{env, merge::Merge, TryFromEnv},
    TRUTHY_REGEX,
};
use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::{env::VarError, net::SocketAddr};

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
            ssl: None,
            host: __default_host(),
            port: __default_port(),
        }
    }
}

impl Config {
    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            host: helpers::env_from_result(
                env!("CHARTED_SERVER_HOST"),
                helpers::env_from_result(env!("HOST"), __default_host())?,
            )?,

            port: match env!("CHARTED_SERVER_PORT") {
                Ok(value) => value.parse::<u16>()?,
                Err(VarError::NotPresent) => match env!("PORT") {
                    Ok(value) => value.parse::<u16>()?,
                    Err(VarError::NotPresent) => __default_port(),
                    Err(VarError::NotUnicode(_)) => Err(eyre!("received non-unicode in environment variable"))?,
                },

                Err(VarError::NotUnicode(_)) => Err(eyre!("received non-unicode in environment variable"))?,
            },

            ssl: {
                let value = env!("CHARTED_SERVER_ENABLE_SSL", |val| TRUTHY_REGEX.is_match(&val); or false)
                    .then(ssl::Config::try_from_env);

                if let Some(res) = value {
                    Some(res?)
                } else {
                    None
                }
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
*/
