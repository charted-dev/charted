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

use crate::{
    auth::{Auth, Context},
    config::Config,
};
use charted_common::os;
use eyre::{Context as _, ContextCompat, Result};
use std::{collections::BTreeMap, path::PathBuf};
use tokio::{
    fs::{create_dir_all, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

/// Common config arguments to resolve a `.charted.yaml` file.
#[derive(Debug, Clone, clap::Args)]
pub struct CommonConfigArgs {
    /// Location to an `.charted.yaml` file for pushing repositories
    /// in charted-server.
    #[arg(long = "config", env = "CHARTED_HELM_CONFIG_PATH")]
    pub config_path: Option<PathBuf>,
}

impl CommonConfigArgs {
    pub async fn resolve_config(&self) -> Result<Option<Config>> {
        let config_file = self.config_path.clone().unwrap_or(PathBuf::from("./.charted.yaml"));
        if !config_file.exists() {
            return Ok(None);
        }

        let mut buf: Vec<u8> = vec![];
        OpenOptions::new()
            .create(false)
            .read(true)
            .write(true)
            .append(false)
            .open(config_file)
            .await?
            .read_to_end(&mut buf)
            .await?;

        serde_yaml::from_slice(buf.as_slice())
            .context("unable to read authentication file")
            .map(Some)
    }
}

/// Common arguments in most `helm charted` commands.
#[derive(Debug, Clone, clap::Args)]
pub struct CommonHelmArgs {
    /// Location to a `auth.yaml` file that can be used to look up
    /// any additional contexts.
    #[arg(long, env = "CHARTED_HELM_CONTEXT_FILE")]
    pub context_file: Option<PathBuf>,

    /// The current context to use when authenticating to registries. By default,
    /// this will look in the following directories below:
    ///
    /// * Windows: `C:\Users\{username}\AppData\Local\Noelware\charted-server\auth.yaml`
    /// * macOS:   `/Users/{username}/Library/Application Support/Noelware/charted-server/auth.yaml`
    /// * Linux:   `/users/{username}/.config/Noelware/charted-server/auth.yaml`
    #[arg(long, short = 'c', env = "CHARTED_HELM_CONTEXT")]
    pub context: Option<String>,

    /// Location to a `helm` binary. If this is not specified, then
    /// it will be looked in $PATH by default.
    #[arg(long, env = "CHARTED_HELM_PATH")]
    pub helm: Option<PathBuf>,
}

impl CommonHelmArgs {
    pub fn locate(file: Option<PathBuf>) -> Result<PathBuf> {
        match file {
            Some(file) => Ok(file),
            None => match os::os_name() {
                "linux" | "macos" => Ok(dirs::config_dir()
                    .context("unable to find config dir")?
                    .join("Noelware/charted-server/auth.yaml")),

                "windows" => Ok(dirs::cache_dir()
                    .context("unable to find cache dir")?
                    .join("Noelware/charted-server/auth.yaml")),

                _ => unreachable!(),
            },
        }
    }

    pub async fn current_context(&self) -> Result<Context> {
        let auth = CommonHelmArgs::auth(self.context_file.clone()).await?;
        Ok(auth.current)
    }

    pub async fn auth(context_file: Option<PathBuf>) -> Result<Auth> {
        let context_file = CommonHelmArgs::locate(context_file.clone())?;
        debug!("locating authentication file in {}", context_file.display());

        if !context_file.exists() {
            warn!(
                "authentication file [{}] didn't exist, creating...",
                context_file.display()
            );

            // only attempt to create from the parent dir
            if let Some(parent) = context_file.parent() {
                create_dir_all(parent).await?;
            }

            // If it doesn't exist, then we will create an empty one
            let auth = Auth {
                context: {
                    let mut context = BTreeMap::new();
                    context.insert(Context::from("default"), vec![]);

                    context
                },
                current: Context::from("default"),
            };

            let serialized = serde_yaml::to_string(&auth)?;
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(context_file)
                .await?;

            file.write_all(serialized.as_ref()).await?;
            return Ok(auth);
        }

        let mut buf: Vec<u8> = vec![];
        OpenOptions::new()
            .create(false)
            .read(true)
            .write(true)
            .append(false)
            .open(context_file)
            .await?
            .read_to_end(&mut buf)
            .await?;

        serde_yaml::from_slice(buf.as_slice()).context("unable to read authentication file")
    }
}
