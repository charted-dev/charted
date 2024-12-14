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

use azalia::remi::StorageService;
use charted_authz::Authenticator;
use charted_config::{sessions::Backend, storage, Config};
use charted_core::ulid::AtomicGenerator;
use charted_database::DbPool;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tracing::info;

/// Represents the application context that runs through the whole lifetime
/// of **charted-server**.
pub struct Context {
    /// Generator that atomically generates monotonic ULIDs.
    pub ulid_gen: AtomicGenerator,

    /// How many requests the server has served.
    pub requests: AtomicUsize,

    /// Data storage.
    pub storage: StorageService,

    /// Parsed configuration from the `charted.hcl` file or system environment variables.
    pub config: Config,

    /// Authenticator that allows authenticating users.
    pub authz: Arc<dyn Authenticator>,

    /// Database pool.
    pub pool: DbPool,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            ulid_gen: self.ulid_gen.clone(),
            requests: AtomicUsize::new(self.requests.load(Ordering::SeqCst)),
            storage: self.storage.clone(),
            config: self.config.clone(),
            authz: self.authz.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl Context {
    /// Creates a new [`Context`] object with the given configuration.
    pub async fn new(config: Config) -> eyre::Result<Self> {
        let pool = charted_database::create_pool(&config.database)?;
        let version = charted_database::version(&pool)?;

        info!("received database version [{version}]: connection succeeded.");
        if config.database.can_run_migrations() {
            info!("performing data migration!");
            charted_database::migrations::migrate(&pool)?;
        }

        info!("initializing data storage!");
        let storage = match config.storage.clone() {
            storage::Config::Filesystem(fs) => {
                StorageService::Filesystem(azalia::remi::fs::StorageService::with_config(fs))
            }
            storage::Config::Azure(azure) => StorageService::Azure(
                azalia::remi::azure::StorageService::new(azure)
                    .expect("should be able to create Azure storage service"),
            ),
            storage::Config::S3(s3) => StorageService::S3(azalia::remi::s3::StorageService::new(s3)),
        };

        <StorageService as azalia::remi::core::StorageService>::init(&storage).await?;

        info!("initializing authentication backend");
        let authz: Arc<dyn Authenticator> = match config.sessions.backend.clone() {
            Backend::Local => Arc::new(charted_authz_local::Backend),
            Backend::Ldap(ldap) => Arc::new(charted_authz_ldap::Backend::new(ldap)),
        };

        Ok(Context {
            ulid_gen: AtomicGenerator::new(),
            requests: AtomicUsize::new(0),
            storage,
            config,
            authz,
            pool,
        })
    }
}
