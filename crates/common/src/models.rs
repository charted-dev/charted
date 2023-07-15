// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env::var;
use utoipa::{
    openapi::{
        schema::{ObjectBuilder, OneOfBuilder, Schema},
        RefOr, SchemaType,
    },
    ToSchema,
};

use crate::ID;

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"^([A-z]|-|_|\\d{0,9}){0,32}").unwrap();
}

pub type DateTime = chrono::DateTime<chrono::Local>;

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
    /// Names are validated with the following regex: `^([A-z]|-|_|\\d{0,9}){0,32}`
    Name(String),
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
    pub fn is_valid(&self) -> Result<(), &'static str> {
        match self {
            NameOrSnowflake::Snowflake(flake) => {
                if *flake < 15 {
                    return Err("was not over or equal to 15 in length");
                }

                Ok(())
            }

            NameOrSnowflake::Name(s) => Name::is_valid(s),
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
        NameOrSnowflake::Name(value.to_string())
    }
}

impl From<String> for NameOrSnowflake {
    fn from(value: String) -> NameOrSnowflake {
        NameOrSnowflake::Name(value)
    }
}

/// Represents the distribution that this instance is running off from.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Distribution {
    /// Running on a Kubernetes cluster, it can also be running
    /// from the [official Helm chart](https://charts.noelware.org/~/charted/server).
    Kubernetes,

    /// Unknown distribution, be cautious!
    #[default]
    Unknown,

    /// Running on the official [Docker image](https://cr.noelware.cloud/~/charted/server).
    Docker,

    /// Running from the official RPM distribution
    /// from [Noelware's Artifacts Registry](https://artifacts.noelware.cloud)
    RPM,

    /// Running from the official Debian distribution
    /// from [Noelware's Artifacts Registry](https://artifacts.noelware.cloud)
    Deb,

    /// Running from the Git repository
    Git,
}

impl Distribution {
    pub fn detect() -> Distribution {
        match var("CHARTED_DISTRIBUTION_TYPE") {
            Ok(s) => match s.as_str() {
                "kubernetes" => Distribution::Kubernetes,
                "docker" => Distribution::Docker,
                "rpm" => Distribution::RPM,
                "deb" => Distribution::Deb,
                "git" => Distribution::Git,
                _ => Distribution::Unknown,
            },
            Err(_) => Distribution::Unknown,
        }
    }
}

impl<'s> ToSchema<'s> for Distribution {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Distribution",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .description(Some(
                        "Represents the distribution that this instance is running off from.",
                    ))
                    .schema_type(SchemaType::String)
                    .enum_values(Some(vec!["kubernetes", "docker", "rpm", "deb", "git", "unknown"]))
                    .default(Some("unknown".into()))
                    .build(),
            )),
        )
    }
}

/// Valid UTF-8 string that is used to point to a entity resource. This
/// is mainly used for `idOrName` path parameters in any of the REST
/// API endpoints to help identify a resource by a Name or Snowflake
/// pointer.
///
/// Names are validated with the following regex: `^([A-z]|-|_|\\d{0,9}){0,32}`
pub struct Name;

impl<'s> ToSchema<'s> for Name {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Name",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .description(Some("Valid UTF-8 string that is used to point to a entity resource. This is mainly used for `idOrName` path parameters in any of the REST API endpoints to help identify a resource by a Name or Snowflake pointer."))
                    .pattern(Some("^([A-z]|-|_|\\d{0,9}){0,32}"))
                    .max_length(Some(32))
                    .build(),
            )),
        )
    }
}

impl Name {
    pub fn is_valid<I: AsRef<str>>(input: I) -> Result<(), &'static str> {
        let s = input.as_ref();
        if s.is_empty() {
            return Err("was empty");
        }

        if s.len() > 32 {
            return Err("exceeded over 32 characters");
        }

        if !NAME_REGEX.is_match(s) {
            return Err("did not match to a valid Name");
        }

        Ok(())
    }
}

