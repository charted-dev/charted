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

use eyre::{eyre, Context, Result};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

/// Finds a binary with a specific path, or finds it under the `$PATH` variable
/// with a given `bin`.
///
/// ## Example
/// ```
/// # use charted_devtools::utils::*;
/// #
/// let bin = find_binary::<&str>(None, "rustc");
/// assert!(bin.is_some());
/// ```
pub fn find_binary<P: AsRef<Path>>(path: Option<P>, bin: &str) -> Option<PathBuf> {
    if let Some(p) = path {
        return Some(p.as_ref().to_path_buf());
    }

    which::which(bin).ok()
}

/// Creates and runs a specific command and returns the standard output as an allocated [`String`]. The
/// second argument is a builder method, which you can modify the command.
///
/// ## Panics
/// `cmd` can panic if the command has failed, which `eyre` will propagate if this was caused. It will
/// print both the standard output and error pipes.
///
/// ## Example
/// ```
/// # use charted_devtools::utils::cmd;
/// # use std::process::Stdio;
/// #
/// let cmd = cmd("ls", |_| {});
///
/// assert!(cmd.is_ok());
/// ```
pub fn cmd<C: AsRef<OsStr>, F: FnOnce(&mut Command)>(command: C, builder: F) -> Result<String> {
    let name = command.as_ref();
    let mut cmd = Command::new(&command);
    cmd.stdin(Stdio::null()).stderr(Stdio::null()).stdout(Stdio::inherit());
    builder(&mut cmd);

    let args = cmd.get_args().map(|x| x.to_string_lossy()).collect::<Vec<_>>();

    info!("$ {} {}", name.to_string_lossy(), args.join(" "));
    let output = cmd.output();

    match output {
        Ok(output) if output.status.success() => {
            Ok(String::from_utf8(output.stdout).context("unable to convert stdout to utf-8")?)
        }

        Ok(output) => {
            let code = output.status.code().unwrap_or(-1);

            // '101' is clap's help command exit code when prompted
            if code == 2 {
                return Ok(String::new());
            }

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            // this happens if Stdio::null()/Stdio::inherit() was used in Command::stdout/Command::stderr
            if stdout.is_empty() && stderr.is_empty() {
                panic!("command ran has failed with exit code {code} but no stdout/stderr logs were captured, view above for more information.");
            }

            panic!(
                "\n-- command has failed --\n~! STDOUT !~\n{stdout}\n\n~! STDERR !~\n{stderr}\n\nExited with code {}",
                output.status.code().unwrap_or(-1)
            );
        }

        Err(e) => Err(e.into()),
    }
}

/// Runs a `cargo` command with specified subcommand and arguments.
pub fn cargo<'s, P: AsRef<Path>, S: AsRef<OsStr>, I: Iterator<Item = S>>(
    cargo: Option<P>,
    subcommand: impl Into<&'s OsStr>,
    args: I,
) -> Result<()> {
    let cargo = find_binary(cargo, "cargo").ok_or_else(|| eyre!("unable to find `cargo` binary"))?;
    cmd(cargo, |cmd| {
        cmd.arg(subcommand.into())
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    })
    .map(|_| ())
}
