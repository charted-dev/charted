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

use crate::{cfg_jsonschema, cfg_openapi};
use serde::{Deserialize, Serialize};

/// Newtype wrapper for [`semver::Version`].
///
/// This newtype wrapper implements all the standard library types and more
/// configured by feature flags.
///
#[cfg_attr(
    feature = "openapi",
    doc = "* [`utoipa::PartialSchema`], [`utoipa::ToSchema`] (via the `openapi` crate feature)"
)]
#[cfg_attr(
    feature = "jsonschema",
    doc = "* [`schemars::JsonSchema`] (via the `jsonschema` crate feature)"
)]
///
/// [`utoipa::PartialSchema`]: https://docs.rs/utoipa/*/utoipa/trait.PartialSchema.html
/// [`utoipa::ToSchema`]: https://docs.rs/utoipa/*/utoipa/trait.ToSchema.html
/// [`schemars::JsonSchema`]: https://docs.rs/schemars/*/utoipa/trait.JsonSchema.html
#[derive(
    Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, derive_more::Display, derive_more::From, derive_more::Deref,
)]
pub struct Version(semver::Version);
impl Version {
    /// Forwarded method to [`semver::Version::parse`] but replaces
    /// all instances of `.x` and `.X` with a zero.
    pub fn parse(v: &str) -> Result<Self, semver::Error> {
        let v = v.trim_start_matches('v').replace(['x', 'X'], "0");
        semver::Version::parse(&v).map(Self)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = Version;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("valid semantic version string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(v.to_string())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Version::parse(&v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
#[test]
fn test_deserialization_of_version() {
    assert!(serde_json::from_str::<Version>("\"1.2.x\"").is_ok());
    assert!(serde_json::from_str::<Version>("\"1.x.x\"").is_ok());
    assert!(serde_json::from_str::<Version>("\"1.2.X\"").is_ok());
    assert!(serde_json::from_str::<Version>("\"1.X.X\"").is_ok());
}

cfg_openapi! {
    use serde_json::json;
    use utoipa::{
        PartialSchema,
        ToSchema,
        openapi::{
            RefOr,
            Schema,
            ObjectBuilder,
            Type,
            schema::SchemaType
        }
    };

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl PartialSchema for Version {
        fn schema() -> RefOr<Schema> {
            let object = ObjectBuilder::new()
                .schema_type(SchemaType::Type(Type::String))
                .description(Some("Type that represents a semantic version (https://semver.org)."))
                .pattern(Some(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"))
                .examples([json!("1.2.3")])
                .build();

            RefOr::T(Schema::Object(object))
        }
    }

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl ToSchema for Version {}
}

cfg_jsonschema! {
    use schemars::JsonSchema;

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "jsonschema")))]
    impl JsonSchema for Version {
        fn schema_id() -> ::std::borrow::Cow<'static, str> {
            <semver::Version as JsonSchema>::schema_id()
        }

        fn schema_name() -> String {
            <semver::Version as JsonSchema>::schema_name()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            <semver::Version as JsonSchema>::json_schema(gen)
        }
    }
}

#[cfg(feature = "__internal_db")]
const _: () = {
    use sea_orm::{
        sea_query::{ArrayType, ColumnType, Value, ValueType, ValueTypeErr},
        ColIdx, DbErr, QueryResult, TryGetError, TryGetable,
    };
    use std::any::type_name;

    impl From<Version> for Value {
        fn from(v: Version) -> Self {
            Value::String(Some(Box::new(v.0.to_string())))
        }
    }

    impl TryGetable for Version {
        fn try_get_by<I: ColIdx>(query: &QueryResult, idx: I) -> Result<Self, TryGetError> {
            let contents = <String as TryGetable>::try_get_by(query, idx)?;
            contents.parse::<semver::Version>().map(Self).map_err(|e| {
                TryGetError::DbErr(DbErr::TryIntoErr {
                    from: type_name::<String>(),
                    into: type_name::<semver::Version>(),
                    source: Box::new(e),
                })
            })
        }
    }

    impl ValueType for Version {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            let contents = <String as ValueType>::try_from(v)?;
            contents.parse::<semver::Version>().map(Self).map_err(|_| ValueTypeErr)
        }

        fn type_name() -> String {
            "Version".to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::String
        }

        fn column_type() -> ColumnType {
            ColumnType::Char(None)
        }
    }
};

