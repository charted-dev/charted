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

use crate::{make_config, var};
use charted_common::TRUTHY_REGEX;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::Level;

/// Represents the output that the Logstash pipeline will be executed from. By default,
/// the [TCP input plugin](https://www.elastic.co/guide/en/logstash/current/plugins-inputs-tcp.html) will be used
/// and will create a TCP stream to write messages to, if this is provided.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum LogstashOutput {
    #[default]
    TCP,
    UDP,
}

impl FromStr for LogstashOutput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(LogstashOutput::TCP),
            "udp" => Ok(LogstashOutput::UDP),
            _ => Err(format!("unknown valid Logstash output: [{s}]")),
        }
    }
}

make_config! {
    /// Represents the configuration to configure charted-server's logging
    /// capabilities. charted-server uses the [`tracing`](https://crates.io/crates/tracing)
    /// crate with custom outputs, so this is how you can configure it.
    LoggingConfig {
        /// Log level to output log messages in.
        #[serde(with = "serde_tracing")]
        pub level: Level {
            default: Level::INFO;
            env_value: ::std::env::var("CHARTED_LOG_LEVEL").map(|val| match val.as_str() {
                "trace" => Level::TRACE,
                "debug" => Level::DEBUG,
                "info" => Level::INFO,
                "warn" => Level::WARN,
                _ => Level::INFO,
            }).unwrap_or(Level::INFO);
        };

        /// The output type to use for outputting events that were received via the JSON visitor into.
        ///
        /// * [`LogstashOutput::TCP`] is for the [TCP input plugin](https://www.elastic.co/guide/en/logstash/current/plugins-inputs-tcp.html)
        ///   that will output everything in a single TCP socket connection.
        /// * [`LogstashOutput::UDP`] is for the [UDP input plugin](https://www.elastic.co/guide/en/logstash/current/plugins-inputs-udp.html)
        ///   that will output everything in a UDP socket connection.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logstash_output: Option<LogstashOutput> {
            default: None;
            env_value: var!("CHARTED_LOGSTASH_OUTPUT", to: LogstashOutput, is_optional: true);
        };

        /// Connection URI to use when connecting to the configured Logstash TCP or UDP stream.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logstash_connect_uri: Option<String> {
            default: None;
            env_value: var!("CHARTED_LOGSTASH_CONNECTION_URI", is_optional: true);
        };

        /// Whether the logger should only output JSON messages or not.
        #[serde(rename = "json", default)]
        pub json_logging: bool {
            default: false;
            env_value: ::std::env::var("CHARTED_LOG_JSON").map(|val| TRUTHY_REGEX.is_match(val.as_str())).unwrap_or(false);
        };
    }
}

pub mod serde_tracing {
    use serde::*;
    use tracing::Level;

    pub fn serialize<S: Serializer>(filter: &Level, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match *filter {
            Level::TRACE => "trace",
            Level::DEBUG => "debug",
            Level::INFO => "info",
            Level::WARN => "warn",
            _ => unreachable!(), // We shouldn't be able to hit here
        })
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Level, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            "trace" => Ok(Level::TRACE),
            "debug" => Ok(Level::DEBUG),
            "info" => Ok(Level::INFO),
            "warn" => Ok(Level::WARN),
            _ => Ok(Level::INFO),
        }
    }
}
