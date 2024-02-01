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

use crate::utils;
use async_trait::async_trait;
use bollard::{network::CreateNetworkOptions, Docker};
use charted::cli::AsyncExecute;
use eyre::{eyre, Report, Result};
use std::{
    env::current_dir,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::{exit, Stdio},
};

fn write_to<P: AsRef<Path>>(path: P, overwrite: bool, buf: String) -> Result<()> {
    let mut options = File::options();
    match overwrite {
        true => options.create(false).append(false).write(true).read(true),
        false => options.create(true).write(true).read(true),
    };

    let mut file = options.open(path.as_ref())?;
    write!(file, "{buf}").map_err(Report::from)
}

/// Starts the development Docker Compose project. This requires the Docker daemon
/// to be present at the Unix socket level (or named pipe on Windows) as it uses the API from that rather
/// from the CLI.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// whether or not to overwrite the `docker-compose.yml` file or just to copy it
    /// to `.cache/docker-compose.yml`
    #[arg(long)]
    overwrite: bool,

    /// Location to a `docker` binary.
    #[arg(long, env = "DOCKER")]
    docker: Option<PathBuf>,

    /// whether or not to run an Elasticsearch cluster for search purposes. This
    /// is mutually exclusive with `--meili`.
    #[arg(long)]
    elastic: bool,

    /// whether or not to run a Meilisearch server for search pirposes. This is
    /// mutually exclusive with `--elastic`
    #[arg(long)]
    meili: bool,
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> Result<()> {
        let wd = current_dir()?;
        let docker =
            utils::find_binary(self.docker.clone(), "docker").ok_or_else(|| eyre!("unable to find `docker` binary"))?;

        let compose_file = wd.join(".cache/docker-compose.yml");
        if !compose_file.try_exists()? || self.overwrite {
            info!(file = %compose_file.display(), "writing new `docker-compose.yml` in");
            let project = include_str!("../../../docker-compose.yml");

            write_to(&compose_file, self.overwrite, project.to_string())?;
        }

        if let (true, true) = (self.elastic, self.meili) {
            error!("--elastic and --meili flags are mutually exclusive and cannot be both used");
            exit(1);
        }

        if self.meili {
            info!("creating directories for meilisearch...");
            let dir = wd.join(".cache/docker/meilisearch");
            if !dir.try_exists()? {
                create_dir_all(&dir)?;
            }

            // check if config.toml exists, if it does
            // then copy it to config dir.
            let config_toml = include_str!("../../../configs/meilisearch/config.toml");
            let config_toml_file = dir.join("config.toml");
            if !config_toml_file.try_exists()? || self.overwrite {
                write_to(&config_toml_file, self.overwrite, config_toml.to_string())?;
            }
        }

        if self.elastic {
            info!("creating files and directories for Elasticsearch");
            let dir = wd.join(".cache/docker/elasticsearch");
            if !dir.try_exists()? {
                create_dir_all(&dir)?;
            }

            let config = dir.join("config");
            if !config.exists() {
                create_dir_all(config.clone())?;
            }

            // check if config/elasticsearch.yml exists, if it does
            // then copy it to config dir.
            let elasticsearch_yml = include_str!("../../../configs/elasticsearch/elasticsearch.yml");
            let elasticsearch_yml_file = config.join("elasticsearch.yml");
            if !elasticsearch_yml_file.exists() || self.overwrite {
                write_to(&elasticsearch_yml_file, self.overwrite, elasticsearch_yml.to_string())?;
            }

            let jvm_options = include_str!("../../../configs/elasticsearch/jvm.options");
            let jvm_options_file = config.join("jvm.options");
            if !jvm_options_file.exists() || self.overwrite {
                write_to(&jvm_options_file, self.overwrite, jvm_options.to_string())?;
            }

            let log4j2_properties = include_str!("../../../configs/elasticsearch/log4j2.properties");
            let log4j2_properties_file = config.join("log4j2.properties");
            if !log4j2_properties_file.exists() || self.overwrite {
                write_to(&log4j2_properties_file, self.overwrite, log4j2_properties.to_string())?;
            }
        }

        // now create fluff network
        {
            let client = Docker::connect_with_socket_defaults()?;
            client.ping().await?;

            let networks = client.list_networks::<String>(None).await?;
            if !networks.iter().any(|x| x.name == Some("fluff".to_string())) {
                info!("`fluff` network doesn't exist! creating...");
                client
                    .create_network(CreateNetworkOptions {
                        name: "fluff",
                        driver: "bridge",
                        ..Default::default()
                    })
                    .await?;

                info!("`fluff` network was created!");
            }
        }

        let root = wd.join(".cache");
        utils::cmd(docker, |cmd| {
            cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(["compose", "-f"])
                .arg(&compose_file)
                .args(["up", "-d", "--wait"])
                .current_dir(&root);

            if self.elastic {
                cmd.args(["--profile", "elasticsearch"]);
            }

            if self.meili {
                cmd.args(["--profile", "meilisearch"]);
            }
        })
        .map(|_| ())
    }
}
