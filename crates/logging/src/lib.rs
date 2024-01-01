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

use ansi_term::Colour::RGB;
use std::{
    fmt::Debug,
    io::{stdout, Result, Write},
};
use tracing::field::{Field, Visit};

pub mod generic;
mod json;
pub mod logstash;
pub mod server;

static GRAY: ansi_term::Colour = RGB(134, 134, 134);

/// Represents a default visitor to pretty-print messages and fields
/// with the same style for the Helm plugin, CLI, and server output.
pub struct DefaultVisitor {
    writer: Box<dyn Write + Send>,
    result: Result<()>,
}

impl Default for DefaultVisitor {
    fn default() -> DefaultVisitor {
        DefaultVisitor {
            writer: Box::new(stdout()),
            result: Ok(()),
        }
    }
}

impl Visit for DefaultVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if self.result.is_err() {
            return;
        }

        if field.name().starts_with("log.") {
            return;
        }

        match field.name() {
            "message" => {
                self.result = write!(self.writer, "{value:?}");
            }

            "summary" => {
                let formatted = format!("{value:?}").replacen('"', "", 2);
                self.result = write!(self.writer, "{formatted}");
            }

            "db.statement" => {
                // trims the first \n\n and "
                let value = format!("{value:?}").replace('\n', "").replace("\\n", " ");
                if value.as_str() != "\"\"" {
                    self.result = write!(
                        self.writer,
                        " ({})",
                        trim_excess_whitspace(&value).replacen('"', "", 2).trim()
                    );
                }
            }

            field => {
                self.result = write!(self.writer, " {}", GRAY.paint(format!("{}={value:?}", field)));
            }
        }
    }
}

// https://stackoverflow.com/a/71864249
fn trim_excess_whitspace(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for ch in value.split_whitespace() {
        if !result.is_empty() {
            result.push(' ');
        }

        result.push_str(ch);
    }

    result
}
