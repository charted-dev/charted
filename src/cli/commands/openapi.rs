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

use crate::server::openapi::Document;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::exit,
};
use utoipa::{openapi::ServerBuilder, OpenApi};

/// Generate the OpenAPI schema document without running the server itself.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// Output file to emit the JSON/YAML schema document into. If not specified, this will be written to stdout.
    output: Option<PathBuf>,

    /// whether or not if the OpenAPI schema document should be encoded into YAML instead of JSON.
    #[arg(long)]
    yaml: bool,
}

pub fn run(args: Args) -> eyre::Result<()> {
    let mut doc = Document::openapi();
    doc.servers = Some(vec![ServerBuilder::new()
        .description(Some("Official home for charted-server, Noelware's public instance"))
        .url("https://charts.noelware.org")
        .build()]);

    let serialized = if args.yaml {
        serde_yaml::to_string(&doc)?
    } else {
        serde_json::to_string(&doc)?
    };

    match args.output {
        Some(ref path) => {
            info!(path = %path.display(), "writing specification in");
            let mut file = match path.try_exists() {
                Ok(true) => File::create(path)?,
                Ok(false) => OpenOptions::new().create_new(true).write(true).open(path)?,
                Err(e) => {
                    error!(error = %e, path = %path.display(), "unable to validate that path exists");
                    exit(1);
                }
            };

            write!(file, "{serialized}")?;
            info!(path = %path.display(), "wrote specification in");

            Ok(())
        }

        None => {
            println!("{serialized}");
            Ok(())
        }
    }
}
