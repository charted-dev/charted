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

pub mod auditlog;
pub mod gc;
pub mod oci;
pub mod totp;
pub mod webhooks;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Config {
    /// Enables the garbage collection feature.
    ///
    /// The **garbage collection** feature allows the API server to collect objects
    /// from the database and delete objects based off conditions that are defined
    /// in the configuration.
    #[serde(rename = "gc")]
    GarbageCollection,

    /// Enables the audit logging feature.
    ///
    /// **NOTE** — This will require a configured [ClickHouse] connection as all
    /// audit logs will be written to ClickHouse.
    ///
    /// The **audit logging** feature allows introspection of API server requests
    /// from authenticated users, repository members, and organization members.
    ///
    /// [ClickHouse]: https://clickhouse.com
    AuditLogging,

    /// Enables the HTTP webhooks feature.
    ///
    /// **NOTE** — This will require a configured [ClickHouse] connection as all
    /// audit logs will be written to ClickHouse.
    ///
    /// The **webhooks** feature implements the [HTTP Standard Webhooks Specification] to allow
    /// receiving events from data that has been modified by the API server.
    ///
    /// [HTTP Standard Webhooks Specification]: https://github.com/standard-webhooks/standard-webhooks/blob/main/spec/standard-webhooks.md
    /// [ClickHouse]: https://clickhouse.com
    Webhooks,

    /// Enables the TOTP (time-based one time password) feature.
    ///
    /// This feature is similar to two-factor authentication.
    Totp,

    /// Enables the OCI Registry feature.
    ///
    /// **NOTE** — If this feature is enabled, you can still use the REST API to download
    /// Helm charts as well. The Helm plugin supports both OCI and REST API.
    ///
    /// This feature will ensure that **charted-server** will act like a OCI Registry
    /// based off the [Registry v1.1.0 Specification]. Techincally, if this feature
    /// is enabled, **charted-server** can host both Helm charts and Docker containers.
    ///
    /// [Registry v1.1.0 Specification]: https://github.com/opencontainers/distribution-spec/blob/v1.1.0/spec.md
    OCI,
}