pub mod helm {
    use super::DateTime;
    use semver::{Version, VersionReq};
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, str::FromStr};
    use utoipa::{
        openapi::{
            schema::{ObjectBuilder, Schema},
            RefOr, SchemaType,
        },
        ToSchema,
    };

    /// The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous
    /// Helm versions have an apiVersion set to v1 and are still installable by Helm 3.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "lowercase")]
    pub enum ChartSpecVersion {
        /// Chart supports running on Helm 2 or 3.
        V1,

        /// Chart supports running only on Helm 3.
        #[default]
        V2,
    }

    impl<'s> ToSchema<'s> for ChartSpecVersion {
        fn schema() -> (&'s str, RefOr<Schema>) {
            (
                "ChartSpecVersion",
                RefOr::T(Schema::Object(
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .description(Some("The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous Helm versions have an apiVersion set to v1 and are still installable by Helm 3."))
                        .default(Some("v2".into()))
                        .enum_values(Some(vec!["v1", "v2"]))
                        .build(),
                )),
            )
        }
    }

    /// Represents what type this chart is. Do note that [`ChartType::Operator`] is not supported
    /// by Helm, but specific to the API server, this will be switched to [`ChartType::Application`]
    /// when serializing to valid Helm objects
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "lowercase")]
    pub enum ChartType {
        #[default]
        Application,
        Library,
        Operator,
    }

    impl<'s> ToSchema<'s> for ChartType {
        fn schema() -> (&'s str, RefOr<Schema>) {
            (
                "ChartType",
                RefOr::T(Schema::Object(
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .description(Some("Represents what type this chart is. Do keep in mind that `operator` is not supported by Helm, but is specific to the API server. For serializing to valid Helm objects, `application` will be the replacement."))
                        .default(Some("application".into()))
                        .enum_values(Some(vec!["application", "library", "operator"]))
                        .build(),
                )),
            )
        }
    }

    impl FromStr for ChartType {
        type Err = String;

        fn from_str(s: &str) -> Result<ChartType, Self::Err> {
            match s {
                "application" => Ok(ChartType::Application),
                "operator" => Ok(ChartType::Operator),
                "library" => Ok(ChartType::Library),
                _ => Err(format!("Unknown chart type: {s}")),
            }
        }
    }

    /// ImportValues hold the mapping of source values to parent key to be imported. Each
    /// item can be a child/parent sublist item or a string.
    #[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ImportValue {
        /// The destination path in the parent chart's values.
        pub parent: String,

        /// The source key of the values to be imported
        pub child: String,
    }

    /// Union enum that can contain a String or a [ImportValue] as the import source
    /// for referencing parent key items to be imported.
    #[derive(Debug, Clone, ToSchema, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    #[serde(untagged)]
    pub enum StringOrImportValue {
        /// String that points to a key to be imported.
        String(String),

        /// Parent/child sublist item.
        ImportValue(ImportValue),
    }

    /// In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies'
    /// field in Chart.yaml or brought in to the charts/ directory and managed manually. The charts required by the current chart are defined
    /// as a list in the dependencies field.
    #[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ChartDependency {
        /// The name of the chart
        pub name: String,

        /// The version of the chart.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<Version>,

        /// Repository URL or alias that should be used to grab
        /// the dependency from.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub repository: Option<String>,

        /// YAML path that resolves to a boolean to enable or disable charts
        /// dynamically.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub condition: Option<String>,

        /// List of tags that can be used to group charts to enable/disable together.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<String>,

        /// [`ImportValues`][ImportValue] holds the mapping of source values to parent key to be imported.
        /// Each item can be a string or pair of child/parent sublist items.
        #[serde(default, rename = "import-values", skip_serializing_if = "Vec::is_empty")]
        pub import_values: Vec<StringOrImportValue>,

        /// Alias that is used to identify a chart. Useful for pointing to the
        /// same chart multiple times
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
    }

    /// Name and URL/email address combination as a maintainer. [ChartMaintainer::name] can be referenced
    /// as a `NameOrSnowflake` union.
    #[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ChartMaintainer {
        /// The maintainer's name
        pub name: String,

        /// The maintainer's email
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// URL for the maintainer
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }

    /// Represents the skeleton of a `Chart.yaml` file.
    #[derive(Debug, Clone, ToSchema, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct Chart {
        /// The `apiVersion` field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous
        /// Helm versions have an apiVersion set to v1 and are still installable by Helm 3.
        pub api_version: ChartSpecVersion,

        /// The name of the chart.
        pub name: String,

        /// A SemVer 2 conformant version string of the chart.
        pub version: Version,

        /// The optional `kubeVersion` field can define SemVer constraints on supported Kubernetes versions.
        /// Helm will validate the version constraints when installing the chart and fail if the
        /// cluster runs an unsupported Kubernetes version.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kube_version: Option<VersionReq>,

        /// A single-sentence description of this project
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The type of the chart.
        #[serde(rename = "type", default)]
        pub r#type: ChartType,

        /// A list of keywords about this project. These keywords can be searched
        /// via the /search endpoint if it's enabled.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub keywords: Vec<String>,

        /// The URL of this project's homepage.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub home: Option<String>,

        /// A list of URLs to the source code for this project
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub sources: Vec<String>,

        /// In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies'
        /// field in Chart.yaml or brought in to the charts/ directory and managed manually. The charts required by the current chart are defined as a list
        /// in the dependencies field.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub dependencies: Vec<ChartDependency>,

        /// A list of name and URL/email address combinations for the maintainer(s)
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub maintainers: Vec<ChartMaintainer>,

        /// A URL or an SVG or PNG image to be used as an icon
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,

        /// Note that the appVersion field is not related to the version field. It is a way of specifying the version of the
        /// application. For example, the drupal chart may have an appVersion: "8.2.1", indicating that the version of Drupal
        /// included in the chart (by default) is 8.2.1. This field is informational, and has no impact on chart version calculations.
        ///
        /// Wrapping the version in quotes is highly recommended. It forces the YAML parser to treat the version number as a string.
        /// Leaving it unquoted can lead to parsing issues in some cases. For example, YAML interprets 1.0 as a floating point value,
        /// and a git commit SHA like 1234e10 as scientific notation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub app_version: Option<String>,

        /// When managing charts in a Chart Repository, it is sometimes necessary to deprecate a chart. The optional deprecated field
        /// in Chart.yaml can be used to mark a chart as deprecated. If the latest version of a chart in the repository is marked
        /// as deprecated, then the chart as a whole is considered to be deprecated.
        ///
        /// The chart name can be later reused by publishing a newer version that is not marked as deprecated.
        #[serde(default = "falsy")]
        pub deprecated: bool,

        /// A list of annotations keyed by name and value.
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        pub annotations: HashMap<String, String>,
    }

    /// Represents the specification for a Chart.yaml-schema from a `index.yaml` reference.
    #[derive(Debug, Clone, ToSchema, Serialize, Deserialize, PartialEq, Eq)]
    pub struct ChartIndexSpec {
        /// The Chart specification itself, this will be flatten when (de)serializing.
        #[serde(flatten)]
        pub spec: Chart,

        // not documented in Helm source code, so I can't really
        // add documentation here.
        //
        // https://github.com/helm/helm/blob/main/pkg/repo/index.go#L255
        #[serde(default)]
        pub urls: Vec<String>,

        #[serde(default)]
        pub created: Option<DateTime>,

        #[serde(default = "falsy")]
        pub removed: bool,

        #[serde(default)]
        pub digest: Option<String>,
    }

    pub(crate) fn falsy() -> bool {
        false
    }
}

