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

use crate::{json::JsonVisitor, server::JsonStorage};
use chrono::Local;
use serde_json::{json, Value};
use std::{collections::BTreeMap, io::Write, net::TcpStream, process, thread};
use tracing::{span, Subscriber};
use tracing_log::NormalizeEvent;
use tracing_subscriber::{registry::LookupSpan, Layer};

pub struct LogstashLayer {
    stream: TcpStream,
}

impl LogstashLayer {
    pub fn new(stream: TcpStream) -> LogstashLayer {
        LogstashLayer { stream }
    }
}

impl<S> Layer<S> for LogstashLayer
where
    S: Subscriber + for<'l> LookupSpan<'l>,
{
    // on new span, we create a `JsonStorage` to record each span data into a `BTreeMap`, which
    // will be used as the `spans` array.
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let ctx = ctx.clone();
        let span = ctx.span(id).unwrap();
        let mut data = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut data);
        attrs.record(&mut visitor);

        let storage = JsonStorage(data);
        span.extensions_mut().insert(storage);
    }

    fn on_record(&self, span: &span::Id, values: &span::Record<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(span).unwrap();
        let mut extensions_mut = span.extensions_mut();
        let storage: &mut JsonStorage = extensions_mut.get_mut::<JsonStorage>().unwrap();
        let mut visitor = JsonVisitor(&mut storage.0);

        values.record(&mut visitor);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let thread = thread::current();
        let pid = process::id();
        let metadata = event.normalized_metadata();
        let metadata = metadata.as_ref().unwrap_or_else(|| event.metadata());

        // first, we need to get all span metadata
        let mut spans = vec![];
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let ext = span.extensions();
                let storage = ext.get::<JsonStorage>().unwrap();
                let data = &storage.0;

                spans.push(json!({
                    // show `null` if there are no fields available
                    "fields": match data.is_empty() {
                        true => None,
                        false => Some(data)
                    },

                    "target": span.metadata().target(),
                    "level": metadata.level().as_str().to_lowercase(),
                    "name": span.metadata().name(),
                    "meta": json!({
                        "module": span.metadata().module_path(),
                        "file": span.metadata().file(),
                        "line": span.metadata().line(),
                    })
                }));
            }
        }

        let mut tree = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut tree);
        event.record(&mut visitor);

        let message = tree
            .remove("message")
            .unwrap_or(Value::String(String::from("{none provided}")));

        let mut stream = &self.stream; // cheap hack that probably works (i dont know lmao)
        let timestamp = Local::now();
        let _ = stream.write(
            serde_json::to_vec(&json!({
                // common ecs fields that Elastic-senpai wants~! uwu!~
                "@timestamp": timestamp.to_rfc3339(),
                "message": message,
                "labels": json!({
                    "service": "charted-server",
                    "vendor": "Noelware, LLC."
                }),

                // not in the format that Elastic-senpai wants
                "thread.name": thread.name().unwrap_or("main"),
                "process.id": pid,
                "fields": match tree.is_empty() {
                    true => None,
                    false => Some(tree),
                },

                "spans": spans,
                "meta": json!({
                    "module": metadata.module_path(),
                    "file": metadata.file(),
                    "line": metadata.line(),
                })
            }))
            .unwrap()
            .as_ref(),
        );

        // write new-line after
        let _ = writeln!(stream);
    }
}
