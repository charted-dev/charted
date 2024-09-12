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

use crate::config::Config;
use charted_types::Version;
use std::{
    path::{Path, PathBuf},
    process::{exit, Command},
};
use tracing::{error, trace, warn};

pub fn get_helm_binary<P: AsRef<Path>>(helm: Option<P>) -> Option<PathBuf> {
    if let Some(helm) = helm {
        return Some(helm.as_ref().to_path_buf());
    }

    match which::which("helm") {
        Ok(path) => Some(path),
        Err(which::Error::CannotFindBinaryPath) => {
            warn!("cannot find `helm` bninary in `$PATH`.");
            None
        }

        Err(e) => {
            error!(error = %e, "received an error while trying to find `helm` binary from $PATH");
            None
        }
    }
}

pub fn validate_version_constraints<P: AsRef<Path>>(config: &Config, helm: Option<P>) -> eyre::Result<()> {
    if !config
        .charted
        .version
        .matches(&Version::parse(charted_core::VERSION)?.into())
    {
        error!(
            ".charted.toml expected that `charted-helm-plugin` to match the version constraint: {}; but `charted-helm-plugin` is at {}",
            config.charted.version,
            charted_core::VERSION
        );

        exit(1);
    }

    let Some(helm) = get_helm_binary(helm) else {
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
            let version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .replacen('\'', "", 2)
                .replacen('v', "", 1);

            let semver = Version::parse(&version)?;
            if !config.charted.helm.matches(&semver.into()) {
                error!(
                    ".charted.toml expected Helm version to match version constraint: {}; but `helm version` printed {version} instead",
                    config.charted.helm
                );

                exit(1);
            }

            Ok(())
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

/*
/// Validate the `charted { version = "..." }` and `charted { helm = "..." }` constraints
/// with a one-liner when called.
pub fn validate_version_constraints<P: AsRef<Path>>(config: &Config, helm: Option<P>) {
    match cmd.output() {
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
*/
