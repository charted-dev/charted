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

use crate::{updaters, utils};
use charted_common::cli::Execute;
use eyre::Result;
use promptly::prompt;
use std::{
    fs::{self, create_dir_all, File},
    io::{IsTerminal, Write},
    path::PathBuf,
    process::exit,
};

#[derive(Debug, Clone, clap::Parser)]
#[command(
    name = "crate",
    about = "Generates a Rust crate that contains a Bazel BUILD, Cargo.toml, and is referenced in the root Cargo.toml file"
)]
pub struct RustModule {
    /// Binary location to a valid `bazel` command.
    #[arg(long)]
    bazel: Option<PathBuf>,

    /// A name for the Rust crate. This will bring a prompt
    /// to name your module, if it is running from a tty.
    name: Option<String>,

    /// The location to your Rust module. It cannot exist outside the Bazel workspace.
    location: Option<PathBuf>,
}

// This needs to model the root Cargo.toml file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RootCargoToml {
    workspace: CargoWorkspace,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CargoWorkspace {
    resolver: String,
    members: Vec<String>,
}

const LICENSE: &str = r#"# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

impl Execute for RustModule {
    fn execute(&self) -> Result<()> {
        let bazel = utils::find_bazel(self.bazel.clone())?;
        let workspace = PathBuf::from(utils::info(bazel.clone(), &["workspace"])?.trim());
        let name = match (self.name.clone(), std::io::stderr().is_terminal()) {
            (Some(name), _) => name,
            (None, true) => prompt("Enter the Rust crate you want to create")?,
            (None, false) => {
                error!("You will need to specify the Rust crate name you want to create as you're not in a tty!");
                exit(1);
            }
        };

        let location = match (self.location.clone(), std::io::stdout().is_terminal()) {
            (Some(path), _) => path,
            (None, true) => prompt::<String, _>("Enter the location to put source code in").map(PathBuf::from)?,
            (None, false) => {
                error!("You will need to specify the path to the crate you want to create as you're not in a tty!");
                exit(1);
            }
        };

        let final_path = workspace.join(location);
        info!("creating rust crate charted-{name} in {}", final_path.display());

        if !final_path.exists() {
            create_dir_all(final_path.clone())?;
            create_dir_all(final_path.clone().join("src"))?;
        }

        let cargo_toml = workspace.join("Cargo.toml");
        let mut root_cargo_toml: RootCargoToml = toml::from_str(fs::read_to_string(cargo_toml.clone())?.as_str())?;
        let stripped = final_path
            .strip_prefix(workspace)?
            .as_os_str()
            .to_str()
            .expect("invalid utf-8 in path");

        root_cargo_toml.workspace.members.push(stripped.to_string());
        root_cargo_toml.workspace.members.sort();

        let mut file = File::options().write(true).open(cargo_toml)?;
        let cargo_toml_contents = toml::to_string_pretty(&root_cargo_toml)?;

        write!(file, "{LICENSE}\n\n{cargo_toml_contents}")?;

        let mut crate_cargo_toml_file = File::options()
            .create_new(true)
            .write(true)
            .open(final_path.clone().join("Cargo.toml"))?;

        let formatted = format!(
            r#"[package]
name = "charted-{}"
description = "🐻‍❄️📦 TODO: fill this out"
version = "0.0.0-devel.0"
edition = "2021"
homepage = "https://charts.noelware.org"
authors = ["Noel Towa <cutie@floofy.dev>", "Noelware Team <team@noelware.org>"]
"#,
            name.clone()
        );

        write!(crate_cargo_toml_file, "{LICENSE}\n\n{formatted}")?;

        let mut crate_bazel_file = File::options()
            .create_new(true)
            .write(true)
            .open(final_path.clone().join("BUILD.bazel"))?;

        let build_file_content = format!(
            r#"load("//:build/rust.bzl", "rust_project")

exports_files(["Cargo.toml"])

rust_project(
    name = "{}",
)"#,
            name.clone()
        );

        write!(crate_bazel_file, "{LICENSE}\n\n{build_file_content}")?;
        let mut lib_rs_file = File::options()
            .create_new(true)
            .write(true)
            .open(final_path.clone().join("src/lib.rs"))?;

        write!(
            lib_rs_file,
            r#"// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
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
"#
        )?;

        updaters::bazel_cargo_manifest_updater(format!("//{stripped}:Cargo.toml"))?;
        info!("updated ./build/manifests.bzl that includes Cargo manifest target [//{stripped}:Cargo.toml]");

        Ok(())
    }
}
