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

mod preinit;
mod setup_logging;
mod start_server;

use self::{preinit::PreinitPhase, setup_logging::SetupLoggingPhase, start_server::StartServerPhase};
use charted_config::Config;
use eyre::Result;
use once_cell::sync::Lazy;
use std::{future::Future, pin::Pin};

pub static PHASES: Lazy<Vec<Box<dyn BootstrapPhase + 'static>>> = Lazy::new(|| {
    vec![
        Box::new(SetupLoggingPhase),
        Box::new(PreinitPhase),
        Box::new(StartServerPhase),
    ]
});

/// Represents a bootstrap phase that runs in chronological order
/// that tells the server how to run a specific "phase"
///
/// ## Order
/// - SetupLogging
/// - Preinit
/// - StartServer
pub trait BootstrapPhase: Send + Sync {
    /// Method to actually run this bootstrap phase.
    fn bootstrap<'l0, 'async_method>(
        &'l0 self,
        config: &'async_method Config,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_method>>
    where
        'l0: 'async_method,
        Self: 'async_method;

    // We can't implement Clone into BootstrapPhase, so we will have to do this
    // "try_clone" method to do so.
    fn try_clone(&self) -> Result<Box<dyn BootstrapPhase>>;
}

impl Clone for Box<dyn BootstrapPhase> {
    fn clone(&self) -> Box<dyn BootstrapPhase> {
        self.try_clone().expect("Unable to clone this bootstrap phase.")
    }
}
