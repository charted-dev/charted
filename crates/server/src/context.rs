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

use crate::feature;
use axum::extract::FromRef;
use azalia::remi::StorageService;
use charted_authz::Authenticator;
use charted_config::Config;
use sea_orm::DatabaseConnection;
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

static SINGLETON: OnceLock<Context> = OnceLock::new();

pub struct Context {
    pub ulid_generator: charted_core::ulid::Generator,
    pub requests: AtomicUsize,
    pub features: feature::Collection,
    pub storage: StorageService,
    pub config: Config,
    pub authz: Arc<dyn Authenticator>,
    pub pool: DatabaseConnection,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            ulid_generator: self.ulid_generator.clone(),
            requests: AtomicUsize::new(self.requests.load(Ordering::SeqCst)),
            features: self.features.clone(),
            storage: self.storage.clone(),
            config: self.config.clone(),
            authz: self.authz.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl Context {
    pub fn get<'ctx>() -> &'ctx Context {
        SINGLETON.get().unwrap()
    }
}

impl FromRef<()> for Context {
    fn from_ref(_: &()) -> Self {
        SINGLETON
            .get()
            .cloned()
            .expect("global server context was never initialized")
    }
}

pub fn set_context(ctx: Context) {
    match SINGLETON.set(ctx) {
        Ok(_) => {}
        Err(_) => panic!("global context was already set"),
    }
}
