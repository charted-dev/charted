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

use crate::utils;
use charted_common::cli::Execute;
use eyre::Result;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

#[derive(Debug, Clone, clap::Parser)]
#[clap(about = "Starts the Docker Compose project for development use")]
pub struct Up {
    /// Whether if the `docker-compose.yml` file should be updated on each
    /// invocation of the `up` command.
    #[arg(long)]
    overwrite: bool,

    /// Location to a `bazel` binary that is used to locate the workspace
    #[arg(long)]
    bazel: Option<PathBuf>,

    /// Location to a `docker` binary that exists on the filesystem.
    #[arg(long)]
    docker: Option<PathBuf>,

    /// Whether if Elasticsearch should be started up. Cannot collide with `--meili`
    #[arg(long)]
    elastic: bool,

    /// Whether if Meilisearch should be started up. Cannot collide with `--elastic`
    #[arg(long)]
    meili: bool,
}

impl Execute for Up {
    fn execute(&self) -> Result<()> {
        let bazel = utils::find_bazel(self.bazel.clone())?;
        let workspace: PathBuf = utils::info(bazel.clone(), &["workspace"])?.trim().into();
        let docker_compose_file = workspace.join(".cache/docker-compose.yml");
        if !docker_compose_file.exists() || self.overwrite {
            info!(
                "Writing new docker compose project in {}/.cache/docker-compose.yml!",
                workspace.display()
            );

            let mut file = File::options()
                .create(true)
                .read(true)
                .write(true)
                .open(docker_compose_file.clone())?;

            let compose_project = include_str!("../../../docker-compose.yml");
            write!(&mut file, "{compose_project}")?;
        }

        if let (true, true) = (self.elastic, self.meili) {
            error!("--elastic and --meili are mutually exclusive.");
            exit(1);
        }

        if self.meili {
            let meilisearch_dir = workspace.join(".cache/docker/meilisearch");
            if !meilisearch_dir.exists() {
                create_dir_all(meilisearch_dir.clone())?;
            }

            // check if config.toml exists, if it does
            // then copy it to config dir.
            let config_toml = include_str!("../../../configs/meilisearch/config.toml");
            let config_toml_file = meilisearch_dir.join("config.toml");
            if !config_toml_file.exists() || self.overwrite {
                let mut file = File::options()
                    .create(true)
                    .read(true)
                    .write(true)
                    .open(config_toml_file)?;

                write!(&mut file, "{config_toml}")?;
            }
        }

        if self.elastic {
            let elasticsearch_dir = workspace.join(".cache/docker/elasticsearch");
            if !elasticsearch_dir.exists() {
                create_dir_all(elasticsearch_dir.clone())?;
            }

            let config = elasticsearch_dir.join("config");
            let data = elasticsearch_dir.join("data");

            if !config.exists() {
                create_dir_all(config.clone())?;
            }

            if !data.exists() {
                create_dir_all(data.clone())?;
            }

            // check if config/elasticsearch.yml exists, if it does
            // then copy it to config dir.
            let elasticsearch_yml = include_str!("../../../configs/elasticsearch/elasticsearch.yml");
            let elasticsearch_yml_file = config.join("elasticsearch.yml");
            if !elasticsearch_yml_file.exists() || self.overwrite {
                let mut file = File::options()
                    .create(true)
                    .read(true)
                    .write(true)
                    .open(elasticsearch_yml_file)?;

                write!(&mut file, "{elasticsearch_yml}")?;
            }

            let jvm_options = include_str!("../../../configs/elasticsearch/jvm.options");
            let jvm_options_file = config.join("jvm.options");
            if !jvm_options_file.exists() || self.overwrite {
                let mut file = File::options()
                    .create(true)
                    .read(true)
                    .write(true)
                    .open(jvm_options_file)?;

                write!(&mut file, "{jvm_options}")?;
            }

            let log4j2_properties = include_str!("../../../configs/elasticsearch/log4j2.properties");
            let log4j2_properties_file = config.join("log4j2.properties");
            if !log4j2_properties_file.exists() || self.overwrite {
                let mut file = File::options()
                    .create(true)
                    .read(true)
                    .write(true)
                    .open(log4j2_properties_file)?;

                write!(&mut file, "{log4j2_properties}")?;
            }
        }

        let dc_file = docker_compose_file.clone();
        let mut args = vec![
            "compose",
            "-f",
            dc_file.as_os_str().to_str().unwrap(),
            "up",
            "-d",
            "--wait",
        ];

        if self.elastic {
            args.push("--profile");
            args.push("elasticsearch");
        }

        if self.meili {
            args.push("--profile");
            args.push("meilisearch");
        }

        let docker = utils::docker::find(self.docker.clone())?;
        utils::docker::exec(docker.clone(), workspace, args.as_slice())?;

        Ok(())
    }
}
