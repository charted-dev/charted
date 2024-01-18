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

use noelware_config::{env, merge::Merge, FromEnv};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
pub struct Config {
    /// list of certificates to validate that we're actually being
    /// sent to the token server.
    pub certs: BTreeSet<PathBuf>,

    /// issuer to validate the JWT token that the token server is tracking.
    #[serde(default = "__default_issuer")]
    pub issuer: String,

    /// The realm in which charted-server authenticates.
    #[serde(default = "__default_realm")]
    pub realm: String,

    /// HTTPS-based URL to the token server.
    pub url: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            issuer: __default_issuer(),
            realm: __default_realm(),
            certs: BTreeSet::new(),
            url: String::from("<not available>"),
        }
    }
}

impl FromEnv for Config {
    type Output = Config;

    fn from_env() -> Self::Output {
        Config {
            issuer: env!("CHARTED_SESSION_TOKEN_SERVER_ISSUER", or_else: __default_issuer()),
            realm: env!("CHARTED_SESSION_TOKEN_SERVER_REALM", or_else: __default_realm()),
            url: env!("CHARTED_SESSION_TOKEN_SERVER_URL", or_else: "<not available>".into()),

            certs: env!("CHARTED_SESSION_TOKEN_SERVER_CERTS", {
                or_else: BTreeSet::new();
                mapper: |val| val.split(',').map(PathBuf::from).collect();
            }),
        }
    }
}

fn __default_issuer() -> String {
    String::from("Noelware/charted-server")
}

fn __default_realm() -> String {
    String::from("token-realm")
}
