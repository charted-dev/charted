// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use comfy_table::{presets, Attribute, Cell, ContentArrangement, Row, Table};
use serde_json::{json, Map, Value};

use crate::{
    args,
    auth::{Auth, Type},
};

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(flatten)]
    auth: args::Auth,

    /// exports it as json
    #[arg(long, short = 'j')]
    json: bool,
}

pub fn run(
    Args {
        json,
        auth: args::Auth { path },
    }: Args,
) -> eyre::Result<()> {
    let auth = Auth::load(path.as_ref())?;
    if json {
        let mut obj = Map::<String, Value>::with_capacity(auth.contexts.len());
        for (name, cx) in &auth.contexts {
            obj.insert(
                name.clone(),
                json!({
                    "current": auth.current == *name,
                    "registry": cx.registry,
                    "kind": match cx.kind {
                        Type::EnvironmentVariable { ..} => "EnvironmentVariable",
                        Type::ApiKey(_) => "ApiKey",
                        Type::Basic { .. } => "Basic",
                        Type::None => "None"
                    }
                }),
            );
        }

        let data = serde_json::to_string_pretty(&Value::Object(obj))?;
        println!("{data}");

        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(presets::ASCII_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Current").add_attribute(Attribute::Bold),
            Cell::new("Registry").add_attribute(Attribute::Bold),
            Cell::new("Kind").add_attribute(Attribute::Bold),
        ]);

    for (name, cx) in &auth.contexts {
        let mut row = Row::new();
        let current = if auth.current == *name { "*" } else { "" };

        row.add_cell(Cell::new(current).add_attribute(Attribute::Bold));
        row.add_cell(Cell::new(&cx.registry));
        row.add_cell(Cell::new(match cx.kind {
            Type::EnvironmentVariable { ref name, .. } => format!("Environment Variable ${name}"),
            Type::ApiKey(_) => "API Key".into(),
            Type::Basic { .. } => "Basic Auth".into(),
            Type::None => "None".into(),
        }));

        table.add_row(row);
    }

    println!("{table}");
    Ok(())
}
