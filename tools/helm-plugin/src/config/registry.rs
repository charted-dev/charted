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

use charted_core::APIVersion;
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Cow, fmt::Display};
use url::Url;

/// Represents the registry configuration, which registers a set list
/// of registries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// API version of the registry.
    #[serde(default)]
    pub version: APIVersion,

    /// URL of the registry to point to. This doesn't include the API version
    /// in the URI itself (i.e, `https://charts.noelware.org/api/v1`).
    pub url: Url,
}

impl JsonSchema for Config {
    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("charted_helm_plugin::config::registry::Config")
    }

    fn schema_name() -> String {
        String::from("Registry")
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut obj = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                required: azalia::btreeset!["url"],
                ..Default::default()
            })),

            ..Default::default()
        };

        obj.metadata().description =
            Some("Represents the list of registries that repositories can be uploaded to".into());

        let validation = obj.object();
        validation.properties.insert("version".into(), {
            let schema = gen.subschema_for::<APIVersion>();
            let mut obj = schema.into_object();

            obj.metadata().description = Some("API version of the registry.".into());
            obj.metadata().default = Some(Value::Number(APIVersion::default().into()));

            Schema::Object(obj)
        });

        validation.properties.insert("url".into(), {
            let schema = gen.subschema_for::<Url>();
            let mut obj = schema.into_object();

            obj.metadata().description = Some("URL of the registry to point to. This doesn't include the API version in the URI itself (i.e, `https://charts.noelware.org/api/v1`).".into());

            Schema::Object(obj)
        });

        Schema::Object(obj)
    }
}

impl Config {
    /// Joins the registry URL via [`Url::join`] and returns a string representation.
    pub fn join_url<T: Display>(&self, input: T) -> Result<String, url::ParseError> {
        // `format!()` is necessary here since if we tried to do 2 joins, it'll only
        // return the second join without applying the first one.
        self.url
            .join(&format!("{}/{input}", self.version))
            .map(|x| x.to_string())
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.url.join(self.version.as_str()).map_err(|_|
                /* this will map all url::ParseError as formatting errors */
                std::fmt::Error)?
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use charted_core::APIVersion;
    use url::Url;

    #[test]
    fn url_joins() {
        let registry = Config {
            version: APIVersion::default(),
            url: Url::parse("https://charts.noelware.org").expect("invalid url"),
        };

        assert_eq!(
            Ok(String::from("https://charts.noelware.org/v1/weow/fluff")),
            registry.join_url("weow/fluff")
        );
    }
}
