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

#![allow(clippy::too_long_first_doc_paragraph)]

//! The `charted-types` crate defines types that can be used within the lifecycle
//! of the API server.

mod db;
pub use db::*;

pub mod helm;
pub mod name;
pub mod payloads;

use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    query_builder::bind_collector::RawBytesBindCollector,
    serialize::ToSql,
    sql_types::{Text, Timestamp, Timestamptz},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{
    openapi::{KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, SchemaType},
    ToSchema,
};

charted_core::mk_from_newtype!(
    DateTime => ::chrono::DateTime<::chrono::Utc>,
    Version => ::semver::Version,
    VersionReq => ::semver::VersionReq,
    Ulid => ::ulid::Ulid
);

charted_core::create_newtype_wrapper! {
    /// Newtype wrapper for the [`chrono::DateTime`]<[`chrono::Utc`]> type. It implements
    /// the following traits:
    ///
    /// * [`AsExpression`]<[`Timestamp`]>
    /// * [`AsExpression`]<[`Timestamptz`]>
    /// * [`utoipa::ToSchema`]
    #[cfg_attr(feature = "jsonschema", doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/schemars/trait.JsonSchema.html)")]
    #[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, AsExpression)]
    #[diesel(sql_type = Timestamp)]
    #[diesel(sql_type = Timestamptz)]
    pub DateTime for ::chrono::DateTime<::chrono::Utc>;
}

impl<'s> ToSchema<'s> for DateTime {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "DateTime",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
                    .description(Some("ISO 8601 combined date and time using local time"))
                    .build(),
            )),
        )
    }
}

#[cfg(feature = "jsonschema")]
impl ::schemars::JsonSchema for DateTime {
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("chrono::DateTime<chrono::Utc>")
    }

    fn schema_name() -> String {
        String::from("DateTime")
    }

    fn json_schema(_: &mut ::schemars::gen::SchemaGenerator) -> ::schemars::schema::Schema {
        ::schemars::schema::SchemaObject {
            instance_type: Some(::schemars::schema::InstanceType::String.into()),
            format: Some("date-time".into()),
            ..Default::default()
        }
        .into()
    }
}

charted_core::create_newtype_wrapper! {
    /// Newtype wrapper for [`semver::Version`] which implements common traits that charted-server uses for
    /// API entities.
    ///
    /// ## Implements
    /// * [`utoipa::ToSchema`]
    /// * [`diesel::AsExpression`], [`diesel::FromSqlRow`]
    #[cfg_attr(feature = "jsonschema", doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/schemars/trait.JsonSchema.html)")]
    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash, AsExpression, FromSqlRow)]
    #[diesel(sql_type = Text)]
    pub Version for semver::Version;
}

impl Version {
    pub fn parse(v: &str) -> Result<Version, semver::Error> {
        semver::Version::parse(v).map(Self)
    }
}

// credit to this impl (i had a hard time doing this myself): https://github.com/oxidecomputer/omicron/blob/d3257b9d8d48fa94ed11020598a723644aec9f05/nexus/db-model/src/semver_version.rs#L124-L136
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

impl<'s, B: Backend> FromSql<Text, B> for Version
where
    &'s str: FromSql<Text, B>,
{
    fn from_sql(bytes: <B as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(semver::Version::parse(<&str as FromSql<Text, B>>::from_sql(bytes)?)
            .map(Self)
            .map_err(Box::new)?)
    }
}

impl<'s> ToSchema<'s> for Version {
    fn schema() -> (&'s str, RefOr<Schema>) {
        ("Version", RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("Type that represents a semantic version (https://semver.org)."))
                .pattern(Some(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"))
                .example(Some(serde_json::json!("1.2.3")))
                .build(),
        )))
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

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <::semver::Version as Display>::fmt(&self.0, f)
    }
}

charted_core::create_newtype_wrapper! {
    /// Newtype wrapper for the [`semver::VersionReq`] type that implements
    /// the following traits:
    ///
    /// * [`utoipa::ToSchema`]
    #[cfg_attr(feature = "jsonschema", doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/schemars/trait.JsonSchema.html)")]
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub VersionReq for ::semver::VersionReq;
}

impl VersionReq {
    pub fn parse(v: &str) -> Result<VersionReq, semver::Error> {
        ::semver::VersionReq::parse(v).map(Self)
    }
}

impl<'s> ToSchema<'s> for VersionReq {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "VersionReq",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .description(Some(
                        "A semantic version requirement (https://semver.org) that Helm and charted-server supports",
                    ))
                    .example(Some(serde_json::json!(">=1.2.3")))
                    .example(Some(serde_json::json!("~1")))
                    .build(),
            )),
        )
    }
}

#[cfg(feature = "jsonschema")]
impl ::schemars::JsonSchema for VersionReq {
    fn schema_id() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("semver::VersionReq")
    }

    fn schema_name() -> String {
        String::from("VersionReq")
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        ::schemars::schema::SchemaObject {
            instance_type: Some(::schemars::schema::InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}

impl Display for VersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <::semver::VersionReq as Display>::fmt(&self.0, f)
    }
}

charted_core::create_newtype_wrapper! {
    /// Newtype wrapper for [`ulid::Ulid`] that implements the following traits:
    ///
    /// * [`AsExpression`]<[`Text`]>
    /// * [`utoipa::ToSchema`]
    /// * [`ToSql`]<[`Text`], [`B`][diesel::backend::Backend]>
    /// * [`FromSql`]<[`Text`], [`B`][diesel::backend::Backend]>
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, AsExpression)]
    #[diesel(sql_type = Text)]
    pub Ulid for ::ulid::Ulid;
}

impl Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <::ulid::Ulid as Display>::fmt(&self.0, f)
    }
}

impl Ulid {
    pub fn new(v: &str) -> Result<Ulid, ulid::DecodeError> {
        ::ulid::Ulid::from_string(v).map(Self)
    }
}

impl<'s> ToSchema<'s> for Ulid {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Ulid",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .description(Some("`Ulid` is a unique 128-bit lexicographically sortable identifier"))
                    .max_length(Some(ulid::ULID_LEN))
                    .example(Some(serde_json::json!("01D39ZY06FGSCTVN4T2V9PKHFZ")))
                    .build(),
            )),
        )
    }
}

impl<B> ToSql<Text, B> for Ulid
where
    for<'c> B: Backend<BindCollector<'c> = RawBytesBindCollector<B>>,
    str: ToSql<Text, B>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, B>) -> diesel::serialize::Result {
        let mut buf = [0; ulid::ULID_LEN];
        let v = self.array_to_str(&mut buf);

        (*v).to_sql(&mut out.reborrow())
    }
}

impl<'s, B: Backend> FromSql<Text, B> for Ulid
where
    &'s str: FromSql<Text, B>,
{
    fn from_sql(bytes: <B as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(ulid::Ulid::from_string(<&str as FromSql<Text, B>>::from_sql(bytes)?)
            .map(Self)
            .map_err(Box::new)?)
    }
}
