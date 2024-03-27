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

use crate::auth::{Auth, Context, Type};
use clap::Parser;
use eyre::{ContextCompat, Report};
use noelware_config::env;
use std::path::PathBuf;

/// Prints out the auth type and token itself, if it can retrieve it. It'll be in the
/// style of `Authorization: [Type] [Token]`
#[derive(Debug, Clone, Parser)]
pub struct Cmd {
    /// which context to print the token to, defaults to the default context
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
    let auth = Auth::load(cmd.auth.as_ref())?;
    let context = cmd.context.map(Context::new).unwrap_or(auth.current);
    let info = auth
        .contexts
        .get(&context)
        .wrap_err_with(|| format!("context `{context}` doesn't exist"))?;

    if let Type::None = info.auth {
        return Ok(());
    }

    match info.auth {
        Type::EnvironmentVariable { ref kind, ref env } => match env!(env) {
            Ok(val) => {
                eprintln!("Authorization: {kind} {val}");
                Ok(())
            }

            Err(std::env::VarError::NotPresent) => {
                warn!("cannot print environment variable [${env}] as it doesn't exist");
                Ok(())
            }

            Err(e) => Err(Report::from(e)),
        },

        Type::ApiKey(ref key) => {
            eprintln!("Authorization: ApiKey {key}");
            Ok(())
        }

        Type::Session { ref access, .. } => {
            eprintln!("Authorization: Bearer {access}");
            Ok(())
        }

        _ => unreachable!(),
    }
}
