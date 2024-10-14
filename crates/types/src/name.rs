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
    serialize::ToSql,
    sql_types::Text,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display, ops::Deref, str::FromStr, sync::Arc};
use utoipa::{
    openapi::{schema::SchemaType, ObjectBuilder, RefOr, Schema, Type},
    PartialSchema, ToSchema,
};

#[cfg(feature = "jsonschema")]
use schemars::{gen::*, schema::*, JsonSchema};

#[derive(Debug)]
pub enum Error {
    /// When a name was over 32 characters. The first element is how many characters
    /// it surpassed.
    ExceededMaximumLength(usize),

    /// Variant where the name was empty.
    Empty,

    /// Variant that the given input had an invalid character.
    InvalidChar {
        /// Input that was given
        input: Cow<'static, str>,

        /// Index from the input where it was found.
        at: usize,

        /// The bad character itself
        ch: char,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error as E;
        match self {
            E::InvalidChar { input, at, ch } => {
                write!(f, "invalid character '{ch}' at {at} from given input: [{input}]")
            }

            E::ExceededMaximumLength(over) => write!(f, "exceeded {over} characters"),
            E::Empty => f.write_str("name cannot be empty"),
        }
    }
}

impl std::error::Error for Error {}

/// Name is a valid UTF-8 string that is used to identify a resource from the REST
/// API in a humane fashion. This is meant to help identify a resource without
/// trying to figure out how to calculate their ID.
///
/// **Name** has a strict ruleset on how it can be parsed:
///
/// * Only UTF-8 strings are valid.
/// * Only alphanumeric characters, `-`, `_`, and `~` are allowed.
/// * They must contain a length of two minimum and 32 maximum.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct Name(Arc<str>);
impl Name {
    /// Constructs a [`Name`] instance if `input` follows the ruleset, otherwise
    /// `Error` is returned.
    pub fn try_new<S: AsRef<str>>(input: S) -> Result<Name, Error> {
        let name = input.as_ref();
        if name.is_empty() {
            return Err(Error::Empty);
        }

        if name.len() > 32 {
            let over = name.len() - 32;
            return Err(Error::ExceededMaximumLength(over));
        }

        let lower = name.to_ascii_lowercase();
        for (at, ch) in lower.chars().enumerate() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '~' {
                continue;
            }

            return Err(Error::InvalidChar {
                input: Cow::Owned(lower),
                at,
                ch,
            });
        }

        // Safety: validated the user input above
        Ok(unsafe { Name::new_unchecked(name) })
    }

    /// Returns a string slice of the given name.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Create a [`Name`] while going through no validation.
    ///
    /// ## Safety
    /// The [`Name::new_unchecked`] method is marked *unsafe* due to giving
    /// any user input, which violates the validation contract.
    pub unsafe fn new_unchecked<S: AsRef<str>>(input: S) -> Name {
        Name(Arc::from(input.as_ref()))
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
        Name::try_new(s)
    }
}

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

impl ToSchema for Name {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Name")
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self)
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let value = String::deserialize(deserializer)?;
        Name::try_new(value).map_err(D::Error::custom)
    }
}

impl<B: Backend> FromSql<Text, B> for Name
where
    String: FromSql<Text, B>,
{
    fn from_sql(bytes: <B as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let name = <String as FromSql<Text, B>>::from_sql(bytes)?;
        Name::try_new(name).map_err(Into::into)
    }
}

impl ToSql<Text, Pg> for Name {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <str as ToSql<Text, Pg>>::to_sql(self.as_str(), out)
    }
}

impl ToSql<Text, Sqlite> for Name {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        <str as ToSql<Text, Sqlite>>::to_sql(self.as_str(), out)
    }
}

#[cfg(feature = "jsonschema")]
impl JsonSchema for Name {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("charted::types::Name")
    }

    fn schema_name() -> String {
        String::from("Name")
    }

    fn json_schema(_: &mut SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(InstanceType::String.into())),
            string: Some(
                StringValidation {
                    min_length: Some(2),
                    max_length: Some(32),
                    pattern: Some("^([A-z]{2,}|[0-9]|_|-)*$".into()),
                }
                .into(),
            ),

            ..Default::default()
        })
    }
}
