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

use crate::{auth::Type, commands::HTTP, config, util};
use charted_core::response::{ApiResponse, ErrorCode};
use charted_entities::{Name, Repository, RepositoryRelease};
use semver::Version;
use std::{
    path::Path,
    process::{exit, Command, Stdio},
};

pub async fn test_heartbeat(registry: &config::registry::Config, ty: &Type) -> eyre::Result<()> {
    let endpoint = registry.join_url("heartbeat")?;
    trace!(%registry, endpoint, "sending heartbeat request to");

    let mut req = HTTP.get(endpoint);
    util::set_auth_details(&mut req, ty)?;

    let res = HTTP.execute(req.build()?).await?;
    if !res.status().is_success() {
        error!(
            "failed to request a heartbeat to the registry due to status: {}",
            res.status()
        );

        exit(1);
    }

    trace!("heartbeat successful -- server is running as expected");
    Ok(())
}

pub async fn get_repository_id(
    registry: &config::registry::Config,
    ty: &Type,
    (owner, name): (Name, Name),
) -> eyre::Result<i64> {
    let endpoint = registry.join_url(format!("repositories/{owner}/{name}"))?;
    trace!(%registry, endpoint, "now testing if repository is available on");

    // We need to determine the ID since charted-server doesn't normalise
    // names for repositories since they can be owned by *either* an organization
    // or a user.
    let mut req = HTTP.get(endpoint);
    util::set_auth_details(&mut req, ty)?;

    let res = HTTP.execute(req.build()?).await?;
    if !res.status().is_success() {
        // Sometimes a 500 can be received if the server isn't acting like it should.
        if res.status().is_server_error() {
            warn!(%registry, "Registry is not properly responding! Please try again later");
            exit(1);
        }

        if res.status().is_client_error() {
            error!(%registry, "Received a client error when contacting the registry! Are your credentials correct? (Use `--log-level=trace` to view what the server sent)");

            let data: serde_json::Value = serde_json::from_slice(res.bytes().await?.as_ref())?;
            trace!("Received data from server:\n{data:#}");

            exit(1);
        }

        error!("received a redirect -- should never happen");
        exit(127);
    }

    let ApiResponse::<Repository> { data: Some(repo), .. } = serde_json::from_slice(res.bytes().await?.as_ref())?
    else {
        trace!("reached unexpected clause -- internal bug");
        unreachable!();
    };

    trace!("pulled [{owner}/{name}] as repository {}", repo.id);

    Ok(repo.id)
}

pub async fn check_if_release_is_avaliable(
    registry: &crate::config::registry::Config,
    ty: &Type,
    version: &Version,
    id: i64,
) -> eyre::Result<bool> {
    let mut req = HTTP.get(registry.join_url(format!("repositories/{id}/releases/{}", version))?);
    util::set_auth_details(&mut req, ty)?;

    let res = HTTP.execute(req.build()?).await?;
    if !res.status().is_success() {
        // Sometimes a 500 can be received if the server isn't acting like it should.
        if res.status().is_server_error() {
            warn!(%registry, "Registry is not properly responding! Please try again later");
            exit(1);
        }

        // a 404 indicates that it was not found, which we will detect
        // since we can't really rely on status codes, more validation is required
        if res.status().is_client_error() && res.status().as_u16() != 404 {
            error!(%registry, "Received a client error when contacting the registry! Are your credentials correct? (Use `--log-level=trace` to view what the server sent)");

            let data: serde_json::Value = serde_json::from_slice(res.bytes().await?.as_ref())?;
            trace!("Received data from server:\n{data:#}");

            exit(1);
        }

        if res.status().as_u16() != 404 {
            error!("received a redirect -- should never happen");
            exit(127);
        }
    }

    match serde_json::from_slice::<ApiResponse<RepositoryRelease>>(res.bytes().await?.as_ref()) {
        Ok(ApiResponse {
            success: true,
            data: Some(_),
            ..
        }) => Ok(true),

        Ok(ApiResponse {
            success: false, errors, ..
        }) => {
            if errors.len() == 1 && errors[0].code == ErrorCode::EntityNotFound {
                return Ok(false);
            }

            error!("received an unknown error when trying to find a repository release for version {version}");
            trace!("{errors:#?}");

            exit(1);
        }

        Ok(resp) => {
            trace!("{resp:#?}");
            Err(eyre!(
                "received invalid response when trying to finding a repository release"
            ))
        }

        Err(e) => Err(e.into()),
    }
}

pub fn package_chart(path: &Path, helm: Option<&Path>) -> eyre::Result<()> {
    trace!(path = %path.display(), "packaging in");

    let helm = util::get_helm_path(helm)
        .ok_or_else(|| eyre!("unable to find `helm` in $PATH (this should be already validated)"))?;

    let mut cmd = Command::new(helm);
    let output = cmd
        .current_dir(path)
        .args(["package", "."])
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .stdout(Stdio::null())
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => Err(eyre!("failed to run 'helm package .', view above on why it failed")),
        Err(err) => Err(err.into()),
    }
}
