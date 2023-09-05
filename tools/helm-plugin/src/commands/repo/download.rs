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

use charted_common::{cli::AsyncExecute, models::Name, COMMIT_HASH, VERSION};
use eyre::{ContextCompat, Result};
use once_cell::sync::Lazy;
use reqwest::Client;
use std::{path::PathBuf, process::exit};
use url::{Host, Url};

use crate::{auth::Context, CommonHelmArgs};

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent(format!("Noelware/charted-helm-plugin (+https://github.com/charted-dev/charted/tree/main/tools/helm-plugin; v{VERSION}+{COMMIT_HASH})"))
        .build()
        .unwrap()
});

#[derive(Debug, Clone, clap::Parser)]
pub struct Download {
    /// Location to download from. This can be any form of URL:
    ///
    /// * `charted://charted/server` ~ Downloads from the default context's registry, if applied. If not,
    /// this will default to `https://charts.noelware.org/api/repositories/{id}/download/latest.tar.gz`. Otherwise,
    /// this will default to `<registry url>/repositories/{id}/download/latest.tar.gz`.
    ///
    /// * `charted://some.registry/charted/server` ~ Downloads from `https://some.registry`.
    repo: Url,

    /// Location to a `auth.yaml` file that can be used to look up
    /// any additional contexts.
    #[arg(long, env = "CHARTED_HELM_CONTEXT_FILE")]
    context_file: Option<PathBuf>,

    /// The current context to use when authenticating to registries. By default,
    /// this will look in the following directories below:
    ///
    /// * Windows: `C:\Users\{username}\AppData\Local\Noelware\charted-server\auth.yaml`
    /// * macOS:   `/Users/{username}/Library/Application Support/Noelware/charted-server/auth.yaml`
    /// * Linux:   `/home/{username}/.config/Noelware/charted-server/auth.yaml`
    #[arg(long, short = 'c', env = "CHARTED_HELM_CONTEXT")]
    context: Option<String>,
}

#[async_trait]
impl AsyncExecute for Download {
    async fn execute(&self) -> Result<()> {
        if self.repo.scheme() != "charted" {
            error!("expected scheme to be [charted], received [{}]", self.repo.scheme());
            exit(1);
        }

        let auth = CommonHelmArgs::auth(self.context_file.clone()).await?;
        let ctx = match self.context.clone() {
            Some(ctx) => {
                let c = Context::new(ctx);
                auth.context
                    .iter()
                    .find(|(ctx, _)| ctx == &&c)
                    .map(|tup| tup.0)
                    .unwrap_or(&auth.current)
            }
            None => &auth.current,
        };

        let info = auth.context.get(ctx).unwrap();
        debug!("using context [{ctx}]");
        debug!("resolving uri [{}] to what we can resolve...", self.repo.clone());

        let registries = match self.repo.host() {
            Some(Host::Domain(dm)) => match dm.contains('.') {
                false => info.iter().map(|reg| reg.registry.clone()).collect::<Vec<_>>(),
                true => vec![Url::parse(format!("https://{dm}").as_str())?],
            },

            Some(_) => {
                error!("charted-helm doesn't support resolving ipv4/ipv6 hosts, yet!");
                exit(1);
            }

            None => {
                error!("expected host to be something!");
                exit(1);
            }
        };

        let (owner, repo) = match (self.repo.host(), self.repo.path_segments()) {
            (Some(Host::Domain(dm)), Some(mut iter)) => match dm.contains('.') {
                false => (Name::new(dm)?, Name::new(iter.next().context("missing path param")?)?),
                true => (
                    Name::new(iter.next().context("missing first param")?)?,
                    Name::new(iter.next().context("missing second param")?)?,
                ),
            },

            _ => unreachable!(),
        };

        info!("resolving repository [{owner}/{repo}]");
        debug!(
            "using the following registries:\n{}",
            registries
                .iter()
                .map(|s| format!("    * {s}"))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // check if the registry is alive
        let mut viable = vec![];
        for registry in registries.iter() {
            debug!("checking heartbeat with uri [{registry}/heartbeat] to determine if it is alive or not.");

            let res = CLIENT.get(format!("{registry}heartbeat")).send().await?;
            if !res.status().is_success() {
                warn!(
                    "registry [{registry}] sent status code [{}], not using as a viable registry!",
                    res.status()
                );

                continue;
            }

            debug!("registry [{registry}] is viable!");
            viable.push(registry.clone());
        }

        if viable.is_empty() {
            error!(
                owner = tracing::field::display(&owner),
                repo = tracing::field::display(&repo),
                "received no viable registries to find repository"
            );

            exit(1);
        }

        Ok(())
    }
}
