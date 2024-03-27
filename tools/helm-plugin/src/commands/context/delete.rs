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

use crate::auth::{Auth, Context};
use clap::Parser;
use std::{path::PathBuf, process::exit};

/// Deletes a context from the auth configuration
#[derive(Debug, Clone, Parser)]
pub struct Cmd {
    /// which context to delete
    context: Option<String>,

    /// Location to an `auth.yaml` file that represents the authentication file
    /// to authenticate between charted instances
    ///
    /// ## Default Locations
    /// | OS               | Location                                                                                                  |
    /// | :--------------- | :-------------------------------------------------------------------------------------------------------- |
    /// | Windows          | `C:\Users\<username>\AppData\Local\Noelware\charted-server\auth.yaml`                                     |
    /// | macOS            | `/Users/<username>/Library/Application Support/Noelware/charted-server/auth.yaml`                         |
    /// | Linux            | `$XDG_CONFIG_DIR/Noelware/charted-server/auth.yaml` or `$HOME/.config/Noelware/charted-server/auth.yaml` |
    #[arg(long, short = 'a', env = "CHARTED_AUTH_YAML_LOCATION")]
    auth: Option<PathBuf>,
}

pub async fn run(cmd: Cmd) -> eyre::Result<()> {
    let mut auth = Auth::load(cmd.auth.as_ref())?;
    let context = cmd.context.as_ref().map(Context::new).unwrap_or(auth.current.clone());

    if !auth.contexts.contains_key(&context) {
        warn!("context `{context}` doesn't exist, cannot delete");
        exit(1);
    }

    if auth.current == context {
        warn!("context `{context}` is the default, please switch to a different context to delete this one");
        exit(1);
    }

    let _ = auth.contexts.remove(&context);
    auth.sync(cmd.auth)?;

    info!("deleted context `{context}` from auth.yaml");
    Ok(())
}
