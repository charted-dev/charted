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

use azalia::config::{env, merge::Merge, TryFromEnv};
use eyre::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const CERT_KEY: &str = "CHARTED_SERVER_SSL_CERT_KEY";
pub const ENABLED: &str = "CHARTED_SERVER_SSL";
pub const CERT: &str = "CHARTED_SERVER_SSL_CERTIFICATE";

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Location to a certificate private key.
    pub cert_key: PathBuf,

    /// Location to a certificate public key.
    pub cert: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        let certs = PathBuf::from("./certs");
        Config {
            cert_key: certs.join("key.pem"),
            cert: certs.join("cert.pem"),
        }
    }
}

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Config;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            cert_key: env!(CERT_KEY)
                .map(PathBuf::from)
                .context("unable to load up `CHARTED_SERVER_SSL_CERT_KEY` env")?,

            cert: env!(CERT)
                .map(PathBuf::from)
                .context("unable to load up `CHARTED_SERVER_SSL_CERT` env")?,
        })
    }
}
