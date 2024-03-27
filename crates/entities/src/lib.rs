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

mod apikeyscope;
pub use apikeyscope::*;

mod member_permission;
pub use member_permission::*;

mod name;
pub use name::*;

pub mod helm;
pub(crate) mod macros;
pub mod payloads;

use charted_search::{Indexable, Searchable};
use helm::ChartType;
use noelware_config::env;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgHasArrayType, Database, Decode, Encode, FromRow, Type};
use std::{fmt::Display, fs, ops::Deref, path::PathBuf, sync::Once};
use utoipa::{
    openapi::{ObjectBuilder, OneOfBuilder, Ref, RefOr, Schema, SchemaType},
    ToSchema,
};
use validator::Validate;

/// Represents a type-alias that wraps [`chrono::DateTime`]<[`chrono::Local`]> for database objects'
/// `created_at` and `updated_at` timestamps.
pub type DateTime = chrono::DateTime<chrono::Local>;

const KUBERNETES_SERVICE_TOKEN_FILE: &str = "/run/secrets/kubernetes.io/serviceaccount/token";
const KUBERNETES_NAMESPACE_FILE: &str = "/run/secrets/kubernetes.io/serviceaccount/namespace";

/// Automatic detection to check if the distribution of charted-server is running on a Kubernetes
/// cluster as a pod or not. It'll check in the following paths and check if they exist:
///
/// * `/run/secrets/kubernetes.io/serviceaccount/token`
/// * `/run/secrets/kubernetes.io/serviceaccount/namespace`
fn is_in_k8s() -> bool {
    static ONCE: Once = Once::new();
    static mut KUBERNETES: bool = false;

    // Safety: `KUBERNETES` is never mutated after the `call_once` method was called.
    unsafe {
        ONCE.call_once(move || {
            if env!("KUBERNETES_SERVICE_HOST").is_ok() {
                KUBERNETES = true;
                return;
            }

            let mut has_service_acc_token = false;
            let mut has_service_acc_ns = false;
            match PathBuf::from(KUBERNETES_SERVICE_TOKEN_FILE).try_exists() {
                Ok(true) => {
                    has_service_acc_token = true;
                }

                Ok(false) => {}
                Err(_) => return,
            }

            match PathBuf::from(KUBERNETES_NAMESPACE_FILE).try_exists() {
                Ok(true) => {
                    has_service_acc_ns = true;
                }

                Ok(false) => {}
                Err(_) => return,
            }

            KUBERNETES = has_service_acc_token && has_service_acc_ns;
        });

        KUBERNETES
    }
}

/// Detects if charted-server is running as a Docker container, it'll check if `/.dockerenv` exists or if
/// `/proc/self/cgroup` contains `docker` in it.
fn is_in_docker_container() -> bool {
    static ONCE: Once = Once::new();
    static mut DOCKER: bool = false;

    // Safety: `DOCKER` is never mutated after the `call_once` method was called.
    unsafe {
        ONCE.call_once(move || {
            let has_dockerenv = match PathBuf::from("/.dockerenv").try_exists() {
                Ok(res) => res,
                Err(_) => return,
            };

            let has_docker_cgroup = {
                let cgroup = PathBuf::from("/proc/self/cgroup");
                let Ok(contents) = fs::read_to_string(cgroup) else {
                    return;
                };

                contents.contains("docker")
            };

            DOCKER = has_dockerenv || has_docker_cgroup
        });

        DOCKER
    }
}

/// Represents the distribution that this instance is running off from.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
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
        static ONCE: Once = Once::new();
        static mut DETECTED: Distribution = Distribution::Unknown;

        // Safety: we are only mutating `DETECTED` once if the fn is called
        //         on the first run of `Distribution::detect`
        unsafe {
            ONCE.call_once(|| {
                if is_in_k8s() {
                    DETECTED = Distribution::Kubernetes;
                    return;
                }

                if is_in_docker_container() {
                    DETECTED = Distribution::Docker;
                    return;
                }

                DETECTED = match env!("CHARTED_DISTRIBUTION_KIND") {
                    Ok(s) => match s.as_str() {
                        // rpm and deb are automatically set in the systemd service
                        // so we don't need to do any detection
                        "rpm" => Distribution::RPM,
                        "deb" => Distribution::Deb,

                        // git is applied when built from source (i.e, ./dev server)
                        "git" => Distribution::Git,

                        // disallow any other value
                        _ => Distribution::Unknown,
                    },
                    Err(_) => Distribution::Unknown,
                };
            });

            DETECTED
        }
    }
}

