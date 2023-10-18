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
use std::fmt::Write as _;

make_config! {
    /// Represents the configuration details for configuring charted-server's
    /// database connections. charted-server uses [SQLx](https://github.com/launchbadge/sqlx) as
    /// the database module used, so you can only configure the maximum amount of connections.
    DatabaseConfig {
        /// Set the maxmium number of connections that the database connection
        /// pool should maintain.
        #[serde(default = "max_connections")]
        pub max_connections: u32 {
            default: 10;
            env_value: var!("CHARTED_DATABASE_MAX_CONNECTIONS", to: u32, or_else: 10);
        };

        /// The password to use for authentication.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) password: Option<String> {
            default: None;
            env_value: var!("CHARTED_DATABASE_PASSWORD", is_optional: true);
        };

        /// The username to use for authentication
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) username: Option<String> {
            default: None;
            env_value: var!("CHARTED_DATABASE_USERNAME", is_optional: true);
        };

        /// Database name to use when connecting.
        #[serde(default = "database")]
        pub database: String {
            default: "charted".into();
            env_value: var!("CHARTED_DATABASE_NAME", or_else: "charted".into());
        };

        /// Database schema to select when querying objects.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub schema: Option<String> {
            default: None;
            env_value: var!("CHARTED_DATABASE_SCHEMA", is_optional: true);
        };

        /// Database host to connect to.
        #[serde(default = "host")]
        pub host: String {
            default: host();
            env_value: var!("CHARTED_DATABASE_HOST", or_else: host());
        };

        /// Database port to connect to.
        #[serde(default = "port")]
        pub port: u16 {
            default: port();
            env_value: var!("CHARTED_DATABASE_PORT", to: u16, or_else: port());
        };
    }
}

impl ToString for DatabaseConfig {
    fn to_string(&self) -> String {
        let mut buf = String::from("postgres://");

        match (self.username.clone(), self.password.clone()) {
            (Some(username), Some(password)) => {
                write!(buf, "{username}:{password}@").unwrap();
            }
            (Some(_), None) => panic!("provided username, but no password?"),
            (None, Some(_)) => panic!("provided password, but no username?"),
            _ => {}
        }

        write!(buf, "{}:{}/{}", self.host, self.port, self.database).unwrap();
        buf
    }
}

fn host() -> String {
    String::from("127.0.0.1")
}

fn port() -> u16 {
    5432
}

fn max_connections() -> u32 {
    10
}

fn database() -> String {
    String::from("charted")
}