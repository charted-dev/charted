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

pub mod common;
pub mod postgresql;
pub mod sqlite;

use azalia::config::{
    env::{self, TryFromEnv, TryParseError},
    merge::Merge,
};
use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::{env::VarError, fmt::Display};

/// The `database` table allows to configure the database that charted-server
/// uses to store persistent data like users, repositories, and more.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum Config {
    /// Uses [PostgreSQL] as the database driver. This is recommended
    /// for production use.
    ///
    /// [PostgreSQL]: https://postgresql.org
    PostgreSQL(postgresql::Config),

    /// Uses [SQLite] as the database driver. This is the recommended
    /// driver for development and evaluation use or don't need
    /// PostgreSQL running.
    ///
    /// [SQLite]: https://sqlite.org
    SQLite(sqlite::Config),
}

impl Default for Config {
    fn default() -> Self {
        Config::SQLite(sqlite::Config::default())
    }
}

impl Merge for Config {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::PostgreSQL(p1), Self::PostgreSQL(p2)) => {
                p1.merge(p2);
            }

            (Self::SQLite(s1), Self::SQLite(s2)) => {
                s1.merge(s2);
            }

            (me, other) => {
                *me = other;
            }
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Config::PostgreSQL(psql) => Display::fmt(psql, f),
            Config::SQLite(sqlite) => Display::fmt(sqlite, f),
        }
    }
}

pub const DRIVER: &str = "CHARTED_DATABASE_DRIVER";

impl TryFromEnv for Config {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        match env::try_parse::<_, String>(DRIVER) {
            Ok(s) => match &*s.to_ascii_lowercase() {
                "postgresql" | "postgres" => Ok(Config::PostgreSQL(postgresql::Config::try_from_env()?)),
                "sqlite" => Ok(Config::SQLite(sqlite::Config::try_from_env()?)),
                s => Err(eyre!("unknown variant for `${}`: {}", DRIVER, s)),
            },

            Err(TryParseError::System(VarError::NotPresent)) => Ok(Config::SQLite(sqlite::Config::try_from_env()?)),
            Err(TryParseError::System(VarError::NotUnicode(_))) => {
                Err(eyre!("received non-unicode in `${}` environment variable", DRIVER))
            }

            Err(err) => Err(err.into()),
        }
    }
}

impl Config {
    pub fn common(&self) -> &common::Config {
        match self {
            Config::PostgreSQL(c) => &c.common,
            Config::SQLite(c) => &c.common,
        }
    }

    pub fn common_mut(&mut self) -> &mut common::Config {
        match self {
            Config::PostgreSQL(c) => &mut c.common,
            Config::SQLite(c) => &mut c.common,
        }
    }

