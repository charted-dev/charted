// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

#![allow(unused)]

use azalia::remi::{core::StorageService as _, StorageService};
use charted_config::Config;
use charted_types::{
    helm::{ChartIndex, ChartIndexSpec},
    name::Name,
    Ulid,
};
use clap::ValueEnum;
use eyre::{eyre, Context};
use rayon::ThreadPoolBuilder;
use std::{fmt::Display, fs::File, path::PathBuf, process::exit};
use tokio::runtime::Handle;
use tracing::{error, info, instrument, trace, warn};
use url::Url;

use crate::util;

#[derive(Debug, Clone, Copy, Default, PartialEq, ValueEnum)]
pub enum RepoOwnerKind {
    #[default]
    User,
    Organization,
}

impl Display for RepoOwnerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoOwnerKind::User => f.write_str("user"),
            RepoOwnerKind::Organization => f.write_str("organization"),
        }
    }
}

/// Migrate a chart index from the internet into charted-server.
///
/// Since charts are repositories, this will create a user/organization account
/// by the name given and will store the `index.yaml` downloaded into its metadata
/// with all URLs to point to us.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// URL that points to a valid index.
    ///
    /// To use a file, use the `file://` scheme.
    url: Url,

    /// The name of the user/organization that will be created
    /// that owns the charts.
    ///
    /// By default, it'll use the hostname as the user or organization name.
    name: Option<Name>,

    /// Owner type, which can be a `User` or `Organization`
    #[arg(short = 'k', long, default_value_t = RepoOwnerKind::User)]
    kind: RepoOwnerKind,

    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.yml`/`./config.yml` if found.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Amount of workers to use when doing filesystem operations.
    ///
    /// This is used to spawn multiple threads if the Helm index is really large, but it might
    /// be useful to set this to `1` if it is a relatively small index.
    #[clap(long, short = 'w', env = "CHARTED_RUNTIME_WORKERS", default_value_t = num_cpus::get())]
    workers: usize,
}

// Credit for the `spawn_handler` code:
// https://users.rust-lang.org/t/can-rayon-and-tokio-cooperate/85022/3
/// This method is invoked by [`run`] to build a global Rayon pool to perform
/// concurrent Tokio tasks.
fn build_rayon_pool(workers: usize) -> eyre::Result<()> {
    ThreadPoolBuilder::new()
        .num_threads(workers)
        .panic_handler(|msg| {
            let msg = azalia::message_from_panic(msg);

            error!(%msg, "rayon thread panicked");
        })
        .thread_name(|idx| format!("charted-rayon-pool[#{idx}]"))
        .spawn_handler(|thread| {
            let rt = Handle::current();
            let mut b = std::thread::Builder::new();
            if let Some(name) = thread.name() {
                b = b.name(name.to_owned());
            }

            if let Some(stack_size) = thread.stack_size() {
                b = b.stack_size(stack_size);
            }

            b.spawn(move || {
                let _guard = rt.enter();
                thread.run()
            })?;

            Ok(())
        })
        .build_global()
        .context("failed to build global rayon pool")
}

fn build_http_client() -> eyre::Result<reqwest::Client> {
    reqwest::ClientBuilder::new()
        .user_agent(format!(
            "Noelware/charted CLI (https://github.com/charted-dev/charted; {})",
            charted_core::version()
        ))
        .use_rustls_tls()
        .build()
        .context("failed to build HTTP client")
}

pub async fn run(
    Args {
        mut url,
        workers,
        config,
        kind,
        name,
    }: Args,
) -> eyre::Result<()> {
    build_rayon_pool(workers)?;

    trace!("building server configuration...");
    let config = util::load_config(config)?;

    trace!("building data storage...");
    let app = charted_app::Context::new(config).await?;
    charted_helm_charts::init(&app.storage).await?;

    let name = match name {
        Some(name) => name,
        None => match url.host_str() {
            Some(host) => host.parse()?,
            None => {
                error!("unable to infer name from URI, please specify it as the second argument");
                exit(1);
            }
        },
    };

    let id = create_account(&app, kind, &name).await?;
    let chart: ChartIndex = match url.scheme() {
        "https" | "http" => {
            let http = build_http_client()?;

            // If it doesn't end with `index.yaml`, then append it
            if !url.path().ends_with("index.yaml") {
                if url.path().ends_with('/') {
                    url = url.join("index.yaml")?;
                } else {
                    url.set_path(&format!("{}/index.yaml", url.path()));
                }
            }

            info!(%url, "attempting to get chart index");
            let resp = http
                .execute(http.get(url.clone()).build().context("failed to build HTTP request")?)
                .await?;

            if !resp.status().is_success() {
                error!("received status code {} with URL {}", resp.status(), url);
                exit(1);
            }

            let bytes = resp.bytes().await?;
            serde_yaml_ng::from_slice(&bytes)?
        }

        "file" => {
            let path = url.path();
            let file = File::open(path).with_context(|| format!("failed to open file {path}"))?;

            serde_yaml_ng::from_reader(file)?
        }

        scheme => return Err(eyre!("unsupported scheme: {scheme}")),
    };

    info!("collected {} charts to dump!", chart.entries.len());

    let base_url = app.config.base_url.as_ref().unwrap();
    for (name, specs) in chart.entries {
        if let Err(e) = dump_chart(&app.storage, base_url, &name, &specs).await {
            error!(error = %e, %name, "failed to dump chart, skipping");
        }
    }

    Ok(())
}

#[instrument(name = "charted.helm.createOwner", skip(ctx))]
async fn create_account(ctx: &charted_app::Context, kind: RepoOwnerKind, name: &Name) -> eyre::Result<Ulid> {
    // If this is a single user registry AND there is not a user, then we will
    // create it and not allow any other users to be created.
    if ctx.config.single_user {
        warn!("this instance is a single user registry, so I will perform a check if any users exist.");
        warn!("if not, then the migration will perform as normal and the {name} user will be created with 'changeme' as the password. otherwise, this operation will fail.");
    }

    // Otherwise, if this is a single organization registry AND there is not an organization,
    // then we will create the organization
    if ctx.config.single_org && kind == RepoOwnerKind::Organization {
        warn!("this instance is a single organization registry, so I will perform a check if any organizations already exist");
        warn!("if not, then the migration will perform as normal and the {name} organization will be created. Otherwise, this operation will fail.");
    }

    todo!()
}

#[instrument(
    name = "charted.helm.migrate",
    skip_all,
    fields(index.name = name)
)]
async fn dump_chart(storage: &StorageService, base: &Url, name: &str, specs: &[ChartIndexSpec]) -> eyre::Result<()> {
    // First, we need to create the repository.

    info!("performing dump with {} index specifications", specs.len());
    for mut spec in specs {
        dbg!(&spec.urls);
    }

    Ok(())
}
