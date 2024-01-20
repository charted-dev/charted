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

use cargo_toml::Manifest;
use charted::cli::Execute;
use eyre::Report;
use heck::ToKebabCase;
use std::{
    env::current_dir,
    fs::{create_dir_all, File},
    io::{self, IsTerminal, Write as _},
    path::PathBuf,
    process::exit,
};

/// Constant of the license header for the `Cargo.toml` file.
const TOML_LICENSE_HEADER: &str = r#"# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License."#;

/// Constant of the license header for the `src/lib.rs` file.
const RUST_LICENSE_HEADER: &str = r#"// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
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
// limitations under the License."#;

/// Generates a Rust crate that contains the necessary components and referenced in the root Cargo.toml's
/// virtual manifest.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// The name of the Rust crate. This will prefix it with `charted-` with the given
    /// name if any. If this argument is not included and you're in a TTY, a prompt
    /// will give you the option to name it.
    name: Option<String>,

    /// Location of the Rust crate. It cannot exist outside of the workspace.
    location: Option<PathBuf>,
}

impl Execute for Cmd {
    fn execute(&self) -> eyre::Result<()> {
        let wd = current_dir()?;
        let name = match (self.name.clone(), io::stderr().is_terminal()) {
            (Some(name), _) => name,
            (None, true) => promptly::prompt("What should your Rust crate be called?")?,
            (None, false) => {
                error!("Please specify a crate name as you're not in a TTY");
                exit(1);
            }
        };

        let location = match (self.location.clone(), io::stderr().is_terminal()) {
            (Some(loc), _) => loc,
            (None, true) => {
                promptly::prompt::<String, _>("Enter the location for the Rust crate").map(PathBuf::from)?
            }

            (None, false) => {
                error!("Please specify a crate location as you're not in a TTY");
                exit(1);
            }
        };

        let final_name = format!("charted-{name}").to_kebab_case();
        let final_path = wd.join(location);
        info!(path = %final_path.display(), crate = final_name, "creating crate...");

        if !final_path.try_exists()? {
            create_dir_all(&final_path)?;
            create_dir_all(final_path.join("src"))?;
        }

        let wd_name = final_path.strip_prefix(&wd)?.as_os_str().to_str().expect("valid utf-8");
        let mut manifest = Manifest::from_path(wd.join("Cargo.toml"))?;
        let mut workspace = manifest.workspace.unwrap();

        workspace.members.push(wd_name.to_string());
        workspace.members.sort();
        manifest.workspace = Some(workspace);

        let mut file = File::options().write(true).open(wd.join("Cargo.toml"))?;
        let contents = toml::to_string(&manifest)?;
        write!(file, "{TOML_LICENSE_HEADER}\n\n{contents}")?;

        let mut crate_cargo_toml = File::options()
            .create_new(true)
            .write(true)
            .open(final_path.join("Cargo.toml"))?;

        let formatted = format!(
            r#"[package]
name = "{}"
description = "🐻‍❄️📦 TODO: fill this out"
version.workspace = true
edition.workspace = true
homepage.workspace = true
authors.workspace = true
rust-version.workspace = true
"#,
            final_name
        );

        write!(crate_cargo_toml, "{TOML_LICENSE_HEADER}\n\n{formatted}")?;

        let mut lib_rs_file = File::options()
            .create_new(true)
            .write(true)
            .open(final_path.join("src/lib.rs"))?;

        write!(
            lib_rs_file,
            r#"{RUST_LICENSE_HEADER}

"#
        )
        .map_err(Report::from)
    }
}
