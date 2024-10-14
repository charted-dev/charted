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
#![feature(decl_macro)]
// #![feature(trivial_bounds)]
// #![deny(trivial_bounds)]

//! The `charted-types` crate defines types that can be used within the lifecycle
//! of the API server.

mod db;
pub use db::*;

pub(crate) mod util;

pub mod helm;
pub mod name;
pub mod payloads;

use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    query_builder::bind_collector::RawBytesBindCollector,
    serialize::{IsNull, ToSql},
    sql_types::{Text, Timestamp, Timestamptz, TimestamptzSqlite},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{borrow::Cow, fmt::Display};
use utoipa::{
    openapi::{schema::SchemaType, KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, Type},
    PartialSchema, ToSchema,
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
    /// * [`AsExpression`]<[`TimestamptzSqlite`]>
    /// * [`AsExpression`]<[`Timestamptz`]>
    /// * [`utoipa::ToSchema`]
    #[cfg_attr(feature = "jsonschema", doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/schemars/trait.JsonSchema.html)")]
    #[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
    #[diesel(sql_type = TimestamptzSqlite)]
    #[diesel(sql_type = Timestamptz)]
    #[diesel(sql_type = Timestamp)]
    pub DateTime for ::chrono::DateTime<::chrono::Utc>;
}

impl PartialSchema for DateTime {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
            .description(Some("ISO 8601 combined date and time using local time"))
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for DateTime {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DateTime")
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

#[allow(trivial_bounds)]
impl ToSql<Timestamptz, Pg> for DateTime
where
    ::chrono::DateTime<::chrono::Utc>: ToSql<Timestamptz, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <chrono::DateTime<chrono::Utc> as diesel::serialize::ToSql<Timestamptz, Pg>>::to_sql(
            &self.0,
            &mut out.reborrow(),
        )
    }
}

#[allow(trivial_bounds)]
impl FromSql<Timestamptz, Pg> for DateTime
where
    ::chrono::DateTime<::chrono::Utc>: FromSql<Timestamptz, Pg>,
{
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let result: ::chrono::DateTime<::chrono::Utc> =
            <::chrono::DateTime<::chrono::Utc> as FromSql<Timestamptz, Pg>>::from_sql(bytes)?;

        Ok(Self(result))
    }
}

#[allow(trivial_bounds)]
impl ToSql<TimestamptzSqlite, Sqlite> for DateTime
where
    ::chrono::DateTime<::chrono::Utc>: ToSql<TimestamptzSqlite, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        <chrono::DateTime<chrono::Utc> as diesel::serialize::ToSql<TimestamptzSqlite, Sqlite>>::to_sql(&self.0, out)
    }
}

#[allow(trivial_bounds)]
impl FromSql<TimestamptzSqlite, Sqlite> for DateTime
where
    ::chrono::DateTime<::chrono::Utc>: FromSql<TimestamptzSqlite, Sqlite>,
{
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let result: ::chrono::DateTime<::chrono::Utc> =
            <::chrono::DateTime<::chrono::Utc> as FromSql<TimestamptzSqlite, Sqlite>>::from_sql(bytes)?;

        Ok(Self(result))
    }
}

impl FromSql<Timestamp, Sqlite> for DateTime
where
    chrono::NaiveDateTime: FromSql<Timestamp, Sqlite>,
{
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let datetime = <chrono::NaiveDateTime as FromSql<Timestamp, Sqlite>>::from_sql(bytes)?;
        let converted = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(datetime, chrono::Utc);

        Ok(Self(converted))
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

impl PartialSchema for VersionReq {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .description(Some(
                "A semantic version requirement (https://semver.org) that Helm and charted-server supports",
            ))
            .examples([json!(">=1.2.3"), json!("~1")])
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for VersionReq {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("VersionReq")
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, AsExpression, FromSqlRow)]
    #[diesel(sql_type = Text)]
    pub Ulid for ::ulid::Ulid;
}

/// Exposes types from the [`ulid`] crate that can be accessible from other `charted` crates.
pub mod ulid {
    pub use ::ulid::{DecodeError, EncodeError, ULID_LEN};
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

impl PartialSchema for Ulid {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .description(Some("ULID is a unique 128-bit lexicographically sortable identifier"))
            .max_length(Some(ulid::ULID_LEN))
            .examples([serde_json::json!("01D39ZY06FGSCTVN4T2V9PKHFZ")])
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for Ulid {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Ulid")
    }
}

impl ToSql<Text, Pg> for Ulid
where
    str: ToSql<Text, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let mut buf = [0; ulid::ULID_LEN];
        let v = self.array_to_str(&mut buf);

        <str as ToSql<Text, diesel::pg::Pg>>::to_sql(&(*v), &mut out.reborrow())
    }
}

// Sqlite's bind collector doesn't use `RawBytesBindCollector` like Postgres does, so we kind have to
// do it like this. Abeit, not being the best way or probably the recommended way.
impl ToSql<Text, Sqlite> for Ulid {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let v = self.to_string();
        out.set_value(v);

        Ok(IsNull::No)
    }
}

impl<B: Backend> FromSql<Text, B> for Ulid
where
    String: FromSql<Text, B>,
{
    fn from_sql(bytes: <B as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(
            ::ulid::Ulid::from_string(&<String as FromSql<Text, B>>::from_sql(bytes)?)
                .map(Self)
                .map_err(Box::new)?,
        )
    }
}
