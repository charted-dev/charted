// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

use crate::{commands::AsyncExecute, BUILD_DATE, COMMIT_HASH, VERSION};
use chrono::DateTime;
use clap::Parser;
use serde_json::Value;

#[derive(Debug, Clone, Copy, Parser)]
#[command(about = "Returns the current version of the CLI and the charted-server instance")]
pub struct Version;

#[async_trait]
impl AsyncExecute for Version {
    async fn execute(
        self,
        settings: &crate::settings::Settings,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let datetime = DateTime::parse_from_rfc3339(BUILD_DATE)
            .unwrap()
            .format("%a, %h %d, %Y at %H:%M:%S %Z");

        let res = settings
            .http_client()
            .get(format!("{}/info", settings.server()))
            .send()
            .await?;

        info!("charted/helm-plugin v{VERSION}+{COMMIT_HASH} (built at {datetime})");
        if res.status().is_success() {
            let blob: Value = serde_json::from_slice(res.bytes().await?.as_ref())?;
            let data = blob["data"].as_object().unwrap();

            info!(
                "==> charted-server [{}] v{}+{} ({})",
                settings.server(),
                data["version"].as_str().unwrap(),
                data["commit_sha"].as_str().unwrap(),
                data["distribution"].as_str().unwrap()
            );
        }

        Ok(())
    }
}
