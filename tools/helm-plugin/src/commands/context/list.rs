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

use crate::{auth::AuthType, CommonHelmArgs};
use ansi_term::Style;
use charted_common::{cli::AsyncExecute, hashmap};
use eyre::Result;
use serde_json::{json, Value};
use std::path::PathBuf;

/// Lists all the contexts available.
#[derive(Debug, Clone, clap::Parser)]
pub struct List {
    /// Location to a `auth.yaml` file that can be used to look up
    /// any additional contexts.
    #[arg(long = "context", env = "CHARTED_HELM_CONTEXT_FILE")]
    context_file: Option<PathBuf>,

    /// If the output of the context file should be printed in JSON. This will
    /// omit all keys when being printed.
    #[arg(short = 'j', long)]
    json: bool,
}

#[async_trait]
impl AsyncExecute for List {
    async fn execute(&self) -> Result<()> {
        let context = CommonHelmArgs::auth(self.context_file.clone()).await?;

        if self.json {
            let mut h = hashmap!(String, Value);
            for (name, config) in context.context.iter() {
                h.insert(
                    name.to_string(),
                    json!({
                        "is_current": context.current == *name,
                        "registries": config.iter().map(|reg| Value::String(reg.registry.to_string())).collect::<Vec<_>>()
                    }),
                );
            }

            println!("{}", serde_json::to_string(&h).unwrap());
            return Ok(());
        }

        println!("=== ~ all available contexts ~ ===");
        for (name, config) in context.context.iter() {
            println!(
                "   * {name}{}",
                match context.current == *name {
                    true => " (current)",
                    false => "",
                },
            );

            for registry in config.iter() {
                println!(
                    "      -> registry {} [{}]",
                    registry.registry.clone(),
                    Style::new().bold().paint(match registry.auth.clone() {
                        AuthType::EnvironmentVariable(_) => "environment variable auth",
                        AuthType::SessionToken { .. } => "session-based auth",
                        AuthType::ApiKey(_) => "API Key auth",
                        AuthType::Basic(_) => "basic auth",
                        AuthType::None => "no auth",
                    })
                );
            }
        }

        Ok(())
    }
}
