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

use azalia::log::{WriteLayer, writers::default::Writer};
use std::{io, time::Duration};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ContainerPort, WaitFor, logs::consumer::logging_consumer::LoggingConsumer},
    runners::AsyncRunner,
};
use tracing::subscriber::DefaultGuard;
use tracing_subscriber::{EnvFilter, prelude::*};

// renovate: datasource=docker image=minio/minio
pub const MINIO_TAG: &str = "RELEASE.2025-04-22T22-12-26Z";
pub const MINIO: &str = "minio/minio";

// renovate: datasource=docker image=mcr.microsoft.com/azure-storage/azurite
pub const AZURITE_TAG: &str = "3.34.0";
pub const AZURITE: &str = "mcr.microsoft.com/azure-storage/azurite";

// Output of the returned futures for `azurite` and `minio`.
type Output = testcontainers::core::error::Result<ContainerAsync<GenericImage>>;

/// Starts a Azurite emulator container.
pub fn azurite() -> impl Future<Output = Output> + Send {
    GenericImage::new(AZURITE, AZURITE_TAG)
        // Azurite runs on port :10000, :10001, :10002
        // https://learn.microsoft.com/en-us/azure/storage/common/storage-use-azurite?tabs=docker-hub%2Cblob-storage#run-azurite
        .with_exposed_port(ContainerPort::Tcp(10000))
        // Only run the Blob Storage emulator, since that's what we're only testing.
        .with_cmd(["azurite-blob", "--blobHost", "0.0.0.0", "--blobPort", "10000", "--inMemoryPersistence", "--disableTelemetry"])
        // Allow viewing the logs of the container
        .with_log_consumer(LoggingConsumer::new())
        .with_network("bridge")
        .start()
}

/// Starts a MinIO container.
pub fn minio() -> impl Future<Output = Output> + Send {
    GenericImage::new(MINIO, MINIO_TAG)
        // Minio runs on :9000 for the main service
        .with_exposed_port(ContainerPort::Tcp(9000))
        // wait 5 seconds for minio to initialize properly
        .with_wait_for(WaitFor::Duration { length: Duration::from_secs(5) })
        .with_network("bridge")
        // Allow viewing the logs of the container
        .with_log_consumer(LoggingConsumer::new())
        .with_env_var("MINIO_ACCESS_KEY", "somedummyaccesskey")
        .with_env_var("MINIO_SECRET_KEY", "somedummysecretkey")
        .with_cmd(["server", "/data", "--address=0.0.0.0:9000"])
        .start()
}

pub fn setup_tracing() -> DefaultGuard {
    tracing_subscriber::registry()
        .with(WriteLayer::new_with(
            io::stderr(),
            Writer::default().with_thread_name(false),
        ))
        .with(EnvFilter::from_env("INTEGTEST_LOG"))
        .set_default()
}
