// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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
use anyhow::Result;
use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;

pub fn setup_logging(level: LevelFilter, colors: bool) -> Result<()> {
    let dispatch = Dispatch::new()
        .format(move |out, message, record| {
            if colors {
                let color = match record.level() {
                    log::Level::Error => RGB(153, 75, 104).bold(),
                    log::Level::Debug => RGB(163, 182, 138).bold(),
                    log::Level::Info => RGB(178, 157, 243).bold(),
                    log::Level::Trace => RGB(163, 182, 138).bold(),
                    log::Level::Warn => RGB(243, 243, 134).bold(),
                };

                let time = RGB(134, 134, 134).paint(format!(
                    "{}",
                    Local::now().format("[%B %d, %G | %H:%M:%S %p]")
                ));

                let level = color.paint(format!("{:<5}", record.level()));
                out.finish(format_args!("{} {:<5} :: {}", time, level, message));
            } else {
                out.finish(format_args!(
                    "{} {:<5} :: {}",
                    Local::now().format("[%B %d, %G | %H:%M:%S %p]"),
                    record.level(),
                    message
                ));
            }
        })
        .chain(std::io::stdout())
        .level(level);

    dispatch.apply()?;
    Ok(())
}
