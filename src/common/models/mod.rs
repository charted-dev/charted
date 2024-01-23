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
pub use distribution::*;

mod name;
pub use name::*;

pub mod entities;
pub mod helm;
pub mod payloads;

pub type DateTime = chrono::DateTime<chrono::Local>;

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{
    openapi::{OneOfBuilder, RefOr, Schema},
    ToSchema,
};

use super::ID;

/// Represents a union enum that can hold a Snowflake ([u64]-based integer)
/// and a Name, which is a String that is validated with the Name regex.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<&NameOrSnowflake> for NameOrSnowflake {
    fn from(value: &NameOrSnowflake) -> Self {
        match value {
            Self::Snowflake(id) => Self::Snowflake(*id),
            Self::Name(name) => Self::Name(name.clone()),
        }
    }
}
