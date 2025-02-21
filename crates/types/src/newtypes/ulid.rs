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
use serde::{Deserialize, Serialize};

/// Newtype wrapper for [`ulid::Ulid`].
///
/// This newtype wrapper implements all the standard library types and more
/// configured by feature flags.
///
#[cfg_attr(
    feature = "openapi",
    doc = "* [`utoipa::PartialSchema`], [`utoipa::ToSchema`] (via the `openapi` crate feature)"
)]
#[cfg_attr(
    feature = "jsonschema",
    doc = "* [`schemars::JsonSchema`] (via the `jsonschema` crate feature)"
)]
///
/// [`ulid::Ulid`]: https://docs.rs/ulid/*/ulid/struct.Ulid.html
/// [`utoipa::PartialSchema`]: https://docs.rs/utoipa/*/utoipa/trait.PartialSchema.html
/// [`utoipa::ToSchema`]: https://docs.rs/utoipa/*/utoipa/trait.ToSchema.html
/// [`schemars::JsonSchema`]: https://docs.rs/schemars/*/utoipa/trait.JsonSchema.html
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::Deref,
)]
pub struct Ulid(::ulid::Ulid);

impl Ulid {
    /// Forwards to the [`Ulid::from_string`] method to create this newtype wrapper.
    ///
    /// [`Ulid::from_string`]: https://docs.rs/ulid/*/ulid/struct.Ulid.html#method.from_string
    pub fn new(id: &str) -> Result<Ulid, ulid::DecodeError> {
        ::ulid::Ulid::from_string(id).map(Self)
    }
}

/// Re-export common types from the [`ulid`][::ulid] crate.
#[allow(clippy::module_inception)]
pub mod ulid {
    #[allow(unused)]
    pub use ::ulid::{DecodeError, EncodeError, ULID_LEN};
}

cfg_openapi! {
    use utoipa::{
        openapi::{schema::SchemaType, ObjectBuilder, RefOr, Schema, Type},
        PartialSchema, ToSchema,
    };

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

    impl ToSchema for Ulid {}
}

cfg_jsonschema! {
    use schemars::{JsonSchema, schema::*, r#gen::SchemaGenerator};
    use std::borrow::Cow;

    impl JsonSchema for Ulid {
        fn is_referenceable() -> bool {
            false
        }

        fn schema_name() -> String {
            "Ulid".to_string()
        }

        fn schema_id() -> Cow<'static, str> {
            Cow::Borrowed("ulid::Ulid")
        }

        fn json_schema(_: &mut SchemaGenerator) -> Schema {
            SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                //format: Some("ulid".to_string()),
                ..Default::default()
            }
            .into()
        }
    }
}

#[cfg(feature = "__internal_db")]
const _: () = {
    use sea_orm::{
        ColIdx, DbErr, QueryResult, TryFromU64, TryGetError, TryGetable,
        sea_query::{ArrayType, ColumnType, Nullable, Value, ValueType, ValueTypeErr},
    };
    use std::any::type_name;

    impl TryFromU64 for Ulid {
        fn try_from_u64(_: u64) -> Result<Self, DbErr> {
            Err(DbErr::ConvertFromU64("ulid"))
        }
    }

    impl Nullable for Ulid {
        fn null() -> sea_orm::Value {
            Value::String(None)
        }
    }

    impl From<Ulid> for Value {
        fn from(value: Ulid) -> Self {
            Value::String(Some(Box::new(value.0.to_string())))
        }
    }

    impl TryGetable for Ulid {
        fn try_get_by<I: ColIdx>(query: &QueryResult, idx: I) -> Result<Self, TryGetError> {
            let contents = <String as TryGetable>::try_get_by(query, idx)?;
            contents.parse::<::ulid::Ulid>().map(Self).map_err(|e| {
                TryGetError::DbErr(DbErr::TryIntoErr {
                    from: type_name::<String>(),
                    into: type_name::<::ulid::Ulid>(),
                    source: Box::new(e),
                })
            })
        }
    }

    impl ValueType for Ulid {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            let contents = <String as ValueType>::try_from(v)?;
            contents.parse::<::ulid::Ulid>().map(Self).map_err(|_| ValueTypeErr)
        }

        fn type_name() -> String {
            "Ulid".to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::String
        }

        fn column_type() -> ColumnType {
            ColumnType::Char(None)
        }
    }
};
