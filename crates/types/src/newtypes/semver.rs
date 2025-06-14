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
use std::hint;

/// Newtype wrapper for [`semver::Version`].
///
/// This newtype wrapper implements all the standard library types and more
/// configured by feature flags.
#[cfg_attr(
    feature = "openapi",
    doc = "* [`utoipa::PartialSchema`](https://docs.rs/utoipa/*/utoipa/trait.PartialSchema.html), [`utoipa::ToSchema`](https://docs.rs/utoipa/*/utoipa/trait.ToSchema.html) (via the `openapi` crate feature)"
)]
#[cfg_attr(
    feature = "jsonschema",
    doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/utoipa/trait.JsonSchema.html) (via the `jsonschema` crate feature)"
)]
#[derive(
    Debug,
    Clone,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::From,
    derive_more::Deref,
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
        IntoParams,
        openapi::{
            Required,
            RefOr,
            Schema,
            ObjectBuilder,
            Type,
            Ref,
            path::{ParameterIn, Parameter},
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

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl IntoParams for Version {
        fn into_params(in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
            [
                Parameter::builder()
                    .name("version")
                    .required(Required::True)
                    .parameter_in(in_provider().unwrap_or_default())
                    .schema(Some(RefOr::Ref(Ref::from_schema_name("Version"))))
                    .build()
            ].to_vec()
        }
    }
}

cfg_jsonschema! {
    use schemars::JsonSchema;

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "jsonschema")))]
    impl schemars::JsonSchema for Version {
        fn schema_name() -> std::borrow::Cow<'static, str> {
            <semver::Version as JsonSchema>::schema_name()
        }

        fn schema_id() -> std::borrow::Cow<'static, str> {
            <semver::Version as JsonSchema>::schema_id()
        }

        fn json_schema(g: &mut schemars::SchemaGenerator) -> schemars::Schema {
            <semver::Version as JsonSchema>::json_schema(g)
        }
    }
}

#[cfg(feature = "__internal_db")]
const _: () = {
    use sea_orm::{
        ColIdx, DbErr, QueryResult, TryGetError, TryGetable,
        sea_query::{ArrayType, ColumnType, Value, ValueType, ValueTypeErr},
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
#[cfg_attr(
    feature = "openapi",
    doc = "* [`utoipa::PartialSchema`](https://docs.rs/utoipa/*/utoipa/trait.PartialSchema.html), [`utoipa::ToSchema`](https://docs.rs/utoipa/*/utoipa/trait.ToSchema.html) (via the `openapi` crate feature)"
)]
#[cfg_attr(
    feature = "jsonschema",
    doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/utoipa/trait.JsonSchema.html) (via the `jsonschema` crate feature)"
)]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::From,
    derive_more::Deref,
)]
pub struct VersionReq(semver::VersionReq);
impl VersionReq {
    pub fn parse(x: &str) -> Result<Self, semver::Error> {
        semver::VersionReq::parse(x).map(Self)
    }

    /// Checks if `self` is a [`VersionReq::STAR`](semver::VersionReq::STAR).
    pub fn is_wildcard(&self) -> bool {
        **self == semver::VersionReq::STAR
    }
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
    use std::borrow::Cow;

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "jsonschema")))]
    impl schemars::JsonSchema for VersionReq {
        fn schema_name() -> Cow<'static, str> {
            Cow::Borrowed("VersionReq")
        }

        fn schema_id() -> Cow<'static, str> {
            Cow::Borrowed("semver::VersionReq")
        }

        fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
            schemars::json_schema!({
                "type": "string",
            })
        }
    }
}

#[cfg(feature = "__internal_db")]
const _: () = {
    use sea_orm::{
        ColIdx, DbErr, QueryResult, TryGetError, TryGetable,
        sea_query::{ArrayType, ColumnType, Value, ValueType, ValueTypeErr},
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

/// A SemVer version that can be queried.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, derive_more::Display)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(untagged, rename_all = "lowercase")]
pub enum QueryableVersion {
    /// Queries the latest version of a chart.
    #[display("latest")]
    Latest,

    /// Queries a specific version of a chart.
    Version(Version),
}

impl QueryableVersion {
    /// Returns `true` if we are the [`Latest`](QueryableVersion::Latest) enum discriminant.
    pub const fn is_latest(&self) -> bool {
        matches!(*self, QueryableVersion::Latest)
    }

    /// Returns `true` if we are the [`Version`](QueryableVersion::Version) enum discriminant.
    pub const fn is_version(&self) -> bool {
        matches!(*self, QueryableVersion::Version(_))
    }

    /// Returns <code>[`Some`]\(&[`Version`]\)</code> if we are in the
    /// [`Version`](QueryableVersion::Version) enum discriminant. Otherwise,
    /// `None` is returned.
    pub const fn as_version(&self) -> Option<&Version> {
        if self.is_version() {
            // Safety: we are `QueryableVersion::Version`
            return Some(unsafe { self.as_version_unchecked() });
        }

        None
    }

    /// Returns a reference from [`QueryableVersion::Version`].
    ///
    /// ## Safety
    /// This will be undefined behaviour if we are [`QueryableVersion::Latest`].
    pub const unsafe fn as_version_unchecked(&self) -> &Version {
        match *self {
            QueryableVersion::Version(ref version) => version,
            QueryableVersion::Latest => unsafe { hint::unreachable_unchecked() },
        }
    }
}

cfg_openapi! {
    use utoipa::{
        IntoParams,
        openapi::{
            RefOr,
            Ref,
            Required,
            path::{ParameterIn, Parameter}
        }
    };

    impl IntoParams for QueryableVersion {
        fn into_params(in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
            vec![
                utoipa::openapi::path::Parameter::builder()
                    .name("version")
                    .required(Required::True)
                    .parameter_in(in_provider().unwrap_or_default())
                    .schema(Some(RefOr::Ref(Ref::from_schema_name("QueryableVersion"))))
                    .build(),
            ]
        }
    }
}
