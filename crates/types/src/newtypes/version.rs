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

use derive_more::Display;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    query_builder::bind_collector::RawBytesBindCollector,
    serialize::ToSql,
    sql_types::Text,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use utoipa::{
    openapi::{schema::*, *},
    PartialSchema, ToSchema,
};

charted_core::create_newtype_wrapper!(
    /// Newtype wrapper for the [`semver::Version`] type.
    ///
    /// The wrapper implements the following types:
    /// * [`AsExpression`]<[`Text`]>
    /// * [`ToSchema`], [`PartialSchema`] for OpenAPI
    #[cfg_attr(feature = "jsonschema", doc = "* [`JsonSchema`][schemasrs::JsonSchema] for JSON schemas")]
    #[derive(Debug, Clone, Display, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, AsExpression, FromSqlRow)]
    #[diesel(sql_type = Text)]
    pub Version for ::semver::Version;
);

charted_core::mk_from_newtype!(from Version as semver::Version);

impl Version {
    /// Forwards to [`semver::Version::parse`] to return this newtype wrapper.
    pub fn parse(v: &str) -> Result<Version, semver::Error> {
        semver::Version::parse(&v.trim_start_matches('v').replace(['x', 'X'], "0")).map(Self)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Version;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("valid semantic version string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(v.to_string())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Version::parse(&v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
#[test]
fn test_deserialization_of_version() {
    assert!(serde_json::from_str::<Version>("\"1.2.x\"").is_ok());
    assert!(serde_json::from_str::<Version>("\"1.x.x\"").is_ok());
    assert!(serde_json::from_str::<Version>("\"1.2.X\"").is_ok());
    assert!(serde_json::from_str::<Version>("\"1.X.X\"").is_ok());
}

////// ============================ SCHEMAS ============================ \\\\\\
impl PartialSchema for Version {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .description(Some("Type that represents a semantic version (https://semver.org)."))
            .pattern(Some(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"))
            .examples([json!("1.2.3")])
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for Version {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Version")
    }
}

#[cfg(feature = "jsonschema")]
impl ::schemars::JsonSchema for Version {
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        <semver::Version as ::schemars::JsonSchema>::schema_id()
    }

    fn schema_name() -> String {
        <semver::Version as ::schemars::JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <semver::Version as ::schemars::JsonSchema>::json_schema(gen)
    }
}

////// ============================ TO SQL ============================ \\\\\\
// credit to this impl (i had a hard time doing this myself):
// https://github.com/oxidecomputer/omicron/blob/d3257b9d8d48fa94ed11020598a723644aec9f05/nexus/db-model/src/semver_version.rs#L124-L136
impl<B> ToSql<Text, B> for Version
where
    for<'c> B: Backend<BindCollector<'c> = RawBytesBindCollector<B>>,
    String: ToSql<Text, B>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, B>) -> diesel::serialize::Result {
        let v = self.0.to_string();
        v.to_sql(&mut out.reborrow())
    }
}

////// ============================ FROM SQL ============================ \\\\\\
impl<B: Backend> FromSql<Text, B> for Version
where
    String: FromSql<Text, B>,
{
    fn from_sql(bytes: <B as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(semver::Version::parse(&<String as FromSql<Text, B>>::from_sql(bytes)?)
            .map(Self)
            .map_err(Box::new)?)
    }
}
