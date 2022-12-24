// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

use chrono::{DateTime, Utc};
use std::{error::Error, ffi::OsStr, process::Command, time::SystemTime, str};
use std::env::var;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use once_cell::sync::Lazy;
use regex::Regex;
use which::which;

macro_rules! to_io_error {
    ($m:expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $m)
    }
}

fn execute<T: AsRef<OsStr>>(command: T, args: &[&str]) -> Result<String, Box<dyn Error + 'static>> {
    let res = Command::new(command).args(args).output()?;
    Ok(String::from_utf8(res.stdout)?)
}

fn find_java_home() -> Result<Option<PathBuf>, Box<dyn Error>> {
    // If the JAVA_HOME environment exists, let's use it
    if let Ok(home) = var("JAVA_HOME") {
        // Check if `home` is a directory
        let path = Path::new(&home);
        if !path.is_dir() {
            return Err(Box::new(to_io_error!(format!("JAVA_HOME {} was not a directory", path.display()))));
        }

        return Ok(Some(path.to_path_buf()));
    }

    // If not, let's find it with `which`
    if let Ok(res) = which("java") {
        // Check if the path is a symbolic link, if it is. Sometimes, /usr/bin/java is a symbolic link
        // to the actual Java installation. So, we need to move the parent twice (if we can) to get the actual
        // JAVA_HOME directory.
        if res.is_symlink() {
            match res.read_link() {
                Ok(p) => {
                    if let Some(parent) = p.parent() {
                        assert!(parent.is_dir());

                        // We also need to go back the given path
                        if let Some(p2) = parent.parent() {
                            assert!(p2.is_dir());
                            return Ok(Some(p2.to_path_buf()));
                        }

                        // assume the parent is the main directory
                        return Ok(None);
                    }
                },

                Err(e) => return Err(Box::new(e))
            }

            // TODO: figure out if /usr/bin/java isn't a symlink or is this a good fallback?
            return Ok(None);
        }
    }

    // otherwise, just fail :<
    Ok(None)
}

fn execute_from_dir<T: AsRef<OsStr>, P: AsRef<Path>>(command: T, args: &[&str], working_directory: P) -> Result<String, Box<dyn Error>> {
    let java_exec = find_java_home()?;
    if java_exec.is_none() {
        return Err(Box::new(to_io_error!("Unable to detect JAVA_HOME")));
    }

    let canonical_path = canonicalize(working_directory.as_ref())?;
    let res = Command::new(command.as_ref())
        .args(args)
        .current_dir(canonical_path.clone())
        .env("JAVA_HOME", java_exec.unwrap())
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if !res.status.success() {
        let cmd_name = command.as_ref();
        let args_as_str = args.join(" ");
        let code = res.status.code().unwrap_or(-1);

        return Err(Box::new(to_io_error!(format!("Received exit code {code} while running command [{cmd_name:?} {args_as_str}] in directory [{}]", canonical_path.display()))));
    }

    let bytes = match str::from_utf8(&res.stdout) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e))
    };

    Ok(bytes.to_string())
}

#[cfg(not(target_os = "windows"))]
fn gradlew_wrapper() -> &'static str {
    "./gradlew"
}

#[cfg(target_os = "windows")]
fn gradlew_wrapper() -> &'static str {
    "./gradlew.bat"
}

// regex borrowed from https://github.com/z4kn4fein/kotlin-semver/blob/main/src/commonMain/kotlin/io/github/z4kn4fein/semver/Patterns.kt#L52-L53
static VERSION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"version: (v?(0|[1-9]\d*)(?:\.(0|[1-9]\d*))?(?:\.(0|[1-9]\d*))?(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:.[0-9a-zA-Z-]+)*))?)"#).unwrap());

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    let result = execute_from_dir(
        gradlew_wrapper(),
        &["properties", "--no-daemon", "--console=plain", "-q"],
        "../.."
    )?;

    let version: Vec<&str> = result
        .split('\n')
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|f| VERSION_REGEX.is_match(f))
        .collect::<Vec<_>>();

    if version.first().is_none() {
        return Err(Box::new(to_io_error!("Unable to determine version from Gradle properties")));
    }

    let v = version.first().unwrap();
    println!("cargo:rustc-env=HELM_PLUGIN_VERSION={}", v.split(':').collect::<Vec<_>>().last().unwrap());

    let commit_hash = execute("git", &["rev-parse", "--short=8", "HEAD"]).unwrap_or_else(|_| "noeluwu8".into());
    let build_date = {
        let now = SystemTime::now();
        let utc: DateTime<Utc> = now.into();

        utc.to_rfc3339()
    };

    println!("cargo:rustc-env=HELM_PLUGIN_COMMIT_HASH={commit_hash}");
    println!("cargo:rustc-env=HELM_PLUGIN_BUILD_DATE={build_date}");

    Ok(())
}
