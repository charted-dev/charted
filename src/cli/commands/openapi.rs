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

use crate::cli::Execute;
use std::path::PathBuf;

/// Generate the OpenAPI schema document without running the server itself.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// Output file to emit the JSON/YAML schema document into. If not specified, this will be written to stdout.
    output: Option<PathBuf>,

    /// whether or not if the OpenAPI schema document should be encoded into YAML instead of JSON.
    #[arg(long)]
    yaml: bool,
}

impl Execute for Cmd {
    fn execute(&self) -> eyre::Result<()> {
        Ok(())
    }
}

/*
impl Execute for OpenAPI {
    fn execute(&self) -> Result<()> {
        // Set a temporary config so we can grab the OpenAPI document without
        // panics.
        Config::temporary();

        let doc = Document::openapi();
        let serialized = if self.yaml {
            serde_yaml::to_string(&doc)?
        } else {
            serde_json::to_string_pretty(&doc)?
        };

        match self.output.clone() {
            Some(file) => {
                info!("Writing OpenAPI specification in {}", file.display());

                let path = file.clone();
                let mut file = match file.try_exists() {
                    Ok(true) => File::create(file)?,
                    Ok(false) => OpenOptions::new().read(true).write(true).create_new(true).open(file)?,
                    Err(e) => {
                        error!(error = %e, "unable to create file {}", file.display());
                        return Ok(());
                    }
                };

                write!(file, "{serialized}")?;
                info!("Successfully wrote OpenAPI specification in {}", path.display());

                Ok(())
            }

            None => {
                println!("{serialized}");
                Ok(())
            }
        }
    }
}

*/
