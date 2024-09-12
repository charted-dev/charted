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

use std::path::PathBuf;

#[derive(Debug, Clone, clap::Args)]
pub struct Common {
    /// Location to a `.charted.hcl` configuration file to be evaluated from. Defaults
    /// to `$CWD/.charted.hcl`.
    #[arg(short = 'c', long = "config", env = "CHARTED_HELM_CONFIG_PATH")]
    pub config_path: Option<PathBuf>,

    /// Location to a `helm` binary. By default, it'll look in the system `$PATH`.
    #[arg(long, env = "CHARTED_HELM_BINARY")]
    pub helm: Option<PathBuf>,
}

#[derive(Debug, Clone, clap::Args)]
pub struct Auth {
    /// Location to a `auth.toml` configuration file which represents the ways to authenticate
    /// with a [charted-server](https://charts.noelware.org/docs/server/latest) registry.
    ///
    /// ## Default Locations
    /// | Operating System | Filesystem Location                                                                                    |
    /// | :--------------- | :----------------------------------------------------------------------------------------------------- |
    /// | Windows          | `C:\Users\{username}\AppData\Local\Noelware\charted-server\auth.toml`                                  |
    /// | macOS            | `/Users/{username}/Library/Application Support/Noelware/charted-server/auth.toml`                      |
    /// | Linux            | `$XDG_CONFIG_DIR/Noelware/charted-server/auth.toml`, `$HOME/.config/Noelware/charted-server/auth.toml` |
    #[arg(long = "auth", short = 'a', env = "CHARTED_HELM_AUTH_TOML_PATH")]
    pub path: Option<PathBuf>,
}