impl Display for Distribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distribution::Kubernetes => f.write_str("kubernetes"),
            Distribution::Docker => f.write_str("docker"),
            Distribution::Git => f.write_str("git"),
            Distribution::Deb => f.write_str("debian"),
            Distribution::RPM => f.write_str("rpm"),
            _ => f.write_str("«unknown»"),
        }
    }
}

fn snowflake_schema() -> impl Into<RefOr<Schema>> {
    Ref::from_schema_name("Snowflake")
}

fn name_schema() -> impl Into<RefOr<Schema>> {
    Ref::from_schema_name("Name")
}

/// Represents an account that can own [repositories][Repository] and [organizations][Organization]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, FromRow, Indexable, Searchable)]
#[indexable(index = "users")]
pub struct User {
    /// Whether if this User is a Verified Publisher or not.
    #[serde(default)]
    #[schema(read_only)]
    pub verified_publisher: bool,

    /// Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar
    #[serde(default)]
    #[search(skip)]
    pub gravatar_email: Option<String>,

    /// Short description about this user, can be `null` if none was provided.
    #[serde(default)]
    pub description: Option<String>,

    /// Unique hash to locate a user's avatar, this also includes the extension that this avatar is, i.e, `png`.
    #[serde(default)]
    pub avatar_hash: Option<String>,

    /// Date of when this user was registered to this instance
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this user
    #[schema(read_only)]
    pub updated_at: DateTime,

    #[serde(skip)]
    #[search(skip)]
    pub password: Option<String>,

    /// Unique username that can be used to locate this user with the API
    pub username: Name,

    #[serde(skip)]
    #[search(skip)]
    pub email: String,

    /// Whether if this User is an Administrator of this instance
    #[serde(default)]
    #[schema(read_only)]
    pub admin: bool,

    /// Display name for this user, it should be displayed as '{name} (@{username})' or just '@{username}' if there is no display name
    #[serde(default)]
    pub name: Option<String>,

    /// Unique identifier to locate this user with the API
    #[schema(read_only, schema_with = snowflake_schema)]
    pub id: i64,
}

/// Represents a collection of a user's connections that can be used
/// to login from different sources (like GitHub OAuth2)
#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
pub struct UserConnections {
    /// Snowflake ID that was sourced from [Noelware's Accounts System](https://accounts.noelware.org)
    #[serde(default)]
    pub noelware_account_id: Option<i64>,

    /// Account ID that was sourced from Google OAuth2
    #[serde(default)]
    pub google_account_id: Option<String>,

    /// Account ID that was sourced from GitHub OAuth2. This can differ from
    /// GitHub (https://github.com) and GitHub Enterprise usage.
    #[serde(default)]
    pub github_account_id: Option<String>,

    /// Date of when this connection was inserted to the database.
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this user's connections.
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// Snowflake of the user that owns this connections object.
    #[schema(read_only, schema_with = snowflake_schema)]
    pub id: i64,
}

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
pub struct Repository {
    /// Short description about this user, can be `null` if none was provided.
    #[serde(default)]
    pub description: Option<String>,

    /// Whether if this repository is deprecated or not
    #[serde(default)]
    pub deprecated: bool,

    /// Date of when this repository was registered to this instance
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this repository
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// Unique hash to locate a repository's icon, this also includes the extension that this avatar is, i.e, `png`.
    #[serde(default)]
    pub icon_hash: Option<String>,

    /// Unique identifier that points to a [`User`] that owns the repository if it is
    /// under an organization.
    #[schema(read_only, schema_with = snowflake_schema)]
    pub creator: Option<i64>,

    /// Whether if this repository is private or not
    #[serde(default)]
    pub private: bool,

    /// Unique identifier that points to a User or Organization resource that owns this repository
    #[schema(read_only, schema_with = snowflake_schema)]
    pub owner: i64,

    /// Unique [Name] to locate this repository from the API
    #[schema(schema_with = name_schema)]
    pub name: Name,

    /// The chart type that this repository is
    pub r#type: ChartType,

    /// Unique identifier to locate this repository from the API
    #[schema(read_only, schema_with = snowflake_schema)]
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
    #[serde(default)]
    pub is_prerelease: bool,

    /// Markdown-formatted string that contains a changelog of this release.
    #[serde(default)]
    pub update_text: Option<String>,

