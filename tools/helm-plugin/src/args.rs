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

use clap::Args;
use std::path::PathBuf;

/// Represents common arguments that can be passed with `helm charted` subcommands.
#[derive(Debug, Clone, Args)]
pub struct CommonArgs {
    /// location to a `.charted.hcl` to be evaluated, will default to `$CWD/.charted.hcl`
    #[arg(long = "config", short = 'c', env = "CHARTED_HELM_CONFIG")]
    pub config_path: Option<PathBuf>,

    /// location to a `helm` binary.
    #[arg(long, env = "CHARTED_HELM_BINARY")]
    pub helm: Option<PathBuf>,
}

#[derive(Debug, Clone, Args)]
pub struct CommonAuthArgs {
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
    pub auth: Option<PathBuf>,
}
