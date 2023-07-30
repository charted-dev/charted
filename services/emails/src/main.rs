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

use charted_common::{COMMIT_HASH, RUSTC_VERSION, VERSION};
use charted_config::var;
use charted_emails::{config::Config, protos::emails_server::EmailsServer, service::Service};
use charted_logging::server::ServerLayer;
use eyre::Result;
use sentry::{
    init,
    integrations::{backtrace::AttachStacktraceIntegration, panic::PanicIntegration},
    types::Dsn,
    ClientOptions,
};
use sentry_tower::NewSentryLayer;
use std::{borrow::Cow, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};
use tonic::transport::Server;
use tonic_health::server::health_reporter;
use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::{prelude::*, registry};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().unwrap_or_default();
    Config::load(var!("EMAILS_CONFIG_FILE", to: PathBuf, is_optional: true))?;

    let config = Config::get();
    let _guard = config.sentry_dsn()?.map(|dsn| {
        init(ClientOptions {
            dsn: Some(Dsn::from_str(dsn.as_str()).expect("unable to parse Sentry DSN")),
            release: Some(Cow::Owned(format!("{VERSION}+{COMMIT_HASH}"))),
            traces_sample_rate: 1.0,
            attach_stacktrace: true,
            integrations: vec![Arc::new(AttachStacktraceIntegration), Arc::new(PanicIntegration::new())],
            ..Default::default()
        })
    });

    registry()
        .with(
            ServerLayer {
                json: config.logging.json_logging,
            }
            .with_filter(LevelFilter::from_level(config.logging.level)),
        )
        .with(config.sentry_dsn()?.map(|_| sentry_tracing::layer()))
        .init();

    info!(
        version = VERSION,
        commit = COMMIT_HASH,
        rustc = RUSTC_VERSION,
        "starting email service!"
    );

    let (mut reporter, health_service) = health_reporter();
    reporter.set_serving::<EmailsServer<Service>>().await;

    let addr = format!("{}:{}", config.server.host, config.server.port).parse::<SocketAddr>()?;
    info!(%addr, "now listening with");

    let service = Service::new().await?;
    Server::builder()
        .layer(NewSentryLayer::new_from_top())
        .add_service(health_service)
        .add_service(EmailsServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
