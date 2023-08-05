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

use super::DatabaseController;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_trait::async_trait;
use charted_common::{
    models::{entities::User, Name},
    Snowflake,
};
use eyre::{eyre, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, PgPool, Postgres, QueryBuilder};
use utoipa::ToSchema;
use validator::validate_email;

static ARGON2: Lazy<Argon2> = Lazy::new(Argon2::default);

#[allow(clippy::invalid_regex)]
static PASSWORD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\d)?(?=.*[!#\$%&? \"])?.*$"#).unwrap());

/// Represents the payload for creating a new user.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUserPayload {
    /// User handle to use to identify yourself.
    #[schema(value_type = Name)]
    pub username: String,

    /// The password to use when authenticating, this is optional on non-local sessions.
    #[schema(
        value_type = password,
        pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$"
    )]
    pub password: Option<String>,

    /// Email address to identify this user
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PatchUserPayload {
    /// Optional field to update this user's gravatar email. If this user doesn't
    /// have an avatar that is used or prefers not to use their previously uploaded
    /// avatars and they set their Gravatar email, their Gravatar will be used.
    pub gravatar_email: Option<String>,

    /// Short description about this user. If this field was provided, then the
    /// description will be overwritten. If this field is `null`, then nothing
    /// will happen. If this field is a empty string, then the description
    /// will be wiped.
    pub description: Option<String>,

    /// Updates this user's username.
    #[schema(value_type = Name, nullable)]
    pub username: Option<String>,

    /// Updates this user's password, if the session manager configured allows it.
    pub password: Option<String>,

    /// Updates this user's email.
    pub email: Option<String>,

    /// Updates this user's display name.
    pub name: Option<String>,
}

/// Represents a [`DatabaseController`] for interacting with users.
#[derive(Debug, Clone)]
pub struct UserDatabaseController {
    pub(crate) pool: PgPool,
    snowflake: Snowflake,
}

impl UserDatabaseController {
    pub fn new(pool: PgPool, snowflake: Snowflake) -> UserDatabaseController {
        Self { pool, snowflake }
    }
}

#[async_trait]
impl DatabaseController for UserDatabaseController {
    type Patched = PatchUserPayload;
    type Created = CreateUserPayload;
    type Entity = User;

    async fn get(&self, id: u64) -> Result<Option<User>> {
        Ok(
            QueryBuilder::<'static, Postgres>::new("SELECT * FROM users WHERE id = ")
                .push_bind(id as i64)
                .build_query_as::<User>()
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    async fn create(&self, payload: Self::Created) -> Result<PgQueryResult> {
        if payload.username.is_empty() {
            return Err(eyre!("no username was provided"));
        }

        if payload.username.len() > 32 {
            return Err(eyre!(
                "username exceeded {} characters, max chars is 32",
                payload.username.len() - 32
            ));
        }

        if let Err(why) = Name::is_valid(payload.username.as_str()) {
            return Err(eyre!("unable to validate username: {why}"));
        }

        if payload.email.is_empty() {
            return Err(eyre!("no email was provided"));
        }

        if !validate_email(payload.email.as_str()) {
            return Err(eyre!("email '{}' was not valid", payload.email));
        }

        if (query_as::<Postgres, User>("SELECT * FROM users WHERE username = $1;")
            .bind(payload.username.clone())
            .fetch_optional(&self.pool)
            .await?)
            .is_some()
        {
            return Err(eyre!("username '{}' is already taken", payload.username));
        }

        if (query_as::<Postgres, User>("SELECT * FROM organizations WHERE name = $1;")
            .bind(payload.username.clone())
            .fetch_optional(&self.pool)
            .await?)
            .is_some()
        {
            return Err(eyre!("username '{}' is already taken", payload.username));
        }

        if (query_as::<Postgres, User>("SELECT * FROM users WHERE email = $1;")
            .bind(payload.email.clone())
            .fetch_optional(&self.pool)
            .await?)
            .is_some()
        {
            return Err(eyre!("email '{}' is already taken", payload.email));
        }

        let mut snowflake = self.snowflake.clone();
        let id = snowflake.generate();

        Ok(
            query("INSERT INTO users(id, username, email, password) VALUES($1, $2, $3, $4);")
                .bind(id.value() as i64)
                .bind(payload.username)
                .bind(payload.email)
                .bind(match payload.password {
                    Some(res) => {
                        if !PASSWORD_REGEX.is_match(res.as_str()) {
                            return Err(eyre!(
                                "passwords can only contain letters, digits, and special characters"
                            ));
                        }

                        let salt = SaltString::generate(&mut OsRng);
                        let hashed = ARGON2
                            .hash_password(res.as_ref(), &salt)
                            .map_err(|e| eyre!("unable to hash password: {e}"))?;

                        Some(hashed.to_string())
                    }
                    None => None,
                })
                .execute(&self.pool)
                .await?,
        )
    }
}
