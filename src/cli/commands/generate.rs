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

use charted_config::Config;
use std::{fs::OpenOptions, io::Write, path::PathBuf, process::exit};

/// Writes a new configuration file in the given `path`. This will bail out of the path already exists.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// Location to write the new configuration file in
    path: PathBuf,
}

pub fn run(args: Args) -> eyre::Result<()> {
    match args.path.try_exists() {
        Ok(false) => {}
        Ok(true) => {
            warn!(path = %args.path.display(), "path already exists");
            exit(1);
        }

        Err(e) => return Err(e.into()),
    }

    info!(path = %args.path.display(), "writing new config file in");
    let mut file = OpenOptions::new().create_new(true).write(true).open(&args.path)?;

    let config = Config::default();
    let serialized = toml::to_string(&config)?;
    file.write_all(serialized.as_ref())?;

    info!(path = %args.path.display(), "wrote new config!");

    Ok(())
}
