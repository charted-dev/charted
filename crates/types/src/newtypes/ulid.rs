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

use std::borrow::Cow;

use derive_more::derive::Display;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{schema::SchemaType, ObjectBuilder, RefOr, Schema, Type},
    PartialSchema, ToSchema,
};

charted_core::create_newtype_wrapper!(
    /// Newtype wrapper for the [`ulid::Ulid`] type.
    ///
    /// This newtype wrapper implements the following traits:
    /// * [`AsExpression`]<[`Text`]>
    /// * [`FromSql`], [`ToSql`]
    /// * [`ToSchema`], [`PartialSchema`] for OpenAPI
    #[derive(
        Debug,
        Clone,
        Copy,
        Display,
        Serialize,
        Deserialize,
        PartialEq, Eq,
        PartialOrd, Ord,
        AsExpression,
        FromSqlRow
    )]
    #[diesel(sql_type = Text)]
    pub Ulid for ::ulid::Ulid;
);

charted_core::mk_from_newtype!(from Ulid as ::ulid::Ulid);

impl Ulid {
    /// Forwards to the [`Ulid::from_string`][::ulid::Ulid::from_string] method
    /// to create this newtype wrapper.
    pub fn new(id: &str) -> Result<Ulid, ulid::DecodeError> {
        ::ulid::Ulid::from_string(id).map(Self)
    }
}

/// Re-export common types from the [`ulid`][::ulid] crate.
pub mod ulid {
    pub use ::ulid::{DecodeError, EncodeError, ULID_LEN};
}

////// ============================ SCHEMAS ============================ \\\\\\
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

////// ============================ TO SQL ============================ \\\\\\
impl ToSql<Text, Pg> for Ulid
where
    str: ToSql<Text, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let mut buf = [0; ulid::ULID_LEN];
        let v = self.array_to_str(&mut buf);

        <str as ToSql<Text, Pg>>::to_sql(&(*v), &mut out.reborrow())
    }
}

// We can't rely on `RawBytesBindCollector` since the SQLite backend doesn't implement it. So,
// we need to do it this way. Why? I wish I knew. - Noel
impl ToSql<Text, Sqlite> for Ulid {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let v = self.to_string();
        out.set_value(v);

        Ok(IsNull::No)
    }
}

////// ============================ FROM SQL ============================ \\\\\\
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
