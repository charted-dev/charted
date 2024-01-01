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

use super::BootstrapPhase;
use async_trait::async_trait;
use charted_config::{Config, ConfigExt};
use charted_logging::{logstash::LogstashLayer, server::ServerLayer};
use eyre::Result;
use sentry_tracing::SentryLayer;
use std::net::TcpStream;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{prelude::*, registry};

#[derive(Debug, Clone)]
pub struct SetupLoggingPhase;

#[async_trait]
impl BootstrapPhase for SetupLoggingPhase {
    async fn bootstrap(&self, config: &Config) -> Result<()> {
        registry()
            .with(ServerLayer::default().with_filter(LevelFilter::from_level(config.logging.level)))
            .with(config.sentry_dsn().ok().and_then(|x| x.map(|_| SentryLayer::default())))
            .with(config.logging.logstash_connect_uri.as_ref().map(|uri| {
                let stream = TcpStream::connect(uri)
                    .unwrap_or_else(|e| panic!("unable to connect to a TCP stream at address {uri}: {e}"));

                LogstashLayer::new(stream)
            }))
            .try_init()?;

        Ok(())
    }

    fn try_clone(&self) -> eyre::Result<Box<dyn BootstrapPhase>> {
        Ok(Box::new(self.clone()))
    }
}
