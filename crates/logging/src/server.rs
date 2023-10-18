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

use crate::{json::JsonVisitor, DefaultVisitor};
use ansi_term::Colour;
use charted_config::Config;
use chrono::Local;
use serde_json::{json, to_string, Value};
use std::{collections::BTreeMap, io, io::Write as _, process, thread};
use tracing::{
    span::{Attributes, Record},
    Event, Id, Level, Subscriber,
};
use tracing_log::NormalizeEvent;
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

static GRAY: Colour = Colour::RGB(134, 134, 134);

pub(crate) struct JsonStorage(pub BTreeMap<String, Value>);

pub struct ServerLayer {
    pub json: bool,
}

impl Default for ServerLayer {
    fn default() -> Self {
        let config = Config::get();
        ServerLayer {
            json: config.logging.json_logging,
        }
    }
}

impl<S> Layer<S> for ServerLayer
where
    S: Subscriber,
    S: for<'l> LookupSpan<'l>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        if self.json {
            let curr_ctx = ctx.clone();
            let span = curr_ctx.span(id).unwrap();
            let mut data = BTreeMap::new();
            let mut visitor = JsonVisitor(&mut data);
            attrs.record(&mut visitor);

            let storage = JsonStorage(data);
            span.extensions_mut().insert(storage);
        }
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        if self.json {
            let span = ctx.span(span).unwrap();
            let mut exts = span.extensions_mut();

            let storage: &mut JsonStorage = exts.get_mut::<JsonStorage>().unwrap();
            let mut visitor = JsonVisitor(&mut storage.0);

            values.record(&mut visitor);
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Collect common variables that we will use in our logging.
        let pid = process::id();
        let thread = thread::current();
        let metadata = event.normalized_metadata();
        let metadata = metadata.as_ref().unwrap_or_else(|| event.metadata());

        if self.json {
            let mut spans = vec![];
            if let Some(scope) = ctx.event_scope(event) {
                for span in scope.from_root() {
                    let ext = span.extensions();
                    let storage = ext.get::<JsonStorage>().unwrap();
                    let data = &storage.0;

                    spans.push(json!({
                        "target": span.metadata().target(),
                        "level": metadata.level().as_str().to_lowercase(),
                        "name": span.metadata().name(),
                        "fields": data,
                        "meta": json!({
                            "module": span.metadata().module_path(),
                            "file": span.metadata().file(),
                            "line": span.metadata().line()
                        })
                    }));
                }
            }

            let mut data = BTreeMap::new();
            let mut visitor = JsonVisitor(&mut data);
            event.record(&mut visitor);

            let default_message = &Value::String("none provided".to_owned());
            let message = data.get("message").unwrap_or(default_message);
            let fields = {
                let mut d = data.clone();
                d.remove_entry("message");

                d
            };

            println!(
                "{}",
                to_string(&json!({
                    "target": metadata.target(),
                    "level": metadata.level().as_str().to_lowercase(),
                    "message": message,
                    "fields": fields,
                    "spans": spans,
                    "meta": json!({
                        "module": metadata.module_path(),
                        "file": metadata.file(),
                        "line": metadata.line(),
                    }),
                    "process": json!({
                        "pid": pid
                    }),
                    "thread": json!({
                        "name": thread.name().unwrap_or("main")
                    })
                }))
                .unwrap()
            );
        } else {
            let mut stdout = io::stdout();
            let time = GRAY.paint(format!("{}", Local::now().format("[%B %d, %G - %H:%M:%S %p]")));
            let thread_name_color = Colour::RGB(255, 105, 189).paint(format!("{:>7}", thread.name().unwrap_or("main")));

            let target =
                Colour::RGB(120, 231, 255).paint(format!("{:<40}", metadata.module_path().unwrap_or("unknown")));

            let (b1, b2) = (GRAY.paint("["), GRAY.paint("]"));
            let level_color = match *metadata.level() {
                Level::DEBUG => Colour::RGB(163, 182, 138).bold(),
                Level::TRACE => Colour::RGB(163, 182, 138).bold(),
                Level::ERROR => Colour::RGB(153, 75, 104).bold(),
                Level::WARN => Colour::RGB(243, 243, 134).bold(),
                Level::INFO => Colour::RGB(178, 157, 243).bold(),
            };

            let _ = write!(
                stdout,
                "{time} {level} {b1}{target} {thread_name_color}{b2} :: ",
                level = level_color.paint(format!("{:<5}", metadata.level().as_str()))
            );

            let mut visitor = DefaultVisitor::default();
            event.record(&mut visitor);

            let _ = writeln!(stdout);
        }
    }
}
