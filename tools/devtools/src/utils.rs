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

use eyre::{Context, Result};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};
use which::which;

pub mod docker {
    use eyre::{eyre, Context, Result};
    use std::{
        path::{Path, PathBuf},
        process::Command,
    };
    use which::which;

    pub fn find<P: AsRef<Path>>(path: Option<P>) -> Result<PathBuf> {
        match path {
            Some(path) => Ok(path.as_ref().to_path_buf()),
            None => which("docker").context("unable to locate 'docker' binary from $PATH"),
        }
    }

    pub fn exec(docker: PathBuf, wd: PathBuf, args: &[&str]) -> Result<String> {
        let cmd = Command::new(docker.clone()).args(args).current_dir(wd).output()?;
        if !cmd.status.success() {
            let stdout = String::from_utf8(cmd.stdout).unwrap_or_else(|_| "<invalid utf-8 output>".into());
            let stderr = String::from_utf8(cmd.stderr).unwrap_or_else(|_| "<invalid utf8 output>".into());

            return Err(eyre!(
                "command '{} {}' has failed:\n--- stdout ---\n{}\n\n--- stderr ---\n{}",
                docker.display(),
                args.join(" "),
                stdout.trim(),
                stderr.trim(),
            ));
        }

        String::from_utf8(cmd.stdout).context("unable to transform stdout to utf-8")
    }
}

pub fn find_bazel<P: AsRef<Path>>(path: Option<P>) -> Result<PathBuf> {
    match path {
        Some(path) => validate(path.as_ref().to_path_buf()),
        None => validate(which("bazel").context("unable to locate 'bazel' binary from $PATH")?),
    }
}

pub fn info(bazel: PathBuf, args: &[&str]) -> Result<String> {
    let cmd = Command::new(bazel.clone()).arg("info").args(args).output()?;
    assert!(cmd.status.success(), "expected success, got failed command");

    String::from_utf8(cmd.stdout).context("unable to transform stdout to utf-8")
}

#[derive(Clone, Default)]
pub struct BuildCliArgs {
    pub release: bool,
    pub bazelrc: Vec<PathBuf>,
    pub args: Vec<OsString>,
    pub run: bool,
}

pub fn build_or_run(bazel: PathBuf, target: &str, args: BuildCliArgs) -> Result<()> {
    let mut cmd = Command::new(bazel.clone());
    cmd.args(args.bazelrc.as_slice());

    let is_build = match (args.release, args.run) {
        (true, true) => {
            cmd.args(["run", "--compilation_mode=opt", target]);
            false
        }

        (false, true) => {
            cmd.args(["run", target]);
            false
        }

        _ => {
            cmd.args(["build", "--compilation_mode=opt", target]);
            true
        }
    };

    if !args.args.is_empty() {
        cmd.arg("--");
    }

    for arg in args.args.iter() {
        cmd.arg(arg);
    }

    cmd.stdin(Stdio::null());
    cmd.env("CHARTED_DISTRIBUTION_KIND", "git");

    // we still want to see bazel stdout/stderr when --release
    // is passed in
    if args.release {
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
    }

    let cmd_args = cmd
        .get_args()
        .map(|arg| arg.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    info!("$ {} {}", bazel.display(), cmd_args.join(" "));

    let result = if args.release {
        cmd.output()?.status
    } else {
        cmd.spawn()?.wait()?
    };

    // exit immediately if the process failed
    if !result.success() {
        exit(result.code().unwrap_or(1));
    }

    if is_build {
        info!("--release was passed in but not --run, printing output location in stderr");
        let cquery = Command::new(bazel.clone())
            .args(args.bazelrc.clone())
            .args(["cquery", target, "--output=files"])
            .stdin(Stdio::null())
            .output()?;

        assert!(cquery.status.success(), "expected success, 'bazel cquery' failed");
        let stdout = String::from_utf8(cquery.stdout).context("unable to convert stdout to utf-8")?;

        eprintln!("{}", stdout.trim());
    }

    Ok(())
}

pub fn build_or_run_cli(bazel: PathBuf, args: BuildCliArgs) -> Result<()> {
    build_or_run(bazel, "//cli", args)
}

fn validate(bazel: PathBuf) -> Result<PathBuf> {
    let output = Command::new(bazel.clone())
        .arg("--version")
        .output()
        .context(format!("unable to run command [{} --version]", bazel.display()))?;

    if !output.status.success() {
        error!("unable to validate bazel binary");
        error!("why: {} --version failed to execute", bazel.display());
        error!("---- stdout ----");

        let stdout = String::from_utf8(output.stdout).context("unable to convert stdout to utf-8")?;
        error!("{stdout}");
        error!("---- stderr ----");

        let stderr = String::from_utf8(output.stderr).context("unable to convert stderr to utf-8")?;
        error!("{stderr}");
        exit(1);
    }

    let stdout = String::from_utf8(output.stdout).context("unable to convert stdout to utf-8")?;
    let split = stdout.split(' ').collect::<Vec<_>>();
    let Some(product) = split.first() else {
        error!("unable to validate bazel binary");
        error!("why: {} --version stdout was empty", bazel.display());

        exit(1);
    };

    match *product {
        "bazel" | "blaze" => {}
        _ => {
            error!("unable to validate bazel binary");
            error!(
                "why: {} --version uses an invalid product (expected: 'bazel', 'blaze'; received: {product})",
                bazel.display()
            );

            exit(1);
        }
    }

    let Some(version) = split.get(1) else {
        error!("unable to validate bazel binary");
        error!(
            "why: {} --version stdout was only '{product}', missing version number",
            bazel.display()
        );

        exit(1);
    };

    debug!("using product {product}, version {}", version.trim());
    Ok(bazel)
}
