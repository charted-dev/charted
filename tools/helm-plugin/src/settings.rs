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

pub const DEFAULT_SERVER_URL: &str = "https://charts.noelware.org";

/// Represents all the global settings that are available to all commands. Though, this is only
/// contains relevant settings that most commands will use.
#[derive(Debug, Clone)]
pub struct Settings {
    server_url: String,
}

impl Settings {
    pub fn new(server_url: Option<String>) -> Settings {
        Settings {
            server_url: server_url.unwrap_or(DEFAULT_SERVER_URL.to_string()),
        }
    }

    pub fn server(&self) -> &String {
        &self.server_url
    }
}
