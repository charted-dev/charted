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

mod distribution;
use axum::{
    extract::{rejection::PathRejection, FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
pub use distribution::*;

mod name;
pub use name::*;
use sqlx::{postgres::PgHasArrayType, Database, Decode, Encode, Type};

pub mod entities;
pub mod helm;
pub mod payloads;

pub type DateTime = chrono::DateTime<chrono::Local>;

use crate::server::models::res::{err, ApiResponse, ErrorCode};

use super::ID;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};
use utoipa::{
    openapi::{ObjectBuilder, OneOfBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};
use validator::Validate;

/// Represents a [`semver::Version`] which is safe to use in [`Path`][axum::extract::Path]
/// or as sqlx types.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version(semver::Version);
impl Deref for Version {
    type Target = semver::Version;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        semver::Version::deserialize(deserializer).map(Self)
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Version {
    type Rejection = ApiResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::<semver::Version>::from_request_parts(parts, state)
            .await
            .map(|version| Self(version.0))
            .inspect_err(|e| {
                error!(error = %e, "unable to parse path parameter as a SemVer version");
                sentry::capture_error(&e);
            })
            .map_err(|e| {
                let (code, message) = match e {
                    PathRejection::FailedToDeserializePathParams(_) => (
                        ErrorCode::UnableToParsePathParameter,
                        "was unable to parse valid semver version from path",
                    ),

                    PathRejection::MissingPathParams(_) => {
                        (ErrorCode::MissingPathParameter, "missing required version parameter")
                    }

                    _ => unreachable!(),
                };

                err(StatusCode::BAD_REQUEST, (code, message))
            })
    }
}

impl<'q, DB: Database> Encode<'q, DB> for Version
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as Encode<'q, DB>>::encode_by_ref(&self.0.to_string(), buf)
    }

    fn produces(&self) -> Option<<DB as Database>::TypeInfo> {
        <String as Encode<'q, DB>>::produces(&self.0.to_string())
    }

    fn size_hint(&self) -> usize {
        <String as Encode<'q, DB>>::size_hint(&self.0.to_string())
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Version
where
    String: Decode<'r, DB>,
{
    fn decode(value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let decoded = <String as Decode<'r, DB>>::decode(value)?;
        match semver::Version::parse(&decoded) {
            Ok(version) => Ok(Self(version)),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl<DB: Database> Type<DB> for Version
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

impl PgHasArrayType for Version
where
    String: PgHasArrayType,
{
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as PgHasArrayType>::array_type_info()
    }
}

impl<'s> ToSchema<'s> for Version {
    fn schema() -> (&'s str, RefOr<Schema>) {
        let obj = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .description(Some("Represents a [semantic version](https://semver.org) that both Helm and charted-server accept as versions for Helm charts"))
            .pattern(Some(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"))
            .build();

        ("Version", RefOr::T(Schema::Object(obj)))
    }
}

/// Represents a union enum that can hold a Snowflake ([u64]-based integer)
/// and a Name, which is a String that is validated with the Name regex.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum NameOrSnowflake {
    /// [u64]-based integer that can point to a entity resource.
    Snowflake(u64),

    /// Valid UTF-8 string that is used to point to a entity resource. This
    /// is mainly used for `idOrName` path parameters in any of the REST
    /// API endpoints to help identify a resource by a Name or Snowflake
    /// pointer.
    ///
    /// Names are validated with the following regex: `^([A-z]|-|_|\d{0,9}){1,32}`
    Name(Name),
}

impl Display for NameOrSnowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameOrSnowflake::Snowflake(id) => Display::fmt(id, f),
            NameOrSnowflake::Name(name) => Display::fmt(name, f),
        }
    }
}

impl<'s> ToSchema<'s> for NameOrSnowflake {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "NameOrSnowflake",
            RefOr::T(Schema::OneOf(
                OneOfBuilder::new()
                    .description(Some("Represents a union enum that can hold a Snowflake and a Name, which is a String that is validated with the Name regex."))
                    .item(ID::schema().1)
                    .item(Name::schema().1)
                    .build(),
            )),
        )
    }
}

impl NameOrSnowflake {
    /// Checks if the value is a valid NameOrSnowflake entity.
    pub fn is_valid(&self) -> Result<(), String> {
        match self {
            NameOrSnowflake::Snowflake(flake) => {
                if *flake < 15 {
                    return Err("was not over or equal to 15 in length".into());
                }

                Ok(())
            }

            NameOrSnowflake::Name(s) => Name::check_is_valid(s.to_string()).map_err(|e| format!("{e}")),
        }
    }
}

impl Validate for NameOrSnowflake {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            NameOrSnowflake::Snowflake(_) => Ok(()),
            NameOrSnowflake::Name(name) => name.validate(),
        }
    }
}

impl From<u64> for NameOrSnowflake {
    fn from(value: u64) -> NameOrSnowflake {
        NameOrSnowflake::Snowflake(value)
    }
}

impl From<&str> for NameOrSnowflake {
    fn from(value: &str) -> NameOrSnowflake {
        NameOrSnowflake::Name(value.to_string().into())
    }
}

impl From<String> for NameOrSnowflake {
    fn from(value: String) -> NameOrSnowflake {
        NameOrSnowflake::Name(value.into())
    }
}

impl From<Name> for NameOrSnowflake {
    fn from(value: Name) -> Self {
        NameOrSnowflake::Name(value)
    }
}

impl From<&Name> for NameOrSnowflake {
    fn from(value: &Name) -> Self {
        NameOrSnowflake::Name(Name::new_unchecked(value.as_str()))
    }
}

impl From<&NameOrSnowflake> for NameOrSnowflake {
    fn from(value: &NameOrSnowflake) -> Self {
        match value {
            Self::Snowflake(id) => Self::Snowflake(*id),
            Self::Name(name) => Self::Name(name.clone()),
        }
    }
}
