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

use std::sync::Arc;
use testcontainers::clients::Cli;

/// Represents a context that should be re-used through out
/// tests, and can be used with the `#[testcontainers]` macro
/// via the `configure` proc-macro attribute.
///
/// ## Example
/// ```no_run
/// # use charted_testcontainers::{testcontainers, Context};
/// #
/// # #[cfg(test)]
/// # mod tests {
/// # use charted_testcontainers::{testcontainers, Context};
/// #
/// async fn configure(ctx: Context) {
///     let ctx = Context::default();
///     if !ctx.is_enabled {
///         return;
///     }
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Context {
    pub enabled: bool,
    docker_client: Arc<Option<Cli>>,
}

impl Context {
    pub fn new(client: Option<Cli>) -> Context {
        Context {
            enabled: client.is_some(),
            docker_client: Arc::new(client),
        }
    }

    pub fn cli(&self) -> Arc<Option<Cli>> {
        Arc::clone(&self.docker_client)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
