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

#![allow(dead_code)]

use charted_helm_types::ChartIndex;
use charted_types::name::Name;
use eyre::{bail, Context};
use rayon::ThreadPoolBuilder;
use std::{fs::File, process::exit, str::FromStr};
use tokio::runtime::Handle;
use tracing::{error, info};
use url::Url;

#[derive(Debug, Clone)]
pub enum Owner {
    Organization(Name),
    User(Name),
}

impl Owner {
    pub const fn name(&self) -> &Name {
        match self {
            Self::Organization(name) | Self::User(name) => name,
        }
    }
}

impl FromStr for Owner {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((_, name)) if name.contains(':') => bail!("cannot parse owner: received more than one ':'"),
            Some((kind, name)) => match &*kind.to_ascii_lowercase() {
                "user" => Ok(Owner::User(name.parse()?)),
                "org" => Ok(Owner::Organization(name.parse()?)),
                input => bail!("expected [user, org] as the prefix, received {} instead", input),
            },

            None => bail!("expected `user:<name>`, `org:<name>` as the input"),
        }
    }
}

/// Migrates a chart index into charted-server.
///
/// Since each index is apart of a **User** or **Organization** entity, this
/// will download a `index.yaml` from the internet or pull it from a file and
/// create the resources given.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// URL that points to a valid Chart index.
    ///
    /// The only acceptable schemes are **http(s)** and **file**.
    url: Url,

    /// The owner that owns this chart index.
    ///
    /// If the resource doesn't exist on the server, then it'll be created. If this is a
    /// single user registry, then this will fail if the owner doesn't exist.
    ///
    /// If this is a single organization registry and if registrations are disabled, then this
    /// will also fail.
    owner: Owner,

    /// the server that we should connect to.
    #[arg(long, short = 's')]
    server: Url,

    /// Number of Tokio workers to use.
    ///
    /// By default, this will use the number of avaliable CPU cores on the system
    /// itself.
    #[arg(long, short = 'w', env = "TOKIO_WORKER_THREADS", default_value_t = num_cpus::get())]
    pub workers: usize,

    /// Flag that only create the repositories for the names in this list. Otherwise,
    /// all charts in the index will be created on the server.
    #[arg(long)]
    only: Vec<String>,
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
            "Noelware/charted[cli] (https://github.com/charted-dev/charted; {})",
            charted_core::version()
        ))
        .use_rustls_tls()
        .build()
        .context("failed to build HTTP client")
}

pub async fn run(mut args: Args) -> eyre::Result<()> {
    build_rayon_pool(args.workers)?;

    let http = build_http_client()?;
    let chart: ChartIndex = match args.url.scheme() {
        "http" | "https" => {
            if !args.url.path().ends_with("index.yaml") {
                if args.url.path().ends_with('/') {
                    args.url = args.url.join("index.yaml")?;
                } else {
                    args.url.set_path(&format!("{}/index.yaml", args.url.path()));
                }
            }

            info!(url = %args.url, "downloading chart index!");
            let resp = http.execute(http.get(args.url.clone()).build()?).await?;

            if !resp.status().is_success() {
                error!(url = %args.url, "server responded with {}", resp.status());
                exit(1);
            }

            let bytes = resp.bytes().await?;
            serde_yaml_ng::from_slice(&bytes)
        }

        "file" => {
            let path = args.url.path();
            let file = File::open(path).with_context(|| format!("failed to open file: {path}"))?;

            serde_yaml_ng::from_reader(file)
        }

        scheme => bail!("unsupported scheme: {}", scheme),
    }?;

    info!("found {} charts to possibly migrate!", chart.entries().len());

    Ok(())
}
