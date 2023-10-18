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

use ansi_term::Colour;
use std::io::{stdout, Write as _};
use tracing::{Event, Level, Subscriber};
use tracing_log::NormalizeEvent;
use tracing_subscriber::{layer::Context, Layer};

use crate::DefaultVisitor;

static FILE_LINE: Colour = Colour::RGB(255, 105, 189);

/// Represents a generic [`Layer`] that is used not on the server level.
pub struct GenericLayer {
    pub verbose: bool,
}

impl<S: Subscriber> Layer<S> for GenericLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut writer = stdout();
        let metadata = event.normalized_metadata();
        let metadata = metadata.as_ref().unwrap_or_else(|| event.metadata());
        let level_color = match *metadata.level() {
            Level::DEBUG => Colour::RGB(163, 182, 138).bold(),
            Level::TRACE => Colour::RGB(163, 182, 138).bold(),
            Level::ERROR => Colour::RGB(153, 75, 104).bold(),
            Level::WARN => Colour::RGB(243, 243, 134).bold(),
            Level::INFO => Colour::RGB(178, 157, 243).bold(),
        };

        if self.verbose {
            let _ = write!(
                writer,
                "{level} {}   ",
                FILE_LINE.paint(format!(
                    "in {}:{}",
                    metadata.file().unwrap_or("<unknown file>"),
                    metadata.line().unwrap_or(0)
                )),
                level = level_color.paint(format!("{:<5}", metadata.level().as_str()))
            );
        } else {
            let _ = write!(
                writer,
                "{level}   ",
                level = level_color.paint(format!("{:<5}", metadata.level().as_str()))
            );
        }

        let mut visitor = DefaultVisitor::default();
        event.record(&mut visitor);

        let _ = writeln!(writer);
    }
}
