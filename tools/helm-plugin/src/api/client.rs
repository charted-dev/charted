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

use reqwest::{Client as RClient, ClientBuilder};

/// Represents an API client that connects to a `charted-server` instance to do API-related things. >:3
#[derive(Debug, Clone)]
pub struct Client<'a> {
    base_url: &'a str,
    inner: RClient
}

impl<'a> Client<'a> {
    pub fn new(server_url: &'a str) -> Client<'a> {
        Client {
            base_url: server_url,
            inner: RClient::new()
        }
    }

    #[allow(clippy::all)]
    pub(crate) fn request<E: AsRef<str>>(&self, endpoint: E) {}
}
