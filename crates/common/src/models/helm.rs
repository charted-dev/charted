use super::DateTime;
use crate::hashmap;
use chrono::Local;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use utoipa::{
    openapi::{
        schema::{AdditionalProperties, ObjectBuilder, Schema},
        ArrayBuilder, KnownFormat, Ref, RefOr, SchemaFormat, SchemaType,
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord, sqlx::Type)]
#[sqlx(type_name = "chart_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
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

    /// Operator is a "non standard" Chart type, and is replaced by [`Application`][ChartType::Application]
    /// in releases. This can also be set with the `charts.noelware.org/type` annotation to be `operator` to have
    /// the same affect.
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

    /// Mapping of custom metadata that can be used for custom attributes.
    ///
    /// ### standardized for charted-server
    /// * `charts.noelware.org/maintainers` ~ a comma-delimited list of all the maintainers
    /// that are mapped by their `Name` or snowflake ID
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

/// Schema skeleton for a `index.yaml` file, that represents
/// a [`Chart`] index.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChartIndex {
    pub api_version: String,
    pub generated_at: DateTime,
    pub entries: HashMap<String, Vec<ChartIndexSpec>>,
}

impl Default for ChartIndex {
    fn default() -> ChartIndex {
        ChartIndex {
            api_version: "v1".into(),
            generated_at: Local::now(),
            entries: hashmap!(),
        }
    }
}

impl<'s> ToSchema<'s> for ChartIndex {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
                "ChartIndex",
                RefOr::T(Schema::Object(
                    ObjectBuilder::new()
                        .description(Some(
                            "Schema skeleton for a `index.yml` file that represents a Chart index.",
                        ))
                        .property(
                            "api_version",
                            RefOr::T(
                                Schema::Object(
                                    ObjectBuilder::new()
                                        .description(Some("API version for the `index.yaml` file. Will be a constant as `v1`."))
                                        .schema_type(SchemaType::String)
                                        .build()
                                )
                            )
                        )
                        .required("api_version")
                        .property(
                            "generated_at",
                            RefOr::T(Schema::Object(
                                ObjectBuilder::new()
                                    .description(Some("DateTime of when this `index.yaml` was last generated. In charted-server, this is relative on when a new chart release was last published."))
                                    .schema_type(SchemaType::String)
                                    .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
                                    .build()
                            ))
                        )
                        .required("generated_at")
                        .property(
                            "entries",
                            RefOr::T(Schema::Object(
                                ObjectBuilder::new()
                                    .description(Some("List of all possible entries for this user/organization."))
                                    .schema_type(SchemaType::Object)
                                    .additional_properties(Some(AdditionalProperties::<Schema>::RefOr(
                                        RefOr::T(Schema::Array(
                                            ArrayBuilder::new()
                                                .description(Some("Index contents of a repository"))
                                                .unique_items(true)
                                                .items(RefOr::Ref(Ref::from_schema_name("ChartIndexSpec")))
                                                .build()
                                        ))
                                    )))
                                    .build()
                            ))
                        )
                        .build(),
                )),
            )
    }
}

pub(crate) fn falsy() -> bool {
    false
}
