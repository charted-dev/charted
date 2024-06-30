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

use axum::extract::FromRef;
use charted_common::Snowflake;
use charted_config::Config;
use charted_helm_charts::HelmCharts;
use noelware_remi::StorageService;
use sqlx::postgres::PgPool;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, OnceLock, RwLock,
};

static STATE: OnceLock<ServerContext> = OnceLock::new();

/// Represents the context of the API server. This holds all the dependencies that the
/// CLI's `server` command implementation creates and uses to start the API server.
pub struct ServerContext {
    /// Registry of all possible database controllers. Some might exist, some might not. we might
    /// never know... :ghost:
    pub controllers: charted_database::controllers::Registry,

    /// Amount of requests that the server has received from clients.
    pub requests: AtomicUsize,

    /// A [`StorageService`] container that wraps our persistent data about all things **charted-server**.
    pub storage: StorageService,

    /// Module for publishing, listing, updating, and deletion of Helm charts. The meat and
    /// potatoes of the functionality as they say.
    pub charts: HelmCharts,

    /// Authentication system for authenticating users.
    pub authz: Arc<dyn charted_authz::Authenticator>,

    /// Snowflake generator for unique IDs
    pub snowflake: Snowflake,

    /// Registry of all possible metric collectors.
    pub metrics: Arc<dyn charted_metrics::Registry>,

    /// Parsed configuration that the server runner has supplied.
    pub config: Config,

    /// Redis client that allows to hold temporary, fast data.
    pub redis: Arc<RwLock<charted_core::redis::Client>>,

    /// HTTP client.
    pub http: reqwest::Client,

    /// Represents the database connection pool.
    pub pool: PgPool,
}

impl Clone for ServerContext {
    fn clone(&self) -> Self {
        ServerContext {
            controllers: self.controllers.clone(),
            snowflake: self.snowflake.clone(),
            requests: AtomicUsize::new(self.requests.load(Ordering::Relaxed)),
            metrics: self.metrics.clone(),
            storage: self.storage.clone(),
            charts: self.charts.clone(),
            config: self.config.clone(),
            authz: self.authz.clone(),
            redis: self.redis.clone(),
            http: self.http.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl ServerContext {
    /// Returns a reference to the global [`ServerContext`]. If [`set_global`] was
    /// never called, then this will panic.
    pub fn get<'ctx>() -> &'ctx ServerContext {
        STATE.get().expect("server context was never initialized")
    }
}

impl FromRef<()> for ServerContext {
    fn from_ref(_: &()) -> Self {
        STATE.get().cloned().expect("server context was never initialized")
    }
}

/// Sets the global [`ServerContext`]. We need a global singleton due to how the authorization
/// middleware works.
pub fn set_global(ctx: ServerContext) {
    match STATE.set(ctx) {
        Ok(()) => {}
        Err(_) => panic!("global server context is already set"),
    }
}
