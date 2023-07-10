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

use crate::routing::*;
use charted_common::{COMMIT_HASH, VERSION};
use charted_config::{Config, ConfigExt, StorageConfig};
use charted_storage::MultiStorageService;
use eyre::Result;
use remi_core::StorageService;
use remi_fs::FilesystemStorageService;
use remi_s3::S3StorageService;
use sentry::{types::Dsn, ClientInitGuard, ClientOptions};
use std::{borrow::Cow, str::FromStr, sync::Arc};

#[derive(Clone)]
pub struct Server {
    /// Represents the configured storage service to use.
    pub storage: MultiStorageService,

    /// Represents a reference to the parsed [`Config`] from
    /// the 'server' subcommand in the CLI.
    pub config: Config,

    // Sentry guard that will be dropped once this struct
    // is done.
    _sentry_guard: Option<Arc<ClientInitGuard>>,
}

impl Server {
    /// Creates a new [`Server`] object.
    pub async fn new() -> Result<Server> {
        let config = Config::get();
        let sentry_guard = match config.sentry_dsn() {
            Ok(Some(dsn)) => Some(Arc::new(sentry::init(ClientOptions {
                dsn: Some(Dsn::from_str(dsn.as_str())?),
                release: Some(Cow::Owned(format!("charted-server v{VERSION}+{COMMIT_HASH}"))),
                debug: charted_common::is_debug_enabled(),
                attach_stacktrace: true,
                ..Default::default()
            }))),

            _ => None,
        };

        info!("server init: init storage service");
        let storage = match config.storage.clone() {
            StorageConfig::Filesystem(fs) => MultiStorageService::Filesystem(FilesystemStorageService::with_config(fs)),
            StorageConfig::S3(s3) => MultiStorageService::S3(S3StorageService::new(s3)),
        };

        storage.init().await?;
        info!("server init: init storage server (success)");

        Ok(Server {
            storage,
            config,
            _sentry_guard: sentry_guard,
        })
    }

    pub async fn run(&self) -> Result<()> {
        let router = create_router();
        let addr = self.config.server.addr();

        info!(%addr, "charted-server is now listening on");
        axum::Server::bind(&addr).serve(router.into_make_service()).await?;

        Ok(())
    }
}
