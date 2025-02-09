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

pub mod database;
pub mod features;
pub mod logging;
pub mod metrics;
pub mod server;
pub mod storage;
pub mod tracing;
pub(crate) mod util;

use azalia::config::merge::Merge;
use sentry_types::{protocol::v7::Url, Dsn};
use serde::{Deserialize, Serialize};

/// The root configuration for the API server.
///
/// **charted-server** uses a TOML-based configuration format for easy accessiblity.
///
/// **charted-server** also supports environment variables that can be overwritten when the configuration is
/// being loaded. The priority is **Environment Variables > Configuration File**.
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Configuration {
    /// A secret key for generating JWT tokens for session-based authentication.
    ///
    /// It is recommended to set this as the `CHARTED_JWT_SECRET_KEY` environment
    /// variable and **charted-server** will load it into this property.
    ///
    /// If this is ever messed with, sessions that are on-going will be permanently corrupted.
    #[serde(default)]
    #[merge(skip)]
    pub jwt_secret_key: String,

    /// Whether if this instance accepts user registrations.
    #[serde(default = "crate::util::truthy")]
    pub registrations: bool,

    /// whether if this instance should like a single user registry.
    ///
    /// If this is the case, most features are disabled like organizations,
    /// repository/organization members, user creation, etc.
    ///
    /// You can use either the `charted admin user new <myname> <myemail> <mypassword>`
    /// if you're going to use the local session backend or use the static backend with
    /// `user = "password"`.
    #[serde(default)]
    pub single_user: bool,

    /// whether if this instance should act like a single organization registry.
    ///
    /// If so, most features are disabled like user creation, repository members, etc.
    #[serde(default)]
    pub single_org: bool,

    /// opt into reporting errors to a [Sentry] server.
    ///
    /// [Sentry]: https://sentry.io
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sentry_dsn: Option<Dsn>,

    /// URI that will redirect all API requests and Helm chart downloads towards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<Url>,

    /// Database configuration.
    #[serde(default)]
    pub database: database::Config,
}
