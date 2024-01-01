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

pub mod sets;

use crate::{make_config, var};
use charted_common::TRUTHY_REGEX;

make_config! {
    /// Represents the configuration for configuring the metrics pipeline.
    MetricsConfig {
        /// Whether if the metrics pipeline should be enabled or not.
        #[serde(default = "falsy")]
        pub enabled: bool {
            default: true;
            env_value: var!("CHARTED_METRICS_ENABLE", {
                or_else: true;
                mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
            });
        };

        /// Whether if the `/admin/stats` would be enabled on the API server or not. This
        /// will allow administrators to pull server statistics to diagnose the server
        /// or whatever your intention is.
        #[serde(default = "falsy")]
        pub admin_endpoint: bool {
            default: false;
            env_value: var!("CHARTED_METRICS_ADMIN_ENDPOINT", {
                or_else: false;
                mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
            });
        };

        /// Whether if the `/metrics` endpoint should be enabled, for Prometheus
        /// servers to scrape the data from.
        #[serde(default)]
        pub prometheus: bool {
            default: false;
            env_value: var!("CHARTED_METRICS_PROMETHEUS", {
                or_else: false;
                mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
            });
        };
    }
}

//make_config! {}

fn falsy() -> bool {
    false
}
