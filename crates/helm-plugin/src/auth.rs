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

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use eyre::{Context as _, ContextCompat};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Read,
    path::Path,
};
use tracing::{info, trace, warn};

/// Represents what "kind" of authentication for the `environmentVariable` type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum EnvironmentVariableKind {
    /// Uses the API key authentication type.
    #[default]
    ApiKey,

    /// Uses the session authentication type.
    Bearer,
}

/// Determines what type of authentication to perform based off a registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    /// Lookup the authentication key via the system environment variable.
    EnvironmentVariable {
        /// What kind of authentication is the environment variable for?
        kind: EnvironmentVariableKind,

        /// The environment variable name
        name: Cow<'static, str>,
    },

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

/// Represents the schematic of the `auth.toml` configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    /// Represents the current context that we are in. To switch to a different one,
    /// you can use the `Auth::switch` method.
    pub current: Cow<'static, str>,

    /// All the avaliable contexts. This can be manually edited or can be manipulated
    /// with the `helm charted auth` subcommand.
    #[serde(rename = "context")]
    pub contexts: HashMap<String, Context>,
}

impl Default for Auth {
    fn default() -> Self {
        let official = Context {
            kind: Type::None,
            registry: "https://charts.noelware.org/api".into(),
        };

        Auth {
            current: Cow::Borrowed("default"),
            contexts: azalia::hashmap!("default" => official),
        }
    }
}

impl Auth {
    /// Loads the `auth.toml` configuration file with a *optional* file path.
    pub fn load<P: AsRef<Path>>(path: Option<P>) -> eyre::Result<Auth> {
        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "org".to_string(),
            author: "Noelware".to_string(),
            app_name: "charted helm plugin".to_string(),
        })?;

        let path = path
            .map(|x| x.as_ref().to_path_buf())
            .unwrap_or(strategy.config_dir().join("Noelware/charted-server/auth.toml"));

        trace!(path = %path.display(), "loading `auth.toml`...");
        if !path.try_exists()? {
            warn!(path = %path.display(), "`auth.toml` doesn't exist! creating...");
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let auth = Auth::default();

            File::create_new(&path)?;
            auth.sync(Some(path))?;

            return Ok(auth);
        }

        let mut contents = String::new();

        {
            let mut file = File::open(&path)?;
            file.read_to_string(&mut contents)?;
        }

        toml::from_str(&contents).context("failed to parse `auth.toml`")
    }

    pub fn sync(&self, path: Option<impl AsRef<Path>>) -> eyre::Result<()> {
        use std::io::Write;

        info!("syncing `auth.toml` changes...");

        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "org".to_string(),
            author: "Noelware".to_string(),
            app_name: "charted helm plugin".to_string(),
        })?;

        let path = path
            .map(|x| x.as_ref().to_path_buf())
            .unwrap_or(strategy.config_dir().join("Noelware/charted-server/auth.toml"));

        let mut file = OpenOptions::new()
            .create(false)
            .truncate(true)
            .write(true)
            .read(true)
            .open(path)?;

        write!(
            file,
            "{}",
            toml::to_string_pretty(self).context("failed to serialize into toml")?
        )
        .context("failed to sync new changes into file")
    }

    pub fn switch(&mut self, path: Option<impl AsRef<Path>>, context: impl Into<String>) -> eyre::Result<()> {
        let context = context.into();
        if self.current == context {
            warn!("not switching to context `{context}` due to being the default already");
            return Ok(());
        }

        if !self.contexts.contains_key(&context) {
            warn!("not switching to context `{context}` due to it not existing");
            return Ok(());
        }

        let old = self.current.clone();
        self.current = Cow::Owned(context.clone());

        info!("switched from {old} ~> {context}, now syncing changes...");
        self.sync(path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub kind: Type,
    pub registry: String,
}
