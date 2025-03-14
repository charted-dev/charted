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

//! # 🐻‍❄️📦 `charted_helm_types`
//! This crate is just a generic crate that has Rust types for
//! the [Helm](https://helm.sh) project.
//!
//! At the moment, it is hand written but in the future it'll probably
//! be code-generated based off which version of Helm we want to support.

#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]

pub use charted_types::ChartType;
use charted_types::{DateTime, Version, VersionReq};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The `apiVersion` field should be `v2` for Helm charts that require at least Helm 3.
///
/// Charts supporting previous Helm versions should have an `apiVersion` set to v1 and are
/// installable by Helm 3.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum ChartSpecVersion {
    /// Chart supports running on Helm 2 or 3.
    V1,

    /// Chart supports running only on Helm 3.
    #[default]
    V2,
}

/// Container that holds the mapping of source values to the parent key to be imported.
///
/// Each item can be a child/parent sublist item or a string, the representation
/// in Rust is [`StringOrImportValue`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ImportValue {
    /// The destination path in the parent chart's values.
    pub parent: String,

    /// The source key of the values to be imported
    pub child: String,
}

/// Discriminated enumeration that can either be a [`String`] or a [`ImportValue`] as
/// the import source for referencing parent key items to be imported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum StringOrImportValue {
    /// String that points to a key to be imported.
    String(String),

    /// Parent/child sublist item.
    ImportValue(ImportValue),
}

/// In Helm, one chart may depend on any number of other charts.
///
/// These dependencies can be dynamically linked using the dependencies' field in
/// `Chart.yaml` or brought in to the `charts/` directory and managed manually. The charts
/// required by the current chart are defined as a list in the `dependencies` field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
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

    /// [`ImportValues`][ImportValue] holds the mapping of source values to parent key to
    /// be imported. Each item can be a string or pair of child/parent sublist items.
    #[serde(default, rename = "import-values", skip_serializing_if = "Vec::is_empty")]
    pub import_values: Vec<StringOrImportValue>,

    /// Alias that is used to identify a chart. Useful for pointing to the
    /// same chart multiple times
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// Name and URL/email address combination as a maintainer.
///
/// The maintainer's name can be a [`ULID`][charted_types::Ulid] or a
/// [`Name`][charted_types::name::Name] and [Hoshi](https://charts.noelware.org/docs/hoshi/latest) can use the information
/// to query the user from the API server and show a "Maintainers" list in the UI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
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

/// Skeleton schema of a `Chart.yaml` file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    /// The `apiVersion` field should be v2 for Helm charts that require at least Helm 3.
    /// Charts supporting previous Helm versions have an apiVersion set to v1 and are
    /// still installable by Helm 3.
    pub api_version: ChartSpecVersion,

    /// The name of the chart.
    pub name: String,

    /// A SemVer 2 conformant version string of the chart.
    pub version: Version,

    /// The optional `kubeVersion` field can define SemVer constraints on supported
    /// Kubernetes versions. Helm will validate the version constraints when
    /// installing the chart and fail if the cluster runs an unsupported Kubernetes
    /// version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kube_version: Option<VersionReq>,

    /// A single-sentence description of this project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The type of the chart.
    #[serde(rename = "type", default)]
    pub type_: ChartType,

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

    /// In Helm, one chart may depend on any number of other charts.
    ///
    /// These dependencies can be dynamically linked using the dependencies' field in
    /// `Chart.yaml` or brought in to the `charts/` directory and managed manually.
    /// The charts required by the current chart are defined as a list in the
    /// dependencies field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<ChartDependency>,

    /// A list of name and URL/email address combinations for the maintainer(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<ChartMaintainer>,

    /// A URL or an SVG or PNG image to be used as an icon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Note that the `appVersion` field is not related to the `version` field.
    ///
    /// It is a way of specifying the version of the application. For example, the
    /// `drupal` chart may have an `appVersion: "8.2.1"`, indicating that the version
    /// of Drupal included in the chart (by default) is `8.2.1`. This field is
    /// informational, and has no impact on chart version calculations.
    ///
    /// Wrapping the version in quotes is highly recommended. It forces the YAML parser to
    /// treat the version number as a string. Leaving it unquoted can lead to parsing
    /// issues in some cases. For example, YAML interprets 1.0 as a floating point value,
    /// and a git commit SHA like 1234e10 as scientific notation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,

    /// When managing charts in a Chart Repository, it is sometimes necessary to deprecate
    /// a chart.
    ///
    /// The optional `deprecated` field in Chart.yaml can be used to mark a chart as
    /// deprecated. If the latest version of a chart in the repository is marked as
    /// deprecated, then the chart as a whole is considered to be deprecated.
    ///
    /// The chart name can be later reused by publishing a newer version that is not
    /// marked as deprecated.
    #[serde(default)]
    pub deprecated: bool,

    /// Mapping of custom metadata that can be used for custom attributes.
    ///
    /// ## `charted-server` specific notes
    /// Some attributes marked with the `charts.noelware.org/` prefix are recognized
    /// by [Hoshi], a web UI for `charted-server`.
    ///
    /// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
    ///
    /// ### Non Exhaustive Attributes
    /// #### `charts.noelware.org/license`
    /// A **SPDX** identified string of the license of the chart.
    ///
    /// #### `charts.noelware.org/images`
    /// A list of Docker images that the chart uses.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, String>,
}

/// Specification of the `index.yaml` file used for Helm chart repositories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", tag = "apiVersion")]
pub enum ChartIndex {
    V1 {
        /// [`DateTime`] constant on when the chart index was generated at, this will not
        /// be modified at all.
        generated: DateTime,

        /// Map of [`ChartIndexSpec`]s for the Helm charts that Helm uses to install a
        /// Helm chart.
        entries: HashMap<String, Vec<ChartIndexSpec>>,
    },
}

impl ChartIndex {
    /// Returns the [`DateTime`] of when this chart index was generated at.
    pub fn generated_at(&self) -> DateTime {
        match self {
            Self::V1 { generated, .. } => *generated,
        }
    }

    /// Returns a referenced [`HashMap`] of all the chart entries avaliable.
    pub fn entries(&self) -> &HashMap<String, Vec<ChartIndexSpec>> {
        match self {
            Self::V1 { entries, .. } => entries,
        }
    }
}

impl Default for ChartIndex {
    fn default() -> ChartIndex {
        ChartIndex::V1 {
            generated: Utc::now().into(),
            entries: HashMap::default(),
        }
    }
}
