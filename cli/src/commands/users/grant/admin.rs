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

use charted_common::{
    cli::AsyncExecute,
    models::{entities::User, Name},
};
use charted_config::Config;
use clap::value_parser;
use eyre::{Context, Result};
use promptly::ReadlineError;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, Connection, PgConnection};
use std::{panic::catch_unwind, path::PathBuf, process::exit, str::FromStr, time::Duration};
use tracing::log::LevelFilter;

/// Grants a user administration access.
#[derive(Debug, Clone, clap::Parser)]
pub struct Admin {
    /// Configuration file to locate, this will look in the default locations and
    /// in the `CHARTED_CONFIG_FILE` environment variable. This is used
    /// to connect to a PostgreSQL server to edit the database.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Deliberately force the operation without a prompt.
    #[arg(long, short = 'f')]
    force: bool,

    /// The user to grant admin access towards.
    user: Name,
}

#[async_trait]
impl AsyncExecute for Admin {
    async fn execute(&self) -> Result<()> {
        let username = match self.user.is_valid() {
            Ok(()) => self.user.to_string(),
            Err(e) => {
                error!("name [{}] is invalid: {e}", self.user);
                exit(1);
            }
        };

        Config::load(self.config.clone())?;
        let config = Config::get();

        info!("🛰️   contacting & connecting to PostgreSQL...");
        let mut conn = PgConnection::connect_with(
            &PgConnectOptions::from_str(config.database.to_string().as_str())?
                .log_slow_statements(LevelFilter::Warn, Duration::from_secs(10))
                .log_statements(LevelFilter::Trace),
        )
        .await?;

        info!("established connection successfully! verifying if user @{username} exists...");
        let user = match sqlx::query_as::<_, User>("select users.* from users where username = $1;")
            .bind(username.clone())
            .fetch_optional(&mut conn)
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => {
                warn!("user @{username} doesn't exist.");
                return Ok(());
            }

            Err(e) => {
                error!("unable to run query: {e}");
                exit(1);
            }
        };

        if !self.force {
            match promptly::prompt::<bool, _>(format!(
                "Are you sure you want to grant @{username} ({}) administration privileges?",
                user.id
            )) {
                Ok(true) => {}
                Ok(false) => {
                    info!("action will not be taken.");
                    return Ok(());
                }

                Err(e) => match e {
                    ReadlineError::Eof => {
                        warn!("received Eof early, is stdin open?");
                        exit(1);
                    }

                    ReadlineError::Interrupted => {
                        warn!("received interrupt during prompt");
                        return Ok(());
                    }

                    e => return Err(e.into()),
                },
            }
        }

        let admin = !user.admin;
        info!("persisting user @{username} ({}) admin state to [{}]", user.id, admin);

        match sqlx::query("update users set admin = $1 where id = $2;")
            .bind(admin)
            .bind(user.id)
            .execute(&mut conn)
            .await
        {
            Ok(_) => {
                let msg = match admin {
                    true => "now an admin",
                    false => "no longer an administrator of this instance",
                };

                info!("persisted state! @{username} ({}) is {msg}", user.id);
                let _ = conn.close().await;

                Ok(())
            }

            Err(e) => {
                error!("unable to persist: {e}");
                let _ = conn.close().await;

                exit(1);
            }
        }
    }
}
