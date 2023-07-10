// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use crate::{make_config, var};
use std::net::SocketAddr;

make_config! {
    /// Represents the configuration for configuring the server that
    /// the CLI will spawn when the `charted server` command is used.
    ServerConfig {
        /// Host binding to create the inner [`SocketAddr`]. This flag
        /// supports using the common `HOST` environment variable.
        #[serde(default = "host")]
        pub host: String {
            default: host();
            env_value: var!("CHARTED_SERVER_HOST", or_else: var!("HOST", or_else: host()));
        };

        /// Host port to bind towards. This flag supports using the common `PORT` environment
        /// variable.
        pub port: u16 {
            default: port();
            env_value: var!("CHARTED_SERVER_PORT", to: u16, or_else: var!("PORT", to: u16, or_else: port()));
        };
    }
}

impl ServerConfig {
    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }
}

fn host() -> String {
    "0.0.0.0".into()
}

fn port() -> u16 {
    3651
}
