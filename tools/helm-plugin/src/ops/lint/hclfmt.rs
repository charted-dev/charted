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

//! The `charted_helm_plugin::ops::lint::hclfmt` module is the main module to operate
//! formatting a HCL-based file with `hclfmt` and provide useful diagnostics and provide
//! auto-fixing.

use eyre::Context;
use owo_colors::{OwoColorize, Style};
use similar::{ChangeTag, TextDiff};
use std::{
    ffi::OsStr,
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write as _,
    path::PathBuf,
    process::{Command, Stdio},
};

/// Invokes `hclfmt` with the given path that resolves to a `hclfmt` binary, gets the difference,
/// then print out the different if `fix` wasn't provide or auto-fixes the file alltogether.
pub fn hclfmt<P: AsRef<OsStr>>(hclfmt: P, fix: bool, config: PathBuf) -> eyre::Result<()> {
    let old = fs::read_to_string(&config)?;
    let mut cmd = Command::new(&hclfmt);
    cmd.arg("-check")
        .arg(&config)
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    let output = cmd.output().with_context(|| {
        format!(
            "failed to run `{} -check {}`",
            hclfmt.as_ref().to_string_lossy(),
            config.display()
        )
    })?;

    if !output.status.success() {
        error!(hclfmt = ?hclfmt.as_ref(), config = %config.display(), "failed to call `hclfmt` on configuration file");
        error!("-- stderr --");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        return Err(eyre!(
            "failed to call `hclfmt` to format `.charted.hcl`, view the above logs on why it failed"
        ));
    }

    // `hclfmt` outputs the difference in stdout. this should never fail unless `hclfmt` emits non UTF-8.
    let contents = String::from_utf8(output.stdout).context("failed to convert `hclfmt` output to UTF-8")?;
    let diff = TextDiff::from_lines(&old, &contents);

    // 1.0 indicates that `old` and the `contents` given from `hclfmt` are the same, so we don't
    // need to format `.charted.hcl` or show the difference
    if diff.ratio() == 1.0 {
        return Ok(());
    }

    if fix {
        info!(config = %config.display(), "auto-fixing configuration file");

        let mut file = OpenOptions::new().write(true).append(false).open(&config)?;
        write!(file, "{contents}")?;

        return Ok(());
    }

    warn!(config = %config.display(), ratio = diff.ratio(), "received mismatched formatting from `hclfmt`:");
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

    Err(eyre!("^^^^ view above logs ^^^^"))
}

// this is very inspired by the `terminal-inline` example from `similar`:
// https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
struct Line(Option<usize>);
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            // we don't expect to reach ~1k lines in `.charted.hcl` (as of 01/06/24), if we do
            // then please report a issue.
            Some(idx) => write!(f, "{idx:<3}"),
            None => write!(f, "   "),
        }
    }
}
