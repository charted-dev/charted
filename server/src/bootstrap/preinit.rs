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

use super::BootstrapPhase;
use charted_common::{BUILD_DATE, COMMIT_HASH, RUSTC_VERSION, VERSION};
use charted_config::Config;
use chrono::DateTime;
use eyre::Result;
use std::{future::Future, pin::Pin};

#[derive(Debug, Clone)]
pub struct PreinitPhase;

impl BootstrapPhase for PreinitPhase {
    fn bootstrap<'l0, 'async_method>(
        &'l0 self,
        _config: &'async_method Config,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_method>>
    where
        'l0: 'async_method,
        Self: 'async_method,
    {
        Box::pin(async move {
            let date = DateTime::parse_from_rfc3339(BUILD_DATE)
                .unwrap()
                .format("%a, %h %d, %Y at %H:%M:%S %Z")
                .to_string();

            info!("Starting up charted-server v{VERSION}+{COMMIT_HASH} ({date})");
            info!("~> Compiled with Rust v{RUSTC_VERSION}");

            Ok(())
        })
    }

    fn try_clone(&self) -> eyre::Result<Box<dyn BootstrapPhase>> {
        Ok(Box::new(self.clone()))
    }
}