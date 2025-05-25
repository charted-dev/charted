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

use super::{Ulid, Version};
use serde::{Deserialize, Serialize};

/// Union enumeration that can represent either a [`Version`] (as a release tag)
/// or [`Ulid`] that is the release's metadata ID.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, derive_more::Display)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum VersionOrUlid {
    Version(Version),
    Ulid(Ulid),
}

impl VersionOrUlid {
    /// Returns [`Some`]\([`Version`]\) that was referenced, otherwise `None` is returned
    /// if this is a ULID instance.
    pub const fn as_version(&self) -> Option<&Version> {
        match self {
            Self::Version(version) => Some(version),
            _ => None,
        }
    }

    /// Returns [`Some`]\([`Ulid`]\) that was referenced, otherwise `None` is returned
    /// if this is a [`Version`].
    pub const fn as_ulid(&self) -> Option<&Ulid> {
        match self {
            Self::Ulid(ulid) => Some(ulid),
            _ => None,
        }
    }
}

impl From<Version> for VersionOrUlid {
    fn from(value: Version) -> Self {
        Self::Version(value)
    }
}

impl From<Ulid> for VersionOrUlid {
    fn from(value: Ulid) -> Self {
        Self::Ulid(value)
    }
}

#[cfg(feature = "openapi")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
/// Produce a [`Parameter`](utoipa::openapi::path::Parameter) with `versionOrId` as the
/// parameter name.
impl utoipa::IntoParams for VersionOrUlid {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        vec![
            utoipa::openapi::path::Parameter::builder()
            .name("versionOrId")
                .required(utoipa::openapi::Required::True)
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .description(Some("A path parameter that can take either a **Version** representing the release tag or a **Ulid** which can represent a specific release by its unique identifier"))
                .schema(Some(utoipa::openapi::RefOr::Ref(
                    utoipa::openapi::Ref::from_schema_name("VersionOrUlid"),
                )))
                .build(),
        ]
    }
}
