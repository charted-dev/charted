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

use crate::{DateTime, Version, VersionReq};
use charted_database::schema::sql_types;
use chrono::Utc;
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::ToSql,
    sql_types::{Binary, Text},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use utoipa::ToSchema;

/// The [`apiVersion`] field should be `v2` for Helm charts that require at least Helm 3.
///
/// Charts supporting previous Helm versions should have an [`apiVersion`] set to v1 and are
/// installable by Helm 3.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ChartSpecVersion {
    /// Chart supports running on Helm 2 or 3.
    V1,

    /// Chart supports running only on Helm 3.
    #[default]
    V2,
}

/// Represents what type this chart is. Do note that [`ChartType::Operator`] is not supported
/// by Helm, but specific to the API server, this will be switched to [`ChartType::Application`]
/// when serializing to valid Helm objects
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, ToSchema, FromSqlRow, AsExpression)]
#[serde(rename_all = "lowercase")]
#[diesel(sql_type = sql_types::ChartType)]
#[diesel(sql_type = Text)]
pub enum ChartType {
    /// Default chart type and represents a standard chart which can operate on a Kubernetes
    /// cluster and spawn in Kubernetes objects.
    ///
    /// **Note**: Application charts can also act as library charts, just set this to [`Library`][ChartType::Library],
    /// and it'll act like a library chart instead.
    #[default]
    Application,

    /// Library charts provide utilities or functions for building Helm charts, it differs
    /// from an [`Application`][ChartType::Application] chart because it cannot create Kubernetes
    /// objects from `helm install`.
    Library,

    /// Operator is a "non standard" Chart type, and is replaced by "application" when a release is made.
    ///
    /// This will be replaced with "application" and the `charts.noelware.org/kind: "operator"`
    /// annotation will be readily avaliable.
    Operator,
}

impl FromSql<sql_types::ChartType, Pg> for ChartType {
    fn from_sql(bytes: <Pg as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = bytes.as_bytes();
        match bytes {
            b"application" => Ok(ChartType::Application),
            b"library" => Ok(ChartType::Library),
            b"operator" => Ok(ChartType::Operator),
            v => Err(format!("unknown enum variant: {}", String::from_utf8_lossy(v)).into()),
        }
    }
}

impl ToSql<sql_types::ChartType, Pg> for ChartType {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <str as ToSql<Text, Pg>>::to_sql(
            match self {
                ChartType::Application => "application",
                ChartType::Library => "library",
                ChartType::Operator => "operator",
            },
            out,
        )
    }
}

impl FromSql<Text, Sqlite> for ChartType {
    fn from_sql(bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let data = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match data.as_bytes() {
            b"application" => Ok(ChartType::Application),
            b"library" => Ok(ChartType::Library),
            b"operator" => Ok(ChartType::Operator),
            v => Err(format!("unknown enum variant: {}", String::from_utf8_lossy(v)).into()),
        }
    }
}

impl ToSql<Text, Sqlite> for ChartType {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        <str as ToSql<Text, Sqlite>>::to_sql(
            match self {
                ChartType::Application => "application",
                ChartType::Library => "library",
                ChartType::Operator => "operator",
            },
            out,
        )
    }
}

impl FromSql<sql_types::ChartType, Sqlite> for ChartType {
    fn from_sql(bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        match bytes.as_slice() {
            b"application" => Ok(ChartType::Application),
            b"library" => Ok(ChartType::Library),
            b"operator" => Ok(ChartType::Operator),
            v => Err(format!("unknown enum variant: {}", String::from_utf8_lossy(v)).into()),
        }
    }
}

impl ToSql<sql_types::ChartType, Sqlite> for ChartType {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        <str as ToSql<Text, Sqlite>>::to_sql(
            match self {
                ChartType::Application => "application",
                ChartType::Library => "library",
                ChartType::Operator => "operator",
            },
            out,
        )
    }
}

impl FromStr for ChartType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_ascii_lowercase() {
            "application" => Ok(ChartType::Application),
            "operator" => Ok(ChartType::Operator),
            "library" => Ok(ChartType::Library),
            _ => Err(format!("unknown type given: '{s}'")),
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
/// as a `Name` or a ULID.
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
    #[serde(default)]
    pub deprecated: bool,

    /// Mapping of custom metadata that can be used for custom attributes. Some attributes
    /// are regconized for [`Hoshi`] to understand some elements that can be represented
    /// in the UI:
    ///
    /// * `charts.noelware.org/maintainers` ~ a YAML sequence of available maintainers, must be prefixed
    ///    with `user:` for a user and `org:` for an organization that maintains the Helm chart.
    ///
    /// ```yaml
    /// annotations:
    ///     charts.noelware.org/maintainers: |-
    ///         - user:noel
    ///         - org:noelware
    /// ```
    ///
    /// * `charts.noelware.org/images` ~ YAML sequence of the Docker images that the chart will install. This
    ///   is used in Hoshi to allow to go to the registry that owns the Docker image.
    ///
    /// ```yaml
    /// charts.noelware.org/images: |-
    ///     # maps to `hub.docker.com/r/charted/server`
    ///     - charted/server:latest
    ///
    ///     # maps to `docker.elastic.co`
    ///     - docker.elastic.co/elasticsearch/elasticsearch
    /// ```
    ///
    /// [`Hoshi`]: https://charts.noelware.org/docs/hoshi/latest
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
    // https://github.com/helm/helm/blob/764557c470533fa57aad99f865c9ff75a64d4163/pkg/repo/index.go#L270-L273
    #[serde(default)]
    pub urls: Vec<String>,

    #[serde(default)]
    pub created: Option<DateTime>,

    #[serde(default)]
    pub removed: bool,

    #[serde(default)]
    pub digest: Option<String>,
}

/// Schema skeleton for a `index.yaml` file, that represents a [`Chart`] index.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChartIndex {
    /// API version for the schema itself, will always be `v1`.
    pub api_version: String,

    /// [`DateTime`] constant on when the chart index was generated at, this will not
    /// be modified at all.
    pub generated: DateTime,

    /// Map of [`ChartIndexSpec`]s for the Helm charts that Helm uses to install a Helm chart.
    pub entries: HashMap<String, Vec<ChartIndexSpec>>,
}

impl Default for ChartIndex {
    fn default() -> ChartIndex {
        ChartIndex {
            api_version: "v1".into(),
            generated: Utc::now().into(),
            entries: azalia::hashmap!(),
        }
    }
}
