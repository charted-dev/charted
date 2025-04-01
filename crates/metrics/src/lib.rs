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

use charted_core::ResultExt;
use eyre::bail;
pub use metrics::*;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use opentelemetry::KeyValue;

pub fn init_prometheus(config: &charted_config::metrics::prometheus::Config) -> eyre::Result<PrometheusHandle> {
    let handle = PrometheusBuilder::new()
        .set_bucket_duration(*config.bucket_duration)?
        .install_recorder()?;

    tokio::spawn({
        let handle = handle.clone();
        let upkeep_interval = config.upkeep_interval;

        async move {
            handle.run_upkeep();

            tokio::select! {
                _ = tokio::time::sleep(*upkeep_interval) => {
                    handle.run_upkeep();
                }
            }
        }
    });

    Ok(handle)
}

pub fn init_opentelemetry(
    config: &charted_config::metrics::opentelemetry::Config,
) -> eyre::Result<metrics_exporter_opentelemetry::Recorder> {
    let builder = metrics_exporter_opentelemetry::Recorder::builder("charted-server").with_instrumentation_scope(
        |builder| {
            builder.with_version(charted_core::version()).with_attributes({
                let mut attrs = config
                    .labels
                    .iter()
                    .map(|(key, value)| KeyValue::new(key.clone(), value.clone()))
                    .collect::<Vec<_>>();

                attrs.push(KeyValue::new("service.name", "charted-server"));
                attrs.push(KeyValue::new("service.vendor", "Noelware, LLC."));
                attrs.push(KeyValue::new("charted.version", charted_core::version()));

                attrs
            })
        },
    );

    let exporter = match config.url.scheme() {
        "http" | "https" => opentelemetry_otlp::MetricExporter::builder().with_tonic().build()?,
        "grpc" | "grpcs" => opentelemetry_otlp::MetricExporter::builder().with_http().build()?,
        scheme => bail!("unexpected URL scheme: {}", scheme),
    };

    builder
        .with_meter_provider(|b| b.with_periodic_exporter(exporter))
        .install_global()
        .into_report()
}
