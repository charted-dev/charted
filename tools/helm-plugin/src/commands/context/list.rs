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

use crate::auth::{Auth, Type};
use clap::Parser;
use cli_table::{Cell, Table};
use std::{borrow::Cow, path::PathBuf};

#[derive(Table)]
struct Row {
    current: &'static str,
    context: String,
    registry: String,
    ty: Cow<'static, str>,
}

/// Lists all available authentication contexts in the given authentication file.
#[derive(Debug, Clone, Parser)]
pub struct Cmd {
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

    /// whether or not to output the context as JSON. This doesn't include tokens.
    #[arg(long)]
    json: bool,
}

pub async fn run(cmd: Cmd) -> eyre::Result<()> {
    let auth = Auth::load(cmd.auth.as_ref())?;
    let mut rows = Vec::<Row>::with_capacity(auth.contexts.len());
    for (context, registry) in auth.contexts {
        rows.push(Row {
            current: match auth.current.as_str() == context.as_str() {
                true => "*",
                false => "",
            },
            context: context.to_string(),
            registry: registry.registry.to_string(),
            ty: match registry.auth {
                Type::EnvironmentVariable { env, .. } => Cow::Owned(format!("environment variable ${env}")),
                Type::Session { .. } => Cow::Borrowed("session token"),
                Type::ApiKey(_) => Cow::Borrowed("api key"),
                Type::None => Cow::Borrowed("none available"),
            },
        });
    }

    cli_table::print_stdout(rows.table().title([
        "Current".cell(),
        "Context".cell(),
        "Registry".cell(),
        "Auth Type".cell(),
    ]))
    .map_err(From::from)
}
