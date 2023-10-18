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

use crate::{auth::Context, CommonHelmArgs};
use charted_common::cli::AsyncExecute;
use eyre::Result;
use std::{path::PathBuf, process::exit};
use tokio::{fs::File, io::AsyncWriteExt};

/// Deletes a context from the `auth.yaml` file.
#[derive(Debug, Clone, clap::Parser)]
pub struct Delete {
    /// Context to delete.
    context: String,

    /// Location to a `auth.yaml` file that can be used to look up
    /// any additional contexts.
    #[arg(long = "context", env = "CHARTED_HELM_CONTEXT_FILE")]
    context_file: Option<PathBuf>,
}

#[async_trait]
impl AsyncExecute for Delete {
    async fn execute(&self) -> Result<()> {
        let mut auth = CommonHelmArgs::auth(self.context_file.clone()).await?;
        let ctx = Context::new(self.context.clone());

        if ctx == Context::new("default") {
            error!("cannot remove default context!");
            exit(1);
        }

        if !auth.context.contains_key(&ctx) {
            error!("cannot remove context {ctx} from auth.yaml: it doesn't exist");
            exit(1);
        }

        let _ = auth.context.remove(&ctx);
        let mut file = File::create(CommonHelmArgs::locate(self.context_file.clone())?).await?;
        file.write_all(serde_yaml::to_string(&auth).unwrap().as_bytes()).await?;

        info!("✔️ deleted context {ctx} successfully");
        Ok(())
    }
}
