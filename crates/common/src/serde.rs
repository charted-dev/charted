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

use humantime::parse_duration;
use serde::{
    de::{self, Deserialize, Visitor},
    ser::Serialize,
};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
    str::FromStr,
};

/// Represents a [`Duration`][std::time::Duration] that can be used with
/// serde's Serializer and Deserializer traits.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Duration(pub std::time::Duration);

impl Debug for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmter = humantime::format_duration(self.0);
        <humantime::FormattedDuration as Display>::fmt(&fmter, f)
    }
}

impl FromStr for Duration {
    type Err = humantime::DurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_duration(s).map(Duration)
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u128(self.0.as_millis())
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string of a valid duration or a u64 in milliseconds")
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Duration(std::time::Duration::from_millis(value)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                parse_duration(v).map(Duration).map_err(de::Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(v.as_str())
            }
        }

        deserializer.deserialize_any(DurationVisitor)
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value)
    }
}

impl Deref for Duration {
    type Target = std::time::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::to_string;

    #[derive(Debug, Serialize, Deserialize)]
    struct SomeStruct {
        dur: Duration,
    }

    #[test]
    fn test_serialize_millis() {
        let d = SomeStruct {
            dur: Duration(std::time::Duration::from_millis(200)),
        };

        let serialized = to_string(&d).unwrap();
        let expected = r#"{"dur":200}"#.to_string();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_serialize_str() {
        let d = Duration::from_str("2s").unwrap();
        let stru = SomeStruct { dur: d };

        let serialized = to_string(&stru).unwrap();
        let expected = r#"{"dur":2000}"#.to_string();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_deserialize_millis() {
        let deserialized: SomeStruct = serde_json::from_str(r#"{"dur":2000}"#).unwrap();
        let expected: SomeStruct = SomeStruct {
            dur: Duration(std::time::Duration::from_secs(2)),
        };

        assert_eq!(expected.dur, deserialized.dur);
    }

    #[test]
    fn test_deserialize_str() {
        let deserialized: SomeStruct = serde_json::from_str(r#"{"dur":"2s"}"#).unwrap();
        let expected: SomeStruct = SomeStruct {
            dur: Duration(std::time::Duration::from_secs(2)),
        };

        assert_eq!(expected.dur, deserialized.dur);
    }
}
