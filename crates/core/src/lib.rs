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

use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use azalia::lazy;
use eyre::eyre;
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use tracing::error;

pub mod avatars;
pub mod multipart;
pub mod openapi;
pub mod pagination;
pub mod redis;
pub mod response;

#[cfg(feature = "testing")]
pub mod testing;

use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};
use serde_json::Value;
use std::borrow::Cow;

/// Represents the REST version that an API controller is supported on.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr,
)]
#[repr(u8)]
#[non_exhaustive]
pub enum APIVersion {
    /// v1
    #[default]
    V1 = 1,
}

impl APIVersion {
    pub fn as_str(&self) -> &str {
        match self {
            APIVersion::V1 => "v1",
        }
    }
}

impl std::fmt::Display for APIVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JsonSchema for APIVersion {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("charted::server::APIVersion")
    }

    fn schema_name() -> String {
        String::from("APIVersion")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(InstanceType::Number.into())),
            enum_values: Some(vec![Value::Number(1.into())]),

            ..Default::default()
        })
    }
}

impl From<u8> for APIVersion {
    fn from(value: u8) -> Self {
        match value {
            1 => APIVersion::V1,
            _ => panic!("reached an unexpected value for From<u8> -> APIVersion"),
        }
    }
}

impl From<APIVersion> for u8 {
    fn from(value: APIVersion) -> Self {
        match value {
            APIVersion::V1 => 1,
        }
    }
}

impl From<APIVersion> for serde_json::Number {
    fn from(value: APIVersion) -> Self {
        match value {
            APIVersion::V1 => serde_json::Number::from(1),
        }
    }
}

pub static ARGON2: Lazy<Argon2<'static>> = lazy!(Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()));

/// Hashes a password into a Argon2-based password that is safely secure to store.
pub fn hash_password<P: AsRef<[u8]>>(password: P) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.as_ref(), &salt)
        .map(|hash| hash.to_string())
        .inspect_err(|e| {
            error!(error = %e, "unable to compute password");
        })
        .map_err(|e| eyre!(e))
}
