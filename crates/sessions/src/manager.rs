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

use crate::{Session, SessionProvider, UserWithPassword};
use charted_common::{hashmap, models::entities::User};
use charted_config::{Config, ConfigExt};
use charted_redis::RedisClient;
use chrono::Local;
use eyre::{eyre, Context, Report, Result};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::Value;
use std::{
    collections::HashMap,
    convert::TryInto,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, info_span, warn};
use uuid::Uuid;

/// An abstraction over handling sessions.
#[derive(Debug, Clone)]
pub struct SessionManager {
    initialized: bool,
    provider: Arc<tokio::sync::Mutex<Box<dyn SessionProvider>>>,
    handles: HashMap<Uuid, Arc<Mutex<JoinHandle<Result<()>>>>>,
    redis: RedisClient,
}

unsafe impl Send for SessionManager {}
unsafe impl Sync for SessionManager {}

impl SessionManager {
    /// Creates a new [`SessionManager`].
    pub fn new(redis: RedisClient, provider: Box<dyn SessionProvider>) -> SessionManager {
        SessionManager {
            initialized: false,
            provider: Arc::new(tokio::sync::Mutex::new(provider)),
            handles: hashmap!(),
            redis,
        }
    }

    pub fn provider(&self) -> Arc<tokio::sync::Mutex<Box<dyn SessionProvider>>> {
        self.provider.clone()
    }

    /// Initializes this [`SessionManager`].
    pub fn init(&mut self) -> Result<()> {
        if self.initialized {
            warn!("SessionManager::init has been called more than once");
            return Ok(());
        }

        let mut client = self
            .redis
            .client()
            .unwrap_or(self.redis.replica()?)
            .get_connection_with_timeout(Duration::from_millis(150))?;

        let mut master = match self.redis.client() {
            Some(_) => None,
            None => Some(
                self.redis
                    .master()?
                    .get_connection_with_timeout(Duration::from_millis(150))?,
            ),
        };

        let mapping: HashMap<String, String> =
            RedisClient::cmd("HGETALL").arg("charted:sessions").query(&mut client)?;

        info!(sessions = mapping.len(), "processing all sessions from Redis");
        let mut handles = hashmap!();

        for (id, payload) in mapping.into_iter() {
            let uuid = Uuid::parse_str(id.as_str())?;
            let Ok(session) = serde_json::from_str::<Session>(&payload) else {
                error!(
                    session.id = uuid.to_string(),
                    "unable to process session! deleting session..."
                );

                RedisClient::cmd("HDEL")
                    .arg("charted:sessions")
                    .arg(id)
                    .query(&mut master.as_mut().unwrap_or(&mut client))?;

                continue;
            };

            let span = info_span!(
                "charted.sessions.process",
                session.user = session.user_id,
                session.id = uuid.to_string()
            );

            let _guard = span.enter();

            debug!("<- TTL charted:sessions:{id}");
            let ttl: i32 = RedisClient::cmd("TTL")
                .arg(format!("charted:sessions:{id}"))
                .query(&mut client)?;

            debug!(
                "-> TTL charted:sessions:{id} :: session {uuid} {}",
                match ttl {
                    -2 => "is invalid, deleting session".into(),
                    -1 => "has expired!".into(),
                    _ => format!("has {ttl} seconds left"),
                }
            );

            match ttl {
                -2 | -1 => {
                    RedisClient::cmd("HDEL")
                        .arg("charted:sessions")
                        .arg(id)
                        .query(&mut master.as_mut().unwrap_or(&mut client))?;

                    // we shouldn't care if it errors (since it could've expired already, so shrug)
                    let _ = RedisClient::cmd("DEL")
                        .arg("charted:sessions:{id}")
                        .query::<()>(&mut master.as_mut().unwrap_or(&mut client));

                    continue;
                }

                _ => {
                    let mut redis = self.redis.clone();
                    let handle: JoinHandle<Result<()>> = tokio::task::spawn(async move {
                        tokio::time::sleep(Duration::from_secs(ttl as u64)).await;

                        // We will need to create a connection to Redis once
                        // the task is completed since it'll be dropped after
                        // we are done initializing.
                        let mut client = redis
                            .client()
                            .unwrap_or(redis.master()?)
                            .get_connection_with_timeout(Duration::from_millis(150))?;

                        RedisClient::cmd("HDEL")
                            .arg("charted:sessions")
                            .arg(uuid.clone().to_string())
                            .query(&mut client)
                            .context("unable to delete session {id} from Redis")
                    });

                    debug!(
                        session.id = uuid.to_string(),
                        session.user = session.user_id,
                        "tokio task has been spawned"
                    );

                    handles.insert(uuid, Arc::new(Mutex::new(handle)));
                }
            }
        }

        self.initialized = true;
        self.handles = handles;

        Ok(())
    }

    pub fn create_task(&mut self, session_id: Uuid, duration: Duration) {
        debug!(
            session.id = session_id.to_string(),
            "spawning task for session with a duration of {:?}", duration
        );

        let mut redis = self.redis.clone();
        self.handles.insert(
            session_id,
            Arc::new(Mutex::new(tokio::spawn(async move {
                tokio::time::sleep(duration).await;

                // We will need to create a connection to Redis once
                // the task is completed since it'll be dropped after
                // we are done initializing.
                let mut client = redis
                    .client()
                    .unwrap_or(redis.master()?)
                    .get_connection_with_timeout(Duration::from_millis(150))?;

                RedisClient::cmd("HDEL")
                    .arg("charted:sessions")
                    .arg(session_id.to_string())
                    .query(&mut client)
                    .context("unable to delete session {id} from Redis")
            }))),
        );
    }

