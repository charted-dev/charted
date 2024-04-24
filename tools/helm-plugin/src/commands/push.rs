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

mod helpers;

use crate::{
    args::{CommonArgs, CommonAuthArgs},
    auth::{Auth, Type},
    commands::HTTP,
    config::{self, Config},
    util,
};
use charted_entities::helm::Chart;
use clap::Parser;
use inquire::InquireError;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::{Form, Part},
};
use std::{fs, path::PathBuf, process::exit};

/// Push one or all Helm charts to a charted-server registry
#[derive(Debug, Clone, Parser)]
pub struct Cmd {
    /// Amount of concurrent workers to push multiple Helm charts to.
    ///
    /// This is used to parallelize uploads to `charted-server` but respects
    /// ratelimiting at the same time. If multiple projects are configured,
    /// then it'll upload a new version to the server if a repository release
    /// wasn't already availiable, if there is, then it'll do nothing.
    #[arg(short = 'w', long, env = "CHARTED_HELM_CONCURRENCY")]
    concurrency: Option<usize>,

    /// Force pushes a new version, this will be ignored if no TTY is attached.
    #[arg(short = 'f', long)]
    force: bool,

    #[command(flatten)]
    common: CommonArgs,

    #[command(flatten)]
    auth: CommonAuthArgs,

    /// The repository to push a new version to. '.' can be referenced to signalify that
    /// multiple charts need to be pushed.
    repository: String,
}

pub async fn run(
    Cmd {
        repository: repo,
        common,
        force,
        auth: CommonAuthArgs { auth },
        ..
    }: Cmd,
) -> eyre::Result<()> {
    // let concurrency = {
    //     let value = concurrency.unwrap_or(2);
    //     min(value, num_cpus::get())
    // };

    let config = util::load_config(common.config_path.as_ref())?;
    util::validate_version_constraints(&config, common.helm.as_ref());

    debug!("found `.charted.hcl` file successfully");
    if config.repositories.is_empty() {
        warn!("cannot create a new upload since there is no repositories configured");
        return Ok(());
    }

    if repo == "." && config.repositories.len() == 1 {
        let mut repositories = config.repositories.clone();
        let Some((repo, repocfg)) = repositories.pop_first() else {
            unreachable!()
        };

        return upload_single_repository(repo, auth, config, &repocfg, force, common.helm).await;
    }

    if repo == "." {
        return Err(eyre!("uploading multiple charts is not supported (yet!)"));
        // return upload_multi_repositories(&config, auth, concurrency).await;
    }

    let repos = config.repositories.clone();
    let Some(cfg) = repos.get(&repo) else {
        error!("Unknown repository: {repo}");
        exit(1);
    };

    upload_single_repository(repo.clone(), auth, config, cfg, force, common.helm).await
}

async fn upload_single_repository(
    repo: String,
    auth_yaml: Option<PathBuf>,
    project: Config,
    config: &config::repository::Config,
    force: bool,
    helm: Option<PathBuf>,
) -> eyre::Result<()> {
    info!(repository = %repo, "now creating a new version for");

    // Check if the registry exists
    let Some(registry) = project.registries.get(&config.registry) else {
        error!(
            "Registry for configured repository [{}] doesn't exist in HCL configuration",
            config.registry
        );

        exit(1);
    };

    trace!(repository = %repo, %registry, "configured registry for");

    // First, we need to load the user's `auth.yaml` file to determine the authentication
    // for the registry
    let auth = Auth::load(auth_yaml)?;
    let __default = crate::auth::Registry {
        auth: Type::None,
        registry: registry.url.clone(),
    };

    let crate::auth::Registry { auth: ty, .. } = auth
        .contexts
        .values()
        .find(|reg| reg.registry == registry.url)
        .unwrap_or(&__default);

    // First, we need to test if we can send any requests to the server
    helpers::test_heartbeat(registry, ty).await?;

    // Next, is to test if the repository is available on the server.
    let id = helpers::get_repository_id(registry, ty, config.path.clone()).await?;

    // Now, we need to check if '$source/Chart.yaml' exists since we'll use that
    // to determine versions
    let chart_yaml = config.source.join("Chart.yaml");
    if !chart_yaml.try_exists()? {
        error!(repository = id, %registry, chart = %chart_yaml.display(), "unable to locate 'Chart.yaml'");
        exit(1);
    }

    info!(repository = id, %registry, chart = %chart_yaml.display(), "located 'Chart.yaml'");
    let chart: Chart = {
        let contents = fs::read_to_string(&chart_yaml)?;
        serde_yaml::from_str(&contents)?
    };

    // If we're in a CI system, then we won't prompt if we should push a new version, so it'll act like `--force`
    // ...without using the `--force` flag.
    if !helpers::check_if_release_is_avaliable(registry, ty, &chart.version, id).await? {
        let should_prompt = !is_ci::cached() && !force;
        if should_prompt {
            match inquire::prompt_confirmation("Do you wish to push a new version?") {
                Ok(true) => {}
                Ok(false) => {
                    info!("told to not push version {} to registry", chart.version);
                    return Ok(());
                }

                Err(InquireError::NotTTY) => {
                    warn!("there is no TTY available, using `--force` is not useful here");
                }

                Err(e) => return Err(e.into()),
            }
        }
    }

    info!("Packaging Helm chart with `helm` CLI...");
    helpers::package_chart(&config.source, helm.as_deref())?;

    // Helm will save it in '{name}-{version}.tgz'
    let pkg = config.source.join(format!("{}-{}.tgz", chart.name, chart.version));
    assert!(
        pkg.try_exists()?,
        "failed to assert that path exist, might be an internal change by Helm"
    );

    info!(chart = %pkg.display(), "successfully packaged chart with Helm! Pushing to server...");

    let bytes = fs::read(&pkg)?;
    let ct = remi_fs::default_resolver(&bytes);

    let form = Form::new().part(
        "main",
        Part::bytes(bytes).headers({
            let mut map = HeaderMap::new();
            map.insert("content-type", HeaderValue::from_str(&ct)?);

            map
        }),
    );

    let mut req = HTTP.put(registry.join_url(format!("repositories/{id}/releases/{}/tarball", chart.version))?);
    util::set_auth_details(&mut req, ty)?;

    let res = HTTP.execute(req.multipart(form).build()?).await?;
    if res.status().is_success() {
        info!("Uploaded Helm chart successfully!");
        return Ok(());
    }

    error!(status = %res.status(), "unable to upload Helm chart :(");
    trace!("{}", res.text().await?);

    Ok(())
}
