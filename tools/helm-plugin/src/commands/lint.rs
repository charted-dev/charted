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

use crate::{args::CommonArgs, util};
use eyre::{Context, Result};
use owo_colors::{OwoColorize, Style};
use similar::{ChangeTag, TextDiff};
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};

/// Lints a `.charted.hcl` file and the repositories inside of the project.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// location to the `hclfmt` command, if it wasn't found or if it is
    /// a invalid absolute path, then it will not attempt to format
    /// the configuration file.
    #[arg(long, env = "HCLFMT")]
    hclfmt: Option<PathBuf>,

    /// overwrites the formatted file instead of checking if it is
    /// formatted successfully or not
    #[arg(long, short = 'f')]
    fix: bool,

    /// whether or not to skip running `helm lint`
    #[arg(long)]
    skip_helm_lint: bool,

    /// list of `helm lint` arguments to pass in
    #[arg(long, short = 'a')]
    lint_args: Vec<OsString>,

    #[command(flatten)]
    common: CommonArgs,
}

pub async fn run(cmd: Cmd) -> Result<()> {
    let config = util::load_config(cmd.common.config_path.as_ref())?;
    util::validate_version_constraints(&config, cmd.common.helm.as_ref());

    let hclfmt = match cmd.hclfmt {
        Some(hclfmt) => hclfmt,
        None => match which::which("hclfmt") {
            Ok(hclfmt) => hclfmt,
            Err(which::Error::CannotFindBinaryPath) => {
                warn!("unable to find `hclfmt`, formatting .charted.hcl will not be performed");

                return perform_helm_lint(cmd.common.helm.as_ref(), cmd.skip_helm_lint, cmd.lint_args.clone());
            }

            Err(e) => return Err(From::from(e)),
        },
    };

    info!(hclfmt = %hclfmt.display(), "attempting to format `.charted.hcl`...");
    perform_hclfmt(&hclfmt, cmd.common.config_path.as_ref(), cmd.fix)?;

    info!("now running `helm lint`...");
    perform_helm_lint(cmd.common.helm.as_ref(), cmd.skip_helm_lint, cmd.lint_args.clone())
}

fn perform_hclfmt(hclfmt: &Path, file: Option<&PathBuf>, fix: bool) -> Result<()> {
    let config = util::get_config(file.cloned());
    let old = fs::read_to_string(&config)?;

    let mut cmd = Command::new(hclfmt);
    cmd.arg("-check")
        .arg(&config)
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    match cmd.output()? {
        output if output.status.success() => {
            // should be impossible unless hclfmt emits non UTF-8
            let contents = String::from_utf8(output.stdout).context("unable to convert to UTF-8")?;
            let diff = TextDiff::from_lines(&old, &contents);

            // if they are the same (ratio() returns 1.0 if matches are the same), then
            // return Ok early as they're the same in contents!
            if diff.ratio() == 1.0 {
                return Ok(());
            }

            if fix {
                let mut file = OpenOptions::new().write(true).append(false).open(&config)?;
                write!(file, "{contents}")?;

                info!("formatted `.charted.hcl` successfully");
                return Ok(());
            }

            error!("received mismatched content when formatting:");

            // this is very inspired by the `terminal-inline` example from `similar`:
            // https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
            struct Line(Option<usize>);
            impl std::fmt::Display for Line {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self.0 {
                        Some(idx) => write!(f, "{idx:<3}"),
                        None => write!(f, "   "),
                    }
                }
            }

            for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
                if idx > 0 {
                    eprintln!("{:-^1$}", "-", 80);
                }

                for op in group {
                    for change in diff.iter_inline_changes(op) {
                        let (sign, s) = match change.tag() {
                            ChangeTag::Delete => ("-", Style::new().red()),
                            ChangeTag::Insert => ("+", Style::new().green()),
                            ChangeTag::Equal => (" ", Style::new().dimmed()),
                        };

                        eprint!(
                            "{}{} |{}",
                            Line(change.old_index()).style(Style::new().dimmed()),
                            Line(change.new_index()).style(Style::new().dimmed()),
                            sign.style(s).bold()
                        );

                        for (emphasized, value) in change.iter_strings_lossy() {
                            if emphasized {
                                eprint!("{}", value.style(s.underline().on_black()));
                            } else {
                                eprint!("{}", value.style(s));
                            }
                        }

                        if change.missing_newline() {
                            eprintln!();
                        }
                    }
                }
            }

            exit(1);
        }

        output => {
            error!(
                "running `hclfmt` has failed with code {}",
                output.status.code().unwrap_or(-1)
            );

            exit(output.status.code().unwrap_or(-1));
        }
    }
}

fn perform_helm_lint(helm: Option<&PathBuf>, skip: bool, extra: Vec<OsString>) -> Result<()> {
    if skip {
        return Ok(());
    }

    let helm = helm
        .map(|x| x.to_path_buf())
        .unwrap_or(which::which("helm").context("unable to find `helm` command")?);

    let mut cmd = Command::new(&helm);
    cmd.arg("lint")
        .args(extra)
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    info!(
        "$ {} {}",
        helm.display(),
        cmd.get_args()
            .map(|x| x.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let output = cmd.output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        error!(
            "`helm lint` has failed with code {}",
            output.status.code().unwrap_or(-1)
        );

        eprintln!("stdout:\n{stdout}\n");
        eprintln!("stderr:\n{stderr}\n");

        exit(output.status.code().unwrap_or(1));
    }

    Ok(())
}
