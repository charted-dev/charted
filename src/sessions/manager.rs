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

use crate::{common::models::entities::User, hashmap, sessions::Session, Instance};
use chrono::Local;
use eyre::{Context, Report};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::task::JoinHandle;
use uuid::Uuid;

#[derive(Clone)]
pub struct Manager {
    initialized: bool,
    handles: BTreeMap<Uuid, Arc<Mutex<JoinHandle<eyre::Result<()>>>>>,
    redis: crate::redis::Client,
}

impl Manager {
    pub fn new(redis: crate::redis::Client) -> Manager {
        Manager {
            initialized: false,
            handles: BTreeMap::new(),
            redis,
        }
    }

    #[instrument(name = "charted.sessions.init", skip_all)]
    pub fn init(&mut self) -> eyre::Result<()> {
        if self.initialized {
            warn!("session manager has been initialized more than once");
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

        let sessions: HashMap<String, String> = redis::cmd("HGETALL").arg("charted:sessions").query(&mut client)?;
        info!(sessions = sessions.len(), "processing sessions from Redis");

        let mut handles = BTreeMap::new();
        for (id, payload) in sessions.into_iter() {
            let uuid = Uuid::parse_str(&id)?;
            let session = match serde_json::from_str::<Session>(&payload) {
                Ok(session) => session,
                Err(e) => {
                    error!(session.id = %uuid, "unable to process session with uuid, deleting");
                    sentry::capture_error(&e);

                    redis::cmd("HDEL")
                        .arg("charted:sessions")
                        .arg(id)
                        .query(&mut master.as_mut().unwrap_or(&mut client))?;

                    continue;
                }
            };

            let span = info_span!("charted.sessions.process", session.user, session.id = %session.session);
            let _guard = span.enter();

            trace!("<- TTL charted:sessions:{id}");
            let ttl: i32 = redis::cmd("TTL")
                .arg(format!("charted:sessions:{id}"))
                .query(&mut client)?;

            debug!(session.id = %session.session, "-> TTL charted:sessions:{id} ~> session {}", match ttl {
                -2 => Cow::Borrowed("is invalid, deleting session"),
                -1 => Cow::Borrowed("has expired"),
                _ => Cow::Owned(format!("has {ttl} seconds left"))
            });

            match ttl {
                -2 | -1 => {
                    redis::cmd("HDEL")
                        .arg("charted:sessions")
                        .arg(&id)
                        .query(&mut master.as_mut().unwrap_or(&mut client))?;

                    let _ = redis::cmd("DEL")
                        .arg(format!("charted:sessions:{id}"))
                        .query::<()>(&mut master.as_mut().unwrap_or(&mut client));

                    continue;
                }

                _ => {
                    let mut redis = self.redis.clone();
                    let handle: JoinHandle<eyre::Result<()>> = tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_secs(ttl.try_into()?)).await;

                        // Open a new connection to Redis once it is complete
                        let mut client = redis
                            .client()
                            .unwrap_or(redis.master()?)
                            .get_connection_with_timeout(Duration::from_millis(150))?;

                        redis::cmd("HDEL")
                            .arg("charted:sessions")
                            .arg(&uuid.to_string())
                            .query(&mut client)
                            .context(format!("unable to delete session [{id}] from Redis"))
                    });

                    #[cfg(tokio_unstable)]
                    debug!(session.id = %session.session, session.user, id = %handle.id(), "created Tokio task for session");

                    #[cfg(not(tokio_unstable))]
                    debug!(session.id = %session.session, session.user, "created Tokio task for session");

                    handles.insert(uuid, Arc::new(Mutex::new(handle)));
                }
            }
        }

        self.initialized = true;
        self.handles = handles;

        Ok(())
    }

    #[instrument(name = "charted.sessions.tasks.create", skip_all)]
    fn create_task(&mut self, id: Uuid, duration: Duration) {
        debug!(session.id = %id, ?duration, "spawning Tokio task for session");

        let mut redis = self.redis.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(duration).await;

            // Open a new connection to Redis once it is complete
            let mut client = redis
                .client()
                .unwrap_or(redis.master()?)
                .get_connection_with_timeout(Duration::from_millis(150))?;

            redis::cmd("HDEL")
                .arg("charted:sessions")
                .arg(&id.to_string())
                .query(&mut client)
                .context(format!("unable to delete session [{id}] from Redis"))
        });

        #[cfg(tokio_unstable)]
        debug!(session.id = %id, id = %handle.id(), "created Tokio task for session");

        #[cfg(not(tokio_unstable))]
        debug!(session.id = %id, "created Tokio task for session");

        self.handles.insert(id, Arc::new(Mutex::new(handle)));
    }

    #[instrument(name = "charted.sessions.kill", skip(self))]
    pub fn kill(&mut self, id: Uuid) -> eyre::Result<()> {
        if let Some((_, handle)) = self.handles.clone().into_iter().find(|(key, _)| key == &id) {
            match handle.try_lock() {
                Ok(handle) => {
                    handle.abort();
                    let _ = self.handles.remove(&id);
                }

                Err(e) => {
                    error!(session.id = %id, error = %e, "was unable to abort JoinHandle for session, did it panic?");
                }
            }
        } else {
            warn!(session.id = %id, "session was already expired");
            return Ok(());
        }

        let mut client = self
            .redis
            .client()
            .unwrap_or(self.redis.master()?)
            .get_connection_with_timeout(Duration::from_millis(150))?;

        redis::pipe()
            .hdel("charted:sessions", &id.to_string())
            .del(format!("charted:sessions:{id}"))
            .query(&mut client)
            .context("unable to delete session from Redis")
    }

    pub async fn from_user(&mut self, id: u64, session_id: Uuid) -> eyre::Result<Option<Session>> {
        let mut client = self
            .redis
            .client()
            .unwrap_or(self.redis.replica()?)
            .get_async_connection()
            .await?;

        let all: HashMap<String, String> = redis::cmd("HGETALL")
            .arg("charted:sessions")
            .query_async(&mut client)
            .await?;

        for json in all.values() {
            let Ok(session) = serde_json::from_str::<Session>(json) else {
                continue;
            };

            if session.user == id && session.session == session_id {
                return Ok(Some(session));
            }
        }

        Ok(None)
    }

    #[instrument(name = "charted.sessions.create", skip_all, fields(user.id))]
    pub async fn create(&mut self, user: User) -> eyre::Result<Session> {
        let session = Uuid::new_v4();
        let now = Local::now();
        let two_days = now + Duration::from_secs(172800);
        let one_week = now + Duration::from_secs(604800);
        let header = Header::new(Algorithm::HS512);

        let instance = Instance::get();
        let access_token = jsonwebtoken::encode(
            &header,
            &hashmap!(&str, Value, {
                "session_id" => Value::String(session.to_string()),
                "user_id"    => Value::Number(user.id.into()),
                "iat"        => Value::Number(now.timestamp().into()),
                "nbf"        => Value::Number(now.timestamp().into()),
                "exp"        => Value::Number(two_days.timestamp().into()),
                "iss"        => Value::String(String::from("Noelware/charted-server"))
            }),
            &EncodingKey::from_secret(instance.config.jwt_secret_key.as_ref()),
        )
        .map_err(|e| {
            error!(user.id, session.id = %session, "unable to create access token");
            sentry::capture_error(&e);

            Report::from(e)
        })?;

        let refresh_token = jsonwebtoken::encode(
            &header,
            &hashmap!(&str, Value, {
                "session_id" => Value::String(session.to_string()),
                "user_id"    => Value::Number(user.id.into()),
                "iat"        => Value::Number(now.timestamp().into()),
                "nbf"        => Value::Number(now.timestamp().into()),
                "exp"        => Value::Number(one_week.timestamp().into()),
                "iss"        => Value::String(String::from("Noelware/charted-server"))
            }),
            &EncodingKey::from_secret(instance.config.jwt_secret_key.as_ref()),
        )
        .map_err(|e| {
            error!(user.id, session.id = %session, "unable to create refresh token");
            sentry::capture_error(&e);

            Report::from(e)
        })?;

        let sess = Session {
            refresh_token: Some(refresh_token),
            access_token: Some(access_token),
            session,
            user: user.id.try_into()?,
        };

        let payload = serde_json::to_string(&sess)?;
        let mut master = self
            .redis
            .master()?
            .get_connection_with_timeout(Duration::from_millis(150))?;

        redis::pipe()
            .hset("chareted:sessions", session.to_string(), payload)
            .set(format!("charted:sessions:{session}"), "<nothing for you to see>")
            .expire_at(format!("charted:sessions:{session}"), one_week.timestamp_millis())
            .query(&mut master)
            .map(|_: ()| sess)
            .context("unable to create session in Redis")
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        let mut failed = 0usize;

        warn!("destroying all sessions");
        for (session, handle) in self.handles.iter() {
            match handle.try_lock() {
                Ok(handle) => handle.abort(),
                Err(e) => {
                    error!(error = %e, session.id = %session, "unable to abort JoinHandle for session, did it panic?");
                    failed += 1;
                }
            }
        }

        info!(%failed, "destroyed all sessions");
    }
}

#[cfg(test)]
fn __assert_send<T: Send>() {}

#[cfg(test)]
fn __assert_sync<T: Sync>() {}

#[cfg(test)]
fn __assertions() {
    __assert_send::<Manager>();
    __assert_sync::<Manager>();
}
