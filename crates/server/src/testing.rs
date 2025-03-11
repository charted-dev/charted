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

use crate::Context;
use azalia::remi::StorageService;
use charted_config::{
    Config, database, metrics,
    sessions::{self, Backend},
};
use charted_core::ulid::Generator;
use sea_orm::DatabaseConnection;
use std::sync::{Arc, atomic::AtomicUsize};

// so that sessions are "consistent" enough between tests
const JWT_SECRET_KEY: &str =
    "ahashthatshouldbeavalidhashfromopensslbutidontwanttodothatandnooneshouldusethisvaluetobeginwithuwu";

/// Creates a configuration object for test purposes.
pub fn create_config(override_fn: impl FnOnce(&mut Config)) -> Config {
    let mut config = Config {
        jwt_secret_key: String::from(JWT_SECRET_KEY),
        registrations: true,
        single_user: false,
        single_org: false,
        sentry_dsn: None,
        base_url: None,
        logging: Default::default(),
        storage: Default::default(),
        tracing: None,
        metrics: metrics::Config::Disabled,
        server: Default::default(),

        sessions: sessions::Config {
            enable_basic_auth: false,
            backend: Backend::Local,
        },

        database: database::Config::SQLite(database::sqlite::Config {
            common: Default::default(),
            path: String::from(":memory:").into(),
        }),
    };

    override_fn(&mut config);
    config
}

/// Sets the global context for tests.
pub fn set_and_use_context(config: Config) -> Context {
    let ctx = Context {
        ulid_generator: Generator::new(),
        requests: AtomicUsize::default(),
        features: azalia::hashmap!(),
        storage: StorageService::__non_exhaustive,
        config,
        pool: DatabaseConnection::Disconnected,

        authz: Arc::new(charted_authz_static::Backend::new(azalia::btreemap! {
            // echo "noeliscutieuwu" | cargo cli admin authz hash-password --stdin
            "noel" => "$argon2id$v=19$m=19456,t=2,p=1$gIcVA4mVHgr8ZWkmDrtJlw$sb5ypFAvphFCGrJXy9fRI1Gb/2vGIH1FTzDax458+xY"
        })),
    };

    crate::set_context(ctx.clone());
    ctx
}
