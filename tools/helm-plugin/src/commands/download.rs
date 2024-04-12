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

use crate::{args::CommonArgs, util};
use clap::Parser;
use std::{path::PathBuf, process::exit};
use url::Url;

/// Attempts to download a chart from the charted-server REST API.
///
/// Do note that this command implements the downloader plugin for Helm
/// and isn't intended to be ran explicitly. For more information,
/// please read: https://helm.sh/docs/topics/plugins/#downloader-plugins
#[derive(Debug, Clone, Parser)]
pub struct Cmd {
    /// Path to a certificate public key. This isn't used.
    cert_file: PathBuf,

    /// Path to a certificate private key. This isn't used.
    key_file: PathBuf,

    /// Path to a certificate authority. This isn't used.
    ca_file: PathBuf,

    /// Download a repository with the given URI. If the scheme is not `charted://`,
    /// then it'll fail with no questions asked.
    repository: Url,

    #[command(flatten)]
    common: CommonArgs,
}

pub async fn run(cmd: Cmd) -> eyre::Result<()> {
    if cmd.repository.scheme() != "charted" {
        error!(repository = %cmd.repository, "expected `scheme` to be `charted://`");
        exit(1);
    }

    trace!("loading configuration");
    let config = util::load_config(cmd.common.config_path.as_ref())?;
    util::validate_version_constraints(&config, cmd.common.helm.as_ref());
    trace!("loaded configuration successfully");

    info!(uri = %cmd.repository, "attempting to download from uri");

    Ok(())
}
