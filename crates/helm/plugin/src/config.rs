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

use charted_core::api;
use charted_types::name::Name;
use eyre::Context;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};
use url::Url;

pub mod global;
pub mod registry;
pub mod repository;

/// A repository path that is joined from the first [`Name`], which is the
/// owner of the repository and the secondary [`Name`], which is the repository
/// name.
///
/// ## Examples
/// ```plaintext
/// noel/ume
/// uwuDaOwO~/name
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    pub owner: Name,
    pub repository: Name,
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}/{}", self.owner, self.repository))
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = Path;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("valid mapping of {{owner}}/{{repo}}")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v.split_once('/') {
                    Some((_, repo)) if repo.contains('/') => Err(E::custom("found more than one slash")),
                    Some((owner, repo)) => Ok(Path {
                        owner: owner.parse().map_err(E::custom)?,
                        repository: repo.parse().map_err(E::custom)?,
                    }),

                    None => Err(E::custom("failed to parse repo path, expected [name/repo] match")),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// Configuration file for configuring repositories when authoring charts
/// that are pushed into [charted-server].
///
/// ## Example
/// ```toml
/// [global]
/// # semver constraint of what version `charted-helm-plugin` to require.
/// plugin = ">= 0.1"
///
/// # semver constraint of what version of `helm` to require.
/// helm   = ">= 3.12"
///
/// [repository."noelware/my-chart"]
/// source = "./charts/my-chart"
/// ```
///
/// To view the properties of **my-chart**, you can use the **repository view**
/// subcommand:
///
/// ```shell
/// $ helm charted repository view my-chart
/// Chart `my-chart`:
///     -> Registry:                    default (https://charts.noelware.org/api/v1)
///     -> Source:                      /git/Noelware/helm-charts/charts/my-chart
///     -> Version (from `Chart.yaml`): 0.1.0
/// ```
///
/// [charted-server]: https://charts.noelware.org
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    /// Global configuration that affects the lifecycle of the plugin.
    #[serde(default)]
    pub global: global::Global,

    /// A set of registries that are avaliable to each repository.
    #[serde(default, rename = "registry", skip_serializing_if = "BTreeMap::is_empty")]
    pub registries: BTreeMap<String, registry::Registry>,

    /// A set of repositories determined by a **path**, which is `{{owner}}/{{repo}}`.
    #[serde(default, rename = "repository", skip_serializing_if = "BTreeMap::is_empty")]
    pub repositories: BTreeMap<Path, repository::Repository>,

    // allows keeping track of what file we are for `flush_and_save`.
    #[serde(skip)]
    #[schemars(skip)]
    opened_from: PathBuf,
}

impl Config {
    pub fn load<P: Into<Option<PathBuf>>>(potential: P) -> eyre::Result<Self> {
        let path = Config::get_potential_default_path(potential)?;
        debug!(path = %path.display(), "loading plugin configuration in");

        if !path.try_exists()? {
            bail!("configuration file in location '{}' doesn't exist", path.display())
        }

        trace!("opening file `{}`", path.display());

        let mut config = toml::from_str::<Self>(&fs::read_to_string(&path)?)
            .with_context(|| format!("failed to deserialize from file: {}", path.display()))?;

        if !config.registries.contains_key("default") {
            config.registries.insert(String::from("default"), registry::Registry {
                version: api::Version::V1,
                url: Url::parse("https://charts.noelware.org/api").unwrap(),
            });
        }

        config.opened_from = path;
        Ok(config)
    }

    pub fn flush_and_save(&self) -> eyre::Result<()> {
        debug!(path = %self.opened_from.display(), "saving and flushing changes");

        let mut file = OpenOptions::new().write(true).open(&self.opened_from)?;
        let serialized = toml::to_string_pretty(self)?;

        write!(file, "{serialized}")?;
        file.flush()?;

        Ok(())
    }

    fn get_potential_default_path<P: Into<Option<PathBuf>>>(potential: P) -> eyre::Result<PathBuf> {
        if let Some(path) = potential.into() {
            return Ok(path);
        }

        for p in [std::path::Path::new("./.charted.toml"), std::path::Path::new("./charted.toml")] {
            trace!(potential = %p.display(), "checking if path exists");

            if p.try_exists()? {
                trace!("using potential configuration file: {}", p.display());
                return Ok(p.to_path_buf());
            }
        }

        bail!(
            "No potential `charted.toml` files found in default locations (.charted.toml, charted.toml in current directory). Initialize with `helm charted init`."
        )
    }
}