    pub fn kill_session(&mut self, session_id: Uuid) -> Result<()> {
        let mut handles = self.handles.clone();
        if let Some((_, handle)) = handles.clone().iter().find(|(key, _)| key == &&session_id) {
            match handle.try_lock() {
                Ok(handle) => {
                    handle.abort();
                    let _ = handles.remove(&session_id);
                }

                Err(_) => {
                    error!(session.id = session_id.to_string(), "unable to abort session handle");
                }
            }
        }

        let mut client = self
            .redis
            .client()
            .unwrap_or(self.redis.master()?)
            .get_connection_with_timeout(Duration::from_millis(150))?;

        RedisClient::cmd("HDEL")
            .arg("charted:sessions")
            .arg(session_id.to_string())
            .query(&mut client)
            .context("unable to delete session {id} from Redis")?;

        RedisClient::cmd("DEL")
            .arg("charted:sessions:{session_id}")
            .query(&mut client)
            .context("unable to delete session {id} from Redis")
    }

    pub fn destroy(&mut self) -> Result<usize> {
        warn!("destroying all sessions");

        let mut failed = 0usize;
        for (session, handle) in self.handles.clone().into_iter() {
            match handle.try_lock() {
                Ok(handle) => {
                    handle.abort();

                    // drop it immedidately so we can continue
                    //
                    // TODO(@auguwu): wait until Mutex::unlock is stable enough,
                    // then switch to that
                    drop(handle);
                }

                Err(_) => {
                    error!(
                        session.id = session.to_string(),
                        "unable to abort task, things might go sour real quick..."
                    );

                    failed += 1;
                }
            }
        }

        if failed > 0 {
            warn!("received [{failed}] failed attempts");
        }

        Ok(failed)
    }

    pub async fn from_user(&mut self, id: u64) -> Result<Option<Session>> {
        let mut client = self
            .redis
            .client()
            .unwrap_or(self.redis.replica()?)
            .get_async_connection()
            .await?;

        let all: HashMap<String, String> = RedisClient::cmd("HGETALL")
            .arg("charted:sessions")
            .query_async(&mut client)
            .await?;

        for json in all.values() {
            let Ok(session) = serde_json::from_str::<Session>(json) else {
                continue;
            };

            if session.user_id == id {
                return Ok(Some(session));
            }
        }

        Ok(None)
    }

    /// Creates a new [`Session`] for a given user.
    #[tracing::instrument(name = "charted.sessions.create", skip_all)]
    pub async fn create_session(&mut self, user: User) -> Result<Session> {
        let session_id = Uuid::new_v4();
        let now = Local::now();
        let two_days = now + Duration::from_secs(172800);
        let one_week = now + Duration::from_secs(604800);
        let header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };

        // create the access token
        let config = Config::get();
        let jwt_secret_key = EncodingKey::from_secret(config.jwt_secret_key().unwrap().as_bytes());
        let access_token = encode::<HashMap<&str, Value>>(
            &header,
            &hashmap! {
                "session_id" => Value::String(session_id.to_string()),
                "user_id"    => Value::Number(user.id.into()),
                "iat"        => Value::Number(now.timestamp().into()),
                "nbf"        => Value::Number(now.timestamp().into()),
                "exp"        => Value::Number(two_days.timestamp().into()),
                "iss"        => Value::String("Noelware/charted-server".into())
            },
            &jwt_secret_key,
        )
        .map_err(|e| {
            error!(user.id, error = %e, "unable to create access token");
            sentry::capture_error(&e);

            Report::from(e)
        })?;

        let refresh_token = encode::<HashMap<&str, Value>>(
            &header,
            &hashmap! {
                "session_id" => Value::String(session_id.to_string()),
                "user_id"    => Value::Number(user.id.into()),
                "iat"        => Value::Number(now.timestamp().into()),
                "nbf"        => Value::Number(now.timestamp().into()),
                "exp"        => Value::Number(one_week.timestamp().into()),
                "iss"        => Value::String("Noelware/charted-server".into())
            },
            &jwt_secret_key,
        )
        .map_err(|e| {
            error!(user.id, error = %e, "unable to create access token");
            sentry::capture_error(&e);

            Report::from(e)
        })?;

        let session = Session {
            refresh_token: Some(refresh_token),
            access_token: Some(access_token),
            session_id,
            user_id: u64::try_from(user.id).unwrap(),
        };

        let as_json = serde_json::to_string(&session).unwrap();
        let mut client = self
            .redis
            .master()?
            .get_connection_with_timeout(Duration::from_millis(150))?;

        RedisClient::pipeline()
            .hset("charted:sessions", session_id.to_string(), as_json)
            .set(format!("charted:sessions:{session_id}"), "dummy payload for now")
            .expire_at(
                format!("charted:sessions:{session_id}"),
                one_week.timestamp_millis().try_into().unwrap(),
            )
            .query(&mut client)?;

        Ok(session)
    }
}

#[async_trait::async_trait]
impl SessionProvider for SessionManager {
    async fn authorize(&mut self, password: String, user: &dyn UserWithPassword) -> Result<()> {
        match self.provider.try_lock() {
            Ok(mut provider) => provider.authorize(password, user).await,
            Err(_) => Err(eyre!("unable to authenticate (mutex is poisioned)")),
        }
    }
}
