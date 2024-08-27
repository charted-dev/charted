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
use charted_authz::Authenticator;
use charted_config::Config;
use charted_database::DbPool;
use charted_features::Feature;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, OnceLock,
};

static INSTANCE: OnceLock<ServerContext> = OnceLock::new();

/// Represents the context of the API server. This contains all the dependencies
/// that are initialized when the '`charted server`' command is executed.
pub struct ServerContext {
    /// Amount of requests the server has been hit with so far.
    pub requests: AtomicUsize,

    /// List of enabled features.
    pub features: Vec<Arc<dyn Feature>>,

    /// Parsed configuration from `charted.hcl` or system environment variables.
    pub config: Config,

    /// [`charted_authz::Authenticator`] for authenticating users from session middleware.
    pub authz: Arc<dyn Authenticator>,

    /// [`DbPool`] that contains the connection pool for all database connections.
    pub pool: DbPool,
}

impl Clone for ServerContext {
    fn clone(&self) -> Self {
        ServerContext {
            requests: AtomicUsize::new(self.requests.load(Ordering::SeqCst)),
            features: self.features.clone(),
            config: self.config.clone(),
            authz: self.authz.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl ServerContext {
    /// Return a reference to the global [`ServerContext`] that was set from [`set_global`]. If
    /// [`set_global`] isn't called, then this will panic.
    pub fn get<'ctx>() -> &'ctx ServerContext {
        INSTANCE.get().expect("global server context was never initialized")
    }
}

impl FromRef<()> for ServerContext {
    fn from_ref(_: &()) -> Self {
        INSTANCE
            .get()
            .cloned()
            .expect("global server context was never initialized")
    }
}

pub(crate) fn set_global(ctx: ServerContext) {
    if INSTANCE.set(ctx).is_err() {
        panic!("a global server context has been set already");
    }
}
