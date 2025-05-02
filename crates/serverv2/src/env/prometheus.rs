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

use crate::env::shutdown_signal;
use axum::{Extension, Router};
use charted_config::metrics;
use charted_core::ResultExt;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn start(config: &metrics::Config) -> eyre::Result<()> {
    let metrics::Config::Prometheus(config) = config else {
        return Ok(());
    };

    let Some(ref standalone) = config.standalone else {
        return Ok(());
    };

    let addr = format!("{}:{}", standalone.host, standalone.port).parse::<SocketAddr>()?;

    info!(
        target: "charted_server",
        "starting standalone Prometheus HTTP scrape endpoint at address {}",
        addr
    );

    let router = Router::new()
        //.route("/", axum::routing::get(routing::v1::prometheus_scrape))
        .layer(Extension(()));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(None))
        .await
        .into_report()
}
