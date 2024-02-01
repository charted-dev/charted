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

use crate::TRUTHY_REGEX;
use noelware_config::{env, merge::Merge, FromEnv, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Debug, Clone, Default, Merge, Serialize, Deserialize)]
pub struct Config {
    /// Allows the API server to accept `Authorization: Basic {base64 of username:password}` when using authenticated
    /// endpoints. This is not recommended in production environments.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub enable_basic_auth: bool,

    /// [`Backend`] to use for authenticating users.
    #[serde(default, with = "serde_yaml::with::singleton_map")]
    pub backend: Backend,
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            enable_basic_auth: env!("CHARTED_SESSION_ENABLE_BASIC_AUTH", {
                or_else: false;
                mapper: |val| TRUTHY_REGEX.is_match(&val);
            }),

            backend: Backend::try_from_env()?,
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Backend {
    /// Uses a list of htpasswd files to define user:password hashes. This will not provide
    /// a `password` field when creating users or when creating users in the db.
    Htpasswd(BTreeSet<PathBuf>),

    /// Uses a LDAP server to authenticate users. This will create a background task to
    /// import LDAP users if requested.
    Ldap(ldap::Config),

    /// Enables the use of disallowing passwords to begin with.
    Passwordless,

    /// Allows to use charted-server's local user system.
    #[default]
    Local,
}

impl Merge for Backend {
    fn merge(&mut self, other: Self) {
        match (self.clone(), other) {
            (Self::Htpasswd(ref mut htpasswd), Self::Htpasswd(htpasswd2)) => htpasswd.merge(htpasswd2),
            (Self::Ldap(ref mut ldap), Self::Ldap(ldap2)) => ldap.merge(ldap2),
            (Self::Passwordless, Self::Passwordless) => {} // don't even merge
            (Self::Local, Self::Local) => {}               // don't merge anything

            (Self::Local, Self::Htpasswd(files)) => {
                *self = Backend::Htpasswd(files);
            }

            (Self::Local, Self::Passwordless) => {
                *self = Backend::Passwordless;
            }

            (Self::Local, Self::Ldap(config)) => {
                *self = Backend::Ldap(config);
            }

            _ => {} // don't do anything if no matches are available
        }
    }
}

impl TryFromEnv for Backend {
    type Output = Backend;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!("CHARTED_SESSION_BACKEND") {
            Ok(res) => match res.as_str() {
                "htpasswd" => Ok(Backend::Htpasswd(
                    env!("CHARTED_SESSIONS_HTPASSWD_FILES")
                        .map(|s| s.split(',').map(PathBuf::from).collect())
                        .unwrap_or_default(),
                )),

                "passwordless" => Ok(Backend::Passwordless),
                "ldap" => Ok(Backend::Ldap(ldap::Config::from_env())),
                "local" => Ok(Backend::Local),
                out if out.is_empty() => Ok(Backend::Local),
                out => Err(eyre!("expected [htpasswd, ldap, local]; received '{out}'")),
            },
            Err(std::env::VarError::NotUnicode(_)) => Err(eyre!(
                "expected a utf-8 encoded string for `CHARTED_SESSION_BACKEND` env variable"
            )),
            Err(_) => Ok(Default::default()),
        }
    }
}
