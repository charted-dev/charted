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

use eyre::{bail, eyre};
use std::{collections::BTreeMap, env::VarError, fmt::Display, str::FromStr};

#[inline(always)]
pub const fn truthy() -> bool {
    true
}

/// Given a <code>[`Result`]<T, [`VarError`]></code> and default value:
///
/// - In variant <code>[`Ok`]\({value}\)</code>, return the `{value}`.
/// - In variant <code>[`Err`]\([`VarError::Present`]\)</code>, return `default`.
/// - Otherwise, bail out.
pub fn env_from_result<T>(result: Result<T, VarError>, default: T) -> eyre::Result<T> {
    match result {
        Ok(value) => Ok(value),
        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => bail!("received non-unicode value in environment variable"),
    }
}

pub fn env_from_result_lazy<T>(
    result: Result<T, VarError>,
    default: impl FnOnce() -> eyre::Result<T>,
) -> eyre::Result<T> {
    match result {
        Ok(value) => Ok(value),
        Err(VarError::NotPresent) => Ok(default()?),
        Err(VarError::NotUnicode(_)) => bail!("received non-unicode value in environment variable"),
    }
}

pub fn env_from_str<F: FromStr>(key: &str, default: F) -> eyre::Result<F>
where
    F::Err: Display,
{
    match azalia::config::env!(key) {
        Ok(value) => value
            .parse::<F>()
            .map_err(|e| eyre!("failed to parse environment variable `${}`: {}", key, e)),

        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => bail!("received non-unicode in environment variable `${}`", key),
    }
}

pub fn bool_env(key: &str) -> eyre::Result<bool> {
    env_from_result(
        azalia::config::env!(key).map(|x| azalia::TRUTHY_REGEX.is_match(&x)),
        false,
    )
}

pub fn btreemap_env<K: FromStr + Ord, V: FromStr>(key: &str) -> eyre::Result<BTreeMap<K, V>>
where
    K::Err: Display,
    V::Err: Display,
{
    let mut map = azalia::btreemap!(K, V);
    let result = match azalia::config::env!(key) {
        Ok(res) => res,
        Err(VarError::NotPresent) => return Ok(map),
        Err(VarError::NotUnicode(_)) => bail!("received non-unicode in environment variable `${}`", key),
    };

    for (i, line) in result.split(',').enumerate() {
        if let Some((key, val)) = line.split_once('=') {
            if val.contains('=') {
                continue;
            }

            map.insert(
                match K::from_str(val) {
                    Ok(v) => v,
                    Err(e) => bail!(
                        "failed to parse environment variable `${}`: at index #{}, failed to parse key: {}",
                        key,
                        i,
                        e
                    ),
                },
                match V::from_str(val) {
                    Ok(v) => v,
                    Err(e) => bail!(
                        "failed to parse environment variable `${}`: at index #{}, failed to parse value: {}",
                        key,
                        i,
                        e
                    ),
                },
            );
        }
    }

    Ok(map)
}

pub fn env_optional_from_str<F: FromStr>(key: &str, default: Option<F>) -> eyre::Result<Option<F>>
where
    F::Err: Display,
{
    match azalia::config::env!(key) {
        Ok(value) => value
            .parse::<F>()
            .map(Some)
            .map_err(|e| eyre!("failed to parse environment variable `${}`: {}", key, e)),

        Err(VarError::NotPresent) => Ok(default),
        Err(VarError::NotUnicode(_)) => Err(eyre!("received non-unicode in `${}` environment variable", key)),
    }
}
