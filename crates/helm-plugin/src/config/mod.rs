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

mod charted;
pub use charted::*;

mod registry;
pub use registry::*;

mod repository;
pub use repository::*;

use eyre::Context;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    path::Path,
};
use tracing::{info, warn};

/// Configuration schematic for the `.charted.toml` configuration file, which is used
/// by the Helm plugin to see what repositories are avaliable to be used.
///
/// ## Example
/// ```toml
/// # `version` and `helm` are version constraints to determine
/// # what versions of `charted-helm-plugin` (`version`) and
/// # Helm (`helm`) is supported.
/// [charted]
/// version = ">=0.1.0"
/// helm    = ">=3.12"
///
/// # The `registry` configuration allows to use other registries
/// # as the official instance is registered by default via
/// # the `default` key.
/// [registry.private]
/// version = 1
/// url = "https://corpo.noelware.dev"
///
/// [[repository]]
/// source = "./charts/charted"
/// name = "server"
/// path = "charted/server"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    /// The `charted` table allows to configure the Helm plugin.
    #[serde(default)]
    pub charted: Charted,

    /// List of configured registries.
    #[serde(default, rename = "registry", skip_serializing_if = "HashMap::is_empty")]
    pub registries: HashMap<String, Registry>,

    /// List of repositories avaliable.
    #[serde(default, rename = "repository", skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
}

impl Config {
    /// Loads the configuration file in the given `path` if provided. If not, it'll
    /// look in `$CWD/.charted.toml` instead.
    pub fn load<P: AsRef<Path>>(path: Option<P>) -> eyre::Result<Config> {
        use std::io::Write;

        let current_dir = env::current_dir()?;
        let path = path
            .map(|x| x.as_ref().to_path_buf())
            .unwrap_or(current_dir.join(".charted.toml"));

        if !path.try_exists()? {
            warn!(path = %path.display(), ".charted.toml doesn't exist! creating default one...");

            let me = Config::default();
            {
                let mut file = File::create_new(&path)?;
                write!(file, "{}", toml::to_string_pretty(&me)?)?;
            }

            info!(path = %path.display(), "created default `.charted.toml` in given path");
            return Ok(me);
        }

        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents).context("failed to parse toml configuration file")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            charted: Charted::default(),
            repositories: Vec::new(),
            registries: azalia::hashmap!(
                "default" => Registry {
                    version: charted_core::api::Version::V1,
                    url: "https://charts.noelware.org/api".parse().unwrap()
                }
            ),
        }
    }
}
