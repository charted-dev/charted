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

use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use utoipa::{
    openapi::{schema::SchemaType, ObjectBuilder, RefOr, Schema, Type},
    PartialSchema, ToSchema,
};

charted_core::create_newtype_wrapper!(
    /// Newtype wrapper for the [`semver::VersionReq`] type.
    ///
    /// This wrapper implements the following types:
    /// * [`ToSchema`], [`PartialSchema`] for OpenAPI
    #[cfg_attr(feature = "jsonschema", doc = "* [`schemars::JsonSchema`](https://docs.rs/schemars/*/schemars/trait.JsonSchema.html) for JSON schemas")]
    #[derive(Debug, Clone, Display, PartialEq, Eq, Serialize, Deserialize)]
    pub VersionReq for semver::VersionReq;
);

charted_core::mk_from_newtype!(from VersionReq as semver::VersionReq);

impl VersionReq {
    /// Forwards to [`semver::VersionReq::parse`] to return this newtype wrapper.
    pub fn parse(v: &str) -> Result<VersionReq, semver::Error> {
        semver::VersionReq::parse(v).map(Self)
    }
}

////// ============================ SCHEMAS ============================ \\\\\\
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

impl ToSchema for VersionReq {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("VersionReq")
    }
}

#[cfg(feature = "jsonschema")]
impl ::schemars::JsonSchema for VersionReq {
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
