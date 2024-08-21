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

use eyre::eyre;
use std::{env::VarError, str::FromStr};

pub fn env_from_result<T>(res: Result<T, VarError>, default: T) -> eyre::Result<T> {
    match res {
        Ok(value) => Ok(value),
        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!("received non-unicode in environment variable")),
    }
}

pub fn env_from_str<F: FromStr>(key: &str, default: F) -> eyre::Result<F> {
    match azalia::config::env!(key) {
        Ok(value) => value
            .parse::<F>()
            .map_err(|_| eyre!("failed to parse environment variable `${key}`")),

        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!("received non-unicode in `${}` environment variable", key)),
    }
}

/*
pub fn env<U: FromStr, S: Into<String>, F>(key: S, default: U, onfail: F) -> eyre::Result<U>
where
    F: FnOnce(<U as FromStr>::Err) -> Cow<'static, str>,
{
    let env = key.into();
    match noelware_config::env!(&env) {
        Ok(val) => match val.parse::<U>() {
            Ok(val) => Ok(val),
            Err(e) => Err(eyre!("failed to parse for env `{}`: {}", env, onfail(e))),
        },

        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!("received invalid unicode data for `{}` env", env)),
    }
}

pub fn env_string<S: Into<String>>(key: S, default: String) -> eyre::Result<String> {
    let env = key.into();
    match noelware_config::env!(&env) {
        Ok(val) => Ok(val),
        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!("received invalid unicode for `{env}` env")),
    }
}


            max_connections: match env!("CHARTED_DATABASE_MAX_CONNECTIONS") {
                Ok(value) => value.parse::<u32>()?,
                Err(VarError::NotPresent) => __max_connections(),
                Err(VarError::NotUnicode(_)) => {
                    return Err(eyre!(
                        "received non-unicode in `$CHARTED_DATABASE_MAX_CONNECTIONS` environment variable"
                    ))
                }
            },
*/
