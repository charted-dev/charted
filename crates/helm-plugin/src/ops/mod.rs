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

//! The `charted_helm_plugin::ops` module contains all the operations.

mod common;
mod download;
mod middleware;

use std::sync::LazyLock;

pub static HTTP: LazyLock<reqwest_middleware::ClientWithMiddleware> = LazyLock::new(|| {
    let reqwest = ::reqwest::ClientBuilder::new()
        .user_agent(format!(
            "Noelware/charted-helm-plugin (+{}; https://github.com/charted-dev/charted/tree/main/crates/helm-plugin)",
            charted_core::version()
        ))
        .build()
        .unwrap();

    reqwest_middleware::ClientBuilder::new(reqwest)
        .with(middleware::logging)
        .build()
});