/// Newtype wrapper for [`semver::VersionReq`].
///
/// This newtype wrapper implements all the standard library types and more
/// configured by feature flags.
///
#[cfg_attr(
    feature = "openapi",
    doc = "* [`utoipa::PartialSchema`], [`utoipa::ToSchema`] (via the `openapi` crate feature)"
)]
#[cfg_attr(
    feature = "jsonschema",
    doc = "* [`schemars::JsonSchema`] (via the `jsonschema` crate feature)"
)]
///
/// [`utoipa::PartialSchema`]: https://docs.rs/utoipa/*/utoipa/trait.PartialSchema.html
/// [`utoipa::ToSchema`]: https://docs.rs/utoipa/*/utoipa/trait.ToSchema.html
/// [`schemars::JsonSchema`]: https://docs.rs/schemars/*/utoipa/trait.JsonSchema.html
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, derive_more::Display, derive_more::From, derive_more::Deref,
)]
pub struct VersionReq(semver::VersionReq);

cfg_openapi! {
    use serde_json::json;
    use utoipa::{
        PartialSchema,
        ToSchema,
        openapi::{
            RefOr,
            Schema,
            ObjectBuilder,
            Type,
            schema::SchemaType
        }
    };

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl PartialSchema for VersionReq {
        fn schema() -> RefOr<Schema> {
            let object = ObjectBuilder::new()
                .schema_type(SchemaType::Type(Type::String))
                .description(Some(
                    "A semantic version requirement (https://semver.org) that Helm and charted-server supports",
                ))
                .examples([json!(">=1.2.3"), json!("~1")])
                .build();

            RefOr::T(Schema::Object(object))
        }
    }

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl ToSchema for VersionReq {}
}

cfg_jsonschema! {
    use schemars::JsonSchema;

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "jsonschema")))]
    impl JsonSchema for VersionReq {
        fn schema_id() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("semver::VersionReq")
        }

        fn schema_name() -> String {
            String::from("VersionReq")
        }

        fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            ::schemars::schema::SchemaObject {
                instance_type: Some(::schemars::schema::InstanceType::String.into()),
                ..Default::default()
            }
            .into()
        }
    }
}

#[cfg(feature = "__internal_db")]
const _: () = {
    use sea_orm::{
        sea_query::{ArrayType, ColumnType, Value, ValueType, ValueTypeErr},
        ColIdx, DbErr, QueryResult, TryGetError, TryGetable,
    };

    impl TryGetable for VersionReq {
        fn try_get_by<I: ColIdx>(query: &QueryResult, idx: I) -> Result<Self, TryGetError> {
            let contents = <String as TryGetable>::try_get_by(query, idx)?;
            contents.parse::<semver::VersionReq>().map(Self).map_err(|e| {
                TryGetError::DbErr(DbErr::TryIntoErr {
                    from: ::std::any::type_name::<String>(),
                    into: ::std::any::type_name::<semver::VersionReq>(),
                    source: Box::new(e),
                })
            })
        }
    }

    impl ValueType for VersionReq {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            let contents = <String as ValueType>::try_from(v)?;
            contents
                .parse::<semver::VersionReq>()
                .map(Self)
                .map_err(|_| ValueTypeErr)
        }

        fn type_name() -> String {
            "VersionReq".to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::String
        }

        fn column_type() -> ColumnType {
            ColumnType::Char(None)
        }
    }
};
