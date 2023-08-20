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

#[macro_use]
extern crate tracing;

mod bootstrap;
mod server;
mod version;

pub(crate) mod extract;
pub(crate) mod middleware;
pub(crate) mod models;
pub mod openapi;
pub mod routing;
pub mod validation;

use charted_config::Config;
use eyre::Result;
pub use server::*;
pub use version::*;

pub async fn bootstrap(config: &Config) -> Result<()> {
    for phase in bootstrap::PHASES.iter() {
        tracing::debug!("entering phase {:?}", phase.clone());
        phase.bootstrap(config).await?;
    }

    Ok(())
}
