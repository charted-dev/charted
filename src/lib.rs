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

use avatars::AvatarsModule;
use axum::extract::FromRef;
use common::Snowflake;
use metrics::Registry;
use noelware_remi::StorageService;
use once_cell::sync::{Lazy, OnceCell};
use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;
use search::JoinedBackend;
use std::{
    any::Any,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
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
pub mod auth;
pub mod avatars;
pub mod caching;
pub mod charts;
pub mod cli;
pub mod common;
pub mod config;
pub mod db;
pub mod emails;
pub mod macros;
pub mod metrics;
pub mod openapi;
pub mod redis;
pub mod search;
pub mod server;
pub mod sessions;

/// Snowflake epoch used for ID generation. (March 1st, 2023)
pub const SNOWFLAKE_EPOCH: usize = 1677654000000;

/// Returns the version of the Rust compiler that charted-server
/// was compiled on.
pub const RUSTC_VERSION: &str = env!("CHARTED_RUSTC_VERSION");

/// Returns the Git commit hash from the charted-server repository that
/// this build was built off from.
pub const COMMIT_HASH: &str = env!("CHARTED_COMMIT_HASH");

/// RFC3339-formatted date of when charted-server was last built at.
pub const BUILD_DATE: &str = env!("CHARTED_BUILD_DATE");

/// Returns the current version of `charted-server`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Generic [`Regex`] implementation for possible truthy boolean values.
pub static TRUTHY_REGEX: Lazy<Regex> = lazy!(Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap());

/// Checks if debug mode is enabled or not.
pub fn is_debug_enabled() -> bool {
    if cfg!(debug_assertions) {
        return true;
    }

    matches!(std::env::var("CHARTED_DEBUG"), Ok(val) if TRUTHY_REGEX.is_match(val.as_str()))
}

/// Returns a randomized alphanumeric string with a specified length.
pub fn rand_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
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
pub fn version() -> String {
    use std::fmt::Write;

    let mut buf = String::new();
    write!(buf, "{VERSION}").unwrap();

    if !COMMIT_HASH.is_empty() && COMMIT_HASH != "d1cebae" {
        write!(buf, "+{COMMIT_HASH}").unwrap();
    }

    buf
}

static GLOBAL: OnceCell<Instance> = OnceCell::new();

/// Represents the core instance of charted-server. It contains a whole bunch of references
/// to be able to operate successfully.
pub struct Instance {
    pub controllers: db::controllers::Controllers,
    pub requests: AtomicUsize,
    pub avatars: AvatarsModule,
    pub storage: StorageService,
    pub charts: charts::HelmCharts,
    pub authz: Arc<dyn auth::Backend>,
    pub search: Option<Arc<JoinedBackend>>,
    pub sessions: Arc<Mutex<sessions::Manager>>,
    pub snowflake: Snowflake,
    pub metrics: Arc<dyn Registry>,
    pub config: config::Config,
    pub redis: Arc<RwLock<redis::Client>>,
    pub pool: sqlx::postgres::PgPool,
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        Instance {
            controllers: self.controllers.clone(),
            snowflake: self.snowflake.clone(),
            requests: AtomicUsize::new(self.requests.load(Ordering::Relaxed)),
            sessions: self.sessions.clone(),
            avatars: self.avatars.clone(),
            metrics: self.metrics.clone(),
            storage: self.storage.clone(),
            charts: self.charts.clone(),
            search: self.search.clone(),
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
