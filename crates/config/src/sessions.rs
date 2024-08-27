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

pub mod ldap;

use azalia::{
    config::{env, merge::Merge, TryFromEnv},
    TRUTHY_REGEX,
};
use serde::{Deserialize, Serialize};
use std::env::VarError;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Ldap(ldap::Config),

    #[default]
    Local,
}

impl Merge for Backend {
    fn merge(&mut self, other: Self) {
        match (self.clone(), other) {
            (Backend::Ldap(ref mut ldap1), Backend::Ldap(ldap2)) => {
                ldap1.merge(ldap2);
            }

            (_, Backend::Ldap(ldap)) => {
                *self = Backend::Ldap(ldap);
            }

            (_, Backend::Local) => {
                *self = Backend::Local;
            }
        }
    }
}

impl TryFromEnv for Backend {
    type Output = Backend;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!("CHARTED_SESSION_BACKEND") {
            Ok(res) => match &*res.to_ascii_lowercase() {
                "ldap" => Ok(Backend::Ldap(ldap::Config::try_from_env()?)),
                "local" | "default" => Ok(Backend::Local),
                s => Err(eyre::eyre!("unknown value [{s}]: expected [ldap, local]")),
            },

            Err(VarError::NotPresent) => Ok(Backend::Local),
            Err(e) => Err(eyre::eyre!(e)),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Merge)]
pub struct Config {
    /// Allows the API server to accept `Authorization: Basic {base64 of username:password}` when using authenticated
    /// endpoints. This is not recommended in production environments.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub enable_basic_auth: bool,

    #[serde(default)]
    pub backend: Backend,
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            enable_basic_auth: env!("CHARTED_SESSION_ENABLE_BASIC_AUTH", |val| TRUTHY_REGEX.is_match(&val); or false),
            backend: Backend::try_from_env()?,
        })
    }
}
