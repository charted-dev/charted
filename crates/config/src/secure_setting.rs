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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ENV_VARIABLE_REGEX: Regex = Regex::new(r"[$]\{([\w.]+)(:-\w+)?}").unwrap();
}

#[derive(Debug, thiserror::Error)]
pub enum SecureSettingError {
    #[error("Environment variable '{0}' doesn't exist.")]
    MissingVariable(String),

    #[error("Unable to get capture groups of configuration key [{key}] from input: {input}")]
    UnableToGetCaptureGroups { key: String, input: String },
}

/// Represents a secure setting that can offload secret-related
/// settings with something that can be fetched from the system
/// environment variables from the YAML DSL with the `${}` syntax.
#[derive(Debug, Clone)]
pub struct SecureSetting(String);

impl SecureSetting {
    /// Creates a new [`SecureSetting`] with config key name to use
    /// for diagnostics
    pub fn new(name: String) -> SecureSetting {
        SecureSetting(name)
    }

    /// Loads in the environment variable and returns the value, or returns nothing
    /// if an error occurred.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted_types::SecureSettting;
    /// #
    /// let sentry_dsn = SecureSetting::new("sentry_dsn");
    /// let dsn = sentry_dsn.load("${CHARTED_SENTRY_DSN:-deeznuts}");
    /// ```
    pub fn load<I: AsRef<str>>(&self, input: I) -> Result<String, SecureSettingError> {
        self.load_optional(input).map(|x| match x {
            Some(val) => val,
            None => "".into(),
        })
    }

    /// Loads in the environment variable and returns a Result of an Option of the contained
    /// environment variable's value.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted_config::SecureSetting;
    /// #
    /// let sentry_dsn = SecureSetting::new("sentry_dsn");
    /// let res = sentry_dsn.load_optional("${CHARTED_SENTRY_DSN:-deeznuts}");
    ///
    /// assert!(res.is_ok());
    /// let value = res.unwrap();
    ///
    /// assert!(res.is_some());
    /// let dsn = value.unwrap();
    /// ```
    pub fn load_optional<I: AsRef<str>>(&self, input: I) -> Result<Option<String>, SecureSettingError> {
        if !ENV_VARIABLE_REGEX.is_match(input.as_ref()) {
            return Ok(Some(input.as_ref().to_string()));
        }

        let groups = match ENV_VARIABLE_REGEX.captures(input.as_ref()) {
            Some(captures) => captures,
            None => {
                return Err(SecureSettingError::UnableToGetCaptureGroups {
                    key: self.0.clone(),
                    input: input.as_ref().to_string(),
                })
            }
        };

        let env_name = match groups.get(0) {
            Some(r#match) => r#match,
            None => return Err(SecureSettingError::MissingVariable(input.as_ref().to_string())),
        };

        match std::env::var(env_name.as_str()) {
            Ok(value) => Ok(Some(value)),
            Err(_) => match groups.get(2) {
                Some(value) => Ok(Some(value.as_str().to_owned())),
                None => Err(SecureSettingError::MissingVariable(env_name.as_str().to_string())),
            },
        }
    }

    /// Loads in the environment variable, and if it exists, transform it with the `F` type.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted_config::SecureSetting;
    /// # use sentry::types::Dsn;
    /// #
    /// let sentry_dsn = SecureSetting::new("sentry_dsn");
    /// sentry_dsn.load_with("${CHARTED_SENTRY_DSN:-deeznuts}", |x| Dsn::from_str(x.as_str()));
    /// // => Result<Option<Result<Dsn, _>>, SecureSettingError>
    /// ```
    pub fn load_with<U, I: AsRef<str>, F>(&self, input: I, with: F) -> Result<Option<U>, SecureSettingError>
    where
        F: Fn(String) -> U,
    {
        match self.load_optional(input) {
            Ok(Some(res)) => Ok(Some(with(res))),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
