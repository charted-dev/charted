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

/// Represents the context of the API server. This holds all the dependencies that the
/// CLI's `server` command implementation creates and uses to start the API server.
#[derive(Debug, Clone)]
pub struct Context {}

/*
/// Represents the core instance of charted-server. It contains a whole bunch of references
/// to be able to operate successfully.
pub struct Instance {
    /// List of database controllers that are used to query objects in the Postgres database.
    pub controllers: db::controllers::Controllers,

    /// Amount of requests this instance has received
    pub requests: AtomicUsize,

    /// [`StorageService`] that holds persistent data about user and organization Helm charts,
    /// user/organization icons, repository avatars, and user and organization Helm indexes.
    pub storage: StorageService,

    /// Module for publishing, listing, updating, and deletion of Helm charts.
    pub charts: charted_helm_charts::HelmCharts,

    /// [`authz::Backend`] that is used to authenticate users.
    pub authz: Arc<dyn authz::Backend>,

    /// Manager for handling all user sessions.
    pub sessions: Arc<Mutex<sessions::Manager>>,

    /// Snowflake generator.
    pub snowflake: Snowflake,

    /// [`Registry`] that allows to pull metrics from this instance.
    pub metrics: Arc<dyn Registry>,

    /// the parsed configuration that the user has supplied.
    pub config: charted_config::Config,

    /// Redis client that supports Redis Sentinel and Clustered modes.
    pub redis: Arc<RwLock<redis::Client>>,

    /// sqlx pool to allow dynamic queries that aren't supported by the db controllers
    /// and holds all connections to the Postgres database.
    pub pool: sqlx::postgres::PgPool,
}
*/
