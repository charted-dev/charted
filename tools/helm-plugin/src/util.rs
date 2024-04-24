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

use crate::{auth::Type, config::Config};
use charted_common::lazy;
use eyre::Context;
use once_cell::sync::Lazy;
use reqwest::{header::AUTHORIZATION, RequestBuilder};
use semver::Version;
use std::{
    path::{Path, PathBuf},
    process::{exit, Command},
};

static DEFAULT_CONFIG_PATH: Lazy<PathBuf> = lazy!(PathBuf::new().join(".charted.hcl"));

/// Return a concrete [`PathBuf`] of the `.charted.hcl` file to format.
pub fn get_config(path: Option<PathBuf>) -> PathBuf {
    match path {
        Some(path) => path,
        None => Lazy::get(&DEFAULT_CONFIG_PATH)
            .unwrap_or(&*DEFAULT_CONFIG_PATH)
            .to_path_buf(),
    }
}

/// Loads a [`Config`] struct easily with one line of code with a optional location.
pub fn load_config<P: AsRef<Path>>(loc: Option<P>) -> eyre::Result<Config> {
    let path = loc.as_ref().map(AsRef::as_ref).unwrap_or(&*DEFAULT_CONFIG_PATH);
    Config::load(path)
}

pub fn get_helm_path<P: AsRef<Path>>(helm: Option<P>) -> Option<PathBuf> {
    match helm {
        Some(path) => Some(path.as_ref().to_path_buf()),
        None => match which::which("helm") {
            Ok(path) => Some(path),

            // don't even bother to exit if not found as it's not required for this
            Err(which::Error::CannotFindBinaryPath) => {
                error!("unable to find `helm` binary!");
                None
            }

            Err(e) => {
                error!(error = %e, "reached error while trying to locate `helm` binary via $PATH");
                None
            }
        },
    }
}

/// Validate the `charted { version = "..." }` and `charted { helm = "..." }` constraints
/// with a one-liner when called.
pub fn validate_version_constraints<P: AsRef<Path>>(config: &Config, helm: Option<P>) {
    if !config.charted.version.matches(&Version::parse(crate::VERSION).unwrap()) {
        error!(
            "configuration expects `charted-helm-plugin` to match its version constraint: {}, but we are on v{}",
            config.charted.version,
            crate::VERSION
        );

        exit(1);
    }

    let Some(helm) = get_helm_path(helm) else {
        exit(1);
    };

    let mut cmd = Command::new(&helm);
    cmd.args(["version", "--template", "'{{ .Version }}'"]);

    trace!(
        "$ {} {}",
        helm.display(),
        cmd.get_args()
            .map(|x| x.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    );

    match cmd.output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).to_string();
            let version = version.trim().replacen('\'', "", 2).replacen('v', "", 1);
            trace!("stdout: {version}");

            if !config
                .charted
                .helm
                .matches(&Version::parse(&version).expect("version to parse via Cargo's semver"))
            {
                error!(
                    "configuration expects Helm to match its version constraint: {}, but it is on {version}",
                    config.charted.helm
                );

                exit(1);
            }
        }

        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            error!(helm = %helm.display(), "received an abnormal status code [{}] with `helm version --template '{{ .Version }}'`", output.status.code().unwrap_or(-1));
            error!("report this to Noelware: https://github.com/charted-dev/charted/issues/new");
            error!("~~~ stdout ~~~");
            error!("{}", stdout.trim());
            error!("~~~ stderr ~~~");
            error!("{}", stderr.trim());

            exit(1);
        }

        Err(e) => {
            error!(error = %e, helm = %helm.display(), "unable to run `helm version --format '{{ .Version }}'`:");
            exit(1);
        }
    }
}

pub fn set_auth_details(req: &mut RequestBuilder, ty: &Type) -> eyre::Result<()> {
    match ty {
        Type::ApiKey(key) => {
            *req = req
                .try_clone()
                .ok_or_else(|| eyre!("failed to clone `RequestBuilder`"))?
                .header(AUTHORIZATION, format!("ApiKey {key}"));
        }

        Type::Session { access, .. } => {
            *req = req
                .try_clone()
                .ok_or_else(|| eyre!("failed to clone `RequestBuilder`"))?
                .bearer_auth(access);
        }

        Type::EnvironmentVariable { kind, env } => {
            let value =
                noelware_config::env!(env).with_context(|| format!("environment variable ${env} doesn't exist"))?;

            *req = req
                .try_clone()
                .ok_or_else(|| eyre!("failed to clone `RequestBuilder`"))?
                .header(AUTHORIZATION, format!("{kind} {value}"));
        }

        Type::Basic { username, password } => {
            *req = req
                .try_clone()
                .ok_or_else(|| eyre!("failed to clone `RequestBuilder`"))?
                .basic_auth(username, Some(password));
        }

        _ => {}
    }

    Ok(())
}
