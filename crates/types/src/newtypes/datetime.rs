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

use crate::{cfg_jsonschema, cfg_openapi};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Newtype wrapper for <code>[`chrono::DateTime`]<[`chrono::Utc`]></code>.
///
/// This newtype wrapper implements all the standard library types and more
/// configured by feature flags:
#[cfg_attr(
    feature = "openapi",
    doc = "* [`utoipa::PartialSchema`](https://docs.rs/utoipa/*/utoipa/trait.PartialSchema.html), [`utoipa::ToSchema`](https://docs.rs/utoipa/*/utoipa/trait.ToSchema.html) (via the `openapi` crate feature)"
)]
#[cfg_attr(
    feature = "jsonschema",
    doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/utoipa/trait.JsonSchema.html) (via the `jsonschema` crate feature)"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::From,
    derive_more::Deref,
)]
#[display("{}", self.0)]
pub struct DateTime(chrono::DateTime<Utc>);

impl DateTime {
    /// Get the current timestamp as the newtype.
    pub fn now() -> Self {
        Utc::now().into()
    }
}

impl From<DateTime> for chrono::DateTime<Utc> {
    fn from(DateTime(value): DateTime) -> Self {
        value
    }
}

cfg_openapi! {
    use utoipa::{
        openapi::{schema::SchemaType, ObjectBuilder, RefOr, Schema, Type, SchemaFormat, KnownFormat},
        PartialSchema, ToSchema,
    };

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl PartialSchema for DateTime {
        fn schema() -> RefOr<Schema> {
            let object = ObjectBuilder::new()
                .schema_type(SchemaType::Type(Type::String))
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
                .build();

            RefOr::T(Schema::Object(object))
        }
    }

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl ToSchema for DateTime {}
}

cfg_jsonschema! {
    use std::borrow::Cow;

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "jsonschema")))]
    impl schemars::JsonSchema for DateTime {
        fn schema_name() -> Cow<'static, str> {
            Cow::Borrowed("DateTime")
        }

        fn schema_id() -> Cow<'static, str> {
            Cow::Borrowed("chrono::DateTime<chrono::Utc>")
        }

        fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
            schemars::json_schema!({
                "type": "string",
                "format": "date-time",
            })
        }
    }
}

#[cfg(feature = "__internal_db")]
const _: () = {
    use sea_orm::{
        ColIdx, QueryResult, TryGetError, TryGetable,
        sea_query::{ArrayType, ColumnType, Value, ValueType, ValueTypeErr},
    };

    impl TryGetable for DateTime {
        fn try_get_by<I: ColIdx>(query: &QueryResult, idx: I) -> Result<Self, TryGetError> {
            <chrono::DateTime<chrono::Utc> as TryGetable>::try_get_by(query, idx).map(DateTime)
        }
    }

    impl ValueType for DateTime {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            <chrono::DateTime<chrono::Utc> as ValueType>::try_from(v).map(Self)
        }

        fn type_name() -> String {
            <chrono::DateTime<chrono::Utc> as ValueType>::type_name()
        }

        fn array_type() -> ArrayType {
            <chrono::DateTime<chrono::Utc> as ValueType>::array_type()
        }

        fn column_type() -> ColumnType {
            <chrono::DateTime<chrono::Utc> as ValueType>::column_type()
        }
    }
};
