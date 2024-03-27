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

use charted_entities::Name;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents the configuration for each repository that the Helm plugin
/// should manage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// whether or not if this repository can be published to a registry.
    #[serde(default)]
    pub publish: bool,

    /// Registry to publish this repository to.
    #[serde(default = "__default_registry")]
    pub registry: String,

    /// Path to the repository's full identifier. This is represented as two [`Name`](charted_entities::Name)s with
    /// a slash: `noel/my-project`
    #[serde(with = "__name_sep")]
    pub path: (Name, Name),

    /// Source directory to where this repository belongs to.
    pub source: PathBuf,

    /// Path to a README file, this will default to `{repository.<name>.source}/README.md` if it exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readme: Option<PathBuf>,
}

fn __default_registry() -> String {
    String::from("default")
}

mod __name_sep {
    use charted_entities::Name;
    use serde::{de, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(value: &(Name, Name), serializer: S) -> Result<S::Ok, S::Error> {
        // TODO(@auguwu): validate that it is a valid `Name/Name` pairing.
        serializer.serialize_str(&format!("{}/{}", value.0, value.1))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(Name, Name), D::Error> {
        use serde::de::Error;

        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = (Name, Name);

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "valid mapping of {{owner}}/{{repo}}")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v.split_once('/') {
                    Some((_, repo)) if repo.contains('/') => Err(E::custom("found more than one slash")),
                    Some((name, repo)) => Ok((name.parse().map_err(E::custom)?, repo.parse().map_err(E::custom)?)),
                    None => Err(E::custom("failed to parse repo path, expected [name/repo] match")),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
