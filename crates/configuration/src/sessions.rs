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

pub mod ldap;

use crate::util;
use azalia::config::{
    env::{self, TryFromEnv, TryParseError},
    merge::Merge,
};
use eyre::bail;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, env::VarError};

pub const BACKEND: &str = "CHARTED_SESSIONS_BACKEND";
pub const STATIC_USERS: &str = "CHARTED_SESSIONS_STATIC_USERS";

#[derive(Debug, Clone, Default, Serialize, Deserialize, derive_more::Display)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum Backend {
    #[display("static")]
    Static(BTreeMap<String, String>),

    #[display("ldap")]
    Ldap(ldap::Config),

    #[default]
    Local,
}

impl Merge for Backend {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::Static(s1), Self::Static(s2)) => {
                s1.merge(s2);
            }

            (Self::Ldap(ldap1), Self::Ldap(ldap2)) => {
                ldap1.merge(ldap2);
            }

            (Self::Local, Self::Local) => {}

            // the case from env -> config
            (Self::Ldap(_), Self::Local) => {}
            (Self::Static(_), Self::Local) => {}

            (me, other) => {
                *me = other;
            }
        }
    }
}

impl TryFromEnv for Backend {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        match env::try_parse_or_else::<_, String>(BACKEND, Default::default()) {
            Ok(input) => match &*input.to_ascii_lowercase() {
                "local" | "default" | "" => Ok(Backend::Local),
                "static" => Ok(Backend::Static(env::try_parse(STATIC_USERS)?)),
                "ldap" => Ok(Backend::Ldap(ldap::Config::try_from_env()?)),
                input => bail!(
                    "unexpected input given from environment variable `${}`: expected `local`, `default`, `static`, or `ldap`; received {} instead",
                    BACKEND,
                    input
                ),
            },

            Err(TryParseError::System(VarError::NotUnicode(_))) => bail!(
                "environment variable `${}` couldn't be loaded due to invalid unicode",
                BACKEND
            ),

            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Clone, Default, Merge, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub enable_basic_auth: bool,

    #[serde(default)]
    pub backend: Backend,
}

pub const ENABLE_BASIC_AUTH: &str = "CHARTED_SESSIONS_ENABLE_BASIC_AUTH";

impl TryFromEnv for Config {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        Ok(Config {
            enable_basic_auth: util::bool_env(ENABLE_BASIC_AUTH)?,
            backend: Backend::try_from_env()?,
        })
    }
}
