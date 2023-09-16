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
mod multipart;
mod server;
mod version;

pub(crate) mod extract;
pub mod metrics;
pub(crate) mod middleware;
pub(crate) mod models;
pub mod openapi;
pub(crate) mod pagination;
pub mod routing;
pub mod validation;

use charted_config::Config;
use eyre::Result;
pub use multipart::*;
pub use server::*;
pub use version::*;

/// Represents the distribution powered by [`rust-embed`]. This is configured
/// via the `--cfg 'bundle_web'` rustc flag that requires the fully built
/// distribution in `server/dist`.
///
/// You can check if it is available with the [`WebDist::available`] method:
///
/// ```no_run
/// # use charted_server::WebDist;
/// #
/// let available = WebDist::available();
/// ```
///
/// [`rust-embed`]: https://docs.rs/rust-embed/*/rust_embed
#[derive(Debug, Clone)]
#[cfg_attr(bundle_web, derive(::rust_embed::RustEmbed))]
#[cfg_attr(bundle_web, folder = "dist")]
pub struct WebDist;
impl WebDist {
    /// Checks if the web distribution is available or not.
    pub fn available() -> bool {
        cfg!(bundle_web)
    }
}

/// Runs all the bootstrap phases.
pub async fn bootstrap(config: &Config) -> Result<()> {
    for phase in bootstrap::PHASES.iter() {
        tracing::debug!("entering phase {:?}", phase.clone());
        phase.bootstrap(config).await?;
    }

    Ok(())
}