    /// Repository that owns this release
    #[schema(read_only, schema_with = snowflake_schema)]
    pub repository: i64,

    /// Date of when this release was registered to this instance
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this repository release
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// SemVer 2 comformant string that represents this tag.
    #[schema(read_only)]
    pub tag: Version,

    /// Unique identifier to locate this repository release resource from the API.
    #[schema(read_only, schema_with = snowflake_schema)]
    pub id: i64,
}

/// Represents a resource that is correlated to a repository or organization member
/// that can control the repository's metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, FromRow)]
pub struct Member {
    /// Display name for this member. This should be formatted as '[{display_name}][Member::display_name] (@[{username}][User::username])' if this
    /// is set, otherwise '@[{username}][User::username]' is used.
    pub display_name: Option<String>,

    /// Bitfield value of this member's permissions.
    pub permissions: u64,

    /// Date-time of when this member resource was last updated by the API server.
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// Date-time of when this member resource was created by the API server.
    #[schema(read_only)]
    pub joined_at: DateTime,

    /// [User] resource that this member is.
    #[schema(read_only, schema_with = snowflake_schema)]
    pub user: i64,

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
    /// # use charted_entities::Member;
    /// #
    /// let member = Member::default();
    /// assert_eq!(member.bitfield().bits(), 0);
    /// ```
    pub fn bitfield(&self) -> MemberPermissions {
        MemberPermissions::init(self.permissions)
    }
}

/// Represents a unified entity that can manage and own repositories outside
/// a User. Organizations to the server is used for business-related Helm charts
/// that aren't tied to a specific User.
#[derive(Debug, Clone, ToSchema, Serialize, Deserialize, Default, FromRow)]
pub struct Organization {
    /// Whether if this Organization is a Verified Publisher or not.
    #[serde(default)]
    #[schema(read_only)]
    pub verified_publisher: bool,

    /// Returns the twitter handle for this organization, if populated.
    #[serde(default)]
    pub twitter_handle: Option<String>,

    /// Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar
    #[serde(default)]
    pub gravatar_email: Option<String>,

    /// Display name for this organization. It should be formatted as '[{display_name}][Organization::display_name] (@[{name}][Organization::name])'
    /// or '@[{name}][Organization::name]'.
    #[serde(default)]
    pub display_name: Option<String>,

    /// Date of when this organization was registered to this instance
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this organization
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// Unique hash to locate an organization's icon, this also includes the extension that this icon is, i.e, `png`.
    #[serde(default)]
    pub icon_hash: Option<String>,

    /// Whether this organization is private and only its member can access this resource.
    #[serde(default)]
    pub private: bool,

    /// User ID that owns this organization
    #[schema(read_only, schema_with = snowflake_schema)]
    pub owner: i64,

    /// The name for this organization.
    #[schema(read_only)]
    pub name: Name,

    /// Unique identifier to locate this organization with the API
    #[schema(read_only, schema_with = snowflake_schema)]
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

    /// Date of when this API key was created
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this API key
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// Date-time of when this API token expires in, `null` can be returned
    /// if the token doesn't expire
    #[serde(default)]
    #[schema(read_only)]
    pub expires_in: Option<DateTime>,

    /// The scopes that are attached to this API key resource.
    pub scopes: i64,

    /// The token itself. This is never revealed when querying, but only revealed
    /// when you create the token.
    #[serde(default)]
    #[schema(read_only)]
    pub token: Option<String>,

    /// User resource that owns this API key. This is skipped
    /// when using the API as this is pretty useless.
    #[serde(skip)]
    #[schema(read_only, schema_with = snowflake_schema)]
    pub owner: i64,

    /// The name of the API key.
    #[schema(read_only, schema_with = name_schema)]
    pub name: Name,

    /// Unique identifer to locate this resource in the API server.
    #[schema(read_only, schema_with = snowflake_schema)]
    pub id: i64,
}

impl ApiKey {
    /// Returns a new [`Bitfield`], but this member's permissions
    /// are filled in for the bitfield.
    ///
    /// ## Example
    /// ```
    /// # use charted_entities::ApiKey;
    /// #
    /// let resource = ApiKey::default();
    /// assert_eq!(resource.bitfield().bits(), 0);
    /// ```
    pub fn bitfield(&self) -> ApiKeyScopes {
        ApiKeyScopes::init(self.scopes.try_into().unwrap())
    }
}