pub mod entities {
    use super::{helm::ChartType, DateTime};
    use crate::{hashmap, Bitfield, ID};
    use once_cell::sync::Lazy;
    use semver::Version;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    use utoipa::{
        openapi::{RefOr, Schema},
        ToSchema,
    };

    /// Returns an empty [Bitfield] with the available flags needed.
    #[allow(non_upper_case_globals)]
    pub static ApiKeyScopes: Lazy<Bitfield> = Lazy::new(|| {
        Bitfield::with_flags(hashmap! {
            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            //           User Scopes
            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            "user:access" => 1 << 0,
            "user:update" => 1 << 1,
            "user:delete" => 1 << 2,
            "user:connections" => 1 << 3,
            "user:notifications" => 1 << 4,
            "user:avatar:update" => 1 << 5,
            "user:sessions:list" => 1 << 6,

            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            //        Repository Scopes
            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            "repo:access" => 1 << 7,
            "repo:create" => 1 << 8,
            "repo:delete" => 1 << 9,
            "repo:update" => 1 << 10,
            "repo:write" => 1 << 11,
            "repo:icon:update" => 1 << 12,
            "repo:releases:create" => 1 << 13,
            "repo:releases:update" => 1 << 14,
            "repo:releases:delete" => 1 << 15,
            "repo:members:list" => 1 << 16,
            "repo:members:update" => 1 << 17,
            "repo:members:kick" => 1 << 18,
            "repo:members:invites:access" => 1 << 19,
            "repo:members:invites:create" => 1 << 20,
            "repo:members:invites:delete" => 1 << 21,
            "repo:webhooks:list" => 1 << 22,
            "repo:webhooks:create" => 1 << 23,
            "repo:webhooks:update" => 1 << 24,
            "repo:webhooks:delete" => 1 << 25,
            "repo:webhooks:events:access" => 1 << 26,
            "repo:webhooks:events:delete" => 1 << 27,

            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            //        API Key Scopes
            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            "apikeys:view" => 1 << 28,
            "apikeys:create" => 1 << 29,
            "apikeys:delete" => 1 << 30,
            "apikeys:update" => 1 << 31,

            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            //      Organization Scopes
            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            "org:access" => 1 << 32,
            "org:create" => 1 << 33,
            "org:update" => 1 << 34,
            "org:delete" => 1 << 35,
            "org:members:invites" => 1 << 36,
            "org:members:list" => 1 << 37,
            "org:members:kick" => 1 << 38,
            "org:members:update" => 1 << 39,
            "org:webhooks:list" => 1 << 40,
            "org:webhooks:create" => 1 << 41,
            "org:webhooks:update" => 1 << 42,
            "org:webhooks:delete" => 1 << 43,
            "org:webhooks:events:list" => 1 << 44,
            "org:webhooks:events:delete" => 1 << 45,

            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            //    Administration Scopes
            // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
            "admin:stats" => 1 << 46,
            "admin:users:create" => 1 << 47,
            "admin:users:delete" => 1 << 48,
            "admin:users:update" => 1 << 49,
            "admin:orgs:delete" => 1 << 50,
            "admin:orgs:update" => 1 << 51
        })
    });

