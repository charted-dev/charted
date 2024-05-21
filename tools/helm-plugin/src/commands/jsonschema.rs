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

use schemars::schema_for;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::exit,
};

/// Generates a JSON schema of the `.charted.hcl` file structure, mainly used internally
/// by charted's CI system.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// Output file to emit the JSON schema document into. If not specified, this will be written to stdout.
    output: Option<PathBuf>,
}

pub fn run(Cmd { output }: Cmd) -> eyre::Result<()> {
    let schema = schema_for!(crate::config::Config);
    match output {
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

            let serialized = serde_json::to_string(&schema)?;
            write!(file, "{serialized}")?;
            info!(path = %path.display(), "wrote specification in");
        }

        None => {
            println!("{}", serde_json::to_string(&schema)?);
        }
    }

    Ok(())
}
