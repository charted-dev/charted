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

use azalia::log::writers;
use charted_server::openapi::Document;
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    process::exit,
};
use tracing::{error, info, trace};
use tracing_subscriber::{prelude::*, EnvFilter};
use utoipa::OpenApi;

const HELP: &str = r#"
━━━━━━━━━━━━━━━━━━━━━━ USAGE ━━━━━━━━━━━━━━━━━━━━━
    cargo dev internals -- <COMMAND> [...ARGS]

━━━━━━━━━━━━━━━━━━━━ COMMANDS ━━━━━━━━━━━━━━━━━━━━
`cargo dev internals -- openapi <PATH>`
    ↳ Generates the OpenAPI specification into <PATH>

`cargo dev internals -- jsonschema <PATH>`
    ↳ Generates the JSON schema for `.charted.toml`
      into <PATH>.
"#;

fn main() -> eyre::Result<()> {
    preinit()?;

    let mut args = env::args();
    match args.nth(1) {
        Some(res) => match &*res.to_ascii_lowercase() {
            "openapi" => openapi(args.next().map(PathBuf::from)),
            "jsonschema" => jsonschema(args.next().map(PathBuf::from)),
            _ => {
                eprintln!("{HELP}");
                exit(1);
            }
        },
        None => {
            eprintln!("{HELP}");
            exit(1);
        }
    }
}

fn preinit() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(azalia::log::WriteLayer::new_with(
            io::stderr(),
            writers::default::Writer {
                print_thread: false,
                print_module: false,

                ..Default::default()
            },
        ))
        .with(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn openapi(path: Option<PathBuf>) -> eyre::Result<()> {
    let default = env::current_dir()?.join("assets/openapi.json");
    let path = path.unwrap_or(default);

    trace!(path = %path.display(), "writing OpenAPI specification");
    let mut file = match path.try_exists() {
        Ok(true) => File::create(&path)?,
        Ok(false) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            OpenOptions::new().create_new(true).write(true).open(&path)?
        }

        Err(e) => {
            error!(error = %e, path = %path.display(), "unable to validate that path exists");
            exit(1);
        }
    };

    let document = Document::openapi();
    let serialized = document.to_json()?;

    write!(file, "{serialized}")?;
    info!(path = %path.display(), "wrote OpenAPI specification");

    Ok(())
}

fn jsonschema(_: Option<PathBuf>) -> eyre::Result<()> {
    Ok(())
}
