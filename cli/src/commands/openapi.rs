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

use charted_common::cli::Execute;
use charted_config::Config;
use charted_server::openapi::document;
use eyre::Result;
use std::{
    fs::{File, OpenOptions},
    io::Write as _,
    path::PathBuf,
};

#[derive(Debug, Clone, clap::Parser)]
#[clap(about = "Generates the OpenAPI document without running the API server")]
pub struct OpenAPI {
    #[arg(help = "Output file to use. If this is not specified, then it will be written to stdout.")]
    output: Option<PathBuf>,

    #[arg(long, help = "Whether if we should encode the OpenAPI document in YAML or not.")]
    yaml: bool,
}

impl Execute for OpenAPI {
    fn execute(&self) -> Result<()> {
        // Set a temporary config so we can grab the OpenAPI document without
        // panics.
        Config::temporary();

        let doc = document();
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
