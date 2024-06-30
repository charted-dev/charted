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

use charted_server::openapi::document;
use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::exit,
};
use utoipa::openapi::ServerBuilder;

const HELP: &str = r#"🐻‍❄️📦 cargo ci <COMMAND> [...ARGS]

This tool is meant to be internal and not consumed outside of the
`charted-dev/charted` repository at any case. This tool is primarily
used in charted's CI system with GitHub Actions.

These commands were originally in the `charted-devtools` package but moved to
a new CLI tool since the `charted-server` and `charted-helm-plugin` dependencies
are too fat for `charted-devtools`.

- `cargo ci jsonschema [PATH]`: Generates a JSON schema in the specified path (or stdout if no `PATH`
                                is missing).
- `cargo ci openapi [PATH]`:    Generates the OpenAPI document in the specified path (or stdout if no `PATH`
                                is missing).
"#;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut args = env::args();
    match args.nth(1) {
        Some(elem) => match &*elem.to_ascii_lowercase() {
            "jsonschema" => jsonschema(args.next().map(PathBuf::from)),
            "openapi" => openapi(args.next().map(PathBuf::from)),
            elem => {
                eprintln!("unknown command: {elem}");
                eprintln!("{HELP}");

                exit(1)
            }
        },
        None => {
            eprintln!("{HELP}");
            exit(1)
        }
    }
}

fn jsonschema(path: Option<PathBuf>) -> eyre::Result<()> {
    let schema = schemars::schema_for!(charted_helm_plugin::config::Config);
    match path {
        Some(ref path) => {
            println!("writing specification in {}", path.display());
            let mut file = match path.try_exists() {
                Ok(true) => File::create(path)?,
                Ok(false) => OpenOptions::new().create_new(true).write(true).open(path)?,
                Err(e) => {
                    eprintln!("ERROR: unable to validate that path [{}] exists: {e}", path.display());
                    exit(1);
                }
            };

            let serialized = serde_json::to_string(&schema)?;
            write!(file, "{serialized}")?;
            println!("wrote specification in {}", path.display());
        }

        None => {
            println!("{}", serde_json::to_string(&schema)?);
        }
    }

    Ok(())
}

fn openapi(path: Option<PathBuf>) -> eyre::Result<()> {
    let mut doc = document();
    doc.servers = Some(vec![ServerBuilder::new()
        .description(Some("Official home for charted-server, Noelware's public instance"))
        .url("https://charts.noelware.org")
        .build()]);

    let serialized = serde_json::to_string(&doc)?;
    match path {
        Some(ref path) => {
            println!("writing specification in {}", path.display());
            let mut file = match path.try_exists() {
                Ok(true) => File::create(path)?,
                Ok(false) => OpenOptions::new().create_new(true).write(true).open(path)?,
                Err(e) => {
                    eprintln!("ERROR: unable to validate that path [{}] exists: {e}", path.display());
                    exit(1);
                }
            };

            write!(file, "{serialized}")?;
            println!("wrote specification in {}", path.display());

            Ok(())
        }

        None => {
            println!("{serialized}");
            Ok(())
        }
    }
}
