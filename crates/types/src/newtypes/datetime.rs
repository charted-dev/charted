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

use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::{Timestamp, Timestamptz, TimestamptzSqlite},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utoipa::{
    openapi::{schema::SchemaType, KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, Type},
    PartialSchema, ToSchema,
};

charted_core::create_newtype_wrapper!(
    /// Newtype wrapper for the <code>[`chrono::DateTime`]<[`chrono::Utc`]></code> type.
    ///
    /// The wrapper implements the following types:
    /// * [`AsExpression`]<[`TimestamptzSqlite`]>
    /// * [`AsExpression`]<[`TimestampTz`]>
    /// * [`AsExpression`]<[`Timestamp`]>
    /// * [`ToSchema`], [`PartialSchema`] for OpenAPI
    #[cfg_attr(feature = "jsonschema", doc = "* [`JsonSchema`][schemasrs::JsonSchema] for JSON schemas")]
    #[derive(
        Debug,
        Clone,
        Copy,
        Default,
        Serialize,
        Deserialize,
        PartialEq, Eq,
        PartialOrd, Ord,
        AsExpression,
        FromSqlRow
    )]
    #[diesel(sql_type = TimestamptzSqlite)]
    #[diesel(sql_type = Timestamptz)]
    #[diesel(sql_type = Timestamp)]
    pub DateTime for ::chrono::DateTime<::chrono::Utc>;
);

charted_core::mk_from_newtype!(from DateTime as chrono::DateTime<chrono::Utc>);

impl PartialSchema for DateTime {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
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

////// ============================ TO SQL ============================ \\\\\\
impl ToSql<Timestamptz, Pg> for DateTime
where
    chrono::DateTime<chrono::Utc>: ToSql<Timestamptz, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <chrono::DateTime<chrono::Utc> as serialize::ToSql<Timestamptz, Pg>>::to_sql(&self.0, &mut out.reborrow())
    }
}

impl ToSql<TimestamptzSqlite, Sqlite> for DateTime
where
    chrono::DateTime<chrono::Utc>: ToSql<TimestamptzSqlite, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        <chrono::DateTime<chrono::Utc> as serialize::ToSql<TimestamptzSqlite, Sqlite>>::to_sql(&self.0, out)
    }
}

////// ============================ FROM SQL ============================ \\\\\\
impl FromSql<Timestamptz, Pg> for DateTime
where
    chrono::DateTime<chrono::Utc>: FromSql<Timestamptz, Pg>,
{
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let result: chrono::DateTime<chrono::Utc> =
            <chrono::DateTime<chrono::Utc> as FromSql<Timestamptz, Pg>>::from_sql(bytes)?;

        Ok(Self(result))
    }
}

impl FromSql<TimestamptzSqlite, Sqlite> for DateTime
where
    chrono::DateTime<chrono::Utc>: FromSql<TimestamptzSqlite, Sqlite>,
{
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let result: chrono::DateTime<::chrono::Utc> =
            <chrono::DateTime<chrono::Utc> as FromSql<TimestamptzSqlite, Sqlite>>::from_sql(bytes)?;

        Ok(Self(result))
    }
}

impl FromSql<Timestamp, Pg> for DateTime
where
    chrono::NaiveDateTime: FromSql<Timestamp, Sqlite>,
{
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let datetime = <chrono::NaiveDateTime as FromSql<Timestamp, Pg>>::from_sql(bytes)?;
        let converted = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(datetime, chrono::Utc);

        Ok(Self(converted))
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
