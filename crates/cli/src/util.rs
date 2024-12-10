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

use charted_config::Config;
use std::path::PathBuf;

pub fn load_config(config: Option<PathBuf>) -> eyre::Result<Config> {
    config
        .map(|path| Config::new(Some(path)))
        .unwrap_or(match Config::get_default_conf_location_if_any() {
            Ok(Some(path)) => Config::new(Some(path)),
            _ => Config::new::<&str>(None),
        })
}