/// Represents a [`semver::Version`] which is safe to use in [`Path`][axum::extract::Path]
/// or as sqlx types.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version(semver::Version);
impl Deref for Version {
    type Target = semver::Version;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        semver::Version::deserialize(deserializer).map(Self)
    }
}

impl<'q, DB: Database> Encode<'q, DB> for Version
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as Encode<'q, DB>>::encode_by_ref(&self.0.to_string(), buf)
    }

    fn produces(&self) -> Option<<DB as Database>::TypeInfo> {
        <String as Encode<'q, DB>>::produces(&self.0.to_string())
    }

    fn size_hint(&self) -> usize {
        <String as Encode<'q, DB>>::size_hint(&self.0.to_string())
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Version
where
    String: Decode<'r, DB>,
{
    fn decode(value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let decoded = <String as Decode<'r, DB>>::decode(value)?;
        match semver::Version::parse(&decoded) {
            Ok(version) => Ok(Self(version)),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl<DB: Database> Type<DB> for Version
where
    String: Type<DB>,
{
    fn type_info() -> <DB as Database>::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

impl PgHasArrayType for Version
where
    String: PgHasArrayType,
{
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as PgHasArrayType>::array_type_info()
    }
}

impl<'s> ToSchema<'s> for Version {
    fn schema() -> (&'s str, RefOr<Schema>) {
        let obj = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .description(Some("Represents a [semantic version](https://semver.org) that both Helm and charted-server accept as versions for Helm charts"))
            .pattern(Some(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"))
            .build();

        ("Version", RefOr::T(Schema::Object(obj)))
    }
}

/// Represents a union enum that can hold a Snowflake ([u64]-based integer)
/// and a Name, which is a String that is validated with the Name regex.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum NameOrSnowflake {
    /// [u64]-based integer that can point to a entity resource.
    Snowflake(u64),

    /// Valid UTF-8 string that is used to point to a entity resource. This
    /// is mainly used for `idOrName` path parameters in any of the REST
    /// API endpoints to help identify a resource by a Name or Snowflake
    /// pointer.
    ///
    /// Names are validated with the following regex: `^([A-z]|-|_|\d{0,9}){1,32}`
    Name(Name),
}

impl Display for NameOrSnowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameOrSnowflake::Snowflake(id) => Display::fmt(id, f),
            NameOrSnowflake::Name(name) => Display::fmt(name, f),
        }
    }
}

impl<'s> ToSchema<'s> for NameOrSnowflake {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "NameOrSnowflake",
            RefOr::T(Schema::OneOf(
                OneOfBuilder::new()
                    .description(Some("Represents a union enum that can hold a Snowflake and a Name, which is a String that is validated with the Name regex."))
                    .item(Ref::from_schema_name("Snowflake"))
                    .item(Ref::from_schema_name("Name"))
                    .build(),
            )),
        )
    }
}

impl NameOrSnowflake {
    /// Checks if the value is a valid NameOrSnowflake entity.
    pub fn is_valid(&self) -> Result<(), String> {
        match self {
            NameOrSnowflake::Snowflake(flake) => {
                if *flake < 15 {
                    return Err("was not over or equal to 15 in length".into());
                }

                Ok(())
            }

            NameOrSnowflake::Name(s) => Name::check_is_valid(s.to_string()).map_err(|e| format!("{e}")),
        }
    }
}

impl Validate for NameOrSnowflake {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            NameOrSnowflake::Snowflake(_) => Ok(()),
            NameOrSnowflake::Name(name) => name.validate(),
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
        NameOrSnowflake::Name(value.to_string().into())
    }
}

impl From<String> for NameOrSnowflake {
    fn from(value: String) -> NameOrSnowflake {
        NameOrSnowflake::Name(value.into())
    }
}

impl From<Name> for NameOrSnowflake {
    fn from(value: Name) -> Self {
        NameOrSnowflake::Name(value)
    }
}

impl From<&Name> for NameOrSnowflake {
    fn from(value: &Name) -> Self {
        NameOrSnowflake::Name(Name::new_unchecked(value.as_str()))
    }
}

impl From<&NameOrSnowflake> for NameOrSnowflake {
    fn from(value: &NameOrSnowflake) -> Self {
        match value {
            Self::Snowflake(id) => Self::Snowflake(*id),
            Self::Name(name) => Self::Name(name.clone()),
        }
    }
}