    #[allow(non_upper_case_globals)]
    pub static MemberPermissions: Lazy<Bitfield> = Lazy::new(|| {
        Bitfield::with_flags(hashmap! {
            // This member has permission to invite new members into the repository or organization, and
            // they can view all the other invites that are pending
            "member:invite" => 1 << 0,

            // This member has permission to update any member's permissions
            "member:update" => 1 << 1,

            // This member has permission to kick any members off the repository
            "member:kick" => 1 << 2,

            // Whether if this member has permission to update the repository or organization metadata.
            "metadata:update" => 1 << 3,

            // Whether if this member has permission to create a repository in this organization. As a repository
            // member, this does nothing.
            "repo:create" => 1 << 4,

            // Whether if this member has permission to delete the repository or not.
            "repo:delete" => 1 << 5,

            // Whether if this member has permission to create additional webhooks in the given
            // repository or organization.
            "webhooks:create" => 1 << 6,

            // Whether if this member has permission to update webhooks in the given
            // repository or organization.
            "webhooks:update" => 1 << 7,

            // Whether if this member has permission to delete webhooks in the given
            // repository or organization.
            "webhooks:delete" => 1 << 8,

            // Whether if this member has permission to delete any repository/organization metadata (i.e. repo releases)
            "metadata:delete" => 1 << 9
        })
    });

    // only used for Utoipa to use the `ID` schema from snowflake.rs
    fn snowflake_schema() -> RefOr<Schema> {
        ID::schema().1
    }

    fn name_schema() -> RefOr<Schema> {
        crate::models::Name::schema().1
    }

    /// Represents an account that can own [repositories][Repository] and [organizations][Organizations]
    #[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, FromRow)]
    pub struct User {
        /// Whether if this User is a Verified Publisher or not.
        #[serde(default = "crate::models::helm::falsy")]
        pub verified_publisher: bool,

        /// Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar
        #[serde(default)]
        pub gravatar_email: Option<String>,

