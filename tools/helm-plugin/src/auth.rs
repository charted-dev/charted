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

use eyre::{Context as _, ContextCompat};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::{create_dir_all, File, OpenOptions},
    io::Write as _,
    ops::Deref,
    path::Path,
    sync::Arc,
};
use url::Url;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnvVarKind {
    #[default]
    Bearer,
    ApiKey,
}

impl Display for EnvVarKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvVarKind::ApiKey => f.write_str("ApiKey"),
            EnvVarKind::Bearer => f.write_str("Bearer"),
        }
    }
}

/// Represents the authentication type on how we should authenticate
/// to a charted-server instance.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    /// Refers to a session token (and refresh token) to do authentication.
    Session { refresh: Option<String>, access: String },

    /// Refer to a environment variable for the API key or bearer token.
    EnvironmentVariable { kind: EnvVarKind, env: String },

    /// Uses an created API key. By default, when `helm charted login` is ran, it'll
    /// create an API key on your user and use it indefinitely (or until you remove it
    /// via REST API or UI (if enabled)).
    ApiKey(String),

    /// Uses basic authentication, not recommended in production!
    Basic { username: String, password: String },

    /// Does no prior authentication.
    #[default]
    None,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "session" => Type::Session {
                refresh: None,
                access: String::default(),
            },

            "env" | "environment" | "envvar" => Type::EnvironmentVariable {
                env: String::default(),
                kind: EnvVarKind::default(),
            },

            "apikey" | "api-key" => Type::ApiKey(Default::default()),
            "basic" => Type::Basic {
                username: Default::default(),
                password: Default::default(),
            },

            "none" | "" => Type::default(),

            s => panic!("unknown type: {s}"),
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
/// personal: # <- `personal` is the context key
///   registry: https://charts.noelware.org/api
///   [...]
/// noelware: # <- also `noelware`
///   registry: https://corpo.noelware.cloud/api
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

impl<S: AsRef<str>> From<S> for Context {
    fn from(value: S) -> Self {
        Context::new(value)
    }
}

impl PartialEq<Context> for String {
    fn eq(&self, other: &Context) -> bool {
        self.as_str() == other.deref()
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
    pub contexts: BTreeMap<Context, Registry>,
}

impl Auth {
    pub fn sync<P: AsRef<Path>>(&self, to: Option<P>) -> eyre::Result<()> {
        let path = to.map(|x| x.as_ref().to_path_buf()).unwrap_or(
            dirs::config_local_dir()
                .expect("support for `dirs::config_local_dir`")
                .join("Noelware/charted-server/auth.yaml"),
        );

        trace!("syncing `auth.yaml` changes...");
        let mut file = OpenOptions::new()
            .append(false)
            .write(true)
            .read(true)
            .create(false)
            .open(path)?;

        write!(file, "{}", serde_yaml::to_string(self)?).context("unable to sync new changes to file")
    }

    pub fn load<P: AsRef<Path>>(path: Option<P>) -> eyre::Result<Self> {
        let path = path.map(|x| x.as_ref().to_path_buf()).unwrap_or(
            dirs::config_local_dir()
                .expect("support for `dirs::config_local_dir`")
                .join("Noelware/charted-server/auth.yaml"),
        );

        trace!(path = %path.display(), "loading `auth.yaml` file...");
        if !path.try_exists()? {
            warn!(path = %path.display(), "`auth.yaml` doesn't exist, creating...");
            let parent = path
                .parent()
                .context("expected parent to be available, is the grandparent root '/'?")?;

            create_dir_all(parent)?;

            let auth = Auth {
                current: Context::new("default"),
                contexts: {
                    let mut b = BTreeMap::new();
                    b.insert(
                        Context::new("default"),
                        Registry {
                            registry: Url::parse("https://charts.noelware.org/api").expect("invalid uri received"),
                            auth: Type::None,
                        },
                    );

                    b
                },
            };

            let mut file = File::create(&path)?;
            write!(file, "{}", serde_yaml::to_string(&auth)?)?;

            info!(path = %path.display(), "created `auth.yaml`! :3");
        }

        serde_yaml::from_reader(File::open(path)?).context("failed to read `auth.yaml` or deserialize it")
    }
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
