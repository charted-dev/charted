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

use argon2::{PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use charted_common::{hashmap, server::ARGON2};
use charted_config::{Config, ConfigExt};
use charted_redis::RedisClient;
use charted_sessions::{Session, SessionProvider, UserWithPassword};
use chrono::{Duration, Local};
use eyre::{eyre, Context, Result};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::Value;
use sqlx::PgPool;
use std::{collections::HashMap, fmt::Debug};
use tracing::{error, info_span, trace};
use uuid::Uuid;

#[derive(Clone)]
pub struct LocalSessionProvider {
    jwt_encoding_key: EncodingKey,
    redis: RedisClient,
    pool: PgPool,
}

unsafe impl Send for LocalSessionProvider {}

impl Debug for LocalSessionProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalSessionProvider").finish_non_exhaustive()
    }
}

impl LocalSessionProvider {
    pub fn new(redis: RedisClient, pool: PgPool) -> Result<LocalSessionProvider> {
        let config = Config::get();
        let jwt_secret_key = config.jwt_secret_key()?;

        Ok(LocalSessionProvider {
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret_key.as_ref()),
            redis,
            pool,
        })
    }
}

#[async_trait]
impl SessionProvider for LocalSessionProvider {
    async fn authorize(&mut self, password: String, user: &dyn UserWithPassword) -> Result<Session> {
        let user = user.user();
        let span = info_span!(
            "sessions.local.authorize",
            user.id,
            user.username = tracing::field::display(user.username.clone())
        );

        let _guard = span.enter();

        match user.password(self.pool.clone(), user.id as u64).await {
            Ok(Some(pass)) => {
                let hash = PasswordHash::new(&pass).map_err(|e| eyre!("unable to compute hash: {e}"))?;
                match ARGON2.verify_password(password.as_ref(), &hash) {
                    Ok(()) => {
                        let session_id = Uuid::new_v4();
                        let now = Local::now();

                        let two_days = now
                            + Duration::from_std(std::time::Duration::from_secs(172800))
                                .context("clock went backwards?!")?;

                        let one_week = now
                            + Duration::from_std(std::time::Duration::from_secs(604800))
                                .context("clock went backwards?!")?;

                        let header = Header {
                            alg: jsonwebtoken::Algorithm::HS512,
                            ..Default::default()
                        };

                        let access_token = encode::<HashMap<&str, Value>>(
                            &header,
                            &hashmap! {
                                "session_id" => Value::String(session_id.to_string()),
                                "user_id" => Value::Number(user.id.into()),
                                "iss" => Value::String("Noelware/charted-server".into()),
                                "exp" => Value::Number(two_days.timestamp().into())
                            },
                            &self.jwt_encoding_key,
                        )
                        .map_err(|e| {
                            error!(user.id, error = %e, "unable to create access token");
                            sentry::capture_error(&e);

                            eyre::Report::from(e)
                        })?;

                        let refresh_token = encode::<HashMap<&str, Value>>(
                            &header,
                            &hashmap! {
                                "session_id" => Value::String(session_id.to_string()),
                                "user_id" => Value::Number(user.id.into()),
                                "iss" => Value::String("Noelware/charted-server".into()),
                                "exp" => Value::Number(one_week.timestamp().into())
                            },
                            &self.jwt_encoding_key,
                        )
                        .map_err(|e| {
                            error!(user.id, error = %e, "unable to create access token");
                            sentry::capture_error(&e);

                            eyre::Report::from(e)
                        })?;

                        let session = Session {
                            refresh_token: Some(refresh_token),
                            access_token: Some(access_token),
                            session_id,
                            user_id: user.id as u64,
                        };

                        let session_json = serde_json::to_string(&session)?;
                        let mut client = self
                            .redis
                            .client()
                            .unwrap_or(self.redis.master()?)
                            .get_connection_with_timeout(std::time::Duration::from_millis(150))?;

                        trace!("<- HSET charted:sessions {session_id} {session_json}");
                        RedisClient::cmd("HSET")
                            .arg("charted:sessions")
                            .arg(session_id.to_string())
                            .arg(session_json)
                            .query::<()>(&mut client)?;

                        trace!(
                            "<- SET charted:sessions:{session_id} \"this is just some dummy payload (for now >:3)\""
                        );

                        RedisClient::cmd("SET")
                            .arg(format!("charted:sessions:{session_id}"))
                            .arg("this is just some dummy payload (for now >:3)")
                            .query::<()>(&mut client)?;

                        trace!(
                            "<- EXPIRE charted:sessions:{session_id} {:?} XX",
                            std::time::Duration::from_secs(604800)
                        );

                        RedisClient::cmd("EXPIRE")
                            .arg(format!("charted:sessions:{session_id}"))
                            .arg(std::time::Duration::from_secs(604800).as_secs())
                            .arg("NX")
                            .query::<()>(&mut client)?;

                        Ok(session)
                    }
                    Err(e) => Err(eyre!("unable to verify password: {e}")),
                }
            }

            Ok(None) => Err(eyre!(
                "Internal Server Error: user @{} ({}) doens't contain a password!",
                user.username,
                user.id
            )),

            Err(e) => Err(eyre!("unable to retrieve password from database: {e}")),
        }
    }
}
