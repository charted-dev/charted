// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

use crate::compile_regex;
use ansi_term::Colour::RGB;
use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
use once_cell::sync::Lazy;
use regex::Regex;
use std::env::var;

static BOOL_MATCHER_REGEX: Lazy<Regex> =
    Lazy::new(|| compile_regex!("^(true|1|si*|y(es|is|us)?)$"));

static IS_COLORS_DISABLED: Lazy<bool> = Lazy::new(|| match var("DISABLE_COLOURS") {
    Ok(e) => BOOL_MATCHER_REGEX.is_match(e.as_str()),
    _ => false,
});

/// This function will setup logging available to the whole app via the [`log`] crate.
pub fn setup(verbose: bool, level: Option<LevelFilter>) -> Result<(), Box<dyn std::error::Error>> {
    let actual_level = match level {
        Some(l) => l,
        None => LevelFilter::Info,
    };

    let dispatch = Dispatch::new()
        .format(move |out, message, record| {
            if verbose {
                if *IS_COLORS_DISABLED {
                    out.finish(format_args!(
                        "[{}] [{:<5} ({})] - {}",
                        Local::now().format("[%B %d, %G | %H:%M:%S %p]"),
                        record.level(),
                        record.target(),
                        message
                    ));
                } else {
                    let color = match record.level() {
                        log::Level::Error => RGB(153, 75, 104).bold(),
                        log::Level::Debug => RGB(163, 182, 138).bold(),
                        log::Level::Info => RGB(178, 157, 243).bold(),
                        log::Level::Trace => RGB(163, 182, 138).bold(),
                        log::Level::Warn => RGB(243, 243, 134).bold(),
                    };

                    let (b1, b2) = (RGB(134, 134, 134).paint("["), RGB(134, 134, 134).paint("]"));
                    let time = RGB(134, 134, 134).paint(format!(
                        "{}",
                        Local::now().format("%B %d, %G @ %H:%M:%S %p")
                    ));

                    out.finish(format_args!(
                        "{b1}{time}{b2} {b1}{:<5} ({}){b2} {}",
                        color.paint(record.level().as_str()),
                        record.target(),
                        message
                    ));
                }
            } else if *IS_COLORS_DISABLED {
                out.finish(format_args!("[{:<5}] {}", record.level(), message));
            } else {
                let color = match record.level() {
                    log::Level::Error => RGB(153, 75, 104).bold(),
                    log::Level::Debug => RGB(163, 182, 138).bold(),
                    log::Level::Info => RGB(178, 157, 243).bold(),
                    log::Level::Trace => RGB(163, 182, 138).bold(),
                    log::Level::Warn => RGB(243, 243, 134).bold(),
                };

                let (b1, b2) = (RGB(134, 134, 134).paint("["), RGB(134, 134, 134).paint("]"));
                out.finish(format_args!(
                    "{b1}{:<5}{b2} {}",
                    color.paint(record.level().as_str()),
                    message
                ));
            }
        })
        .level(actual_level)
        .level_for("want", LevelFilter::Off)
        .level_for("mio::poll", LevelFilter::Off)
        .level_for("reqwest::connect", LevelFilter::Off)
        .chain(std::io::stdout());

    dispatch.apply()?;
    Ok(())
}
