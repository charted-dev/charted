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

use crate::{
    metrics::{
        registries::{default::Default, prometheus::Prometheus},
        Registry,
    },
    server::models::res::{err, internal_server_error, ok, ErrorCode, Result},
    Instance,
};
use axum::{extract::State, http::StatusCode};
use serde_json::{Map, Value};

// Endpoint to collect raw metrics from this instance.
pub async fn stats(State(Instance { metrics, .. }): State<Instance>) -> Result<Map<String, Value>> {
    // We first need to check if downcasting to `Prometheus` is successful since it contains
    // an inner registry that can be used for this specific reason.
    let registry: Option<&Default> = if let Some(prometheus) = metrics.downcast::<Prometheus>() {
        let inner = prometheus.inner();
        inner.downcast()
    } else {
        metrics.downcast()
    };

    if let Some(metrics) = registry {
        let mut data = Map::new();
        for collector in metrics.collectors() {
            let name = collector.name();
            data[name] = serde_json::to_value(collector.collect_serialized())
                .inspect_err(|e| {
                    error!(collector.name = name, "failed to collect metrics from collector");
                    sentry::capture_error(&e);
                })
                .map_err(|_| internal_server_error())?;
        }

        return Ok(ok(StatusCode::OK, data));
    }

    Err(err(StatusCode::NOT_IMPLEMENTED, (ErrorCode::HandlerNotFound, "/admin/stats requires to have either `config.metrics.enabled` enabled or both `config.metrics.enabled` and `config.metrics.prometheus` before using this!")))
}
