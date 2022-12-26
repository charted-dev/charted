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

use chrono::DateTime;
use clap::Parser;

use super::Execute;

#[derive(Debug, Clone, Copy, Parser)]
#[command(about = "Returns the current version of the CLI and the charted-server instance")]
pub struct Version;

impl Execute for Version {
    fn execute(
        self,
        _settings: &crate::settings::Settings,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let build_date = env!("HELM_PLUGIN_BUILD_DATE");
        let version = env!("HELM_PLUGIN_VERSION").trim();
        let commit = env!("HELM_PLUGIN_COMMIT_HASH");

        let datetime = DateTime::parse_from_rfc3339(build_date)
            .unwrap()
            .format("%a, %h %d, %Y at %H:%M:%S %Z");

        info!("charted/helm-plugin v{version}+{commit} (built on {datetime})");
        Ok(())
    }
}
