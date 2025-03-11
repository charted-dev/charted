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

use azalia::config::{TryFromEnv, env, merge::Merge};
use eyre::bail;
use serde::{Deserialize, Serialize};
use std::env::VarError;

pub mod opentelemetry;
pub mod prometheus;

pub const DRIVER: &str = "CHARTED_METRICS_DRIVER";

/// ## `[metrics]` table
/// Allows **charted-server** to collect metrics about itself and push to a
/// OpenTelemetry-supported collector or as a Prometheus scraper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Config {
    Prometheus(prometheus::Config),
    OpenTelemetry(opentelemetry::Config),

    #[default]
    Disabled,
}

impl Merge for Config {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::Disabled, Self::Disabled) => {}
            (Self::Prometheus(prom1), Self::Prometheus(prom2)) => {
                prom1.merge(prom2);
            }

            (Self::OpenTelemetry(otel1), Self::OpenTelemetry(otel2)) => {
                otel1.merge(otel2);
            }

            (me, other) => {
                *me = other;
            }
        }
    }
}

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Self;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!(DRIVER) {
            Ok(input) => match &*input.to_ascii_lowercase() {
                "opentelemetry" | "otel" => Ok(Self::OpenTelemetry(opentelemetry::Config::try_from_env()?)),
                "prometheus" => Ok(Self::Prometheus(prometheus::Config::try_from_env()?)),
                "" | "disabled" | "disable" | "0" | "n" | "no" => Ok(Self::Disabled),
                _ => bail!(
                    "environment variable `${}` received invalid input: expected either opentelemetry, prometheus, or disabled",
                    DRIVER
                ),
            },

            Err(VarError::NotPresent) => Ok(Default::default()),
            Err(VarError::NotUnicode(_)) => bail!("environment variable `${}` received invalid unicode", DRIVER),
        }
    }
}

impl Config {
    pub fn as_prometheus(&self) -> Option<&prometheus::Config> {
        match self {
            Config::Prometheus(c) => Some(c),
            _ => None,
        }
    }
}
