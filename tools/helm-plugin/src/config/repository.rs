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
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject, StringValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Cow, path::PathBuf};

/// Represents the configuration for each repository that the Helm plugin
/// should manage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// whether or not if this repository can be published to a registry.
    #[serde(default = "__true")]
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

impl JsonSchema for Config {
    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("charted_helm_plugin::config::repository::Config")
    }

    fn schema_name() -> String {
        String::from("Repository")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        let mut obj = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                required: azalia::btreeset!["path", "source"],
                ..Default::default()
            })),

            ..Default::default()
        };

        obj.metadata().description =
            Some("Represents a single repository, that allows to be uploaded onto `charted-server`".into());

        let validation = obj.object();
        validation.properties.insert("publish".into(), {
            let mut us = SchemaObject {
                instance_type: Some(InstanceType::Boolean.into()),
                ..Default::default()
            };

            us.metadata().description = Some(
                "whether or not if this repository can be published to a registry, by default, this will be `true`."
                    .into(),
            );

            us.metadata().default = Some(Value::Bool(true));

            Schema::Object(us)
        });

        validation.properties.insert("registry".into(), {
            let mut us = SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                ..Default::default()
            };

            us.metadata().description = Some("Registry to publish this repository to.".into());
            us.metadata().default = Some(Value::String(__default_registry()));

            Schema::Object(us)
        });

        validation.properties.insert("path".into(), {
            let mut us = SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                string: Some(Box::new(StringValidation {
                    pattern: Some("^([A-z]{2,}|[0-9]|_|-)\\/([A-z]{2,}|[0-9]|_|-)$".into()),
                    ..Default::default()
                })),

                ..Default::default()
            };

            us.metadata().description = Some("Path to the repository that'll be referenced when uploading to `charted-server`. It must be a string of `owner/repo` and entries before and after the `/` must match to be a `Name`.".into());
            Schema::Object(us)
        });

        validation.properties.insert("source".into(), {
            let mut us = SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                ..Default::default()
            };

            us.metadata().description = Some("Source location to where the Helm chart resides in, must be a *relative* path from `.charted.hcl` or an absolute path".into());
            Schema::Object(us)
        });

        validation.properties.insert("readme".into(), {
            let mut us = SchemaObject {
                instance_type: Some(vec![InstanceType::String, InstanceType::Null].into()),
                ..Default::default()
            };

            us.metadata().description = Some(
                "Path to a README file, this will default to `{repository.<name>.source}/README.md` if it exists."
                    .into(),
            );

            Schema::Object(us)
        });

        Schema::Object(obj)
    }
}

fn __default_registry() -> String {
    String::from("default")
}

const fn __true() -> bool {
    true
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
