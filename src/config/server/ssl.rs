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

use eyre::Context;
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::TRUTHY_REGEX;

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
pub struct Config {
    /// Allows to redirect all HTTP traffic to the HTTPS server instead. This will listen on port
    /// `:7015` for all redirection traffic to `https://{server.host}:{server.port}[/...]`
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub allow_redirections: bool,

    /// Location to a certificate private key.
    #[merge(skip)]
    pub cert_key: PathBuf,

    /// Location to a certificate public key.
    #[merge(skip)]
    pub cert: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        let certs = PathBuf::from("./certs");
        Config {
            allow_redirections: true,
            cert_key: certs.join("key.pem"),
            cert: certs.join("cert.pem"),
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            allow_redirections: env!("CHARTED_SERVER_SSL_ALLOW_REDIRECTIONS", {
                or_else: true;
                mapper: |val| TRUTHY_REGEX.is_match(&val);
            }),

            cert_key: env!("CHARTED_SERVER_SSL_CERT_KEY")
                .map(PathBuf::from)
                .context("unable to load up `CHARTED_SERVER_SSL_CERT_KEY` env")?,

            cert: env!("CHARTED_SERVER_SSL_CERT")
                .map(PathBuf::from)
                .context("unable to load up `CHARTED_SERVER_SSL_CERT` env")?,
        })
    }
}
