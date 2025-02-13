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

use crate::{name::Name, Ulid};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `NameOrUlid` is a "union" enum that can represent either:
///
/// * [`Name`]
/// * [`Ulid`]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum NameOrUlid {
    Ulid(Ulid),
    Name(Name),
}

impl Display for NameOrUlid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ulid(ulid) => Display::fmt(ulid, f),
            Self::Name(name) => Display::fmt(name, f),
        }
    }
}

impl NameOrUlid {
    /// Returns [`Some`]\([`Name`]\) that was referenced, otherwise `None` is returned
    /// if this is a ULID instance.
    pub fn as_name(&self) -> Option<&Name> {
        match self {
            Self::Name(name) => Some(name),
            _ => None,
        }
    }

    /// Returns [`Some`]\([`Ulid`]\) that was referenced, otherwise `None` is returned
    /// if this is a [`Name`].
    pub fn as_ulid(&self) -> Option<&Ulid> {
        match self {
            Self::Ulid(ulid) => Some(ulid),
            _ => None,
        }
    }
}

impl From<Name> for NameOrUlid {
    fn from(value: Name) -> Self {
        Self::Name(value)
    }
}

impl From<Ulid> for NameOrUlid {
    fn from(value: Ulid) -> Self {
        Self::Ulid(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_name_or_ulid() {
        // Safety: this passes all the validation it requires
        let x = unsafe { Name::new_unchecked("noel") };
        let deserialized = serde_json::from_str::<NameOrUlid>("\"noel\"");
        assert_eq!(deserialized.expect("shouldn't happen"), NameOrUlid::Name(x));

        let x = Ulid::new("01J647WVTPF2W5W99H5MBT0YQE").expect("failed to parse as ulid");
        let deserialized = serde_json::from_str::<NameOrUlid>(&format!("\"{x}\""));
        assert_eq!(deserialized.expect("shouldn't happen"), NameOrUlid::Ulid(x));

        // this could be considered an edge-case since names without `-`, `_`, or `~`
        // can be considered "valid" ulids. so, let's see what happens?
        assert_eq!(
            serde_json::from_str::<NameOrUlid>("\"some3name1with6numbers\"").unwrap(),
            NameOrUlid::Name(unsafe {
                /* Safety: this passes all the validation it requires */
                Name::new_unchecked("some3name1with6numbers")
            })
        );
    }
}
