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

//! Valid UTF-8 string that can be used for names that can be
//! addressed by the API server.
//!
//! * A **Name** is a wrapper for <code>[`Arc`]<[`str`]></code> as opposed of a [`String`]
//!   since a **Name** can be never modified and reflected on the database.
//!
//! * A **Name** is also URL-encoded safe since we only use alphanumeric characters, `-`,
//!   `_`, and `~`.
//!
//! * A **Name** can never overflow since we require names to have a minimum length of 2
//!   and a maximum length of 32.

use crate::{cfg_jsonschema, cfg_openapi};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, ops::Deref, str::FromStr, sync::Arc};

const MAX_LENGTH: usize = 32;
const MIN_LENGTH: usize = 2;

/// Error type when name validation goes wrong.
#[derive(Debug, derive_more::Display)]
pub enum Error {
    #[display("name was over 32 characters")]
    ExceededLength,

    #[display("minimum length is lower or equal to 2.")]
    Minimum,

    #[display("invalid character '{}' received (index {} in input: \"{}\")", ch, at, input)]
    InvalidCharacter {
        input: Cow<'static, str>,
        at: usize,
        ch: char,
    },

    #[display("name cannot be empty")]
    Empty,
}

impl std::error::Error for Error {}

/// Valid UTF-8 string that can be used for names that can be
/// addressed by the API server.
///
/// * A **Name** is a wrapper for <code>[`Arc`]<[`str`]></code> as opposed of a [`String`]
///   since a **Name** can be never modified and reflected on the database.
///
/// * A **Name** is also URL-encoded safe since we only use alphanumeric characters, `-`,
///   `_`, and `~`.
///
/// * A **Name** can never overflow since we require names to have a minimum length of 2
///   and a maximum length of 32.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
#[display("{}", self.as_str())]
pub struct Name(Arc<str>);
impl Name {
    /// Create a new [`Name`] without any input validation.
    ///
    /// ## Safety
    /// We marked this method as `unsafe` since it doesn't do any
    /// input validation. This should be only used by unit
    /// tests.
    pub unsafe fn new_unchecked(v: impl AsRef<str>) -> Name {
        Name(Arc::from(v.as_ref()))
    }

    /// Returns as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Create a new [`Name`] object if `v` is valid input.
    pub fn try_new(v: impl AsRef<str>) -> Result<Name, Error> {
        let name = v.as_ref();
        if name.is_empty() {
            return Err(Error::Empty);
        }

        if name.len() <= MIN_LENGTH {
            return Err(Error::Minimum);
        }

        if name.len() > MAX_LENGTH {
            return Err(Error::ExceededLength);
        }

        let as_lower = name.to_ascii_lowercase();
        for (at, ch) in as_lower.chars().enumerate() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '~' {
                continue;
            }

            return Err(Error::InvalidCharacter {
                input: Cow::Owned(as_lower),
                at,
                ch,
            });
        }

        // Safety: validated the user input above
        Ok(unsafe { Name::new_unchecked(name) })
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s)
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        Name::try_new(s).map_err(D::Error::custom)
    }
}

#[cfg(feature = "__internal_db")]
impl Name {
    pub fn into_column<T: sea_orm::sea_query::IntoIden>(col: T) -> sea_orm::sea_query::ColumnDef {
        sea_orm::sea_query::ColumnDef::new(col).string_len(32).not_null().take()
    }
}

#[cfg(feature = "__internal_db")]
const _: () = {
    use sea_orm::{
        ColIdx, DbErr, QueryResult, TryGetError, TryGetable,
        sea_query::{ArrayType, ColumnType, Value, ValueType, ValueTypeErr},
    };
    use std::any::type_name;

    impl From<Name> for Value {
        fn from(value: Name) -> Self {
            Value::String(Some(Box::new(value.as_str().to_owned())))
        }
    }

    impl TryGetable for Name {
        fn try_get_by<I: ColIdx>(query: &QueryResult, idx: I) -> Result<Self, TryGetError> {
            let contents = <String as TryGetable>::try_get_by(query, idx)?;
            contents.parse::<Name>().map_err(|e| {
                TryGetError::DbErr(DbErr::TryIntoErr {
                    from: type_name::<String>(),
                    into: type_name::<Name>(),
                    source: Box::new(e),
                })
            })
        }
    }

    impl ValueType for Name {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            let contents = <String as ValueType>::try_from(v)?;
            contents.parse::<Name>().map_err(|_| ValueTypeErr)
        }

        fn type_name() -> String {
            "Name".to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::String
        }

        fn column_type() -> ColumnType {
            ColumnType::Char(Some(32))
        }
    }
};

cfg_openapi! {
    use utoipa::{
        PartialSchema,
        ToSchema,
        openapi::{
            RefOr,
            Schema,
            ObjectBuilder,
            Type,

            schema::SchemaType,
        }
    };

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl PartialSchema for Name {
        fn schema() -> RefOr<Schema> {
            let object = ObjectBuilder::new()
                .schema_type(SchemaType::Type(Type::String))
                    .description(Some("Valid UTF-8 string that is used to identify a resource from the REST API in a humane fashion. This is meant to help identify a resource without trying to figure out how to calculate their ID."))
                    .pattern(Some(r"^(?<name>[A-z]|-|_|~|\d{0,9}){1,32}$"))
                    .min_length(Some(1))
                    .max_length(Some(32))
                .build();

            RefOr::T(Schema::Object(object))
        }
    }

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl ToSchema for Name {}
}

cfg_jsonschema! {
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl schemars::JsonSchema for Name {
        fn schema_name() -> Cow<'static, str> {
            Cow::Borrowed("Name")
        }

        fn schema_id() -> Cow<'static, str> {
            Cow::Borrowed("charted::types::Name")
        }

        fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
            schemars::json_schema!({
                "type": "string",
                "minLength": 2,
                "maxLength": 32,
                "pattern": "^([A-z]{2,}|[0-9]|_|-)*$",
            })
        }

        fn inline_schema() -> bool {
            false
        }
    }
}