    #[cfg(test)]
    pub(crate) fn as_sqlite(&self) -> Option<&sqlite::Config> {
        match self {
            Config::SQLite(c) => Some(c),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn as_postgresql(&self) -> Option<&postgresql::Config> {
        match self {
            Config::PostgreSQL(c) => Some(c),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalia::config::env::MultipleEnvGuard;

    // A test that is similar to `merge sqlite -> sqlite` but uses
    // the actual system environment variables via `MultipleEnvGuard`
    #[test]
    fn merge_sqlite_to_sqlite_via_environment_variables() {
        let _guard = MultipleEnvGuard::enter([
            (DRIVER, "sqlite"),
            (common::MAX_CONNECTIONS, "30"),
            (common::RUN_PENDING_MIGRATIONS, "yes"),
        ]);

        let mut c1 = Config::default();
        let c2 = Config::try_from_env().expect("failed to parse configuration from system environment variables");

        let old = c1.clone();

        c1.merge(c2);
        let Some(c1) = c1.as_sqlite() else {
            unreachable!()
        };
        let Some(old_c1) = old.as_sqlite() else {
            unreachable!()
        };

        assert_eq!(c1.max_connections, 30);
        assert!(c1.run_migrations);
        assert_eq!(c1.path, old_c1.path);
    }

    // Similar to the below test but loads it via the TOML configuration
    // to see if it works correctly.
    #[test]
    #[ignore = "toml parsing is wack at the moment but it should work once this attr is removed"]
    fn merge_sqlite_to_sqlite_via_configuration_file() {
        #[derive(Deserialize, Default, derive_more::Deref)]
        struct Config {
            database: super::Config,
        }

        const CONFIG: &str = r#"""
        [database.sqlite]
        max_connections = 30
        run_migrations = true
        """#;

        let mut c1 = Config::default();
        let c2: Config = toml::from_str(CONFIG).unwrap();

        let old = c1.clone();

        c1.database.merge(c2.database);
        let Some(c1) = c1.as_sqlite() else {
            unreachable!()
        };
        let Some(old_c1) = old.as_sqlite() else {
            unreachable!()
        };

        assert_eq!(c1.max_connections, 30);
        assert!(c1.run_migrations);
        assert_eq!(c1.path, old_c1.path);
    }

    // given configuration:
    //
    // $CHARTED_DATABASE_DRIVER = sqlite
    // $CHARTED_DATABASE_MAX_CONNECTIONS = 30
    // $CHARTED_DATABASE_RUN_PENDING_MIGRATIONS = true
    #[test]
    #[ignore = "MultipleEnvGuard has issues at the moment and needs to be resolved"]
    fn merge_sqlite_to_sqlite() {
        let mut c1 = Config::default();
        let c2 = Config::SQLite(sqlite::Config {
            common: common::Config {
                max_connections: 30,
                run_migrations: true,

                ..Default::default()
            },

            path: c1.as_sqlite().unwrap().path.clone(),
        });

        let old = c1.clone();

        c1.merge(c2);
        let Some(c1) = c1.as_sqlite() else {
            unreachable!()
        };
        let Some(old_c1) = old.as_sqlite() else {
            unreachable!()
        };

        assert_eq!(c1.max_connections, 30);
        assert!(c1.run_migrations);
        assert_eq!(c1.path, old_c1.path);
    }

    // given configuration:
    // $CHARTED_DATABASE_DRIVER = postgresql
    // $CHARTED_DATABASE_MAX_CONNECTIONS = 100
    // $CHARTED_DATABASE_RUN_PENDING_MIGRATIONS = true
    // $CHARTED_DATABASE_USERNAME = noel
    #[test]
    fn merge_psql_to_psql() {
        let mut c1 = Config::PostgreSQL(postgresql::Config::default());
        let c2 = Config::PostgreSQL(postgresql::Config {
            common: common::Config {
                max_connections: 100,
                run_migrations: true,

                ..Default::default()
            },

            username: Some(String::from("noel")),
            ..Default::default()
        });

        let old = c1.clone();
        c1.merge(c2);

        let Some(c1) = c1.as_postgresql() else {
            unreachable!()
        };
        let Some(old_c1) = old.as_postgresql() else {
            unreachable!()
        };

        assert_eq!(c1.max_connections, 100);
        assert!(c1.run_migrations);
        assert_eq!(c1.username, Some(String::from("noel")));
        assert_eq!(c1.password, old_c1.password);
        assert_eq!(c1.database, old_c1.database);
        assert_eq!(c1.schema, old_c1.schema);
        assert_eq!(c1.url, old_c1.url);
    }

    // A test that is similar to `merge sqlite -> sqlite` but uses
    // the actual system environment variables via `MultipleEnvGuard`
    #[test]
    #[ignore = "MultipleEnvGuard has issues at the moment and needs to be resolved"]
    fn merge_psql_to_psql_via_environment_variables() {
        let _guard = MultipleEnvGuard::enter([
            (DRIVER, "postgres"),
            (common::MAX_CONNECTIONS, "100"),
            (common::RUN_PENDING_MIGRATIONS, "yes"),
            (postgresql::USERNAME, "noel"),
        ]);

        let mut c1 = Config::PostgreSQL(postgresql::Config::default());
        let c2 = Config::try_from_env().expect("failed to parse configuration from system environment variables");

        let old = c1.clone();
        c1.merge(c2);

        let Some(c1) = c1.as_postgresql() else {
            unreachable!()
        };
        let Some(old_c1) = old.as_postgresql() else {
            unreachable!()
        };

        assert_eq!(c1.max_connections, 100);
        assert!(c1.run_migrations);
        assert_eq!(c1.username, Some(String::from("noel")));
        assert_eq!(c1.password, old_c1.password);
        assert_eq!(c1.database, old_c1.database);
        assert_eq!(c1.schema, old_c1.schema);
        assert_eq!(c1.url, old_c1.url);
    }

    // Similar to the below test but loads it via the TOML configuration
    // to see if it works correctly.
    #[test]
    #[ignore = "toml parsing is wack at the moment but it should work once this attr is removed"]
    fn merge_psql_to_psql_via_configuration_file() {
        #[derive(Deserialize, Default, derive_more::Deref)]
        struct Config {
            database: super::Config,
        }

        const _: &str = r#"""
        [database.postgresql]
        max_connections = 30
        run_migrations = true
        url = "postgres://noel@noeliscutieuwu:localhost:5432/charted"
        """#;

        // let mut c1 = Config::default();
        // let c2: Config = toml::from_str(CONFIG).unwrap();

        // let old = c1.clone();

        // c1.database.merge(c2.database);
        // let Some(c1) = c1.as_sqlite() else { unreachable!() };
        // let Some(old_c1) = old.as_sqlite() else { unreachable!() };

        // assert_eq!(c1.max_connections, 30);
        // assert!(c1.run_migrations);
        // assert_eq!(c1.db_path, old_c1.db_path);
    }
}
