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
use std::{borrow::Cow, collections::BTreeMap, fmt::Display, ops::Deref, sync::Arc};
use url::Url;

/// Represents the authentication type on how we should authenticate
/// to a charted-server instance.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    /// Refers to a session token (and refresh token) to do authentication.
    Session { refresh: Option<String>, access: String },

    /// Refer to a environment variable for the API key or bearer token.
    EnvironmentVariable(String),

    /// Uses an created API key. By default, when `helm charted login` is ran, it'll
    /// create an API key on your user and use it indefinitely (or until you remove it
    /// via REST API or UI (if enabled)).
    ApiKey(String),

    /// Does no prior authentication.
    #[default]
    None,
}

impl Type {
    /// Does extra validation that the given auth type is correct.
    pub fn validate(&self) -> Result<(), Cow<'static, str>> {
        match self {
            Type::EnvironmentVariable(var) if std::env::var(var).is_err() => {
                Err(Cow::Owned(format!("environment variable [{var}] doesn't exist!")))
            }

            _ => Ok(()),
        }
    }
}

/// Represents the context that the `auth.yaml` file contains. A context is
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
pub struct Auth {
    #[serde(default = "__default_context")]
    pub current: Context,

    #[serde(default)]
    pub contexts: BTreeMap<Context, Vec<Registry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    #[serde(default, with = "serde_yaml::with::singleton_map")]
    pub auth: Type,
    pub registry: Url,
}

fn __default_context() -> Context {
    Context::new("default")
}
