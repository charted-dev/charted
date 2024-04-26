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
use charted_common::{lazy, Snowflake};
use charted_metrics::Registry;
use noelware_remi::StorageService;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use std::{
    any::Any,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Once, OnceLock,
    },
};
use tokio::sync::{Mutex, RwLock};

// useful global macros to use in the whole crate
#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate eyre;

// modules
pub mod authz;
pub mod avatars;
pub mod caching;
pub mod cli;
pub mod db;
pub mod emails;
pub mod metrics;
pub mod openapi;
pub mod redis;
pub mod server;
pub mod sessions;

/// Snowflake epoch used for ID generation. (March 1st, 2023)
pub const SNOWFLAKE_EPOCH: usize = 1677654000000;

/// Returns the version of the Rust compiler that charted-server
/// was compiled on.
#[deprecated(
    since = "0.1.0-beta",
    note = "Replaced by `charted_common::VERSION`, will be removed in v0.2.0-beta"
)]
pub const RUSTC_VERSION: &str = charted_common::RUSTC_VERSION;

/// Returns the Git commit hash from the charted-server repository that
/// this build was built off from.
#[deprecated(
    since = "0.1.0-beta",
    note = "Replaced by `charted_common::VERSION`, will be removed in v0.2.0-beta"
)]
pub const COMMIT_HASH: &str = charted_common::COMMIT_HASH;

/// RFC3339-formatted date of when charted-server was last built at.
#[deprecated(
    since = "0.1.0-beta",
    note = "Replaced by `charted_common::VERSION`, will be removed in v0.2.0-beta"
)]
pub const BUILD_DATE: &str = charted_common::BUILD_DATE;

/// Returns the current version of `charted-server`.
#[deprecated(
    since = "0.1.0-beta",
    note = "Replaced by `charted_common::VERSION`, will be removed in v0.2.0-beta"
)]
pub const VERSION: &str = charted_common::VERSION;

/// Generic [`Regex`] implementation for possible truthy boolean values.
pub static TRUTHY_REGEX: Lazy<Regex> = lazy!(Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap());

/// Lazily constructed [`reqwest::Client`] that allows the instance to send HTTP requests
/// to other servers.
pub static HTTP_CLIENT: Lazy<Client> = lazy!(Client::builder()
    .user_agent(format!(
        "Noelware/charted-server (+https://github.com/charted-dev/charted; v{})",
        crate::version(),
    ))
    .build()
    .unwrap());

/// Checks if debug mode is enabled or not.
pub fn is_debug_enabled() -> bool {
    if cfg!(debug_assertions) {
        return true;
    }

    matches!(std::env::var("CHARTED_DEBUG"), Ok(val) if TRUTHY_REGEX.is_match(val.as_str()))
}

/// Returns a [`String`] buffer that represents a panic message from methods like [`catch_unwind`](std::panic::catch_unwind).
pub fn panic_message(error: Box<dyn Any + Send + 'static>) -> String {
    if let Some(s) = error.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = error.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "unknown panic message received".into()
    }
}

/// Returns a version that includes the Git commit hash in the version string.
pub fn version<'v>() -> &'v str {
    static ONCE: Once = Once::new();
    static mut VALUE: String = String::new();

    // Safety: `VALUE` is only mutated by the `ONCE.call_once` block and nowhere else and
    //         cannot be mutated as it'll return an immutable string.
    unsafe {
        ONCE.call_once(move || {
            use std::fmt::Write;

            let mut buf = String::new();
            write!(buf, "{}", charted_common::VERSION).unwrap();

            if !charted_common::COMMIT_HASH.is_empty() && charted_common::COMMIT_HASH != "d1cebae" {
                write!(buf, "+{}", charted_common::COMMIT_HASH).unwrap();
            }

            VALUE = buf;
        });

        VALUE.as_str()
    }
}

static GLOBAL: OnceLock<Instance> = OnceLock::new();

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

impl Clone for Instance {
    fn clone(&self) -> Self {
        Instance {
            controllers: self.controllers.clone(),
            snowflake: self.snowflake.clone(),
            requests: AtomicUsize::new(self.requests.load(Ordering::Relaxed)),
            sessions: self.sessions.clone(),
            metrics: self.metrics.clone(),
            storage: self.storage.clone(),
            charts: self.charts.clone(),
            config: self.config.clone(),
            authz: self.authz.clone(),
            redis: self.redis.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl Instance {
    /// Returns a reference to a initialized [`Instance`]. This will panic
    /// if [`set_instance`] wasn't called.
    pub fn get<'s>() -> &'s Instance {
        GLOBAL.get().unwrap()
    }
}

impl FromRef<()> for Instance {
    fn from_ref(_input: &()) -> Self {
        GLOBAL.get().expect("instance to be available").clone()
    }
}

/// Sets a global [`Instance`]. This can be only called once.
pub fn set_instance(instance: Instance) {
    match GLOBAL.set(instance) {
        Ok(()) => {}
        Err(_) => panic!("already set a global instance"),
    }
}

#[cfg(test)]
fn __assert_send<T: Send>() {}

#[cfg(test)]
fn __assert_sync<T: Sync>() {}

#[cfg(test)]
fn __assertions() {
    __assert_send::<Instance>();
    __assert_sync::<Instance>();
}
