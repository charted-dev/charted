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

use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    borrow::Cow,
    fmt::{self, Display},
    ops::Deref,
};

/// Newtype wrapper for [`semver::VersionReq`] that supports the [`JsonSchema`] trait.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionReq(semver::VersionReq);
impl VersionReq {
    /// Wraps [`VersionReq::parse`][semver::VersionReq::parse] and returns the newtype wrapper
    /// instead of the inner [`semver::VersionReq`].
    pub fn parse(input: &str) -> Result<VersionReq, semver::Error> {
        semver::VersionReq::parse(input).map(VersionReq)
    }
}

impl Display for VersionReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Deref for VersionReq {
    type Target = semver::VersionReq;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsonSchema for VersionReq {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        String::from("VersionReq")
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("semver::VersionReq")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
        })
    }
}

/// Configures the Helm plugin with the `charted {}` block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Version requirement for the charted Helm plugin.
    #[serde(default = "__default_charted_req")]
    pub version: VersionReq,

    /// Version requirement for Helm itself.
    #[serde(default = "__default_helm_req")]
    pub helm: VersionReq,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: __default_charted_req(),
            helm: __default_helm_req(),
        }
    }
}

impl JsonSchema for Config {
    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("charted_helm_plugin::config::charted::Config")
    }

    fn schema_name() -> String {
        String::from("PluginConfig")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut obj = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            ..Default::default()
        };

        obj.metadata().description = Some("Represents the configuration for `charted-helm-plugin` itself".into());

        let validation = obj.object();
        validation.properties.insert("version".into(), {
            let schema = gen.subschema_for::<VersionReq>();
            let mut obj = schema.into_object();

            obj.metadata().description = Some("Version requirement for the charted Helm plugin.".into());
            obj.metadata().default = Some(Value::String(__default_charted_req().to_string()));

            Schema::Object(obj)
        });

        validation.properties.insert("helm".into(), {
            let schema = gen.subschema_for::<VersionReq>();
            let mut obj = schema.into_object();

            obj.metadata().description = Some("Version requirement for Helm itself.".into());
            obj.metadata().default = Some(Value::String(__default_helm_req().to_string()));

            Schema::Object(obj)
        });

        Schema::Object(obj)
    }
}

#[inline]
fn __default_charted_req() -> VersionReq {
    VersionReq::parse(format!(">={}", crate::VERSION).as_str()).unwrap()
}

#[inline]
fn __default_helm_req() -> VersionReq {
    VersionReq::parse(">=3.10").unwrap()
}
