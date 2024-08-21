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

use serde::{de, Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
    str::FromStr,
};
use utoipa::{
    openapi::{KnownFormat, ObjectBuilder, OneOfBuilder, RefOr, Schema, SchemaFormat, SchemaType},
    ToSchema,
};

/// Newtype wrapper for [`std::time::Duration`] that implements [`serde::Serialize`], [`serde::Deserialize`]
/// and [`utoipa::ToSchema`].
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Duration(::std::time::Duration);

impl Duration {
    /// Creates a new `Duration` from the specified number of whole seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use charted_core::serde::Duration;
    ///
    /// let duration = Duration::from_secs(5);
    ///
    /// assert_eq!(5, duration.as_secs());
    /// assert_eq!(0, duration.subsec_nanos());
    /// ```
    pub const fn from_secs(secs: u64) -> Duration {
        Duration(::std::time::Duration::from_secs(secs))
    }
}

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
        humantime::parse_duration(s).map(Duration)
    }
}

/// [`serde::Serialize`] for [`std::time::Duration`]: serialized as a u128 value
/// pointed to the whole millisecond duration.
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
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Duration;

            fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.write_str("a string of a valid duration or a `u64` value")
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Duration(std::time::Duration::from_millis(value)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                humantime::parse_duration(v).map(Duration).map_err(de::Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(v.as_str())
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value)
    }
}

impl From<Duration> for std::time::Duration {
    fn from(value: Duration) -> Self {
        value.0
    }
}

impl Deref for Duration {
    type Target = ::std::time::Duration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> ToSchema<'s> for Duration {
    fn schema() -> (&'s str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
        let oneof = OneOfBuilder::new()
            .description(Some("Generalised type to represents a span of time. charted-server supports passing in a unsigned 64-bit integer (represented in milliseconds) or a human-based format (like `1s`)."))
            .item({
                let object = ObjectBuilder::new()
                    .schema_type(SchemaType::Number)
                    .format(Some(SchemaFormat::KnownFormat(KnownFormat::UInt64)))
                    .description(Some("A span of time represented in milliseconds"));

                Schema::Object(object.build())
            })
            .item({
                let object = ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .description(Some("Allows passing in durations as a string (i.e, `1s`, `15 days`, etc.)"));

                Schema::Object(object.build())
            })
            .build();

        ("Duration", RefOr::T(Schema::OneOf(oneof)))
    }
}

#[cfg(feature = "merge")]
impl ::azalia::config::merge::Merge for Duration {
    fn merge(&mut self, other: Duration) {
        // if both durations are zero, then don't attempt to merge
        if self.is_zero() && other.is_zero() {
            return;
        }

        // If `self` isn't zero AND `other` is zero, don't attempt to merge
        if !self.is_zero() && other.is_zero() {
            return;
        }

        *self = other;
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
