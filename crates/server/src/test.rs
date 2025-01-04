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

use crate::ServerContext;
use charted_config::{
    database::{self, sqlite},
    logging, server,
    sessions::{self, Backend},
    storage, Config,
};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use tracing::Level;

const JWT_SECRET_KEY: &str = "pleasedontbedumbandsetthisasyourjwtsecretkeyuseopensslinsteadlol";

pub fn create_config<F: Fn(&mut Config)>(dir: &Path, modifications: &[F]) -> Config {
    let mut config = Config {
        base_url: Some("http://localhost:3651".parse().unwrap()),
        registrations: true,
        jwt_secret_key: String::from(JWT_SECRET_KEY),
        sentry_dsn: None,
        single_user: true,
        single_org: false,

        database: database::Config::SQLite(sqlite::Config {
            run_migrations: true,
            max_connections: 2,
            db_path: PathBuf::from(":memory:"),
        }),

        storage: storage::Config::Filesystem(azalia::remi::fs::StorageConfig {
            directory: dir.join("data"),
        }),

        logging: logging::Config {
            level: Level::INFO,
            json: false,
        },

        sessions: sessions::Config {
            backend: Backend::Local,
            enable_basic_auth: true,
        },

        server: server::Config {
            host: "0.0.0.0".into(),
            port: 3651,
            ssl: None,
        },
    };

    for m in modifications {
        m(&mut config);
    }

    config
}

pub async fn create_server_context(cfgs: &[impl Fn(&mut Config)]) -> eyre::Result<(TempDir, ServerContext)> {
    let tmpdir = tempfile::tempdir().expect("temporary directory should be present");
    let config = create_config(tmpdir.path(), cfgs);

    Ok((
        tmpdir,
        ServerContext::new(charted_app::Context::new(config).await?, vec![]),
    ))
}
