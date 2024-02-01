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
use charted::cli::AsyncExecute;
use clap::Parser;
use std::process::exit;
use url::Url;

/// Attempt to download a Helm chart from the charted-server REST API. This is also used
/// with `helm install` with the `charted://` URI scheme.
#[derive(Debug, Clone, Parser)]
pub struct Cmd {
    /// Download a repository with the given URI. If the scheme is not `charted://`,
    /// then it'll fail with no questions asked.
    #[arg()]
    repository: Url,

    #[command(flatten)]
    common: CommonArgs,
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        if self.repository.scheme() != "charted" {
            error!(repository = %self.repository, "expected `scheme` to be `charted://`");
            exit(1);
        }

        trace!("loading configuration");
        let config = util::load_config(self.common.config_path.clone())?;
        util::validate_version_constraints(&config, self.common.helm.clone());
        trace!("loaded configuration successfully");

        info!(uri = %self.repository, "attempting to download from uri");

        Ok(())
    }
}
