// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use crate::routing::create_router;
use charted_common::{Snowflake, COMMIT_HASH, VERSION};
use charted_config::{Config, ConfigExt, StorageConfig};
use charted_database::controllers::users::UserDatabaseController;
use charted_database::controllers::DatabaseController;
use charted_database::MIGRATIONS;
use charted_storage::MultiStorageService;
use eyre::Result;
use remi_core::StorageService;
use remi_fs::FilesystemStorageService;
use remi_s3::S3StorageService;
use sentry::types::Dsn;
use sentry::{ClientInitGuard, ClientOptions};
use sqlx::ConnectOptions;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::time::Duration;
use std::{any::Any, borrow::Cow, str::FromStr, sync::Arc};

/// A default implemention of a [`Server`].
#[derive(Clone)]
pub struct Server {
    _sentry_guard: Option<Arc<ClientInitGuard>>,

    // a hacky solution to beat associated types
    //
    // is it wrong? probably.
    // does it work? most likely.
    controllers: Vec<Arc<(dyn Any + Send + Sync)>>,
    snowflake: Snowflake,

    pub storage: MultiStorageService,
    pub config: Config,
    pub pool: PgPool,
}

impl Server {
    pub async fn new(config: Config) -> Result<Server> {
        let sentry_guard = match config.sentry_dsn() {
            Ok(Some(dsn)) => Some(Arc::new(sentry::init(ClientOptions {
                dsn: Some(Dsn::from_str(dsn.as_str())?),
                release: Some(Cow::Owned(format!("charted-server v{VERSION}+{COMMIT_HASH}"))),
                debug: charted_common::is_debug_enabled(),
                attach_stacktrace: true,
                ..Default::default()
            }))),

            Err(e) => {
                error!("unable to get Sentry DSN: {e}");
                None
            }

            _ => None,
        };

        let storage = match config.storage.clone() {
            StorageConfig::Filesystem(fs) => MultiStorageService::Filesystem(FilesystemStorageService::with_config(fs)),
            StorageConfig::S3(s3) => MultiStorageService::S3(S3StorageService::new(s3)),
        };

        storage.init().await?;

        let pool = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .connect_with(
                PgConnectOptions::from_str(config.database.to_string().as_str())?
                    .log_statements(tracing::log::LevelFilter::Trace)
                    .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_secs(5)),
            )
            .await?;

        {
            let pool = pool.clone();
            let _guard = info_span!("database.migrate.run");

            info!("running migrations");
            MIGRATIONS.run(&pool).await?;

            info!("done!");
        }

        // TODO(@auguwu): create cluster of snowflakes with 1023 nodes per
        // server instance?
        let snowflake = Snowflake::new(0);
        let users = UserDatabaseController::new(pool.clone(), snowflake.clone());

        Ok(Server {
            _sentry_guard: sentry_guard,
            controllers: vec![Arc::new(users)],
            snowflake,
            storage,
            config,
            pool,
        })
    }

    pub fn controller<D: DatabaseController + 'static>(&self) -> &D {
        self.controllers
            .iter()
            .find(move |f| f.is::<D>())
            .expect("unable to find any db controller references with specified type")
            .downcast_ref()
            .expect("unable to downcast to &D")
    }

    pub fn snowflake(&mut self) -> &mut Snowflake {
        &mut self.snowflake
    }

    pub async fn run(&self) -> Result<()> {
        let addr = self.config.server.addr();
        info!(%addr, "now listening on");

        let router: axum::Router = create_router().with_state(self.clone());
        axum::Server::bind(&addr).serve(router.into_make_service()).await?;

        Ok(())
    }
}