        /// Short description about this user, can be `null` if none was provided.
        #[serde(default)]
        pub description: Option<String>,

        /// Unique hash to locate a user's avatar, this also includes the extension that this avatar is, i.e, `png`.
        #[serde(default)]
        pub avatar_hash: Option<String>,

        /// Date of when this user was registered to this instance
        pub created_at: DateTime,

        /// Date of when the server has last updated this user
        pub updated_at: DateTime,

        /// Unique username that can be used to locate this user with the API
        pub username: String,

        /// Whether if this User is an Administrator of this instance
        #[serde(default = "crate::models::helm::falsy")]
        pub admin: bool,

        /// Display name for this user, it should be displayed as '{name} (@{username})' or just '@{username}' if there is no display name
        #[serde(default)]
        #[schema(schema_with = name_schema)]
        pub name: Option<String>,

        /// Unique identifier to locate this user with the API
        #[schema(schema_with = snowflake_schema)]
        pub id: i64,
    }

    /// Represents a collection of a user's connections that can be used
    /// to login from different sources (like GitHub OAuth2)
    #[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
    pub struct UserConnections {
        /// Snowflake ID that was sourced from [Noelware's Accounts System](https://accounts.noelware.org)
        #[serde(default)]
        pub noelware_account_id: Option<u64>,

        /// Account ID that was sourced from Google OAuth2
        #[serde(default)]
        pub google_account_id: Option<String>,

        /// Account ID that was sourced from GitHub OAuth2. This can differ from
        /// GitHub (https://github.com) and GitHub Enterprise usage.
        #[serde(default)]
        pub github_account_id: Option<String>,

        /// Date of when this connection was inserted to the database.
        pub created_at: DateTime,

        /// Date of when the server has last updated this user's connections.
        pub updated_at: DateTime,

        /// Snowflake of the user that owns this connections object.
        #[schema(schema_with = snowflake_schema)]
        pub id: i64,
    }

    #[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
    pub struct Repository {
        /// Short description about this user, can be `null` if none was provided.
        #[serde(default)]
        pub description: Option<String>,

        /// Whether if this repository is deprecated or not
        #[serde(default = "crate::models::helm::falsy")]
        pub deprecated: bool,

        /// Date of when this repository was registered to this instance
        pub created_at: DateTime,

        /// Date of when the server has last updated this repository
        pub updated_at: DateTime,

        /// Unique hash to locate a repository's icon, this also includes the extension that this avatar is, i.e, `png`.
        #[serde(default)]
        pub icon_hash: Option<String>,

        /// Whether if this repository is private or not
        #[serde(default = "crate::models::helm::falsy")]
        pub private: bool,

        /// Unique identifier that points to a User or Organization resource that owns this repository
        pub owner_id: u64,

        /// Unique [Name] to locate this repository from the API
        pub name: String,

        /// The chart type that this repository is
        pub r#type: ChartType,

        /// Unique identifier to locate this repository from the API
        #[schema(schema_with = snowflake_schema)]
        pub id: i64,
    }

    /// Represents a resource that contains a release from a [Repository] release. Releases
    /// are a way to group releases of new versions of Helm charts that can be easily
    /// fetched from the API server.
    ///
    /// Any repository can have an unlimited amount of releases, but tags cannot clash
    /// into each other, so the API server will not accept it. Each tag should be
    /// a SemVer 2 comformant string, parsing is related to how Cargo evaluates SemVer 2 tags.
    #[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
    pub struct RepositoryRelease {
        /// Whether if this release is a pre-release or not.
        #[serde(default = "crate::models::helm::falsy")]
        pub is_prerelease: bool,

        /// Markdown-formatted string that contains a changelog of this release.
        #[serde(default)]
        pub update_test: Option<String>,

        /// Date of when this release was registered to this instance
        pub created_at: DateTime,

        /// Date of when the server has last updated this repository release
        pub updated_at: DateTime,

        /// SemVer 2 comformant string that represents this tag.
        pub tag: Version,

        /// Unique identifier to locate this repository release resource from the API.
        #[schema(schema_with = snowflake_schema)]
        pub id: i64,
    }

