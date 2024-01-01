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

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, ops::Deref, sync::Arc};
use url::Url;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AuthType {
    EnvironmentVariable(String),
    ApiKey(String),
    Basic(String),

    SessionToken {
        access: String,
        refresh: Option<String>,
    },

    #[default]
    None,
}

impl AuthType {
    /// This only validates the [`AuthType::FromEnvironmentVariable`] authentication types as
    /// others might require asynchronous work to be used with.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            AuthType::EnvironmentVariable(var) => {
                if std::env::var(var.clone()).is_err() {
                    return Err(format!("environment variable {var} doesn't exist"));
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }

    /// Returns the header name for this authentication type.
    pub fn header(&self) -> Option<&'static str> {
        match self {
            AuthType::EnvironmentVariable(_) | AuthType::ApiKey(_) => Some("ApiKey"),
            AuthType::SessionToken { .. } => Some("Bearer"),
            AuthType::Basic(_) => Some("Basic"),
            _ => None,
        }
    }
}

impl ToString for AuthType {
    fn to_string(&self) -> String {
        let Some(header) = self.header() else {
            return String::new();
        };

        match self {
            AuthType::EnvironmentVariable(var) => {
                let value = std::env::var(var).expect("to be validated with AuthType::validate");
                format!("{header} {value}")
            }

            AuthType::SessionToken { access, .. } => format!("{header} {access}"),
            AuthType::ApiKey(key) => format!("{header} {key}"),
            AuthType::Basic(b64) => format!("{header} {b64}"),
            _ => unreachable!(),
        }
    }
}

/// Represents the context that the `.auth.yaml` file contains. A context is
/// relative to what registries a user has access towards.
///
/// For instance, if the current context is `personal`, then the `personal` context
/// can only control which registries it can push to. If context is `noelware`, then
/// it can only control which registries from the `noelware` context it has.
///
/// The available context is contained in a `context` object in the file:
/// ```yaml
/// personal:
/// - registry: https://charts.noelware.org/api
///   [...]
/// noelware:
/// - registry: https://corpo.noelware.cloud/api
///   [...]
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Context(Arc<str>);

impl Context {
    /// Creates a new [`Context`] key.
    pub fn new<I: AsRef<str>>(data: I) -> Context {
        Context(Arc::from(data.as_ref()))
    }

    /// Returns an immutable string slice of the context name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for Context {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Context {
    fn from(value: &str) -> Self {
        Context(Arc::from(value))
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub registry: Url,

    #[serde(default)]
    pub auth: AuthType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    #[serde(default = "default_context")]
    pub current: Context,

    #[serde(default)]
    pub context: BTreeMap<Context, Vec<RegistryConfig>>,
}

fn default_context() -> Context {
    "default".into()
}
