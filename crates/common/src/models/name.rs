// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use crate::hashmap;
use serde::{
    de::{Deserialize, Visitor},
    ser::Serialize,
};
use serde_json::{Number, Value};
use sqlx::{
    database::{HasArguments, HasValueRef},
    encode::IsNull,
    error::BoxDynError,
    postgres::PgHasArrayType,
    Database, Decode, Encode, Type,
};
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Clone)]
pub enum NameError {
    /// Variant that the given input had an invalid character.
    InvalidCharacter {
        input: String,
        at: usize,
        ch: char,
    },
    ExceededMax(usize),

    /// Variant that the given input was not valid UTF-8.
    InvalidUtf8,
    Empty,
}

impl Debug for NameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter { input, at, ch } => f.write_fmt(format_args!(
                "invalid input [{input}]: character '{ch}' in index {at} was not a valid character"
            )),

            Self::ExceededMax(over) => f.write_fmt(format_args!(
                "name went over {over} characters, expected Name to contain 1..=32 in length"
            )),

            Self::InvalidUtf8 => f.write_str("received invalid utf-8"),
            Self::Empty => f.write_str("name received was empty"),
        }
    }
}

impl Display for NameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter { input, at, ch } => f.write_fmt(format_args!(
                "invalid input [{input}]: character '{ch}' in index {at} was not a valid character"
            )),

            Self::ExceededMax(over) => f.write_fmt(format_args!(
                "name went over {over} characters, expected Name to contain 1..=32 in length"
            )),

            Self::InvalidUtf8 => f.write_str("received invalid utf-8"),
            Self::Empty => f.write_str("name received was empty"),
        }
    }
}

impl std::error::Error for NameError {}

impl NameError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidCharacter { .. } => "INVALID_NAME",
            Self::ExceededMax(_) => "EXCEEDED_NAME_MAX_LENGTH",
            Self::InvalidUtf8 => "INVALID_UTF8",
            Self::Empty => "EMPTY_NAME",
        }
    }
}

/// Name is a valid UTF-8 string that is used to identify a resource from the REST
/// API in a humane fashion. This is meant to help identify a resource without
/// trying to figure out how to calculate their ID.
///
/// **Name** has a strict ruleset on how it can be parsed:
///
/// * Only UTF-8 strings are valid.
/// * Only alphanumeric characters, `-`, and `_` are allowed.
/// * They must contain a length of two minimum and 32 maximum.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(Arc<str>);
impl Name {
    /// Checks whether if the `input` is a valid Name or not.
    #[tracing::instrument(name = "charted.name.validate")]
    pub fn check_is_valid<S: AsRef<str> + Debug>(input: S) -> Result<(), NameError> {
        let name = input.as_ref();
        if name.is_empty() {
            return Err(NameError::Empty);
        }

        if name.len() > 32 {
            let over = name.len() - 32;
            return Err(NameError::ExceededMax(over));
        }

        std::str::from_utf8(name.as_bytes()).map_err(|_| NameError::InvalidUtf8)?;
        for (at, ch) in name.chars().enumerate() {
            // if the character is alphanumeric (a-z | A-Z | 0-9), then let's
            // continue.
            if ch.is_alphanumeric() {
                continue;
            }

            // Names are allowed to have underscores
            if ch == '_' {
                continue;
            }

            // Names are allowed to have dashes
            if ch == '-' {
                continue;
            }

            return Err(NameError::InvalidCharacter {
                input: name.to_string(),
                at,
                ch,
            });
        }

        Ok(())
    }

    /// Creates a new [`Name`], but does check if it is a valid Name. You
    /// can use the `new_unchecked` method to not check if it is valid.
    pub fn new<S: AsRef<str>>(input: S) -> Result<Name, NameError> {
        match Name::check_is_valid(input.as_ref()) {
            Ok(()) => Ok(Name::new_unchecked(input)),
            Err(e) => Err(e),
        }
    }

    /// Creates a new [`Name`], but doesn't check if the name is valid or not.
    pub fn new_unchecked<S: AsRef<str>>(input: S) -> Name {
        Name(Arc::from(input.as_ref()))
    }

    /// Checks whether if this [`Name`] is valid or not. This is useful if this
    /// came from the `new_unchecked` method.
    pub fn is_valid(&self) -> Result<(), NameError> {
        Name::check_is_valid(self.0.clone())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Name {
        Name(Arc::from(value))
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Name(Arc::from(value.as_str()))
    }
}

impl Validate for Name {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        match self.is_valid() {
            Ok(()) => Ok(()),
            Err(e) => {
                let mut params = hashmap!(Cow<'_, str>, serde_json::Value);
                if let NameError::InvalidCharacter { at, ch, .. } = e {
                    params.insert(Cow::Borrowed("index"), Value::Number(Number::from(at)));
                    params.insert(Cow::Borrowed("char"), Value::String(ch.to_string()));
                }

                errors.add(
                    "name",
                    ValidationError {
                        code: Cow::Borrowed(e.code()),
                        message: Some(Cow::Owned(e.to_string())),
                        params,
                    },
                );

                Err(errors)
            }
        }
    }
}

impl<'s> ToSchema<'s> for Name {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Name",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .description(Some("A valid UTF-8 string that is used to identify a resource from the REST API in a humane fashion. This is meant to help identify a resource without trying to calculate the resource's Snowflake on the first try."))
                    .pattern(Some("([A-z]|-|_|\\d{0,9}){1,32}"))
                    .min_length(Some(1))
                    .max_length(Some(32))
                    .build()
            ))
        )
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Name").field(&self.0.deref()).finish()
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
        struct NameVisitor;

        impl<'de> Visitor<'de> for NameVisitor {
            type Value = Name;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("generic string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Name::new_unchecked(v))
            }
        }

        deserializer.deserialize_str(NameVisitor)
    }
}

impl Default for Name {
    fn default() -> Self {
        Self::new_unchecked("")
    }
}

impl<'q, DB: Database> Encode<'q, DB> for Name
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        <String as Encode<'q, DB>>::encode_by_ref(&self.to_string(), buf)
    }

    fn produces(&self) -> Option<<DB as Database>::TypeInfo> {
        <String as Encode<'q, DB>>::produces(&self.to_string())
    }

    fn size_hint(&self) -> usize {
        <String as Encode<'q, DB>>::size_hint(&self.to_string())
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Name
where
    String: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Name, BoxDynError> {
        <String as Decode<'r, DB>>::decode(value).map(Name::from)
    }
}

impl<DB: Database> Type<DB> for Name
where
    String: Type<DB>,
{
    fn type_info() -> <DB as Database>::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

impl PgHasArrayType for Name
where
    String: PgHasArrayType,
{
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as PgHasArrayType>::array_type_info()
    }
}