    /// Represents a resource that is correlated to a repository or organization member
    /// that can control the repository's metadata.
    #[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, FromRow)]
    pub struct Member {
        /// Display name for this member. This should be formatted as '{display_name} (@{username})' if this
        /// is set, otherwise '@{username}' is used.
        pub display_name: Option<String>,

        /// Bitfield value of this member's permissions.
        pub permissions: u64,

        /// Date-time of when this member resource was last updated by the API server.
        pub updated_at: DateTime,

        /// Date-time of when this member resource was created by the API server.
        pub joined_at: DateTime,

        /// [User] resource that this member is.
        pub user: User,

        /// Unique identifier to locate this member with the API
        #[schema(schema_with = snowflake_schema)]
        pub id: i64,
    }

    impl Member {
        /// Returns a new [`Bitfield`], but this member's permissions
        /// are filled in for the bitfield.
        ///
        /// ## Example
        /// ```
        /// # use charted_common::models::entities::Member;
        /// #
        /// let member = Member::default();
        /// assert_eq!(member.bitfield().bits(), 0);
        /// ```
        pub fn bitfield<'a>(&self) -> Bitfield<'a> {
            MemberPermissions.init(self.permissions)
        }
    }

    /// Represents a unified entity that can manage and own repositories outside
    /// a User. Organizations to the server is used for business-related Helm charts
    /// that aren't tied to a specific User.
    #[derive(Debug, Clone, ToSchema, Serialize, Deserialize, Default, FromRow)]
    pub struct Organization {
        /// Whether if this Organization is a Verified Publisher or not.
        #[serde(default = "crate::models::helm::falsy")]
        pub verified_publisher: bool,

        /// Returns the twitter handle for this organization, if populated.
        #[serde(default)]
        pub twitter_handle: Option<String>,

        /// Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar
        #[serde(default)]
        pub gravatar_email: Option<String>,

        /// Display name for this organization. It should be formatted as '{display_name} (@{name})'
        /// or '@{name}'.
        #[serde(default)]
        pub display_name: Option<String>,

        /// Date of when this organization was registered to this instance
        pub created_at: DateTime,

        /// Date of when the server has last updated this organization
        pub updated_at: DateTime,

        /// Unique hash to locate an organization's icon, this also includes the extension that this icon is, i.e, `png`.
        #[serde(default)]
        pub icon_hash: Option<String>,

        /// Whether this organization is private and only its member can access this resource.
        #[serde(default = "crate::models::helm::falsy")]
        pub private: bool,

        /// The User resource that owns this organization
        pub owner: User,

        /// The name for this organization.
        pub name: String,

        /// Unique identifier to locate this organization with the API
        #[schema(schema_with = snowflake_schema)]
        pub id: i64,
    }

    /// A resource for personal-managed API tokens that is created by a User. This is useful
    /// for command line tools or scripts that need to interact with charted-server, but
    /// the main use-case is for the [Helm plugin](https://charts.noelware.org/docs/helm-plugin/current).
    #[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
    pub struct ApiKey {
        /// Short description about this API key.
        #[serde(default)]
        pub description: Option<String>,

        /// Date-time of when this API token expires in, `null` can be returned
        /// if the token doesn't expire
        #[serde(default)]
        pub expires_in: Option<DateTime>,

        /// The scopes that are attached to this API key resource.
        pub scopes: u64,

        /// The token itself. This is never revealed when querying, but only revealed
        /// when you create the token.
        #[serde(default)]
        pub token: Option<String>,

        /// User resource that owns this API key.
        pub owner: User,

        /// The name of the API key.
        pub name: String,

        /// Unique identifer to locate this resource in the API server.
        #[schema(schema_with = snowflake_schema)]
        pub id: i64,
    }

    impl ApiKey {
        /// Returns a new [`Bitfield`], but this member's permissions
        /// are filled in for the bitfield.
        ///
        /// ## Example
        /// ```
        /// # use charted_common::models::entities::ApiKey;
        /// #
        /// let resource = ApiKey::default();
        /// assert_eq!(resource.bitfield().bits(), 0);
        /// ```
        pub fn bitfield<'a>(&self) -> Bitfield<'a> {
            ApiKeyScopes.init(self.scopes)
        }
    }
}
